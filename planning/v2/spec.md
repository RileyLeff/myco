# Myco V2 Language Specification

This is the working specification for Myco V2. It covers the world-model
language, the mathematical substrate, the compiler internals, and the workflow
layer.

For earlier design exploration that led to this spec, see `v2_prep/`.

For the Sperry model mock implementation that stress-tested this spec, see
`mock_sperry.myco`.

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

#### 1.3 Circular imports

Circular imports are disallowed. The module dependency graph must be a DAG. The
compiler reports a cycle with the full import chain if one is detected.

### 2. Nodes

A node is the single structural primitive of the world model. A node is an
instance of a type — it occupies a specific position in the containment tree
and represents a concrete thing in the world.

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

Type parameters must satisfy a declared contract (see section 3.3). Const
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

The count is a const generic or a literal. Variable-length collections are out
of scope.

### 3. Types

A type declares what must be true about a structural pattern. Types are
reusable definitions; nodes are instances. This is analogous to the
struct/instance distinction. If you imagine 100 trees, they all share the type
`Tree<V, P>`; each tree is a node.

#### 3.1 Scalar types

A scalar type wraps a numeric value with a unit:

```myco
type Potential : Scalar<MPa>
type Conductance : Scalar<mol_m2_s> { self >= 0 }
```

Constraints on a scalar type are predicates over `self` (see section 5).

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

**Contract invocation is function-like.** A contract implementation maps inputs
to outputs. Invocation always passes arguments explicitly:

```myco
vc(current_pressure).plc            // evaluate at current_pressure
vc(historical_pressure).plc         // evaluate at a different pressure
```

There is no implicit "current context" binding. This keeps the semantics
mathematical and predictable — a contract implementation is a function with
named outputs, not a stateful object.

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

An implementation may override any default. If it doesn't, the default relation
is included automatically. This avoids repeating `conductance_fraction = 1.0 -
plc` in every VC implementation.

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

Units are first-class, not string labels.

#### 4.1 Dimensions vs units

A dimension is a physical kind: pressure, conductance, ratio, area.

A unit is a specific scale within a dimension: MPa, Pa, kPa are all units of
pressure.

The system tracks both. Two quantities with the same dimension but different
units are compatible via conversion. Two quantities with different dimensions
are incompatible.

#### 4.2 Import and definition

Standard unit systems are importable:

```myco
use units::si::{
    megapascal as MPa,
    mole_per_square_meter_second as mol_m2_s,
    ratio,
}
```

Derived units can be defined locally:

```myco
unit mmol_m2_s = 1e-3 * mol_m2_s
```

#### 4.3 Affine unit transforms

Some unit conversions are affine (offset + scale) rather than purely
multiplicative. The canonical example: Celsius to Kelvin.

```myco
unit kelvin
unit celsius : affine(kelvin, offset=-273.15)
```

The compiler handles affine conversions automatically. When a relation mixes
Celsius and Kelvin quantities, the compiler inserts the correct conversion. This
eliminates manual `+ 273.15` scattered through model code — the Arrhenius
functions in the Sperry mock, for example, should accept a `Temperature` type
and the compiler handles the conversion to absolute temperature internally.

Affine transforms also cover other offset-based systems (Fahrenheit, gauge
pressure vs absolute pressure).

#### 4.4 Compile-time checking

The compiler checks dimensional consistency in relations:

- Both sides of an equation must have the same dimension
- Arithmetic operations follow standard dimensional analysis rules
  (multiplication multiplies dimensions, addition requires matching dimensions)
- Type annotations on nodes provide the expected unit
- Binding-time validation checks that supplied data matches expected units

#### 4.5 What is out of scope

- Arbitrary symbolic unit algebra in the source language
- Full theorem-prover-level unit reasoning

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

The predicate language supports:

- **Arithmetic**: `+`, `-`, `*`, `/`, `**`
- **Comparison**: `<`, `<=`, `=`, `>=`, `>`
- **Logical connectives**: `and`, `or`, `not`, `implies`
- **Quantifiers**: `forall`, `exists` over index ranges
- **Comprehensions**: `sum`, `count`, `mean`, `min`, `max` with `where`
  filtering
- **Let bindings**: for readability, not mutation
- **Function calls**: any function from the registry (section 9)

Note: `=` in constraints means equality (the same as in relations). There is no
assignment operator in Myco. The compiler may solve in either direction.

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

Both `is` and `has` are compile-time predicates. They are only valid in model
modules where concrete types are known.

#### 5.5 Structural introspection

Constraints may quantify over the fields of a node:

```myco
constraint all_finite:
    forall field in self.fields where field.type <: Scalar:
        is_finite(field)
```

This requires the set of fields a node owns to be well-defined at compile time,
which the containment model guarantees. After monomorphization, `self.fields`
is a concrete list. The `forall` expands to one constraint per matching field.

Structural introspection is also available in derive macros (see section 18).

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

Constraints compose conjunctively. All constraints from a node, its type, and
all containing scopes must hold simultaneously. There is no override or
relaxation mechanism.

### 6. Relations

Relations connect quantities across nodes. They are the equations of the world.

All relations must hold simultaneously. The user does not annotate which
equations form coupled systems — the compiler discovers this automatically (see
section 12.5).

The `=` in a relation means equality. Both sides are symmetric and the compiler
may solve in either direction. There is no assignment operator in Myco.

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

#### 6.4 Overdetermined quantities

A quantity may be computable by more than one relation. This is intentional and
is one of the core reasons Myco exists.

When a quantity is overdetermined:

- The planner selects one relation as the canonical computation path
- Remaining relations become alternative paths
- The compiler may emit consistency losses from the alternatives (see section
  12)
- Path selection is informed by the operation algebra (see section 8)

### 7. Slots

A slot is a declared interface for a component that will be provided at workflow
time.

#### 7.1 Slot declaration

```myco
slot stomatal_control provides [stomata]:
    inputs = [*]
```

The slot declares what it provides and what it needs. The `[*]` wildcard means
"all quantities available at this point in the plan." The compiler resolves this
to a concrete list during planning.

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
- Choose canonical paths for overdetermined quantities
- Reject plans that route a `train`-mode compilation through a
  `non_differentiable` operation

#### 8.5 Conditioning-aware path selection

For overdetermined quantities where multiple paths exist, the emitter may
generate conditioning-aware code that dynamically weights paths based on
numerical stability.

The default algorithm uses condition-number-based weighting: each path is
evaluated, and paths whose intermediate quantities are better-conditioned
receive higher weight. The blend is smooth (differentiable) so gradients flow
through both paths.

Configuration via compiler config (section 14):

```python
artifact = experiment.compile(
    backend="jax",
    path_blending={
        "enabled": True,                  # default: True
        "sharpness": 10.0,                # higher = sharper selection
        "method": "condition_number",      # default method
    },
)
```

When `sharpness` is very high, this approximates hard path selection. When low,
it's a soft blend. The default provides both numerical stability in the forward
pass and well-behaved gradients in the backward pass.

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

#### 9.3 Inverse verification

If a user declares an explicit inverse, the compiler verifies it via round-trip
testing at compile time: for a set of sample inputs in the declared domain, the
compiler checks that `inverse(f(x)) ≈ x` within numerical tolerance. If
verification fails, the compiler errors with the failing test cases.

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
pub fn peaked_arrhenius(
    value_25: PositiveScalar,
    ha: Scalar<J_mol>,
    hd: Scalar<J_mol>,
    sv: Scalar<J_mol>,
    temperature: Temperature,
) -> PositiveScalar { ... }
```

```myco
// In a user's model
use physiology::temperature::peaked_arrhenius
```

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

The baseline internal strategy is interval propagation over the constraint
graph. This is:

- Linear in the number of constraints per pass
- Typically converges in a small number of fixed-point iterations
- Sufficient for most bound-based and linear constraints
- Cheap enough to run on every compilation

For constraints involving logical connectives (`implies`, `or`) or nonlinear
operations, the compiler may fall back to conservative approximation. If
conservative approximation is insufficient, the compiler errors (it does not
silently assume the constraint holds).

#### 11.3 Runtime interaction

Where static proof succeeds, the compiler may elide runtime checks. Where
explicit acknowledgment is given (`#[verified_externally]`), runtime behavior
depends on mode:

- In `simulate` mode: runtime assertions that raise on violation
- In `train` mode: differentiable penalty losses (soft constraint enforcement)
- In both modes: diagnostic reporting for constraint violations

#### 11.4 Interaction with algebraic loops

Proven bounds from the constraint analysis can improve solver behavior for
algebraic loops (section 12.5). If the compiler can prove that a quantity in an
SCC is bounded to a narrow range, that information can:

- Provide better initial guesses for numerical solvers
- Enable analytical simplification by narrowing the search space
- Allow the compiler to prove solver convergence for some systems

#### 11.5 Upgrade path

The constraint analysis system should be designed so that richer abstract
domains (polyhedra, symbolic predicates) can be added later without changing the
user-facing constraint language.

### 12. Planning

The planner takes the flat graph from the flattening pass and produces an
ordered execution plan for a single timestep.

#### 12.1 Core algorithm

The planning algorithm is fundamentally the same as v1:

1. Start from what is directly available (assumed quantities, slot outputs,
   temporal carry-forward)
2. Determine which relations are buildable (all dependencies satisfied)
3. Use the e-graph equality core to extract candidate expressions
4. Choose a canonical path for each computable quantity
5. Record alternative paths for consistency loss
6. Separately handle temporal equations for `t -> t+1`

#### 12.2 Operation algebra integration

Path selection is informed by the operation algebra:

- Prefer `bijective`, `smooth` paths over `injective_restricted` or `fragile`
  ones
- Assign higher cost to inversions through ill-conditioned operations
- Reject inversions through `lossy` or `opaque` operations
- In `train` mode, reject canonical paths through `non_differentiable`
  operations

#### 12.3 Overdetermined quantities

When a quantity can be computed by multiple relations:

- The lowest-cost path becomes canonical
- Remaining paths become alternatives
- The compiler may emit consistency losses from the alternatives
- The compiler may emit conditioning-aware path blending (section 8.5)

#### 12.4 Underdetermined quantities

When a quantity cannot be computed from assumptions and the model structure
alone, the planner reports it as unresolved. The workflow layer must address it
via:

- Additional assumptions
- A learned trajectory binding (section 16)
- A learned constant binding

If an unresolved quantity remains after all bindings are applied, compilation
fails with a diagnostic.

#### 12.5 Algebraic loop detection

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
- Each SCC is treated as an implicit system that must be solved simultaneously.
- If a slot's output feeds into an SCC, the slot is part of that SCC (see
  section 7.3).

**Classification**: The planner classifies each SCC by examining the structure
of its equations:

- **Linear**: all relations are linear in the SCC unknowns. Solve with direct
  linear algebra (LU decomposition).
- **Polynomial**: relations are polynomial in the SCC unknowns. For
  single-unknown low-degree systems, attempt analytical solution. Otherwise,
  numerical solve.
- **General nonlinear**: the default case. Requires a numerical solver.

**Solver emission**: For each SCC requiring a numerical solve, the emitter
generates backend-appropriate solver code:

- **JAX**: `jax.lax.custom_root` or a Newton-Raphson loop with
  `jax.jacfwd` for the Jacobian. Implicit differentiation via the implicit
  function theorem provides exact gradients.
- **Rust**: standard NR with LU decomposition.
- **PyTorch**: `torch.autograd.Function` with implicit differentiation.

**Binding-dependent loops**: Different bindings may produce different SCCs from
the same model. If a quantity in a loop is assumed as a constant, the loop may
break into acyclic components. The planner handles this naturally — SCC analysis
runs on the dependency graph after bindings are applied.

### 13. Code Emission / Backends

The emitter takes the plan and produces executable source code for a specific
backend.

#### 13.1 Plan representation

The plan is backend-agnostic: an ordered list of computation steps (including
solver blocks for SCCs), each with dependencies, expressions, and metadata
(which path, whether it's canonical or alternative, operation algebra
annotations).

#### 13.2 JAX emitter (primary)

The JAX emitter produces a Python module using:

- `jax.numpy` for array operations
- `jax.lax.scan` for rollout
- `jax.nn` for smooth projections
- `jax.lax.custom_root` for implicit solves within a step
- Standard pytree conventions for state and parameters

The emitted module includes:

- `step()`, `rollout()` (same as v1)
- `obs_loss()`, `consistency_loss()`, `constraint_violation_loss()`,
  `soft_penalty_loss()`, `loss_components()`, `total_loss()` (for `train` mode)
- `init_params()`, `validate_rollout_inputs()`, `validate_observations()`
- Metadata constants and slot interface declarations

The emitter uses differentiability metadata from the operation algebra to:

- Choose smooth approximations where needed (e.g., `softplus` instead of
  `relu` for projections)
- Warn about fragile gradient paths
- Reject `train`-mode plans with non-differentiable canonical paths

#### 13.3 Backend interface

The compiler defines a backend interface that any emitter must implement. This
allows future backends (PyTorch, Rust, Rust+PyO3 hybrid) to be added without
modifying the planner or flattener.

The interface requires:

- Emit scalar computation steps
- Emit solver blocks for SCCs (with backend-appropriate solver)
- Emit rollout/scan structure for temporal equations
- Emit loss functions for `train` mode
- Emit parameter initialization

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

#### 14.2 Path blending configuration

See section 8.5 for the path blending algorithm and configuration options.

#### 14.3 Other configuration

- Constraint enforcement mode (project vs penalize)
- Debug mode (extra runtime checks and diagnostics)
- Property verification strictness (how many sample points for round-trip
  inverse testing, symbolic analysis depth)

These are analogous to optimization levels in a C compiler: they affect how the
code runs, not what it computes.

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
```

See section 16 for details on learned trajectories.

#### 15.4 Slot binding

```python
# Import a published controller
experiment.bind_slot("stomatal_control", "sperry/controllers/gain_risk")

# Or supply raw data for the slot's outputs
experiment.assume_series("stomata", observed_stomata_data)
```

#### 15.5 Path-based binding

All binding operations accept paths with wildcards:

```python
experiment.assume_constant("canopy.leaves[*].jmax")
experiment.observe_sparse("canopy.leaves[*].water", steps)
```

Wildcards expand to all matching instances in the flattened graph.

#### 15.6 Unit validation

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

#### 16.4 Compiler support

The compiler treats a learned trajectory similarly to an assumed series, except:

- Its values are learnable parameters (included in the gradient computation)
- The emitter allocates parameter arrays for the trajectory representation
- Constraint penalties are added to the loss

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
    obs_loss_k + consistency_loss_k + constraint_penalty_k
```

Each experiment compiles to its own artifact. The controller parameters are
shared. The joint gradient is the sum of per-experiment gradients.

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
```

No observations, no loss helpers. The same structural model produces a different
compiled artifact because the binding changed.

---

## Appendix B: Worked Example — Sperry Hydraulic-Stomatal Model

See `mock_sperry.myco` for the full mock implementation. Key features exercised:

- **Contracts**: `VulnerabilityCurve` with `WeibullVC` and `SigmoidVC`
  implementations; function-like invocation with explicit arguments
- **Generics**: `XylemSegment<V: VulnerabilityCurve>`,
  `SperryTree<V, N_SOIL, N_CANOPY>`
- **Const generics**: parameterized soil layers and canopy layers (sun/shade)
- **Algebraic loops**: hydraulic flow-pressure coupling, Farquhar A-C_i
  coupling, energy balance T_leaf-E coupling — all discovered automatically by
  the planner
- **Temporal accumulators**: `min` for irreversible cavitation tracking
- **Overdetermined quantities**: supply vs demand transpiration
- **Full-graph slot inputs**: `inputs = [*]` for the stomatal controller, with
  the slot joining the hydraulic SCC when its output feeds the loop
- **Pluggable controllers**: slot can be filled by gain-risk optimization,
  Ball-Berry model, or learned neural network
- **Normal `dt`**: timestep is a declared quantity, not a magic name

---

## Appendix C: Implementation Priority

The following is a suggested implementation order based on dependency structure.
Items earlier in the list are prerequisites for items later.

1. **Nodes and types** (sections 2, 3) — the structural core
2. **Units and dimensions** (section 4) — needed by types, including affine
   transforms
3. **Constraint language** (section 5) — needed by types and nodes
4. **Relations and temporal** (section 6) — the equation layer
5. **Contracts with function-like invocation** (section 3.4) — trait system
6. **Generics and `dyn`** (sections 2.4, 2.5) — parameterized structure
7. **Slots** (section 7) — declared interfaces with SCC participation
8. **Operation algebra** (section 8) — metadata for all operations
9. **Function registry** (section 9) — user-defined operations with inverse
   verification
10. **Declarative macros** (section 18.1) — template expansion
11. **Flattening pass** (section 10) — bridge to the planner, including macro
    expansion and `dyn` monomorphization
12. **Planning with SCC detection** (section 12) — causal ordering + loop
    discovery
13. **JAX emitter with solver emission** (section 13) — code generation
14. **Compiler configuration** (section 14) — solver strategy, path blending
15. **Constraint analysis** (section 11) — static reasoning, property
    verification, no-trust enforcement
16. **Binding vocabulary** (section 15) — path-based workflow binding
17. **Modules and visibility** (section 1) — namespacing, `pub`, lib vs model
18. **Structural introspection and type-aware `where`** (sections 5.4, 5.5) —
    compile-time meta-programming
19. **Derive macros** (section 18.2) — annotation-driven code generation
20. **Learned trajectories** (section 16) — structured latent variables
21. **Study-level training** (section 17) — multi-experiment joint learning
