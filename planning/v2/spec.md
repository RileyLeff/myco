# Myco V2 Language Specification

This is the working specification for Myco V2. It covers the world-model
language, the mathematical substrate, the compiler internals, and the workflow
layer.

For earlier design exploration that led to this spec, see `v2_prep/`.

For the Sperry model mock implementation that stress-tested this spec, see
`mock_sperry.myco`. For the Potkay GOSM mock that stress-tests carbon-water-
turgor coupling and library reuse, see `mock_potkay.myco`.

For the invariant design principles that guide all decisions, see
`../soul.md`.

## Design Philosophy

The `.myco` representation should approach the minimum description length of
the science. If the implementation complexity vastly exceeds the description
complexity, the gap is incidental complexity that belongs in the compiler, not
in the model.

The model describes what is true about the world. The compiler figures out how
to compute it. The user never annotates solution strategies, solver choices, or
execution order — those are compiler concerns.

The compiler never silently trusts claims it cannot verify. If the compiler
cannot prove a property, it errors with an actionable diagnostic. The user may
explicitly acknowledge unverifiable properties, but silence is never consent.

---

## Part I: The World Model

These sections define what users write in `.myco` files. The world model
describes structure. It does not describe workflow.

### 1. Modules

A `.myco` file begins with a module declaration:

```myco
module plant::sperry
```

Module identity replaces the v1 `model Name` form. The module path provides
namespacing and becomes the basis for imports:

```myco
use plant::sperry::{Leaf, Environment}
use units::si::{megapascal as MPa, mole_per_second as mol_s}
```

Modules may re-export items from other modules.

#### 1.1 Library vs model modules

A **library module** defines reusable components — types, nodes, contracts,
functions, macros. It may contain unresolved generics and `dyn` contract
references (see section 2.5). Library modules are imported by other modules but
are not directly compilable.

A **model module** is a top-level entry point for compilation. All generics must
be resolved to concrete types, all `dyn` references must be specialized, and all
const generics must have values. A model module is what the compiler accepts as
input.

The distinction is analogous to Rust's lib vs bin crates. A library defines
`Ecosystem<const N: usize>` with `pfts: [PFT<dyn Photosynthesis>; N]`. A model
module writes `MySite` with `eco: Ecosystem<3>` and specifies concrete types for
each element.

#### 1.2 Visibility

Items are private by default. The `pub` keyword makes an item visible to
importing modules:

```myco
pub type WaterPotential : Scalar<MPa>
pub node XylemSegment<V: VulnerabilityCurve> { ... }
pub contract VulnerabilityCurve { ... }
pub fn arrhenius(...) -> ... { ... }
```

Fields within a node follow the same rule: private by default, `pub` to expose.
A library author controls exactly what surface area is importable.

**`pub` controls inter-module visibility only.** The workflow layer (Python API)
can bind any path in the model regardless of visibility. `pub` is about library
encapsulation — "don't let other module authors depend on my internals" — not
about hiding quantities from the scientist running the experiment. A field
marked private is invisible to other `.myco` modules but fully accessible to
`assume_constant()`, `learn_constant()`, `observe_dense()`, etc.

#### 1.3 Circular imports

Circular imports are disallowed. The module dependency graph must be a DAG. The
compiler reports a cycle with the full import chain if one is detected.

### 2. Nodes

A **node declaration** (`node Foo { ... }`) defines a reusable structural
schema — analogous to a struct definition. A node becomes an **instance** when
it is declared as a field inside another node or as a module's root node.

The distinction between `type` and `node` is semantic:

- **`type`**: value-level schemas. Scalars, simple composites, anything that is
  pure data without relations or slots. Types may carry constraints over their
  fields.
- **`node`**: entity-level schemas. Nodes can own children (including other
  nodes), define relations, contain slots, and participate in the containment
  tree. A node implementing a contract must be declared with `node`, not `type`.

This is analogous to the struct/instance distinction: if you imagine 100 trees,
they all share the node definition `Tree<V, P>`; each tree is an instance
occupying a unique position in the containment tree.

**Module-scope relations** (relations, temporals, slots declared outside any
node body) are implicitly scoped to the module's root node. A model module must
have exactly one root node. Paths in module-scope relations refer to fields
of that root. For example, if the root is `SperryTree`, a module-scope relation
can reference `soil.layers[j].element.water_potential` without a `SperryTree.`
prefix.

#### 2.1 Atomic nodes

An atomic node owns one typed value:

```myco
node stomata: Conductance
```

#### 2.2 Composite nodes

A composite node owns other nodes and may own atomic values:

```myco
node Leaf {
    water: Potential { self <= 0 MPa }
    stomata: Conductance
    g_max: Conductance
    area: Area
    nsc: NscComposition
    transpiration: WaterFlux
}
```

Inline `{ ... }` blocks on field declarations are syntactic sugar for a named
constraint at the containing node's scope. In this sugar, `self` refers to the
declared field, and sibling field names are in scope. For example,
`stomata: Conductance { self <= g_max }` is equivalent to writing a separate
`constraint stomatal_cap: stomata <= g_max` in the containing node.

#### 2.3 Containment model

Containment is a tree. Each node has exactly one structural parent. This makes
paths unambiguous:

```myco
tree.hydraulics.leaf.water      // always refers to one thing
```

Cross-subtree coupling is expressed via explicit relations at the parent scope
that can see both sides. The constraint graph is a full graph; the containment
tree is only for naming and structural organization.

#### 2.4 Generics

Nodes may be parameterized by types and const values:

```myco
node Canopy<const N: usize, P: Photosynthesis> {
    leaves: [Leaf<P>; N]
}
```

Type parameters must satisfy a declared contract (see section 3.4). Const
parameters must be compile-time-known positive integers.

Nodes may be generic over multiple contracts:

```myco
node ConductingElement<V: VulnerabilityCurve, PV: PressureVolumeCurve> {
    k_max: HydraulicConductance
    vc: V
    pv: PV
    water_potential: WaterPotential
    conductance: HydraulicConductance

    conductance = k_max * (1.0 - vc(water_potential).plc)
}
```

This enables composable subsystem design: roots, stems, and leaves can all be
`ConductingElement<WeibullVC, StandardPV>` with different parameter values.
A storage-capable element might be
`StorageConductingElement<WeibullVC, CapacitivePV>` — same vulnerability curve,
different pressure-volume behavior.

#### 2.5 Heterogeneous collections with `dyn`

Arrays of nodes are normally homogeneous:

```myco
leaves: [Leaf<FarquharC3>; N]      // all leaves use C3 photosynthesis
```

When a collection must contain elements with different contract implementations,
use `dyn`:

```myco
// Library module — ships generic
node Ecosystem<const N: usize> {
    pfts: [PFT<dyn Photosynthesis, dyn VulnerabilityCurve>; N]

    total_lai: PositiveScalar

    constraint total_lai_sum:
        total_lai = sum(pfts[i].lai for i in 0..N)
}
```

`dyn Photosynthesis` means "some type satisfying the `Photosynthesis` contract,
determined later." Library code may only access fields declared in the contract
on `dyn` elements.

The user's model module specializes each element to a concrete type:

```myco
// Model module — fully concrete
node MySite {
    eco: Ecosystem<3> {
        pfts[0]: PFT<FarquharC3, WeibullVC>     // oak tree
        pfts[1]: PFT<C4Photo, SigmoidVC>         // C4 grass
        pfts[2]: PFT<FarquharC3, SigmoidVC>      // shrub
    }
    atmosphere: Atmosphere
    soil: Soil<4>
}
```

After specialization, the compiler monomorphizes each element independently.
`pfts[0]` and `pfts[2]` share the same Photosynthesis type but different VC
types. The flat graph has different quantities for each element, but aggregation
constraints from the library (which only reference contract-declared fields)
remain valid.

**Per-element relations**: In the model module, where concrete types are known,
the user may write relations that use implementation-specific fields:

```myco
// Valid because pfts[0] is known to be FarquharC3
relation oak_rubisco:
    eco.pfts[0].canopy.photo.rubisco_specificity = some_value
```

Accessing `pfts[1].canopy.photo.rubisco_specificity` would be a compile error
because `C4Photo` does not have that field.

**`dyn` is a pre-monomorphization concept.** It exists in the source language
for library authoring. After the user specifies concrete types, `dyn` is
resolved. The flattener never sees it. No runtime dispatch is emitted.

#### 2.6 Repeated structure

Arrays of nodes use fixed-size syntax:

```myco
leaves: [Leaf<P>; N]
layers: [SoilLayer; M]
```

The count is a const generic, a literal, or a const generic expression using
`+`, `-`, `*`:

```myco
interlayer_flow: [TranspirationRate; M-1]    // M soil layers → M-1 interfaces
```

The compiler must be able to prove the result is a positive integer. For
example, `M-1` requires `M >= 2`, which may be inferred from constraints on `M`
or required as an explicit constraint. If positivity cannot be proven, the
compiler errors.

Variable-length collections are out of scope.

### 3. Types

A type declares what must be true about a structural pattern. See section 2 for
the distinction between `type` (value-level) and `node` (entity-level).

#### 3.1 Scalar types

A scalar type wraps a numeric value with a unit:

```myco
type Potential : Scalar<MPa>
type Conductance : Scalar<mol_m2_s> { self >= 0 }
```

Constraints on a scalar type are predicates over `self` (see section 5).

The `:` in a type declaration establishes a **subtype relationship**. Writing
`type A : B` means `A <: B` — every value of type `A` is also a valid `B`.
Similarly, `node X : Contract` means `X <: Contract`. The subtype operator
`<:` is used in structural introspection (section 5.5) to filter by type:
`field.type <: Scalar` matches any field whose type is a subtype of `Scalar`.

#### 3.2 Generic types

Types may be parameterized, just like nodes:

```myco
type BoundedScalar<U: Unit> : Scalar<U> {
    self >= 0
}

type ConstrainedPair<U: Unit> {
    lower: Scalar<U>
    upper: Scalar<U>

    constraint ordered:
        lower <= upper
}
```

This allows reusable type patterns without repeating constraint definitions.

#### 3.3 User-defined types

Users may define composite types that carry constraints over their internal
structure:

```myco
type Fraction : Scalar<ratio> {
    0 <= self <= 1
}

type NscComposition {
    sugar: Fraction
    starch: Fraction

    constraint normalized:
        sugar + starch = 1
}
```

Types may impose constraints on their own fields (intra-type constraints).
Types may not impose constraints on their parent's fields or on nodes they do
not contain — cross-node constraints belong at the containing scope.

#### 3.4 Contracts

A contract is a trait-like interface. It declares required inputs, outputs, and
constraints without providing an implementation:

```myco
contract Photosynthesis {
    input ci: Potential
    input par: Scalar<ratio>
    input temperature: Scalar<ratio>
    input jmax: Conductance
    input vmax: Conductance

    output assimilation: CarbonMass
}
```

A node satisfies a contract if it provides all required fields with compatible
types and satisfies all declared constraints.

**Contract invocation is purely functional.** A contract implementation is a
hybrid: its **non-input fields** (parameters like `b`, `c` for WeibullVC) are
real nodes in the containment tree, bindable via the workflow. Its **input
fields** are formal parameters — they do not exist as quantities in the model
graph. Invocation evaluates the contract's output relations as a function of
the given arguments, without creating persistent bindings:

```myco
vc(current_pressure).plc            // evaluate at current_pressure
vc(historical_pressure).plc         // evaluate at a different pressure
```

The same contract instance can be invoked at different argument values because
inputs are formal parameters, not graph quantities. There is no implicit
"current context" binding.

Each invocation inlines the contract's output relations as a fresh anonymous
scope — `vc(p1).plc` and `vc(p2).plc` produce two independent expressions
with no shared intermediate variables, even if the contract's implementation
uses internal intermediates. The flattener expands each call site into its own
subexpression. This means contracts with internal variables (like FarquharC3's
`j_c` and `j_e`) are safe to invoke multiple times — each invocation gets its
own copies of those intermediates.

**Named arguments.** For single-input contracts, positional invocation is
natural: `vc(pressure).plc`. For multi-input contracts, named arguments are
supported:

```myco
photo(temperature=leaf_temp, par=layer_par, g_c=g_w/1.6,
      co2=atm.co2, o2=atm.o2, atm_pressure=atm.pressure).assimilation
```

**Wiring pattern.** As sugar for multi-input contracts, inputs may be wired via
individual relations:

```myco
relation photo_temp[i in 0..N]:
    canopy[i].photo.temperature = canopy[i].leaf_temperature

relation photo_par[i in 0..N]:
    canopy[i].photo.par = radiation.layers[i].par
```

When all inputs of a contract instance are wired via relations, the compiler
collects these bindings into a single implicit call site. The contract's outputs
(e.g., `photo.assimilation`) are then accessible as fields. If not all inputs
are wired, the compiler errors. Wiring and explicit call syntax are equivalent
and may not be mixed for the same contract instance.

Contracts enable generic subsystem swapping:

```myco
node Leaf<P: Photosynthesis> {
    photo: P
    // ...
}
```

Different photosynthesis implementations (C3, C4, CAM) can be plugged in
without changing the containing structure.

#### 3.4.1 Contract default implementations

Contracts may provide default relations for outputs that most implementations
share:

```myco
contract VulnerabilityCurve {
    input pressure: WaterPotential
    output plc: Fraction
    output conductance_fraction: Fraction

    property monotone: increasing(pressure -> plc)

    // Default: most VCs compute conductance_fraction this way
    default conductance_fraction = 1.0 - plc
}
```

A default relation is included if and only if the implementation does not
provide its own relation for that output. This is simple fallback, not conflict
resolution — there is no ambiguity about which relation wins. If the
implementation provides `conductance_fraction = some_other_expression`, the
default is silently excluded. This avoids repeating `conductance_fraction =
1.0 - plc` in every VC implementation.

**Constraint and property inheritance.** A node implementing a contract inherits
all constraints and properties declared in the contract. These compose
conjunctively with the node's own constraints (see section 5.7). For example,
if `VulnerabilityCurve` declares `property monotone: increasing(pressure ->
plc)`, every implementation inherits this property without redeclaring it. The
compiler verifies it against the implementation's actual relations.

#### 3.4.2 Worked example: Vulnerability curves

A vulnerability curve maps water potential to fractional loss of hydraulic
conductivity. The scientific community uses multiple parameterizations. This is
a natural contract:

```myco
contract VulnerabilityCurve {
    input pressure: WaterPotential
    output plc: Fraction

    property monotone: increasing(pressure -> plc)
}
```

Implementations:

```myco
// Weibull (the standard Sperry parameterization)
node WeibullVC : VulnerabilityCurve {
    b: PositiveScalar
    c: PositiveScalar

    plc = 1.0 - exp(-(-pressure / b) ** c)
}

// P50-slope sigmoid (cleaner parameterization, trivially invertible)
node SigmoidVC : VulnerabilityCurve {
    p50: WaterPotential
    slope: PositiveScalar

    plc = 1.0 / (1.0 + exp(slope * (pressure - p50)))
}
```

A plant hydraulics node is generic over the VC type:

```myco
node XylemSegment<V: VulnerabilityCurve> {
    k_max: HydraulicConductance
    vc: V
    water_potential: WaterPotential
    conductance: HydraulicConductance

    constraint conductance_from_vc:
        conductance = k_max * (1.0 - vc(water_potential).plc)
}
```

This allows users to swap vulnerability curve implementations without changing
the plant model.

### 4. Units and Dimensions

Units are first-class, not string labels. The goal is Rust-`uom`-level rigor:
if you try to add a pressure to a length, it is a compile error. The compiler
enforces dimensional consistency throughout the world model, and the workflow
layer validates unit consistency at binding boundaries.

The unit system is **not hardcoded to SI**. SI ships as a standard library
package that everyone can import, but the language provides primitives for
defining any unit system. Users can define custom base units, derived units,
and unit packages.

#### 4.1 Base units and dimensions

A **base unit** declares a fundamental measurement scale and implicitly
introduces a **base dimension**. A dimension is a physical kind, represented
internally as an integer exponent vector over all declared base dimensions.

Base units are declared with the `base_unit` keyword:

```myco
// In the SI standard library (units/si.myco)
pub base_unit kilogram     // introduces base dimension: Mass
pub base_unit meter        // introduces base dimension: Length
pub base_unit second       // introduces base dimension: Time
pub base_unit kelvin       // introduces base dimension: Temperature
pub base_unit mole         // introduces base dimension: Amount
pub base_unit ampere       // introduces base dimension: Current
pub base_unit candela      // introduces base dimension: Luminosity
```

Each `base_unit` declaration introduces a new orthogonal axis in the dimension
exponent vector. The SI package introduces seven; a custom package could
introduce more or fewer.

Every dimension is a product of powers of base dimensions:

| Dimension   | Exponent vector          |
|-------------|--------------------------|
| Length      | L¹                       |
| Area        | L²                       |
| Velocity    | L¹·T⁻¹                  |
| Pressure    | M¹·L⁻¹·T⁻²             |
| Force       | M¹·L¹·T⁻²              |
| Energy      | M¹·L²·T⁻²              |
| Conductance (molar) | N¹·L⁻²·T⁻¹     |
| Dimensionless | (all zeros)            |

The compiler propagates dimensions through all expressions using exponent
vector arithmetic. This is a standard, well-understood approach (see Rust's
`uom`, F#'s units of measure, Boost.Units in C++).

#### 4.2 Unit declarations and the package model

A **unit** is a specific scale (and optional offset) within a dimension. MPa,
Pa, and kPa all have dimension `M¹·L⁻¹·T⁻²` but differ by scale factors.

**Derived units** are defined as products, quotients, and scalar multiples of
existing units:

```myco
// In the SI standard library
pub unit newton = kilogram * meter / second ** 2
pub unit pascal = newton / meter ** 2
pub unit megapascal = 1e6 * pascal
pub unit joule = newton * meter
pub unit watt = joule / second

// Affine unit (section 4.4)
pub unit celsius : affine(kelvin, offset = -273.15)

// Dimensionless
pub unit ratio = 1

// Compound units
pub unit mol_m2_s = mole / meter ** 2 / second
pub unit mmol_m2_s = 1e-3 * mol_m2_s
pub unit J_mol = joule / mole
pub unit J_mol_K = joule / mole / kelvin
```

The compiler infers the dimension of a derived unit from its definition. In the
last example, `joule / mole / kelvin` has dimension
`(M¹·L²·T⁻²) · N⁻¹ · Θ⁻¹ = M¹·L²·T⁻²·N⁻¹·Θ⁻¹`.

**The SI package** ships as a standard library. Models import from it:

```myco
use units::si::{
    megapascal as MPa,
    mole_per_square_meter_second as mol_m2_s,
    ratio,
}
```

**Custom unit packages** follow the same pattern. A domain-specific package can
define units built on SI base units, or define entirely new base units:

```myco
// In a domain package: forestry/units.myco
module forestry::units

use units::si::{mole, kilogram, second, meter}

pub unit mol_C = mole           // same dimension as mole (Amount)
pub unit mol_C_s = mol_C / second
pub unit gC = 12.011e-3 * kilogram
pub unit m2_leaf = meter ** 2   // same dimension as m² (Area)
```

Note that `mol_C` and `mole` have the same dimension (Amount). The unit system
does not distinguish them dimensionally — both are `N¹`. Semantic distinction
(preventing accidental mixing of carbon moles and water moles) is handled at
the **type level**, not the unit level (see section 4.7).

**Non-SI systems** (CGS, imperial, etc.) are defined the same way. A CGS
package would either define its own base units (introducing independent
dimensions) or define CGS units as derived from SI base units (allowing
cross-system conversion).

#### 4.3 `Scalar<U>` — the parameterized quantity type

`Scalar<U>` is the built-in parameterized type meaning "a real number measured
in unit U." Every quantity in the world model has a `Scalar<U>` type (or a
named type that derives from one).

```myco
pub type WaterPotential : Scalar<MPa>     // dimension: pressure
pub type Temperature : Scalar<degC>       // dimension: temperature
pub type Fraction : Scalar<ratio> {       // dimension: dimensionless
    0 <= self <= 1
}
```

The type parameter `U` carries both the dimension (for compile-time checking)
and the unit scale (for runtime value interpretation). When the user writes
`type WaterPotential : Scalar<MPa>`, they are declaring:

- **Dimension**: pressure (M¹·L⁻¹·T⁻²)
- **Unit**: megapascals (the runtime value 1.0 means 1.0 MPa)
- **Subtype**: `WaterPotential <: Scalar<MPa>`

Generic functions parameterized over `U: Unit` (section 9.2) accept any unit
and preserve it through the computation:

```myco
fn arrhenius<U: Unit>(value_25: Scalar<U>, ...) -> Scalar<U>
```

The compiler monomorphizes each call site to the concrete unit type and verifies
that the function body is dimensionally consistent for that unit.

#### 4.4 Affine unit transforms

Some unit conversions are affine (offset + scale) rather than purely
multiplicative. The canonical example: Celsius to Kelvin.

```myco
pub unit celsius : affine(kelvin, offset = -273.15)
```

The compiler handles affine conversions automatically. When a relation mixes
Celsius and Kelvin quantities, the compiler inserts the correct conversion. This
eliminates manual `+ 273.15` scattered through model code — the Arrhenius
functions in the Sperry mock accept a `Temperature` type and the compiler
handles the conversion to absolute temperature internally.

**Affine caveat**: affine units cannot be freely multiplied or divided. `20°C *
2` is physically meaningless (is it 40°C or 586.3 K?). The compiler requires
conversion to the absolute unit (Kelvin) before multiplication. Expressions
like `temperature * temperature` in an energy balance equation trigger
automatic conversion to Kelvin; the result has dimension Θ² in absolute units.
Addition and subtraction of two affine quantities is permitted (the offsets
cancel for subtraction, producing a temperature *difference* which is purely
multiplicative).

**Escape hatch for empirical equations.** Many empirical equations in the
scientific literature expect raw numeric values in a specific unit (e.g., Buck's
saturated vapor pressure equation expects a Celsius float, Q10 temperature
responses expect a Celsius difference). The `value_in(unit)` primitive extracts
a dimensionless scalar representing the quantity's value in the requested unit:

```myco
let t_c = temperature.value_in(degC)    // dimensionless float in Celsius
let svp = 0.61121 * exp((18.678 - t_c / 234.5) * t_c / (257.14 + t_c))
```

`value_in` is the only way to exit the dimension system. It strips the
dimension entirely — the result is `Scalar<ratio>` (dimensionless). This is
intentionally explicit: it forces the user to name which unit scale the
empirical equation was calibrated for. Without `value_in`, empirical equations
that depend on a specific unit scale (not just a dimension) cannot be written,
since the compiler stores all quantities in base units internally.

Affine transforms also cover other offset-based systems (Fahrenheit, gauge
pressure vs absolute pressure).

#### 4.5 Internal representation and storage model

Internally, **all math happens in base units**. Declared units are a user-facing
layer. This is the same approach as Rust's `uom`: values are converted to base
units on entry, all computation uses base units, and results are converted back
to declared units on output.

The mental model:

```
User declares:      temperature: Temperature      // type uses degC
User provides:      temperature = 25              // means 25°C
Stored internally:  298.15                        // in kelvin (base unit)
User reads result:  25                            // converted back to degC
```

For purely multiplicative units (MPa vs Pa), the conversion is a scale factor:

```
User declares:      psi: WaterPotential           // type uses MPa
User provides:      psi = -1.5                    // means -1.5 MPa
Stored internally:  -1.5e6                        // in pascal (base unit)
User reads result:  -1.5                          // converted back to MPa
```

This design guarantees:

- **No unit mixing in computation**: intermediate values are always in base
  units. The compiler never needs to track "which unit is this temporary in?"
- **Constraints work across units**: `constraint: temperature > 0 degC` and
  `constraint: temperature > 273.15 K` are compile-time equivalent. The
  compiler converts annotated literals to base units. `0 degC` → `273.15`
  (base). `32 F` → `273.15` (base). The comparison happens in base units.
- **No runtime overhead**: scale factors are compile-time constants. The
  compiler folds conversions into the emitted expressions. The only cost is
  the initial conversion on input and final conversion on output.

Each base dimension has exactly one base unit (the one declared with
`base_unit`). For SI: kilogram, meter, second, kelvin, mole, ampere, candela.
All other units in that dimension are derived and carry a known conversion
factor to the base.

#### 4.6 Dimensional algebra rules

The compiler enforces these rules for every subexpression in every relation,
constraint, registered function body, and inline expression:

**Addition and subtraction**: both operands must have the same dimension. The
result has that dimension.

```myco
// OK: pressure - pressure = pressure
turgor = psi_stem - osmotic_potential

// ERROR: pressure + length — dimension mismatch
bad = psi_stem + height
```

**Multiplication**: dimensions multiply (exponent vectors add). The result
dimension is the product of the operand dimensions.

```myco
// conductance [N·L⁻²·T⁻¹] × pressure [M·L⁻¹·T⁻²]
// = [M·N·L⁻³·T⁻³] (a flux)
flow = conductance * pressure_drop
```

**Division**: dimensions divide (exponent vectors subtract).

```myco
// energy [M·L²·T⁻²] / (amount [N] × temperature [Θ])
// = [M·L²·T⁻²·N⁻¹·Θ⁻¹] — the gas constant R
R = 8.314 J_mol_K
```

**Exponentiation by literal integer or const generic**: dimension is raised to
that power.

```myco
// temperature⁴ in Stefan-Boltzmann term
atm.temperature ** 4   // dimension: Θ⁴
```

Exponentiation by a non-integer or by a quantity with non-zero dimension is a
compile error.

**Transcendental functions** (`exp`, `log`, `sin`, `cos`, `sqrt`, etc.): the
argument must be **dimensionless**. The result is dimensionless (except `sqrt`,
which halves the dimension exponents).

```myco
// OK: the Arrhenius exponent is dimensionless
//   [J/mol] × [K] / ([K] × [J/(mol·K)] × [K])
//   = [J·K/mol] / [J·K/mol] = dimensionless ✓
exp(ha * (T - T_ref) / (T_ref * R * T))

// ERROR: exp of a pressure
exp(psi_stem)   // compile error: argument has dimension M·L⁻¹·T⁻²
```

This rule catches a large class of physics errors. If a user writes
`exp(activation_energy / temperature)` and forgets to divide by R, the
compiler errors: the argument has dimension `M·L²·T⁻²·N⁻¹·Θ⁻¹`, not
dimensionless.

**Comparison operators** (`=`, `>=`, `<=`, `>`, `<`): both sides must have the
same dimension. If both sides have the same dimension but different units, the
compiler converts both to base units before comparison.

**Literal numbers**: bare numeric literals (e.g., `1.6`, `0.01`) are
dimensionless. To give a literal a unit, annotate it: `298.15 K`, `0.75 MPa`.
Annotated literals are converted to base units at compile time. `0 degC`,
`273.15 K`, and `32 F` are all compile-time equivalent — they all resolve to
`273.15` in the base unit (kelvin).

#### 4.7 Type-level semantic distinction

The dimension system catches physics errors (pressure + length). The **type
system** catches semantic errors (carbon moles + water moles).

Two quantities with the same dimension but different semantic meaning should
have different named types:

```myco
use forestry::units::{mol_C, mol_C_s}

type CarbonPool : Scalar<mol_C>     // dimension: Amount
type WaterPool : Scalar<mole>       // dimension: Amount (same!)

type GrowthRate : Scalar<mol_C_s>   // dimension: Amount·Time⁻¹
type TranspirationRate : Scalar<mol_m2_s>  // dimension: Amount·Length⁻²·Time⁻¹
```

`CarbonPool` and `WaterPool` both have dimension `Amount` (N¹). The unit
system would dimensionally allow adding them. But the type system prevents it:

```myco
// ERROR: CarbonPool + WaterPool — different types
total = carbon_pool + water_pool

// OK: CarbonPool + CarbonPool — same type
total_carbon = nsc + structural_carbon
```

This is the same separation of concerns as Rust newtypes. Dimensions catch
broad physics errors (you can't add meters and seconds). Types catch narrow
semantic errors (you shouldn't add carbon and water even though both are
counted in moles).

Named types are optional — `Scalar<MPa>` works fine without a wrapper. But for
quantities where semantic confusion is possible, named types are the defense.

#### 4.8 Dimension checking through registered functions

Registered functions declare signatures with typed parameters. The compiler
checks:

1. At the **definition site**: the function body is dimensionally consistent
   given the declared parameter types and return type.
2. At each **call site**: the supplied arguments have the correct dimensions,
   and the return value is used in a dimensionally consistent context.

For generic functions like `arrhenius<U: Unit>`, the compiler checks that the
body is consistent for *any* unit `U`. This means the body can only use `U` in
ways that are valid regardless of dimension — e.g., multiplying by a
dimensionless factor. The compiler verifies this parametrically at definition
time, not just at each monomorphized call site.

#### 4.9 Dimension checking through contracts

Contract declarations include typed inputs and outputs. When a contract is
invoked (section 3.4), the compiler verifies:

- The supplied arguments match the declared input dimensions
- The produced outputs are used consistently with their declared dimensions
- The contract implementation's body is dimensionally consistent

Contract implementations inherit the input/output types from the contract
declaration. If an implementation's body produces a result with the wrong
dimension, it is a compile error on the implementation.

#### 4.10 Unit validation at binding boundaries

The `.myco` file is dimensionally self-consistent by construction (the compiler
enforces it). The remaining boundary is between the world model and the
workflow layer, where external values enter the system.

**Assumed data**: when the user supplies data via `assume_series`,
`assume_constant`, or `assume_initial`, the binding layer validates that the
target quantity's declared unit matches the data. The workflow API accepts an
optional `unit` parameter for explicit declaration:

```python
# Data is in the quantity's declared unit (degC) by default
experiment.assume_series("atm.temperature", data)

# Explicit unit — binding layer converts if compatible, errors if not
experiment.assume_series("atm.temperature", data_in_kelvin, unit="K")
```

If `unit` is omitted, the data is assumed to be in the quantity's declared
unit. If provided and the dimension matches, the binding layer converts to the
declared unit. If the dimension doesn't match, it errors.

Internally, the binding layer converts all supplied data to base units (section
4.5) before passing it to the compiled model. Results are converted back to
declared units before returning to the user.

**Observed data**: same validation as assumed data.

**Slot bindings**: a slot declares typed outputs (e.g., `provides [g_s]` where
`g_s: Conductance`). For `.myco` controller bindings (section 7.2), the
compiler checks dimensional consistency of the controller's output relation —
this is just normal compile-time checking.

For opaque slot bindings (neural nets, Python callables), the binding layer
converts the slot's output from base units to declared units (and vice versa
for inputs) at the boundary. The implementation operates in base units. This
means a neural net that provides stomatal conductance outputs a value in the
base unit for that dimension (e.g., `mol·m⁻²·s⁻¹` rather than
`mmol·m⁻²·s⁻¹`). The binding layer handles the conversion. This is simpler and
less error-prone than requiring the neural net to know about declared units.

**Cross-module imports**: imported types carry their unit and dimension. If
module A exports `pub type WaterPotential : Scalar<MPa>` and module B imports
it, the dimension and unit are carried across. No re-declaration is needed.

#### 4.11 What is out of scope

- **Dependent unit types**: the system does not support units that depend on
  runtime values (e.g., "per unit leaf area" where leaf area is a model
  quantity).
- **Full symbolic simplification**: the compiler checks dimensional consistency
  but does not simplify compound unit expressions for display. Diagnostics show
  the dimension exponent vector, not a simplified name.
- **Automatic unit inference for bare literals in typed contexts**: if a field
  is declared as `Scalar<MPa>` and the user writes `psi = -1.5`, the literal is
  dimensionless and the compiler errors. The user must write `psi = -1.5 MPa`.
  This is intentional — explicit unit annotations prevent silent errors.

### 5. Constraints (The Predicate Language)

Constraints declare what must be true. They are purely descriptive.

#### 5.1 Syntax

Constraints are named, first-class members of a node or type:

```myco
node Leaf {
    stomata: Conductance
    g_max: Conductance
    water: Potential
    turgor_loss_point: Potential

    constraint stomatal_bounds:
        0 <= stomata <= g_max

    constraint wilting:
        water < turgor_loss_point implies stomata = 0
}
```

#### 5.2 Available operations

The predicate/expression language supports:

- **Arithmetic**: `+`, `-`, `*`, `/`, `**`
- **Comparison**: `<`, `<=`, `=`, `>=`, `>`
- **Logical connectives**: `and`, `or`, `not`, `implies`
- **Quantifiers**: `forall`, `exists` over index ranges
- **Comprehensions**: `sum`, `count`, `mean`, `min`, `max` with `where`
  filtering
- **Conditional expressions**: `if cond then expr else expr` — a value-level
  conditional. During flattening, conditionals over compile-time-known
  predicates (index comparisons, type tests) expand to the appropriate branch.
  Conditionals over runtime values produce piecewise expressions in the emitted
  code, with smooth approximation if needed for differentiability.
- **Let bindings**: named subexpressions for readability, not mutation (see
  below)
- **Function calls**: any function from the registry (section 9)

Note: `=` in constraints means equality (the same as in relations). There is no
assignment operator in Myco. The compiler may solve in either direction.

**`let` binding semantics.** A `let` binding introduces a named subexpression
within the enclosing body (node, relation, constraint, temporal, or function).
`let` has lexical scope — all names visible at the binding site are in scope,
including the enclosing node's fields, contract inputs (if inside a contract
implementation), and earlier `let` bindings. A `let` binding does **not**
introduce a new quantity in the model graph; it is purely a readability
mechanism that the compiler inlines during flattening.

#### 5.3 Examples

Universal quantification:

```myco
constraint all_stomata_positive:
    forall i in 0..N: canopy.leaves[i].stomata >= 0
```

Filtered aggregation:

```myco
constraint positive_total_area:
    sum(canopy.leaves[i].area for i in 0..N) > 0 m2
```

Conditional:

```myco
constraint active_conductance:
    let active = [i for i in 0..N
        where canopy.leaves[i].water > canopy.leaves[i].turgor_loss_point]
    count(active) = 0 or
        mean(canopy.leaves[i].stomata for i in active) >= 0.1 *
        mean(canopy.leaves[i].g_max for i in active)
```

Pairwise:

```myco
constraint max_gradient:
    forall i in 0..(M-1):
        abs(soil.layers[i].water - soil.layers[i+1].water) <=
            MAX_PRESSURE_GRADIENT * soil.layers[i].thickness
```

#### 5.4 Type-aware where clauses

When iterating over heterogeneous collections (those using `dyn` contracts,
see section 2.5), `where` clauses can filter by concrete type:

```myco
// Applies only to PFTs whose photosynthesis is FarquharC3
relation c3_nitrogen_scaling[i in 0..N where pfts[i].photo is FarquharC3]:
    eco.pfts[i].canopy.photo.v_max_25 = nitrogen_to_vmax(eco.pfts[i].nitrogen)
```

The `is` predicate tests the concrete type of a `dyn` element. After
monomorphization, this resolves to a compile-time-known subset of indices. No
runtime branching is emitted.

A `has` predicate tests for the presence of a field:

```myco
constraint rubisco_positive[i in 0..N where pfts[i].photo has rubisco_specificity]:
    eco.pfts[i].canopy.photo.rubisco_specificity > 0
```

Both `is` and `has` are compile-time predicates resolved after monomorphization.
They may appear in both library and model modules. In library modules, `is` and
`has` predicates on `dyn` elements remain unresolved until the model module
specializes the concrete types. The flattener resolves them in the same pass
that monomorphizes `dyn` — no additional mechanism is needed.

#### 5.5 Structural introspection

Constraints, relations, and temporal blocks may quantify over the structure of
a node using type-filtered iteration.

**Field-level introspection** iterates over the direct fields of a node:

```myco
constraint all_finite:
    forall field in self.fields where field.type <: Scalar:
        is_finite(field)
```

**Subtree introspection** iterates recursively over all descendants of a node,
filtered by type:

```myco
temporal cavitation[seg in pathway where seg is XylemSegment]:
    seg.min_historical_pressure[t+1] =
        min(seg.min_historical_pressure[t], seg.core.water_potential[t])
```

Here `seg in pathway` walks the entire containment subtree rooted at `pathway`,
and `where seg is XylemSegment` filters to only those descendants whose type
matches. Array elements are expanded individually — if `pathway.root` is
`[XylemSegment<V>; M]`, each `root[j]` matches independently.

Both forms require the set of fields to be well-defined at compile time, which
the containment model guarantees. After monomorphization, the iteration expands
to one instance per matching element.

Structural introspection is available in constraints, relations, temporal
blocks, and derive macros (see section 18).

#### 5.6 Properties (continuous invariants)

Some invariants cannot be verified by expansion over discrete indices. These are
declared with the `property` keyword:

```myco
contract VulnerabilityCurve {
    input pressure: WaterPotential
    output plc: Fraction

    property monotone: increasing(pressure -> plc)
}
```

**Quantification scope.** A property is an obligation over the full admissible
domain induced by the node's declared fields and constraints. For example,
`increasing(pressure -> plc)` on `VulnerabilityCurve` means "for all values of
`pressure` and for all parameter values satisfying the node's own constraints,
`plc` is increasing in `pressure`." Property satisfaction must not depend on
workflow bindings — it is a structural guarantee of the implementation.

Properties are verified by the compiler where possible:

- For simple expressions (polynomial, exponential, sigmoid), the compiler
  performs symbolic analysis to verify the property
- For complex expressions where symbolic verification fails, the compiler
  **errors** with a diagnostic explaining what it could not prove

The user may explicitly acknowledge an unverifiable property:

```myco
property monotone: increasing(pressure -> plc) #[verified_externally]
```

The `#[verified_externally]` annotation suppresses the error and records the
assumption in the compilation report. The compiler never silently trusts.

#### 5.7 Composition

Constraints compose conjunctively. All constraints from a node, its type, its
implemented contracts, and all containing scopes must hold simultaneously. There
is no override or relaxation mechanism. Contract constraints and properties are
inherited by implementations (see section 3.4.1).

### 6. Relations

Relations connect quantities across nodes. They are the equations of the world.

All relations must hold simultaneously. The user does not annotate which
equations form coupled systems — the compiler discovers this automatically (see
section 12.5).

The `=` in a relation means equality. Both sides are symmetric and the compiler
may solve in either direction. There is no assignment operator in Myco.

**`constraint` vs `relation` keywords.** Both keywords can contain equalities,
and `=` has the same meaning in both: equality that the compiler may use for
computation in either direction. The `constraint` keyword is a naming/grouping
mechanism, not a semantic distinction. The rule is:

- **Equalities** (`=`) are always solver-eligible, regardless of whether they
  appear in a `relation` block, a `constraint` block, or bare inside a node
  body. The planner may use any equality as a computational path.
- **Inequalities and logical predicates** (`>=`, `<=`, `implies`, `and`, `or`,
  etc.) are enforcement-only. They constrain the solution space but do not
  provide computational paths.

In practice, `relation` is conventional for cross-node equations, and
`constraint` is conventional for invariants that live inside a node definition.
Both are valid anywhere. The compiler treats them identically — what matters is
the form of the expression, not the keyword.

#### 6.1 Named relations

```myco
relation demand_transpiration:
    leaf.transpiration = leaf.stomata * env.vpd_scale
```

#### 6.2 Indexed relations

Over repeated structure:

```myco
relation demand_transpiration[i in 0..N]:
    canopy.leaves[i].transpiration =
        canopy.leaves[i].stomata * env.vpd_scale
```

#### 6.3 Temporal relations

Temporal relations describe how state evolves:

```myco
temporal water_step[i in 0..N]:
    canopy.leaves[i].water[t+1] =
        canopy.leaves[i].water[t] - config.dt * canopy.leaves[i].transpiration[t]
```

A quantity that appears on the left-hand side of a temporal relation is
automatically inferred as persistent (requires initial state in the workflow
binding).

**`dt` is a normal quantity.** The timestep is not a magic name or built-in.
It is a node in the world model, assumed or learned through the normal binding
vocabulary:

```myco
node SimulationConfig {
    dt: Scalar<seconds>
}
```

```python
experiment.assume_constant("config.dt", value=1800.0)
```

This means `dt` participates in dimensional analysis, can have constraints
(`dt > 0`, `dt <= max_safe_timestep`), and is visible in the model structure.
The compiler recognizes the `[t+1]` / `[t]` pattern as temporal and handles
rollout generation, but the timestep scalar itself is just a quantity.

Any quantity name may serve as the timestep — there is no reserved name.

`dt` may be rollout-stable (constant via `assume_constant`) or may vary per
timestep (via `assume_series`). The planner handles both — a per-step `dt` is
treated identically to any other per-step forcing. This enables variable
time-stepping driven by external schedules.

Temporal relations may use any function from the registry. In particular,
accumulator patterns with `min` and `max` are supported:

```myco
temporal cavitation_tracking[i in 0..N]:
    pathway.segments[i].min_historical_pressure[t+1] =
        min(pathway.segments[i].min_historical_pressure[t],
            pathway.segments[i].water_potential[t])
```

This pattern enables irreversible cavitation tracking: the worst pressure ever
experienced becomes a permanent ceiling on future conductance, enforced via
a constraint:

```myco
constraint irreversible_cavitation[i in 0..N]:
    pathway.segments[i].conductance <=
        k_max * (1.0 - vc(pathway.segments[i].min_historical_pressure).plc)
```

#### 6.4 Multiple relations for the same quantity

A quantity may participate in more than one relation. This is intentional and
is one of the core reasons Myco exists — the same physical quantity (e.g.,
transpiration) can be derived from demand-side logic, supply-side logic, or
energy balance, and the model should express all of these.

Multiple relations for the same quantity do NOT necessarily mean
"overdetermination." The planner classifies coupled components by their
equation/unknown structure (section 12.3). The four possibilities are:

- **Computational redundancy**: algebraically equivalent evaluators of the
  same solved component. The compiler picks a canonical evaluator. No user
  action needed.
- **Square implicit** (n_eq = n_unknown): mutual dependencies like Farquhar
  A-Ci (two equations, two unknowns: assimilation and c_i). These are
  solver blocks (SCCs), not overdetermination.
- **Underdetermined residual** (n_eq < n_unknown): more unknowns than
  equations. Requires additional bindings or latent owners (section 12.4).
- **Overconstrained residual** (n_eq > n_unknown): more equations than
  unknowns — simultaneous world-claims that may disagree. Requires an
  explicit **closure policy** if the user wants a single forward value
  (section 12.3, 14.6).

This classification is **context-dependent** — it depends on which bindings
are applied. The same component may be square in one experiment (where enough
quantities are assumed) and underdetermined in another (where fewer
observations are available). The planner performs this classification at plan
time, after bindings.

### 7. Slots

A slot is a declared interface for a component that will be provided at workflow
time.

#### 7.1 Slot declaration

```myco
slot stomatal_control provides [stomata]:
    inputs = [*]
```

The slot declares what it provides and what it needs. The `[*]` wildcard means
"all quantities structurally reachable from the slot's outputs via the model's
relation graph, excluding the slot's own outputs." Reachability is undirected —
it traverses the relation graph as an undirected constraint graph, not a causal
DAG. This means `[*]` may resolve to a large set (potentially all quantities in
a connected component), which is intentional: the neural network receives a
superset and learns to weight relevant inputs. This is the slot's **structural
interface** — it is determined from the model structure alone and is invariant
across experiments.

The structural interface is resolved once at model load time, not per-experiment.
This is critical for shared controllers in multi-experiment training (section
17): the controller architecture must be fixed across experiments even when
different experiments provide different subsets of the inputs as concrete
values. In experiments where some structural inputs are not concretely
available, they are backed by learned trajectories, learned constants, or
other latent owners — the slot always receives the same named inputs.

The slot's outputs then extend the computable set and planning continues. This
is order-dependent (the slot sees everything computed before it), but the
planner's topological ordering makes this deterministic.

Alternatively, inputs may be listed explicitly for documentation and interface
clarity:

```myco
slot controller provides [canopy.leaves[*].stomata]:
    inputs = [
        canopy.leaves[*].water,
        canopy.leaves[*].nsc.sugar,
        env.vpd_scale,
        env.soil_water,
    ]
```

Path wildcards (`[*]`) indicate that the slot operates over all instances of a
repeated structure.

#### 7.2 Slot binding modes

A slot does not declare an implementation. The implementation is supplied at
binding time via one of three modes:

- **Learned**: a trainable function (neural network, linear model, etc.) whose
  parameters are optimized during training.
- **Bound**: an imported controller from a `.myco` module. Controllers are
  normal Myco components — they define relations for the quantities the slot
  provides. When imported, their relations merge into the model graph and are
  planned like any other relations.
- **Assumed**: raw data supplied directly for the slot's output quantities.

Controllers are not a special file format. A controller is just a `.myco` module
that provides relations for the right quantities. When the compiler merges a
controller's relations into the host model's graph, it may introduce new
algebraic loops — the SCC detection (section 12.5) handles this naturally.

Example package layout:

```
sperry/
    mechanics.myco              # hydraulics, photosynthesis, carbon balance
    controllers/
        gain_risk.myco          # Sperry gain-risk optimization
        ball_berry.myco         # Ball-Berry empirical model
        medlyn.myco             # Medlyn optimality model
```

And the user's workflow:

```python
model = myco.load("sperry/mechanics.myco")

# For synthetic data generation with Sperry's original criterion:
experiment.bind_slot("stomatal_control", "sperry/controllers/gain_risk")

# For learning from data:
experiment.learn_slot("stomatal_control")

# Or just plug in observed stomata directly:
experiment.assume_series("stomata", observed_stomata_data)
```

#### 7.3 Slot interaction with algebraic loops

If a slot's output is needed inside an algebraic loop (SCC), the slot becomes
part of that SCC. The user does not need to think about this — the compiler
discovers and handles it.

In the Sperry model, the stomatal controller produces `stomata`, which feeds
into demand transpiration, which is part of the hydraulic SCC. The slot is
therefore part of the SCC. When the slot is a learned function, the compiler
differentiates through the solver (implicit function theorem) to get gradients
to the learned function's parameters.

This is consistent with the soul: "the compiler does the work." The user
declares what the slot provides; the compiler figures out where it sits in the
execution plan.

#### 7.4 Input introspection

The Python API supports enumerating the resolved inputs of a slot:

```python
plan = experiment.explain_plan()
interface = plan.slot_interface("stomatal_control")
print(interface.resolved_inputs)
# ['canopy.water_potential', 'soil.layers[0].water_potential', ...]
```

This allows users to inspect what the learned controller will see without
manually listing every input.

---

## Part II: The Math Substrate

These sections define how mathematical operations behave in the system,
independent of any specific model.

### 8. Operation Algebra

Every mathematical operation in Myco carries metadata that the planner and
emitter use to make informed decisions.

#### 8.1 Invertibility classes

Each operation declares an invertibility class:

- **`bijective`**: uniquely invertible. Given the output and all other inputs,
  any one input can be recovered exactly. Examples: `+`, `-`, `*` (when other
  operand is nonzero), `/` (when divisor is nonzero).

- **`injective_restricted`**: invertible on a restricted domain. Examples:
  `exp`/`log` (domain `x > 0`), `sqrt` (domain `x >= 0`, non-negative branch),
  trig functions (on restricted intervals).

- **`lossy`**: information is destroyed. The output does not determine the
  input, but may bound it. Examples: `abs`, `max`, `relu`, `floor`. Useful for
  constraint narrowing but not for exact inversion.

- **`opaque`**: no useful inverse. Forward-only computation. Examples: lookup
  tables, hash functions, complex black-box functions.

#### 8.2 Differentiability classes

Each operation also declares a differentiability class:

- **`smooth`**: continuously differentiable, gradients are well-behaved.
- **`subgradient`**: has subgradients suitable for SGD-based training. Examples:
  `relu`, `abs`, `max`.
- **`fragile`**: differentiable but gradients may be numerically unstable in
  some regions. Examples: division near zero, `log` near zero.
- **`non_differentiable`**: cannot be usefully differentiated. Examples:
  discrete operations, rounding.

#### 8.3 Domain restrictions

Operations may declare domain restrictions on their inputs:

```
log: domain(x > 0)
sqrt: domain(x >= 0)
div: domain(denominator != 0)
```

These restrictions are checked by the constraint analysis system (section 11)
and may be enforced at runtime.

#### 8.4 Planner interaction

The planner uses this metadata to:

- Decide which inversions are valid (only `bijective` and
  `injective_restricted`, and only when domain restrictions are satisfiable)
- Assign path costs (ill-conditioned paths cost more)
- Choose canonical evaluators for computationally redundant paths
- Inform closure policies for overconstrained components (section 12.3)
- Reject plans that route a `train`-mode compilation through a
  `non_differentiable` operation

#### 8.5 Operation algebra and coupled components

The operation algebra's invertibility and differentiability metadata informs
the planner's treatment of all coupled components (section 12.3):

- For **computational redundancy**, the metadata determines which evaluator is
  numerically preferred (better-conditioned, smoother).
- For **square implicit components** (SCCs), it determines solver strategy and
  whether the component is differentiable for training.
- For **overconstrained residuals** where the user applies a closure policy
  (section 14.6), the metadata is available to the policy — e.g., the
  `condition_weighted` policy uses condition-number estimates to weight paths.

The metadata is available to both standard library closure policies and
user-defined ones via structural introspection on the competing paths.

### 9. Function Registry

The function registry provides named mathematical functions beyond primitive
arithmetic.

#### 9.1 Built-in functions

Standard mathematical functions are available without import:

- `exp`, `log`, `sqrt`, `abs`
- `sin`, `cos`, `tan`
- `min`, `max`, `clamp`
- `sigmoid`, `softplus`

Each carries operation algebra metadata (invertibility, differentiability,
domain).

#### 9.2 User-registered functions

Users may register domain-specific functions:

```myco
fn vulnerability_curve(pressure: Potential, p50: Potential, slope: Scalar<ratio>)
    -> Fraction
{
    invertibility: injective_restricted
    differentiability: smooth
    domain: slope > 0

    1.0 - exp(-((pressure / p50) ** slope))
}

fn inverse vulnerability_curve(
    plc: Fraction, p50: Potential, slope: Scalar<ratio>
) -> Potential {
    p50 * (-log(1.0 - plc)) ** (1.0 / slope)
}
```

A registered function declares:

- Signature with units
- Operation algebra metadata
- The function body (an expression, not imperative code)
- Optionally: an explicit inverse function

**Function-level generics.** Functions may be generic over unit types, enabling
unit-polymorphic helpers:

```myco
fn arrhenius<U: Unit>(
    value_25: Scalar<U>,
    activation_energy: Scalar<J_mol>,
    temperature: Temperature,
) -> Scalar<U> {
    invertibility: bijective
    differentiability: smooth

    let R = 8.314 J_mol_K
    value_25 * exp(activation_energy * (temperature - 298.15 K) / (298.15 K * R * temperature))
}
```

This allows `arrhenius` to accept `CarbonFlux`, `Pressure`, `Conductance`, or
any other unit type for `value_25` and return the same unit. The compiler
monomorphizes each call site to the concrete unit type, the same way it
monomorphizes generic nodes. Function-level generics use the same `<T: Bound>`
syntax as node generics.

#### 9.3 Inverse verification

If a user declares an explicit inverse, the compiler performs two levels of
checking:

1. **Symbolic verification** (where possible): for restricted function families
   (monotone bijections, polynomial inverses, compositions of known-invertible
   operations), the compiler attempts to prove correctness symbolically.

2. **Round-trip sanity check** (always): for a set of sample inputs in the
   declared domain, the compiler checks that `inverse(f(x)) ≈ x` within
   numerical tolerance. If the sanity check fails, the compiler errors with the
   failing test cases.

If symbolic verification succeeds, the inverse is fully trusted. If only the
sanity check passes, the inverse is treated as `#[verified_externally]` — the
compiler records the assumption in the compilation report and generates runtime
monitoring. Finite samples are not verification; they silently trust unsampled
regions. The no-trust principle requires this distinction.

If no explicit inverse is provided and the invertibility class is `bijective` or
`injective_restricted`, the compiler may attempt symbolic inversion for simple
expression bodies. If it cannot derive an inverse, the function is treated as
`opaque` for inversion purposes — and the compiler emits a warning if the
declared invertibility class was higher than `opaque`, since the declaration
cannot be honored.

#### 9.4 Importable function packages

Functions ship with library modules. A library that defines a contract (e.g.,
`VulnerabilityCurve`) can also ship helper functions (e.g., `peaked_arrhenius`,
`collatz_smooth_min`) that models using that contract will need:

```myco
// In physiology/temperature.myco
pub fn peaked_arrhenius<U: Unit>(
    value_25: Scalar<U>,
    ha: Scalar<J_mol>,
    hd: Scalar<J_mol>,
    sv: Scalar<J_mol>,
    temperature: Temperature,
) -> Scalar<U> { ... }
```

```myco
// In a user's model
use physiology::temperature::peaked_arrhenius
```

#### 9.5 Compiler primitives: differentiation and integration

Two expression-level primitives allow models to reference derivatives and
integrals of other model quantities. These are not runtime functions — they are
compiler directives that the compiler resolves during compilation.

**`deriv(quantity_a, quantity_b)`** — the partial derivative of `quantity_a`
with respect to `quantity_b`, evaluated at the current operating point.

```myco
// Marginal carbon gain — sensitivity of assimilation to stomatal conductance
let dA_dgs = deriv(photo.assimilation, gas.g_s)

// Allocation rule: invest carbon where marginal return is highest
allocation_leaves = dA_dgs / (dA_dgs + dG_dk_root)
```

The compiler resolves `deriv` by walking the expression graph from
`quantity_a` back to `quantity_b` and applying the chain rule symbolically.
Because all registered functions have transparent expression bodies (section
9.2), the compiler always has the full expression chain. No numerical fallback
is needed for `deriv` within the world model — symbolic differentiation is
mechanical (the chain rule always works on a known expression graph).

`deriv` produces a new expression that the planner treats like any other
relation. It participates in SCC detection, path selection, and code emission
normally. The emitted code evaluates the derivative expression directly — there
is no AD overhead at runtime because the derivative has been resolved to a
concrete expression at compile time.

**`integrate(expr, var, lower, upper)`** — the definite integral of `expr` with
respect to `var` over the interval `[lower, upper]`.

```myco
// Lockhart growth integral — turgor excess integrated along stem height
G_0 = phi * (C_wood / u_s)
    * integrate(max(P_0(z) - turgor_threshold, 0), z, 0, 1)
```

Unlike `deriv`, symbolic integration is not always possible. The compiler
attempts symbolic resolution for known integrand forms (polynomials,
piecewise-linear, compositions of elementary functions). If symbolic resolution
succeeds, the integral is replaced by its closed-form expression at compile
time.

If symbolic resolution fails, the compiler emits a numerical quadrature call.
The quadrature strategy is configurable (section 14.4). The default is
Gauss-Legendre with a point count chosen by the compiler based on the
integrand's differentiability metadata. The compiler reports which integrals
were resolved symbolically and which require numerical quadrature in the
compilation plan (section 14.5).

`integrate` introduces a runtime cost proportional to the number of quadrature
points. The compilation plan makes this cost visible so users can tune the
strategy or restructure the model.

**Scope and limitations.** Both `deriv` and `integrate` operate within the
world model's expression graph.

- `deriv` can differentiate through **contract invocations** (which have
  transparent expression bodies) and through **square implicit components**
  (SCCs) via the implicit function theorem. For example,
  `deriv(photo.assimilation, gas.g_s)` differentiates through the Farquhar
  A-Ci SCC, which the compiler handles by applying the implicit function
  theorem to the SCC's equation system.
- **`deriv` through SCCs is not fully symbolic.** The implicit function theorem
  requires the Jacobian of the SCC's equation system. For small SCCs (e.g.,
  the 2×2 Farquhar A-Ci system), the compiler can invert the Jacobian
  symbolically and produce a compile-time expression. For larger SCCs (e.g.,
  an N-segment hydraulic network), symbolic Jacobian inversion is
  intractable. In these cases, the compiler emits a runtime autodiff call
  (`jax.jacfwd` over the `custom_root` solver) rather than a symbolic
  expression. The compilation plan (section 14.5) reports which `deriv`
  expressions were resolved symbolically and which require runtime AD.
- `deriv` **cannot** differentiate through slots (slots are opaque) or through
  underdetermined residual blocks (where the system is not closed).
- `integrate` cannot integrate over model structure (e.g., "integrate over all
  soil layers") — use indexed comprehensions for that.
- `deriv` cannot differentiate across timesteps (e.g., d/dt) — use temporal
  blocks for time evolution.
- If an `integrate` expression depends on SCC-resolved quantities, the
  integration occurs after the SCC solver runs. If the integral's value feeds
  back into the SCC, the quadrature is nested inside the solver loop — the
  emitter includes the quadrature call in the residual function passed to
  `custom_root`. The plan inspection (section 14.5) reports this nesting and
  its cost implications.

---

## Part III: The Compiler

These sections define the internal compilation pipeline. Users do not interact
with these stages directly, but the spec must define them to ensure correctness.

### 10. Flattening Pass

The flattening pass is an explicit compiler phase between type-checking and
planning. It transforms the recursive, generic world model into a flat
quantity/relation/constraint graph that the planner can consume.

#### 10.1 Monomorphization

All generic type parameters, const generic parameters, and `dyn` references are
resolved to concrete types and values:

```
Canopy<3, FarquharC3> --> a concrete Canopy with 3 leaves, each using FarquharC3
Ecosystem<3> with pfts[0]: PFT<C3, Weibull>, pfts[1]: PFT<C4, Sigmoid>, ...
    --> three differently-typed PFT instances
```

This is analogous to Rust monomorphization. No generic or `dyn` code survives
past this phase. Heterogeneous collections are expanded into per-element
concrete types.

#### 10.2 Structural expansion

Repeated structure is expanded into concrete paths:

```
canopy.leaves: [Leaf<FarquharC3>; 3]
-->
canopy.leaves[0].water: Potential
canopy.leaves[0].stomata: Conductance
canopy.leaves[1].water: Potential
canopy.leaves[1].stomata: Conductance
canopy.leaves[2].water: Potential
canopy.leaves[2].stomata: Conductance
```

Indexed relations are expanded similarly:

```
relation demand[i in 0..3]:
    canopy.leaves[i].transpiration = canopy.leaves[i].stomata * env.vpd_scale
-->
demand_0: canopy.leaves[0].transpiration = canopy.leaves[0].stomata * env.vpd_scale
demand_1: canopy.leaves[1].transpiration = canopy.leaves[1].stomata * env.vpd_scale
demand_2: canopy.leaves[2].transpiration = canopy.leaves[2].stomata * env.vpd_scale
```

Type-aware `where` clauses are resolved during expansion. A clause like
`[i in 0..3 where pfts[i].photo is FarquharC3]` produces relations for only
the matching indices.

#### 10.3 Constraint lowering

Group constraints and quantified constraints are lowered to concrete form:

```
forall i in 0..3: canopy.leaves[i].stomata >= 0
-->
canopy.leaves[0].stomata >= 0
canopy.leaves[1].stomata >= 0
canopy.leaves[2].stomata >= 0
```

Aggregation constraints become concrete expressions:

```
sum(canopy.leaves[i].area for i in 0..3) > 0
-->
canopy.leaves[0].area + canopy.leaves[1].area + canopy.leaves[2].area > 0
```

Structural introspection (`forall field in self.fields`) is resolved during
this phase. The set of fields is known after monomorphization, so the `forall`
expands to concrete constraints.

#### 10.4 Macro expansion

Declarative macros and derive macros (section 18) are expanded before
flattening. Macro expansion occurs after parsing but before type-checking,
producing standard Myco AST nodes. No macro survives past this phase.

#### 10.5 Output

The output is a flat graph of:

- Concrete quantities with types, units, and constraints
- Concrete relations with resolved expressions
- Concrete temporal equations
- Concrete slot interfaces with resolved paths

This graph is the input to the planner. The planner does not need to understand
generics, recursion, `dyn`, macros, or repeated structure.

### 11. Constraint Analysis

The compiler uses constraint information both statically (at compile time) and
to inform runtime code generation.

#### 11.1 What the compiler proves

The compiler attempts to prove:

- **Dimensional consistency** of all relations (always succeeds or produces a
  compile error)
- **Domain satisfaction** for operations with restricted domains (e.g., that the
  argument to `log` is provably positive)
- **Bound satisfaction** where type constraints and relation structure are
  sufficient (e.g., if `x >= 0` and `y >= 0`, then `x + y >= 0`)
- **Constraint compatibility** when multiple constraints apply to the same
  quantity (detect provably impossible combinations)
- **Properties** declared on contracts (monotonicity, etc.) via symbolic
  analysis where possible

If the compiler cannot prove a property or domain restriction, it **errors**
with an actionable diagnostic. The user may suppress the error with an explicit
acknowledgment annotation (`#[verified_externally]`), which is recorded in the
compilation report.

#### 11.2 Internal strategy

The constraint analysis uses a stack of progressively richer analyzers:

**Interval propagation** (mandatory baseline). Propagates interval bounds
through the constraint graph via fixed-point iteration. Linear in the number of
constraints per pass, typically converges in a small number of iterations.
Sufficient for most bound-based and linear constraints. Cheap enough to run on
every compilation.

**Monotonicity-aware propagation.** Tracks per-argument monotone direction for
operations and registered functions on their declared domains. Bounds propagate
by endpoint evaluation instead of naive interval arithmetic, avoiding the
"dependency problem" (e.g., evaluating `X - X` yields a wide interval instead
of `0` under naive intervals).

Example: `conductance = k_max * (1 - vc(psi).plc)`. If `k_max > 0` and the
contract declares `plc` is increasing in `psi`, then conductance is decreasing
in `psi`. A bound on `psi` gives a tight bound on conductance, and a bound on
conductance plus invertibility on the monotone segment contracts `psi`. Harmonic
means of positive conductances are monotone in each argument. These facts
enable much tighter bounds through the hydraulic pathway than naive intervals.

**Contractor passes.** For coupled components, the analyzer runs forward and
backward passes that shrink participating domains while preserving every
feasible solution. For each relation, a bound on some quantities contracts
others. Applied repeatedly, this is stronger than one-shot propagation. For
SCCs, the contractor operates on the block as a whole (interval Newton,
Krawczyk-like contraction, or Gauss-Seidel residual pruning).

For constraints involving logical connectives (`implies`, `or`) or nonlinear
operations, the compiler may fall back to conservative approximation. If
conservative approximation is insufficient, the compiler errors (it does not
silently assume the constraint holds).

**Compile-time bounds vs. runtime enforcement.** Compile-time bounds from
constraint propagation are useful for: initialization (providing initial guesses
to SCC solvers), numerical stability (detecting potential domain violations
before runtime), and simple proofs (verifying type constraints and linear
properties). For highly coupled nonlinear systems (like the hydraulic SCC),
compile-time bounds may widen to near-type-bounds and provide limited signal.

The primary mechanism for enforcing physical consistency during training is
**runtime constraint enforcement**: the SCC solver must find a solution at each
timestep, and that solution must satisfy all observation and constraint losses.
This is the mechanism by which the mechanistic graph acts as an effective
inductive bias on the learned controller — not compile-time static bounds.
The spec should not be read as claiming that compile-time bounds are the
training signal; they are structural aids.

#### 11.3 Runtime interaction

Where static proof succeeds, the compiler may elide runtime checks. Where
explicit acknowledgment is given (`#[verified_externally]`) — including inverse
declarations verified only by sanity check (section 9.3) — runtime behavior
depends on mode:

- In `simulate` mode: runtime assertions that raise on violation
- In `train` mode: differentiable penalty losses (soft constraint enforcement)
- In both modes: diagnostic reporting for constraint violations

This ensures that `#[verified_externally]` properties are never silently
trusted at runtime — the user discovers violations even though the compiler
could not prove them statically.

**Admissibility projections.** Constraints that guard the definedness of
downstream operations require stronger enforcement than soft penalties. If a
quantity feeds into `log`, `sqrt`, or division, and the constraint
`quantity > 0` (or `!= 0`) is not statically proven, the compiler must inject
a **differentiable projection** (e.g., `softplus` for positivity) at the
boundary rather than relying solely on penalty losses. Without projections,
the controller may produce domain-violating values during early training that
generate `NaN` gradients before the penalty can correct.

The distinction:
- **Admissibility constraints** (guard definedness): enforced by projection or
  reparameterization. The operation algebra's domain restrictions (section 8.3)
  determine which constraints are admissibility guards.
- **Scientific feasibility constraints** (do not guard definedness): enforced
  by penalty in `train` mode, assertion in `simulate` mode.

#### 11.4 Interaction with algebraic loops

Proven bounds from the constraint analysis can improve solver behavior for
algebraic loops (section 12.5). If the compiler can prove that a quantity in an
SCC is bounded to a narrow range, that information can:

- Provide better initial guesses for numerical solvers
- Enable analytical simplification by narrowing the search space
- Allow the compiler to prove solver convergence for some systems

#### 11.5 Upgrade path

The constraint analysis system should be designed so that richer abstract
domains can be added later without changing the user-facing constraint language.
The analyzer stack (section 11.2) is ordered by cost and precision:

- **Mandatory**: interval propagation, monotonicity-aware propagation
- **Optional**: contractor passes (stronger but more expensive)
- **Future**: polyhedral domains, symbolic predicates, relational abstract
  domains

Each layer refines the bounds produced by the layer below. The knowledge
envelope (section 12.6) records which analyzers contributed to each quantity's
bounds via the provenance field.

### 12. Planning

The planner takes the flat graph from the flattening pass and produces a
**residual graph** — a deterministic factor graph that represents the complete
structural knowledge of the system given the current bindings.

The residual graph is the core semantic object of the compiler. From it, the
emitter derives executable code (section 13). The plan inspection API (section
14.5) exposes the residual graph to the user as the primary diagnostic tool.

#### 12.1 Residual graph structure

The residual graph contains:

- **Variable nodes**: quantities that remain free after bindings, including
  explicit latent owners (learned trajectories, learned constants, slot
  parameters) and time-indexed state variables.
- **Derived nodes**: quantities that can be eliminated explicitly from the
  variable nodes via acyclic forward computation.
- **Residual factors**: equalities from relations and temporal equations,
  inequality and domain constraints, observation terms, and any explicit
  closure or discrepancy relations.
- **Slot nodes**: explicit numeric functions from inputs and parameters to
  provided outputs.
- **Metadata**: per-quantity knowledge envelopes (section 12.6) with bounds,
  monotonicity facts, differentiability class, and provenance.

#### 12.2 Core algorithm

The planner builds the residual graph by:

1. Start from the flat graph (section 10.5)
2. Apply bindings — mark assumed quantities as fixed, mark learned quantities
   as latent-owned variable nodes, mark slot outputs as slot-provided
3. Build the dependency graph from the relation set
4. Identify strongly connected components (SCCs) — see section 12.5
5. Classify each coupled component by equation/unknown structure (section 12.3)
6. **Eliminate** what is eliminable: acyclic derivations become derived nodes,
   square implicit components become solver blocks
7. **Leave** the rest as residual blocks: underdetermined components (more
   unknowns than equations) and overconstrained components (more equations
   than unknowns)
8. Run constraint analysis (section 11) to populate knowledge envelopes
9. Handle temporal equations for `t -> t+1`

Path selection within eliminable components is informed by the operation algebra
(section 8):

- Prefer `bijective`, `smooth` paths over `injective_restricted` or `fragile`
- Assign higher cost to inversions through ill-conditioned operations
- Reject inversions through `lossy` or `opaque` operations
- In `train` mode, reject canonical paths through `non_differentiable`
  operations

#### 12.3 Component classification

After SCC detection, the planner classifies each coupled component by counting
equations and unknowns. This is the discriminator — not path counting.

**Computational redundancy.** The same underlying system admits multiple
algebraically equivalent evaluators (e.g., the same expression simplified
differently). The planner picks a canonical evaluator using the
operation algebra's cost model. This is compiler-internal and does not affect
the science. Users do not need to configure it.

**Square implicit component** (n_eq = n_unknown). Mutual dependencies like
Farquhar A-Ci (two equations: biochemical supply and diffusion demand; two
unknowns: assimilation and internal CO2). These form solver blocks. See section
12.5 for solver classification and emission.

**Underdetermined residual** (n_eq < n_unknown). More unknowns than equations.
The component cannot be solved without additional information. The planner
records it as a residual block and adds its unknowns to the resolution frontier
(section 12.4).

**Overconstrained residual** (n_eq > n_unknown). More equations than unknowns.
These are simultaneous world-claims — e.g., supply transpiration and demand
transpiration are both valid derivations but may not agree given approximations
in the model. The equations must remain as residual constraints unless the user
explicitly applies a **closure policy** (section 14.6).

If a closure policy is specified, the planner applies it: the policy selects or
blends a forward value, and the remaining equations become residual factors
that contribute consistency losses. If no closure policy is specified, the
overconstrained component remains as residual factors. In `simulate` mode,
unresolved overconstrained components error (you need a single forward value).
In `train` mode, they contribute residual losses.

**Consistency losses.** For overconstrained components (whether closed by a
policy or left as residuals), the compiler emits consistency losses from the
extra equations. In `train` mode, these losses penalize disagreement between
world-claims. In `simulate` mode with a closure policy, they become diagnostic
assertions.

#### 12.4 Underdetermined quantities and the resolution frontier

When the residual graph contains underdetermined components (more unknowns than
equations), the planner produces a **resolution frontier**: the minimal set of
additional bindings or latent-owner declarations that would close the system.

The resolution frontier reports:
- Each unresolved quantity's full path
- What it depends on (which other unknowns, which relations)
- What bindings would close it (assume, learn_trajectory, learn_constant)
- How closing it would cascade (what additional quantities become derivable)

**In `simulate` mode**: unresolved quantities always error. The frontier
provides the actionable diagnostic.

**In `train` mode**: unresolved quantities error UNLESS every remaining unknown
has an explicit latent owner (learned slot, learned trajectory, learned
constant, or learned initial). The compiler will not silently invent latent
owners — the user must explicitly declare what is learned. Once all unknowns
are owned, the residual graph is closed and the emitter can produce executable
code.

This follows the no-trust principle: the compiler guides the user to close the
system but never does it for them.

#### 12.5 Algebraic loop detection and solver emission

Some sets of relations form circular dependencies within a single timestep. For
example, in the Sperry hydraulic model:

- Flow depends on conductance
- Conductance depends on pressure
- Pressure depends on flow

Similarly, Farquhar photosynthesis creates a loop between assimilation and
internal CO2, and the leaf energy balance creates a loop between temperature,
VPD, and transpiration.

The user does not annotate these loops. The planner discovers them
automatically.

**Detection**: After building the dependency graph from the flat relation set,
the planner identifies **strongly connected components** (SCCs). An SCC is a
maximal set of quantities where each depends (directly or transitively) on every
other.

- Quantities not in any SCC are ordered topologically as usual.
- Each SCC is classified by equation/unknown structure (section 12.3).
- If a slot's output feeds into an SCC, the slot is part of that SCC (see
  section 7.3).

**Solver classification** (for square implicit components): The planner
classifies each square SCC by examining the structure of its equations:

- **Linear**: all relations are linear in the SCC unknowns. Solve with direct
  linear algebra (LU decomposition).
- **Polynomial**: relations are polynomial in the SCC unknowns. For
  single-unknown low-degree systems, attempt analytical solution. Otherwise,
  numerical solve.
- **General nonlinear**: the default case. Requires a numerical solver.

**Solver emission**: For each square SCC requiring a numerical solve, the
emitter generates backend-appropriate solver code:

- **JAX**: `jax.lax.custom_root` or a Newton-Raphson loop with
  `jax.jacfwd` for the Jacobian. Implicit differentiation via the implicit
  function theorem provides exact gradients.
- **Rust**: standard NR with LU decomposition.
- **PyTorch**: `torch.autograd.Function` with implicit differentiation.

**Multiple SCCs and gradient chains.** When multiple SCCs exist in a single
timestep (e.g., hydraulic SCC, A-Ci SCC, energy balance SCC) and they depend
on each other through shared quantities, the emitter generates nested
`custom_root` calls. Gradients flow through the full chain via composed
implicit differentiation. The plan inspection reports the SCC dependency order.

**Binding-dependent loops**: Different bindings may produce different SCCs from
the same model. If a quantity in a loop is assumed as a constant, the loop may
break into acyclic components. The planner handles this naturally — SCC analysis
runs on the dependency graph after bindings are applied. In multi-experiment
training (section 17), different experiments may produce different SCC
configurations for the same model. The training infrastructure handles this by
compiling per-experiment artifacts with different solver structures but shared
slot parameters.

#### 12.6 Knowledge envelopes

Each quantity in the residual graph carries a **knowledge envelope** —
orthogonal fields that represent everything the compiler knows about the
quantity given the current bindings and constraint analysis.

The fields:

- **`realization`**: `explicit(expr)` (the quantity has a forward computation
  path), `implicit(residual_block)` (the quantity participates in a residual
  system), or `opaque(provider)` (the quantity is provided by a slot or
  external binding).
- **`free_variables`**: the set of latent or still-unbound symbols the
  quantity depends on. Empty for concrete quantities.
- **`bounds`**: the current abstract value from constraint analysis. Initially
  an interval, refined by monotonicity-aware propagation and contractor passes
  (section 11.2). May be as tight as a point value or as loose as the type
  bounds.
- **`obligations`**: residual equations and inequality/domain constraints that
  this quantity participates in but that have not been eliminated.
- **`resolver_sets`**: minimal additional bindings or latent-owner declarations
  that would make the quantity's realization explicit. Empty for concrete
  quantities.
- **`provenance`**: which assumptions, equations, properties, and analyzers
  contributed to the envelope's current state.

From these fields, familiar summary labels are derived views:
- **Concrete**: `realization = explicit` and `free_variables` is empty
- **Symbolic**: `realization = explicit` with free variables, or `implicit`
- **Bounded**: `bounds` is tighter than the type's declared bounds
- **Unresolved**: `resolver_sets` is non-empty

The knowledge envelope is the user-facing representation exposed by
`plan.knowledge(path)` (section 14.5). It is also used internally by the
constraint analysis (section 11) and the emitter (section 13).

#### 12.7 Temporal semantics

Temporal equations (`[t] -> [t+1]`) define how state evolves across timesteps.
In the residual graph, temporal equations are factors that connect quantity
nodes across timestep boundaries.

**Semantic model.** Temporal equations lower to **horizon-wide factors** — they
connect quantities at all timesteps into a single residual graph that spans the
full simulation horizon. This is the semantic model regardless of mode.

**Execution strategy.** When the within-timestep residual graph is fully closed
(all unknowns have owners), the emitter can optimize the horizon-wide factor
graph into a forward rollout (`lax.scan` in JAX). This is the common case in
both `simulate` and `train` modes after all latent owners are declared.

In `train` mode, sparse observations contribute loss only at observed
timesteps. Backpropagation through time (BPTT) provides backward information
flow through the forward rollout — later observations constrain earlier states
via gradient propagation through the temporal equations. This handles temporal
data gaps naturally without requiring a special bidirectional planning pass.

**Learned trajectories and temporal equations.** When a quantity has both a
temporal equation and a learned trajectory binding (section 16), the learned
trajectory provides the values (it is the latent owner) and the temporal
equation becomes a **physics residual factor** — a loss term penalizing
deviation between the trajectory's values and what the temporal equation
predicts. This is the PINN (physics-informed neural network) pattern and
falls out naturally from the residual graph design: the temporal equation is
a factor, the trajectory provides the variable values, and the factor's
residual becomes a loss.

**Constraint propagation across timesteps.** Compile-time constraint
propagation (section 11) operates within a single timestep. Cross-timestep
constraint propagation (e.g., "given observations at t=0 and t=20, what bounds
can we derive for t=10?") requires unrolling the temporal factors symbolically,
which is expensive for long horizons. For v2, cross-timestep reasoning is
handled by gradient-based training (BPTT), not by compile-time constraint
propagation. Extending the constraint system to reason across timesteps is an
upgrade path (section 11.5).

### 13. Code Emission / Backends

The emitter takes the closed residual graph and produces executable source code
for a specific backend.

**Closure requirement.** The emitter requires that every variable node in the
residual graph has an explicit owner (assumed, learned, or slot-provided). If
unowned variables remain, the planner has already errored with the resolution
frontier (section 12.4). The emitter never receives a residual graph with
anonymous free variables.

#### 13.1 Plan representation

The plan is backend-agnostic. From the closed residual graph, the emitter
derives:

- Forward computation steps for derived nodes (topologically ordered)
- Solver blocks for square implicit components (SCCs)
- Residual evaluators for overconstrained components and physics residuals
- Observation loss terms
- Admissibility projections and constraint penalty terms
- Temporal rollout structure

Each component carries metadata: dependencies, expressions, operation algebra
annotations, and provenance from the knowledge envelope.

#### 13.2 JAX emitter (primary)

The JAX emitter produces a Python module using:

- `jax.numpy` for array operations
- `jax.lax.scan` for rollout
- `jax.nn` for smooth projections and admissibility projections
- `jax.lax.custom_root` for implicit solves within a step
- `jax.checkpoint` for gradient checkpointing on long rollouts
- Standard pytree conventions for state and parameters

The emitted module includes:

- `step()`, `rollout()` (same as v1)
- `obs_loss()` — from observations
- `consistency_loss()` — from overconstrained residuals (section 12.3)
- `physics_residual_loss()` — from temporal equations when a learned
  trajectory coexists with a temporal relation (section 12.7)
- `constraint_violation_loss()` — from user-declared constraints
- `admissibility_loss()` — from propagation-derived bounds (section 11.2)
- `soft_penalty_loss()` — from `#[verified_externally]` properties
- `loss_components()`, `total_loss()` — weighted aggregation
- `init_params()`, `validate_rollout_inputs()`, `validate_observations()`
- Metadata constants and slot interface declarations

The emitter uses differentiability metadata from the operation algebra to:

- Inject admissibility projections at slot boundaries where domain restrictions
  are not statically proven (section 11.3)
- Choose smooth approximations where needed
- Warn about fragile gradient paths
- Reject `train`-mode plans with non-differentiable canonical paths

**Heterogeneous collections and vectorization.** Monomorphized `dyn` arrays
(section 2.5) are flattened into structurally distinct leaves in the JAX
pytree. For small heterogeneous collections (e.g., N=2 sun/shade canopy layers
with different photosynthesis implementations), this is fine. For large
collections (N >> 10), the unrolled graph may cause slow JAX compilation and
lose `vmap` vectorization benefits. Users modeling large ecosystems with many
structurally identical individuals should prefer homogeneous generics (e.g.,
`Canopy<N, FarquharC3>`) where possible, which the emitter can vectorize
efficiently.

**Long rollout stability.** For temporal rollouts, the emitter supports
gradient checkpointing via `jax.checkpoint` on the scan function to trade
compute for memory. Truncated backpropagation through time (limiting the
temporal gradient horizon) is configurable in section 14.7.

#### 13.3 Backend interface

The compiler defines a backend interface that any emitter must implement. This
allows future backends (PyTorch, Rust, Rust+PyO3 hybrid) to be added without
modifying the planner or flattener.

The interface requires:

- Emit scalar computation steps (derived nodes)
- Emit solver blocks for square implicit components (backend-appropriate solver)
- Emit residual evaluators for overconstrained and physics residual factors
- Emit rollout/scan structure for temporal equations
- Emit loss functions for `train` mode (observation, consistency, physics
  residual, constraint, admissibility)
- Emit admissibility projections at slot boundaries
- Emit parameter initialization
- Emit numerical quadrature calls for unresolved `integrate` expressions

`deriv` through acyclic paths is fully resolved at compile time — the backend
never sees it. However, `deriv` through large SCCs may require runtime
autodiff (section 9.5), in which case the backend must support differentiation
through its solver primitive (e.g., `jax.jacfwd` over `custom_root` for JAX).
`integrate` may require a runtime primitive (numerical quadrature) when
symbolic resolution fails.

The JAX backend is the primary implementation for v2. Other backends are
specified here for interface design but implemented post-v2.

### 14. Compiler Configuration

Compiler behavior is configurable separately from the world model. The `.myco`
file describes what is true; compiler configuration describes how to compute it.

#### 14.1 Solver configuration

Solver strategy for algebraic loops can be controlled per-SCC or globally:

```python
artifact = experiment.compile(
    backend="jax",
    solver_config={
        "default_strategy": "auto",       # compiler picks based on SCC class
        "default_tolerance": 1e-8,
        "default_max_iterations": 100,
    },
)
```

Available strategies:

- **`auto`** (default): compiler classifies the SCC and chooses appropriately.
  Linear -> direct solve. Small polynomial -> symbolic if possible. General ->
  numerical.
- **`newton`**: force Newton-Raphson for a specific SCC.
- **`fixed_point`**: force fixed-point iteration (may be more autodiff-friendly
  for some systems).
- **`analytical`**: force analytical solution (fails if the compiler cannot
  derive one).

#### 14.2 Closure policy configuration

Closure policies for overconstrained components are configured in section 14.6.
The `condition_weighted` policy in `myco::closure` provides conditioning-aware
blending. See section 8.5 for how the operation algebra informs the policies.

#### 14.3 Other configuration

- Constraint enforcement mode (project vs penalize)
- Debug mode (extra runtime checks and diagnostics)
- Property verification strictness (how many sample points for round-trip
  inverse testing, symbolic analysis depth)

These are analogous to optimization levels in a C compiler: they affect how the
code runs, not what it computes.

#### 14.4 Integration configuration

When `integrate` expressions cannot be resolved symbolically, the compiler emits
numerical quadrature. The quadrature strategy is configurable per-integral or
globally:

```python
artifact = experiment.compile(
    backend="jax",
    integration_config={
        "default_strategy": "auto",         # compiler picks based on integrand
        "default_quadrature": "gauss_legendre",
        "default_points": 16,
    },
)
```

Available strategies:

- **`auto`** (default): compiler inspects the integrand's differentiability
  metadata and chooses appropriately. Smooth integrands → Gauss-Legendre.
  Piecewise/subgradient integrands → adaptive Simpson or Clenshaw-Curtis.
- **`gauss_legendre`**: fixed-point Gauss-Legendre quadrature. Efficient for
  smooth integrands. Configurable point count.
- **`adaptive_simpson`**: adaptive Simpson's rule. Better for integrands with
  kinks or rapid variation. Configurable tolerance.
- **`trapezoid`**: simple trapezoidal rule. Configurable point count. Lowest
  accuracy but most predictable cost.

Per-integral overrides use the integral's label (derived from the containing
relation or named explicitly):

```python
artifact = experiment.compile(
    backend="jax",
    integration_config={
        "default_strategy": "auto",
        "overrides": {
            "turgor_integral": {"strategy": "adaptive_simpson", "tolerance": 1e-10},
        },
    },
)
```

Integrals that were resolved symbolically are not affected by this
configuration — they have been replaced by closed-form expressions and have no
runtime cost.

**Train-mode restriction.** In `train` mode, the compiler forces fixed-shape
quadrature (Gauss-Legendre or fixed-point trapezoid) for all numerical
integrals. Adaptive strategies (`adaptive_simpson`) are rejected because
discrete changes in the number of quadrature points create discontinuous loss
landscapes that break gradient quality. Adaptive strategies are available in
`simulate` mode only.

#### 14.5 Plan inspection

The residual graph is an inspectable artifact. After compilation, the user
can examine what strategies the compiler chose, query the knowledge state of
any quantity, and explore hypotheticals before execution. This is the primary
discovery mechanism for configurable behavior and the primary diagnostic tool
for understanding the model's structural properties.

```python
artifact = experiment.compile(backend="jax")
print(artifact.plan)
```

The plan reports:

- **Component classification**: which quantities form square implicit
  components (SCCs), which are overconstrained residuals, which are
  underdetermined, and what solver/closure strategy was chosen for each
- **Symbolic resolutions**: which `deriv` and `integrate` expressions were
  resolved at compile time, and what the resulting expressions are
- **Numerical fallbacks**: which `integrate` expressions require runtime
  quadrature, what strategy was chosen, and how to override it
- **Slot bindings**: which slots are bound, to what, and whether the binding
  is opaque (neural net) or transparent (`.myco` controller)
- **Execution order**: the topologically sorted sequence of computation steps
  within a timestep
- **Temporal state**: which quantities carry forward across timesteps
- **Resolution frontier**: if the system is not fully closed, the minimal
  set of additional bindings that would close it (section 12.4)

**Per-quantity knowledge queries.** The user can query the knowledge envelope
(section 12.6) for any quantity in the model:

```python
envelope = artifact.plan.knowledge("leaf.water_potential")
# envelope.realization    → explicit(expr) | implicit(block) | opaque(slot)
# envelope.free_variables → set of unbound symbols
# envelope.bounds         → Interval(-3.0, -0.1, unit="MPa")
# envelope.obligations    → list of residual factors
# envelope.resolver_sets  → minimal bindings to make concrete
# envelope.provenance     → which analyzers/assumptions contributed
```

**Hypothetical reasoning.** The user can explore the consequences of additional
bindings without committing:

```python
plan_b = artifact.plan.with_assumption("soil.water_potential", -0.5)
plan_b.knowledge("leaf.water_potential")
# → bounds narrowed, expression simplified, resolver_sets reduced
```

This is plan re-evaluation with additional constraints — the planner reruns
from the augmented binding set. It enables the scientist to reason about
experimental design: "if I collect this measurement, how much additional
information does the model give me?"

Note: the resolution frontier is a structural/computational heuristic ("binding
X unlocks the most computation"). It does not measure information gain or
identifiability, which are properties of the loss landscape and require runtime
analysis.

The plan follows the same principle as the rest of the compiler configuration:
defaults work out of the box, inspection reveals what was decided, and
overrides are available for power users. The plan is analogous to a SQL
`EXPLAIN` — it shows the execution strategy without changing the semantics.

#### 14.6 Closure policies for overconstrained components

When the planner detects overconstrained residual components (section 12.3) —
more equations than unknowns — the user may specify a **closure policy** to
produce a single forward value. This is an explicit approximation that relaxes
the overconstrained system into a computable form.

Closure policies are configured per-component or globally via `closure_config`:

```python
artifact = experiment.compile(
    backend="jax",
    closure_config={
        "default_policy": None,                 # no default — leave as residual
        "overrides": {
            "leaf.transpiration": "weighted_average",
            "canopy.assimilation": {
                "policy": "soft_select",
                "preference": ["demand_transpiration", "supply_transpiration"],
                "sharpness": 10.0,
            },
        },
    },
)
```

Setting `"default_policy"` to `None` (the default) leaves overconstrained
components as residual factors — their extra equations become consistency losses
in `train` mode and diagnostic assertions in `simulate` mode. In `simulate`
mode, if the component needs a single forward value and no closure policy is
specified, the planner errors with an actionable diagnostic.

**Closure policies are approximations.** They change the science of the
executed artifact by choosing how to reconcile simultaneous world-claims. The
plan inspection (section 14.5) reports when a closure policy has been applied
and which original equations were relaxed.

This distinction matters: closure policies are NOT "path selection" (choosing
among equivalent evaluators, which is compiler-internal) and NOT "resolution
strategies" (a neutral-sounding name that hides the fact that science is being
approximated). They are explicit, user-chosen approximations that the compiler
surfaces transparently.

If the reconciliation is itself part of the world claim — e.g., a sensor fusion
model, a discrepancy model, or a model-structural assertion that two
derivations should agree — it belongs in the `.myco` file as an explicit
relation, not in compiler configuration.

**Standard library: `myco::closure`**

Common closure policies ship as a standard library package. These are ordinary
`.myco` relations — convenience shorthand for patterns users could write
themselves. The package includes:

- **`weighted_average`**: arithmetic mean of competing path outputs. Simple,
  differentiable. Appropriate when paths are expected to agree and
  discrepancies should be averaged out.

- **`soft_select`**: differentiable soft selection with a preference ranking.
  `sharpness` controls how hard the selection is. Appropriate when one path
  is theoretically preferred but alternatives provide fallback.

- **`condition_weighted`**: weights paths by numerical conditioning (section
  8.5). Appropriate for purely numerical stability concerns where all paths
  are theoretically equivalent.

- **`hard_select`**: chooses a single path, discarding alternatives. Non-
  differentiable — rejected in `train` mode unless the discarded paths have
  no learned parameters upstream.

**Custom policies.** Users can write their own closure policies as `.myco`
relations:

```myco
fn my_blend(
    path_a: Scalar<U>,
    path_b: Scalar<U>,
    confidence_a: Scalar<ratio>,
) -> Scalar<U> {
    invertibility: bijective
    differentiability: smooth

    path_a * confidence_a + path_b * (1.0 - confidence_a)
}
```

Because policies are `.myco` relations, they participate in dimensional
checking, are differentiable when needed, and are backend-agnostic.

**Consistency losses.** Regardless of whether a closure policy is applied,
the extra equations in overconstrained components generate consistency losses.
In `train` mode, these losses penalize disagreement between world-claims. The
closure policy controls the forward value; the consistency loss provides a
training signal from all equations. Consistency loss weight is configurable:

```python
artifact = experiment.compile(
    backend="jax",
    closure_config={
        "consistency_loss_weight": 0.1,  # default: 0.1
    },
)
```

#### 14.7 Rollout stability configuration

Long temporal rollouts (growing seasons, multi-year ecosystem simulations) can
produce vanishing or exploding gradients during backpropagation through time.
The rollout configuration provides controls:

```python
artifact = experiment.compile(
    backend="jax",
    rollout_config={
        "gradient_checkpointing": True,     # default: True for long rollouts
        "checkpoint_interval": 50,          # checkpoint every N steps
        "truncated_bptt_horizon": None,     # None = full BPTT (default)
    },
)
```

- **Gradient checkpointing**: trades compute for memory by recomputing
  intermediate states during the backward pass rather than storing them.
  Enabled by default when the rollout horizon exceeds a threshold.
- **Truncated BPTT**: limits the temporal gradient horizon. Gradients do not
  propagate further than `truncated_bptt_horizon` steps backward. This
  sacrifices long-range temporal gradient signal for stability. Default is
  `None` (full BPTT).

---

## Part IV: The Workflow Layer

These sections define how users bind a world model to a specific workflow and
how training works.

### 15. Binding Vocabulary

The workflow vocabulary remains small:

- **`assume`**: supply a value. The world model doesn't care where it came from.
- **`observe`**: provide evidence. Contributes to loss in training.
- **`learn`**: declare something as trainable.
- **`bind`**: provide a specific implementation for a slot.

#### 15.1 Assumption modes

```python
experiment.assume_series("env.vpd_scale", steps)       # per-step forcing
experiment.assume_constant("env.hydraulic_cond")        # rollout-stable scalar
experiment.assume_initial("canopy.leaves[*].water")     # initial state value
```

#### 15.2 Observation modes

```python
experiment.observe_dense("canopy.leaves[*].transpiration")
experiment.observe_sparse("tree.growth", measured_steps)
```

Observations take a loss function (default `mse`, also `huber`) and a schedule.

**On measurement uncertainty:** The workflow API intentionally does not include
uncertainty parameters on observations. Measurement error is part of the world
— a sap flow sensor is a physical object with properties. The relationship
between true flux and measured flux is a relation:

```myco
node SapFlowSensor {
    true_flux: TranspirationRate
    measured_flux: TranspirationRate
    measurement_noise: TranspirationRate

    measured_flux = true_flux + measurement_noise
}
```

The workflow binds `measurement_noise` (assumed from calibration, learned, etc.)
and observes `measured_flux`. This keeps the language purely descriptive — the
model describes the world including imperfect sensors, and the workflow decides
what to do about it.

Probabilistic backends (e.g., NumPyro emission) are a natural future extension
that would enable full Bayesian inference over this same model structure.

#### 15.3 Learning modes

```python
experiment.learn_slot("controller")                              # shared function
experiment.learn_trajectory("soil_water", parameterization="spline", knots=12)
experiment.learn_constant("site_hydraulic_cond")                 # per-experiment scalar
experiment.learn_initial("soil_water")                           # initial condition of temporal quantity
```

`learn_initial` declares that a temporal quantity's t=0 value is a free
parameter to be optimized during training. It applies only to quantities with
temporal equations. The compiler wires the learned value to the initial timestep
and rolls forward from there. Semantically this is distinct from `learn_constant`
(which makes a quantity constant for all time) — a learned initial is the
starting point of a dynamic trajectory.

See section 16 for details on learned trajectories.

#### 15.4 Slot binding

```python
# Import a published controller
experiment.bind_slot("stomatal_control", "sperry/controllers/gain_risk")

# Or supply raw data for the slot's outputs
experiment.assume_series("stomata", observed_stomata_data)
```

#### 15.5 Slot metadata

Slots operate on continuous physical quantities from the model graph. Some
controller architectures also need discrete, experiment-level metadata that has
no representation in the `.myco` world model — for example, a taxonomic
identifier for FiLM conditioning, a site index, or a categorical treatment
label.

```python
experiment.bind_slot_metadata("stomatal_control", {
    "taxon_id": 4,              # integer index for FiLM embedding
    "site_elevation": 1200.0,   # auxiliary float not in the model graph
})
```

Slot metadata is concatenated with the slot's structural inputs before being
passed to the controller. The metadata values are not quantities in the model
graph — they carry no units, no dimensions, and no constraints. They are
opaque scalars passed through to the controller's input vector.

This keeps the `.myco` world model purely physical while allowing controllers
to condition on discrete or auxiliary information that varies per experiment.

#### 15.6 Path-based binding

All binding operations accept paths with wildcards:

```python
experiment.assume_constant("canopy.leaves[*].jmax")
experiment.observe_sparse("canopy.leaves[*].water", steps)
```

Wildcards expand to all matching instances in the flattened graph.

#### 15.7 Unit validation

The binding layer validates that supplied data matches the expected units
declared in the world model. Mismatched units produce a diagnostic. Convertible
units may be automatically converted with a warning.

### 16. Learned Trajectories

A learned trajectory is a structured latent variable for an unobserved,
time-varying quantity.

#### 16.1 Motivation

In real scientific workflows, some quantities are never directly observed but
affect downstream observables. Rather than assuming values for these quantities,
the user declares them as learned trajectories. The optimizer infers their
values jointly with the controller, shaped by:

- The mechanistic model (the trajectory must be consistent with the dynamics)
- Declared constraints (rate-of-change bounds, smoothness, value bounds)
- Downstream observations (gradients flow backward through the mechanistic
  equations)

#### 16.2 Parameterization

Learned trajectories are not free values at every timestep. They are
parameterized to reduce the number of learnable parameters and enforce
structure:

- **Spline**: a small number of control points, interpolated smoothly. Good
  for slowly-varying environmental quantities.
- **Fourier**: truncated Fourier series. Good for periodic quantities.
- **Direct with penalty**: a value at every timestep with a smoothness penalty
  loss. More flexible, more parameters.

The parameterization is a binding-time choice, not a world-model property.

#### 16.3 Constraints on trajectories

Learned trajectories are subject to the same constraint system as any other
quantity:

- Type constraints (bounds, positivity)
- Rate-of-change constraints declared in the world model
- Cross-node constraints

These are enforced via smooth penalty losses in training mode and hard checks
in simulation mode.

#### 16.4 Interaction with temporal equations

If a user declares `learn_trajectory` for a quantity that also has a temporal
equation (section 6.3), the trajectory is the **latent owner** — it provides
the values for that quantity at each timestep. The temporal equation becomes a
**physics residual factor** in the residual graph (section 12.7): a loss term
penalizing deviation between the trajectory's values and what the temporal
equation predicts.

This is the PINN (physics-informed neural network) pattern and falls out
naturally from the residual graph design. The temporal equation is a factor,
the trajectory provides the variable values, and the factor's residual becomes
a loss. The physics residual loss is reported separately from observation
losses in the emitted module (`physics_residual_loss()`).

#### 16.5 Compiler support

The compiler treats a learned trajectory similarly to an assumed series, except:

- Its values are learnable parameters (included in the gradient computation)
- The emitter allocates parameter arrays for the trajectory representation
- Constraint penalties are added to the loss
- If the quantity has a temporal equation, the temporal residual is added to
  the loss (section 16.4)

### 17. Study-Level Training

A study is a collection of experiments over the same world model.

#### 17.1 Structure

```python
study = myco.Study(model)

for dataset in datasets:
    exp = study.add_experiment(horizon_steps=len(dataset))
    exp.assume_series("env.vpd_scale", dataset.vpd)
    exp.learn_trajectory("soil_water", parameterization="spline", knots=12)
    exp.observe_sparse("canopy_health", dataset.health_steps)
    # ... per-experiment bindings

study.learn_slot("controller")  # shared across all experiments
```

#### 17.2 Shared vs per-experiment parameters

- **Shared**: learned slots (the controller). Same parameters across all
  experiments. This is what is being identified.
- **Per-experiment**: learned trajectories, learned constants. Different values
  for each experiment. These absorb per-context variation.

#### 17.3 Joint optimization

The optimizer minimizes the joint loss:

```
L = sum over experiments k:
    w_k * (obs_loss_k + consistency_loss_k + physics_residual_loss_k
           + constraint_penalty_k + admissibility_loss_k)
```

Each experiment compiles to its own artifact. The controller parameters are
shared. The joint gradient is the weighted sum of per-experiment gradients.

**Study weighting.** Different experiments may have very different loss
magnitudes (a study with 1000 transpiration observations vs. a study with 5
NSC measurements). Without configurable weighting, data-rich experiments
dominate gradients and may prevent the shared controller from learning
generalizable behavior. The `w_k` weights are configurable per-experiment:

```python
study = myco.Study(model)

exp_a = study.add_experiment(horizon_steps=1000)
exp_a.set_weight(1.0)  # default

exp_b = study.add_experiment(horizon_steps=50)
exp_b.set_weight(5.0)  # upweight small study

study.learn_slot("controller")
```

Per-loss-family weighting (e.g., observation vs. consistency vs. physics
residual) is also configurable, either globally or per-experiment, via the
compiler configuration.

#### 17.4 Identifiability

With enough diverse experiments, the shared controller is increasingly
constrained because:

- Per-experiment latent trajectories vary across experiments and cannot absorb
  signal that belongs to the shared controller
- Mechanistic structure ties latent trajectories to downstream observations
- Cross-experiment variation in environmental forcing exercises different
  regions of the controller's input space

The progressive data erasure benchmark (removing observations across
experiments) directly probes the boundary of identifiability for a given model
and dataset collection.

#### 17.5 Implementation path

Study-level training can start as a Python-side pattern: compile multiple
experiments, share controller parameters, sum losses. It does not initially
require new Rust-core abstractions. It should become a first-class concept when
the pattern is validated.

---

## Part V: Macros

These sections define the compile-time code generation facilities.

### 18. Macros

Macros generate Myco AST at compile time. They are expanded before
type-checking and flattening. No macro survives past expansion.

#### 18.1 Declarative macros

Declarative macros are pattern-matching template expanders:

```myco
macro temperature_adjust!(param, ha, hd, sv, temp):
    relation adjust_${param}:
        params.${param} = peaked_arrhenius(params.${param}_25, $ha, $hd, $sv, $temp)
```

Usage:

```myco
temperature_adjust!(v_max, 65330.0, 200000.0, 650.0, leaf_temperature)
temperature_adjust!(j_max, 43540.0, 200000.0, 650.0, leaf_temperature)
```

Expands to:

```myco
relation adjust_v_max:
    params.v_max = peaked_arrhenius(params.v_max_25, 65330.0, 200000.0, 650.0, leaf_temperature)

relation adjust_j_max:
    params.j_max = peaked_arrhenius(params.j_max_25, 43540.0, 200000.0, 650.0, leaf_temperature)
```

Declarative macros:
- Accept literal values, identifiers, and paths as arguments
- Produce relations, constraints, fields, or other Myco constructs
- Are hygienic (generated names don't collide with user names)
- Are expanded in declaration order

#### 18.2 Derive macros

Derive macros introspect a node's structure and generate code based on field
annotations:

```myco
#[derive(TemperatureAdjusted)]
node FarquharParams {
    #[arrhenius(ha=arrhenius_ha_vmax)]
    v_max_25: CarbonFlux

    #[arrhenius(ha=arrhenius_ha_jmax)]
    j_max_25: CarbonFlux

    // Arrhenius energy parameters — nodes, so they can be assumed or learned
    arrhenius_ha_vmax: Scalar<J_mol>
    arrhenius_ha_jmax: Scalar<J_mol>
}
```

The `TemperatureAdjusted` derive macro inspects each annotated field and
generates:

- A temperature-adjusted output field (`v_max`, `j_max`)
- A relation linking the base value to the adjusted value via the specified
  function
- A temperature input that all adjustments share

Derive macro annotations may reference:
- **Literal values**: `#[arrhenius(ha=65330.0)]` — hardcoded constant
- **Sibling node fields**: `#[arrhenius(ha=arrhenius_ha_vmax)]` — the parameter
  is itself a node, bindable via the workflow (assumable, learnable)

This distinction is critical: when annotation parameters reference nodes rather
than literals, the derived relations connect to quantities that participate in
the full model graph. The activation energy becomes something you can
`assume_constant()` or `learn_constant()`, not a magic number.

#### 18.3 Macro expansion order

1. Declarative macros expand first (textual substitution)
2. Derive macros expand second (require parsed node structure)
3. Type-checking runs on the fully expanded AST
4. Flattening proceeds as normal

Macros may not invoke other macros (no recursive expansion). This keeps
expansion predictable and debuggable.

---

## Appendix A: Worked Example — TinyTree

This appendix shows what the TinyTree model looks like in v2 syntax to provide
a concrete bridge from v1.

### A.1 World model

```myco
module plant::tiny_tree

use units::si::{
    megapascal as MPa,
    mole_per_square_meter_second as mol_m2_s,
    mole_per_second as mol_s,
    gram_carbon as gC,
    ratio,
    seconds,
}

type Potential : Scalar<MPa>
type Conductance : Scalar<mol_m2_s> { self >= 0 }
type WaterFlux : Scalar<mol_s> { self >= 0 }
type CarbonMass : Scalar<gC> { self >= 0 }

node TinyTree {
    vpd_scale: Potential
    soil_water: Potential
    hydraulic_cond: Conductance

    water: Potential { self <= 0 MPa }
    carbon: CarbonMass

    stomata: Conductance { self <= g_max }
    transpiration: WaterFlux
    g_max: Conductance

    dt: Scalar<seconds>
}

relation demand_transpiration:
    transpiration = stomata * vpd_scale

relation supply_transpiration:
    transpiration = hydraulic_cond * (soil_water - water)

slot controller provides [stomata]:
    inputs = [water, carbon, vpd_scale, soil_water, hydraulic_cond, g_max]

temporal water_step:
    water[t+1] = water[t] - dt * transpiration[t]
```

### A.2 Training workflow

```python
import myco

model = myco.load("plant/tiny_tree.myco")
experiment = model.experiment(mode="train", horizon_steps=64)

experiment.assume_series("vpd_scale", range(64))
experiment.assume_series("soil_water", range(64))
experiment.assume_constant("hydraulic_cond")
experiment.assume_constant("g_max")
experiment.assume_constant("dt", value=1800.0)
experiment.assume_initial("water")
experiment.assume_initial("carbon")
experiment.learn_slot("controller")
experiment.observe_dense("transpiration")
experiment.observe_sparse("water", range(0, 64, 8))

artifact = experiment.compile(backend="jax")
```

### A.3 Simulation workflow

Same model, different binding:

```python
experiment = model.experiment(mode="simulate", horizon_steps=64)

experiment.assume_series("vpd_scale", range(64))
experiment.assume_series("soil_water", range(64))
experiment.assume_constant("hydraulic_cond")
experiment.assume_constant("g_max")
experiment.assume_constant("dt", value=1800.0)
experiment.assume_initial("water")
experiment.assume_initial("carbon")
experiment.bind_slot("controller", "path/to/trained_controller")

artifact = experiment.compile(backend="jax")

# Inspect the compilation plan (section 14.5)
print(artifact.plan)
```

No observations, no loss helpers. The same structural model produces a different
compiled artifact because the binding changed. The plan shows the execution
strategy: SCCs, solver choices, slot bindings, and execution order.

---

## Appendix B: Worked Example — Sperry Hydraulic-Stomatal Model

See `mock_sperry.myco` for the full mock implementation. Key features exercised:

- **Contracts**: `VulnerabilityCurve` with `WeibullVC`, `SigmoidVC`, and
  `VanGenuchtenVC` implementations; function-like invocation with explicit
  arguments; wiring pattern for multi-input `Photosynthesis` contract
- **Generics**: `XylemSegment<V: VulnerabilityCurve>`,
  `SperryTree<V, P, N_SOIL, N_CANOPY>`, `LeafGasExchange<P: Photosynthesis>`
- **Generic functions**: `arrhenius<U: Unit>`, `peaked_arrhenius<U: Unit>` —
  unit-polymorphic temperature response functions
- **Const generics**: parameterized soil layers and canopy layers (sun/shade),
  const arithmetic in array sizes (`[T; M-1]`)
- **Algebraic loops**: hydraulic flow-pressure coupling, Farquhar A-C_i
  coupling, energy balance T_leaf-E coupling — all discovered automatically by
  the planner
- **Temporal accumulators**: `min` for irreversible cavitation tracking
- **Structural introspection**: `temporal cavitation[seg in pathway where seg is
  XylemSegment]` — type-filtered subtree iteration
- **Conditional expressions**: `if j > 0 then ... else 0` in soil water step
- **Overconstrained quantities**: supply vs demand transpiration — handled by closure policies (section 14.6)
- **Full-graph slot inputs**: `inputs = [*]` for the stomatal controller, with
  the slot joining the hydraulic SCC when its output feeds the loop
- **Pluggable controllers**: slot can be filled by gain-risk optimization,
  Ball-Berry model, or learned neural network
- **Affine unit transforms**: Temperature declared in degC, compiler handles
  Kelvin conversion in Arrhenius functions and Stefan-Boltzmann term
- **Normal `dt`**: timestep is a declared quantity, not a magic name

**Abstraction boundaries worth noting:**

- `VulnerabilityCurve` is a **single-input contract** (pressure → PLC). This
  covers the dominant paradigm in plant hydraulics (Sperry, Tyree). Multi-driver
  hydraulics (e.g., freeze-thaw embolism depending on both pressure and
  temperature, or ABA-regulated aquaporin conductance) would use a richer
  contract with additional inputs — the same generic mechanism, just a different
  contract. This is extension, not redesign.

- **Multi-output relations** are not supported as a general mechanism. A
  relation defines a single equality. Coupled multi-output computations are
  expressed via contracts (which have multiple named outputs) or via multiple
  relations that the planner couples into an SCC. This is a deliberate design
  choice — contracts handle the common case cleanly.

---

## Appendix B.2: Worked Example — Potkay GOSM (Carbon-Water-Turgor Coupling)

See `mock_potkay.myco` for the full mock implementation of Potkay & Feng (2023),
"Do stomata optimize turgor-driven growth?" This model stress-tests features
beyond those exercised by the Sperry mock:

- **Carbon-turgor coupling across timesteps**: NSC dynamics feed the Lockhart
  growth equation, which feeds back into the carbon balance. The substrate
  limitation functions (sigma_g, sigma_r) throttle both growth and respiration
  based on NSC reserves.
- **Library reuse**: imports `VulnerabilityCurve`, `SigmoidVC`,
  `ConductingElement`, `XylemSegment` from `plant::hydraulics` and
  `Photosynthesis`, `FarquharC3` from `plant::photosynthesis` — the same
  contracts and implementations used in the Sperry mock.
- **Piecewise registered functions**: `mean_turgor_excess` with
  `differentiability: subgradient` annotation, exercising the conditional
  expression system and differentiability metadata.
- **Q10 temperature response**: an alternative to Arrhenius for maintenance
  respiration, as a generic function `q10_response<U: Unit>`.
- **Peaked Arrhenius with cold limit**: `extensibility_temperature` for the
  cell wall extensibility, with a smooth sigmoid ramp to zero below 5C.
- **Phloem osmotic potential**: computed from stem water potential via an
  empirical phloem molality relation, creating an additional algebraic coupling
  path between water status and turgor.
- **GOH as slot training objective**: stomatal control is a slot; the growth
  optimization hypothesis (maximize integral of G over lifetime) becomes the
  training objective in the workflow layer, not a hardcoded optimality condition.

**Candidate use of `integrate` (section 9.5):** The `mean_turgor_excess`
registered function (Eqn 7 of the paper) hardcodes the analytical solution to a
piecewise-linear integral. With `integrate`, this could be expressed directly:

```myco
G_0 = phi * (C_wood / u_s)
    * integrate(max(P_apex + (P_base - P_apex) * z - turgor_threshold, 0), z, 0, 1)
```

The compiler would attempt symbolic resolution (the integrand is piecewise-
linear, so a closed form exists) and fall back to numerical quadrature if
needed.

**Candidate use of `deriv` (section 9.5):** The paper's optimality condition
(Eqn 8) requires the marginal carbon cost of water: chi_w = dG/dE. In the mock,
this is handled by the slot mechanism (the slot is trained to maximize growth,
and the optimality condition emerges from training). For models that use
analytical optimality rules (e.g., Potkay et al. 2021, THORP), `deriv` would
express the marginal gain directly:

```myco
let dA_dgs = deriv(gas.photo.assimilation, gas.g_s)
let dE_dgs = deriv(hydraulics.transpiration, gas.g_s)
marginal_carbon_cost_of_water = dA_dgs / dE_dgs
```

---

## Appendix C: Developer Experience

These are not core language features but are essential for making Myco pleasant
and productive to use.

### C.1 VSCode syntax highlighting

A TextMate grammar for `.myco` files providing syntax highlighting in VSCode
(and other editors that support TextMate grammars). This is low effort and
high impact — colored keywords, strings, numbers, comments, and type
annotations make `.myco` files immediately more readable.

### C.2 Language Server Protocol (LSP)

An LSP server for `.myco` files enabling:

- **Go-to-definition**: click on a type, contract, node, or function name to
  jump to its declaration
- **Hover information**: hover over a quantity to see its type, unit, and
  constraints; hover over a relation to see which quantities it connects
- **Autocomplete**: path completion (`pathway.stem.` suggests `core`,
  `min_historical_pressure`, etc.), contract field completion, import
  suggestions
- **Inline diagnostics**: type errors, unit mismatches, and constraint
  violations shown as you type
- **Rename symbol**: rename a node, type, or quantity across all files

This is the single most impactful developer experience feature. Syntax
highlighting makes files readable; LSP makes them navigable.

### C.3 Formatter

A canonical formatter for `.myco` files (like `rustfmt` or `gofmt`). Enforces
consistent indentation, line width, spacing, and ordering of node members.
Run as `myco fmt` or on save in the editor.

Opinionated formatting removes style debates and makes diffs cleaner.

### C.4 Doc comments and documentation generation

Support `///` doc comments on nodes, types, contracts, functions, and fields:

```myco
/// Weibull vulnerability curve.
///
/// Maps water potential to fractional loss of hydraulic conductivity
/// using the standard Sperry parameterization.
pub node WeibullVC : VulnerabilityCurve {
    /// Weibull scale parameter (related to P50)
    b: PositiveScalar
    /// Weibull shape parameter (>1 sigmoidal, =1 exponential)
    c: PositiveScalar

    plc = 1.0 - exp(-(-pressure / b) ** c)
}
```

Generate browsable HTML documentation for library packages (like `rustdoc`).
Documentation should include:

- Type signatures with units
- Contract interfaces and their implementations
- Constraint listings
- Cross-references between related items

### C.5 Graph rendering architecture

Both plan visualization and model graph visualization share a common rendering
architecture. The compiler emits a **backend-agnostic graph intermediate
representation** — a JSON format with nodes, edges, clusters, and metadata
(SCC membership, solver strategy, path selection, constraint type, etc.). Thin
adapters render this IR to different targets:

```python
plan = experiment.explain_plan()

# Static output
plan.graph.to_dot("plan.dot")          # Graphviz (.dot)
plan.graph.to_d2("plan.d2")            # D2 diagramming language
plan.graph.to_mermaid()                # Mermaid string (for markdown/GitHub)

# Interactive
plan.graph.serve()                      # Cytoscape.js in browser
```

**Rendering targets:**

- **Graphviz**: battle-tested DAG layout. SCC clusters map to
  `subgraph cluster_*`. Best for static plan diagrams. CLI:
  `myco plan --dot | dot -Tsvg > plan.svg`
- **D2**: modern text-to-diagram with better default styling. Good for
  documentation and presentations
- **Mermaid**: renders in GitHub markdown, VSCode preview, Jupyter notebooks.
  Best for inline documentation
- **Cytoscape.js**: JavaScript graph library with pan/zoom, filtering, and
  multiple layout algorithms (hierarchical for containment, force-directed for
  constraints). Best for interactive exploration of large models

The JSON IR means any new renderer (vis.js, Excalidraw, custom WebGL, etc.)
can be added without changing the compiler.

### C.6 Plan visualization

After compilation, render the execution plan:

- Quantities are nodes; computational dependencies are edges
- SCCs are highlighted as clusters with labeled solver strategy (linear,
  polynomial, Newton-Raphson)
- Overconstrained components show closure policy and residual factors (with
  path costs from the operation algebra)
- Slot boundaries are visible, with SCC membership indicated
- Temporal equations shown as a separate layer

Essential for debugging "why did the compiler choose this path?" and for
understanding how complex models decompose into solver blocks.

### C.7 Model graph visualization

Render the structural containment tree and constraint graph:

- Containment tree shows parent-child relationships (collapsible tree view
  in VSCode sidebar via LSP extension)
- Constraint graph shows cross-node couplings as edges
- Color-code by node type, contract implementation, or constraint kind
- Filterable — show only hydraulic quantities, only constraints, etc.
- Interactive Cytoscape.js view for large models (50+ quantities)

For a model like Sperry, visual structure is the fastest way to understand
the model.

### C.8 Interactive exploration (REPL)

An interactive mode for incremental model exploration:

```
$ myco repl sperry/mechanics.myco
myco> :bindings
  [nothing bound yet]
myco> :assume atm.co2 = 40 Pa
myco> :assume atm.temperature = 25 degC
myco> :computable
  [lists quantities computable from current bindings]
myco> :unresolved
  [lists quantities that still need bindings]
myco> :plan stomata
  [shows the dependency chain for computing stomata]
```

This supports iterative workflow development — the user progressively adds
bindings and sees what becomes computable. Faster than edit-compile-run cycles
for understanding model structure.

### C.9 Package registry

A registry for sharing and discovering Myco library packages:

- Publish packages with contracts, implementations, and helper functions
- Semantic versioning for compatibility
- Dependency resolution
- Searchable by domain (hydraulics, photosynthesis, soil physics, etc.)

This is what turns Myco from a single-user tool into an ecosystem. A plant
physiologist publishes a Farquhar implementation; a hydrologist publishes soil
models; a modeler composes both without reimplementing either.

The registry should support:

- `myco add sperry-hydraulics` — add a dependency
- `myco publish` — publish a package
- `myco search "vulnerability curve"` — find packages

### C.10 Compilation diagnostics

Error messages should be clear, specific, and actionable:

- **Source spans**: point to the exact `.myco` line and column
- **Causal chains**: "quantity X is underdetermined because relation Y requires
  Z, which is not provided by any binding or relation"
- **Suggestions**: "did you mean to assume `soil.layers[0].water_potential`?"
- **Unit mismatch details**: "left side has dimension [pressure], right side
  has dimension [conductance * pressure] — did you forget to divide by
  conductance?"
- **SCC diagnostics**: "relations R1, R2, R3 form an algebraic loop involving
  quantities Q1, Q2 — the compiler will emit a Newton-Raphson solver"

The Rust compiler's error messages are the gold standard here.

---

## Appendix D: Implementation Priority

The following is a suggested implementation order based on dependency structure.
Items earlier in the list are prerequisites for items later.

**Core language:**

1. **Nodes and types** (sections 2, 3) — the structural core
2. **Units and dimensions** (section 4) — needed by types, including affine
   transforms
3. **Constraint language** (section 5) — needed by types and nodes
4. **Relations and temporal** (section 6) — the equation layer
5. **Contracts with function-like invocation** (section 3.4) — trait system
6. **Generics and `dyn`** (sections 2.4, 2.5) — parameterized structure
7. **Slots** (section 7) — declared interfaces with SCC participation

**Math substrate:**

8. **Operation algebra** (section 8) — metadata for all operations
9. **Function registry** (section 9) — user-defined operations with inverse
   verification

**Metaprogramming:**

10. **Declarative macros** (section 18.1) — template expansion
11. **Structural introspection and type-aware `where`** (sections 5.4, 5.5) —
    compile-time meta-programming
12. **Derive macros** (section 18.2) — annotation-driven code generation

**Compiler pipeline:**

13. **Flattening pass** (section 10) — macro expansion, `dyn` monomorphization,
    structural expansion
14. **Planning with SCC detection** (section 12) — causal ordering + loop
    discovery
15. **JAX emitter with solver emission** (section 13) — code generation
16. **Compiler configuration** (section 14) — solver strategy, closure policies
17. **Constraint analysis** (section 11) — static reasoning, property
    verification, no-trust enforcement

**Workflow layer:**

18. **Binding vocabulary** (section 15) — path-based workflow binding
19. **Modules and visibility** (section 1) — namespacing, `pub`, lib vs model
20. **Learned trajectories** (section 16) — structured latent variables
21. **Study-level training** (section 17) — multi-experiment joint learning

**Developer experience** (can be developed in parallel with the above):

22. **VSCode syntax highlighting** (appendix C.1) — TextMate grammar, low
    effort / high impact
23. **Compilation diagnostics** (appendix C.9) — clear errors with source spans
24. **Formatter** (appendix C.3) — `myco fmt`
25. **LSP server** (appendix C.2) — go-to-definition, hover, autocomplete
26. **Plan visualization** (appendix C.5) — dependency graph rendering
27. **Model graph visualization** (appendix C.6) — containment + constraint
    graph
28. **Interactive REPL** (appendix C.7) — incremental exploration
29. **Doc comments and generation** (appendix C.4) — `///` comments, HTML docs
30. **Package registry** (appendix C.8) — publish, discover, depend on packages
