# Myco V2 Language Specification

This is the working specification for Myco V2. It covers the world-model
language, the mathematical substrate, the compiler internals, and the workflow
layer.

For earlier design exploration that led to this spec, see `v2_prep/`.

For the Sperry model mock implementation that stress-tested this spec, see
`mock_sperry.myco`.

## Status

This document is a draft. Sections marked *deferred* describe features that
should be designed for but not implemented until a concrete model forces them.

## Design Philosophy

The `.myco` representation should approach the minimum description length of
the science. If the implementation complexity vastly exceeds the description
complexity, the gap is incidental complexity that belongs in the compiler, not
in the model.

The model describes what is true about the world. The compiler figures out how
to compute it. The user never annotates solution strategies, solver choices, or
execution order — those are compiler concerns.

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

Modules may re-export items from other modules. The exact visibility rules
(public by default vs explicit `pub`) are deferred until the package registry
design is needed.

#### Open questions

- Should there be a distinction between library modules (reusable components)
  and model modules (top-level entry points for compilation)?
- How should circular imports be handled? Likely: disallow them.

### 2. Nodes

A node is the single structural primitive of the world model.

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

#### 2.5 Repeated structure

Arrays of nodes use fixed-size syntax:

```myco
leaves: [Leaf<P>; N]
layers: [SoilLayer; M]
```

The count is a const generic or a literal. Variable-length collections are out
of scope.

### 3. Types

A type declares what must be true about what a node owns.

#### 3.1 Scalar types

A scalar type wraps a numeric value with a unit:

```myco
type Potential : Scalar<MPa>
type Conductance : Scalar<mol_m2_s> { self >= 0 }
```

Constraints on a scalar type are predicates over `self` (see section 5).

#### 3.2 User-defined types

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

The distinction between a "type" and a "node" is: a type is a reusable
structural pattern. A node is an instance of a type (or an anonymous structure)
in the world model. This is analogous to the struct/instance distinction.

#### 3.3 Contracts

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

Contracts enable generic subsystem swapping:

```myco
node Leaf<P: Photosynthesis> {
  photo: P
  // ...
}
```

Different photosynthesis implementations (C3, C4, CAM) can be plugged in
without changing the containing structure.

#### 3.3.1 Worked example: Vulnerability curves

A vulnerability curve maps water potential to fractional loss of hydraulic
conductivity. The scientific community uses multiple parameterizations. This is
a natural contract:

```myco
contract VulnerabilityCurve {
  input pressure: WaterPotential
  output plc: Fraction

  constraint monotonic:
    forall p1, p2 where p1 < p2:
      plc(p1) <= plc(p2)
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
    conductance == k_max * (1.0 - vc.plc)
}
```

This allows users to swap vulnerability curve implementations without changing
the plant model.

#### 3.4 Open questions

- Can types own constraints over nodes they don't directly contain? Likely: no.
  Cross-node constraints belong at the containing scope.
- Should contracts support default implementations for some fields? Deferred.
- Should types themselves be generic? Likely: yes, `type BoundedScalar<U: Unit>
  : Scalar<U> { self >= 0 }`.

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

#### 4.3 Compile-time checking

The compiler checks dimensional consistency in relations:

- Both sides of an equation must have the same dimension
- Arithmetic operations follow standard dimensional analysis rules
  (multiplication multiplies dimensions, addition requires matching dimensions)
- Type annotations on nodes provide the expected unit
- Binding-time validation checks that supplied data matches expected units

#### 4.4 What is deferred

- Nonlinear unit systems (temperature offsets, decibels)
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
    water < turgor_loss_point implies stomata == 0
}
```

#### 5.2 Available operations

The predicate language supports:

- **Arithmetic**: `+`, `-`, `*`, `/`, `**`
- **Comparison**: `<`, `<=`, `==`, `>=`, `>`
- **Logical connectives**: `and`, `or`, `not`, `implies`
- **Quantifiers**: `forall`, `exists` over index ranges
- **Comprehensions**: `sum`, `count`, `mean`, `min`, `max` with `where`
  filtering
- **Let bindings**: for readability, not mutation
- **Function calls**: any function from the registry (section 9)

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
  count(active) == 0 or
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

#### 5.4 Composition

Constraints compose conjunctively. All constraints from a node, its type, and
all containing scopes must hold simultaneously. There is no override or
relaxation mechanism.

#### 5.5 Deferred: structural introspection

A future extension may allow quantification over the fields of a node:

```myco
constraint all_finite:
  forall field in self.fields where field.type <: Scalar:
    is_finite(field)
```

This requires the set of fields a node owns to be well-defined at compile time,
which the current design supports. The feature itself is deferred past v2.

### 6. Relations

Relations connect quantities across nodes. They are the equations of the world.

All relations must hold simultaneously. The user does not annotate which
equations form coupled systems — the compiler discovers this automatically (see
section 12.5).

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
    canopy.leaves[i].water[t] - dt * canopy.leaves[i].transpiration[t]
```

A quantity that appears on the left-hand side of a temporal relation is
automatically inferred as persistent (requires initial state in the workflow
binding).

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
- **Bound**: an imported controller implementation from a package or module.
- **Assumed**: raw data supplied directly for the slot's output quantities.

This means a model package can ship with the base mechanics and multiple
optional controllers:

```
sperry/
  mechanics.myco              # hydraulics, photosynthesis, carbon balance
  controllers/
    gain_risk.myco            # Sperry gain-risk optimization
    ball_berry.myco           # Ball-Berry empirical model
    medlyn.myco               # Medlyn optimality model
```

And the user's workflow becomes:

```python
model = myco.load("sperry/mechanics.myco")

# For synthetic data generation with Sperry's original criterion:
experiment.bind_slot("stomatal_control", "sperry/controllers/gain_risk")

# For learning from data:
experiment.learn_slot("stomatal_control")

# Or just plug in observed stomata directly:
experiment.assume_series("stomata", observed_stomata_data)
```

#### 7.3 Input introspection

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
numerical stability:

```python
# Compiler-emitted, not user-written
w = conditioning_weight(vpd_scale, soil_water - water)
transpiration = (
    w * (stomata * vpd_scale) +
    (1 - w) * (hydraulic_cond * (soil_water - water))
)
```

This is a smooth blend that automatically uses the better-conditioned path more
heavily. It provides both numerical stability in the forward pass and
well-behaved gradients in the backward pass.

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

If no explicit inverse is provided and the invertibility class is `bijective` or
`injective_restricted`, the compiler may attempt symbolic inversion for simple
expression bodies. If it cannot, the function is treated as `opaque` for
inversion purposes regardless of the declared class.

#### 9.3 Deferred: integration primitive

Some models may require definite integration as a primitive operation:

```myco
fn integral(f: fn(Scalar) -> Scalar, lower: Scalar, upper: Scalar) -> Scalar {
  invertibility: opaque
  differentiability: smooth  // Leibniz rule
}
```

This is deferred unless a concrete model forces it. The explicit physics
approach (declaring the conservation equations directly rather than
pre-integrating curves) may eliminate the need.

#### 9.4 Open questions

- Should the first registry be entirely project-local, or should importable
  function packages be supported immediately?
- How should the compiler verify that a user-declared inverse is actually
  correct? Likely: trust-but-verify at test time, with an optional consistency
  check mode.

---

## Part III: The Compiler

These sections define the internal compilation pipeline. Users do not interact
with these stages directly, but the spec must define them to ensure correctness.

### 10. Flattening Pass

The flattening pass is an explicit compiler phase between type-checking and
planning. It transforms the recursive, generic world model into a flat
quantity/relation/constraint graph that the planner can consume.

#### 10.1 Monomorphization

All generic type parameters and const generic parameters are resolved to
concrete types and values:

```
Canopy<3, FarquharC3> --> a concrete Canopy with 3 leaves, each using FarquharC3
```

This is analogous to Rust monomorphization. No generic code survives past this
phase.

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

#### 10.4 Output

The output is a flat graph of:

- Concrete quantities with types, units, and constraints
- Concrete relations with resolved expressions
- Concrete temporal equations
- Concrete slot interfaces with resolved paths

This graph is the input to the planner. The planner does not need to understand
generics, recursion, or repeated structure.

#### 10.5 Future: vectorization recognition

After flattening, the emitter may recognize that expanded structure came from
repeated nodes and emit vectorized backend code (e.g., `jax.vmap`) instead of
unrolled scalar operations. This is an optimization, not a correctness
requirement. Deferred.

### 11. Constraint Analysis

The compiler uses constraint information both statically (at compile time) and
to inform runtime code generation.

#### 11.1 What the compiler proves

The compiler should attempt to prove:

- **Dimensional consistency** of all relations (should always succeed or
  produce a compile error)
- **Domain satisfaction** for operations with restricted domains (e.g., that the
  argument to `log` is provably positive)
- **Bound satisfaction** where type constraints and relation structure are
  sufficient (e.g., if `x >= 0` and `y >= 0`, then `x + y >= 0`)
- **Constraint compatibility** when multiple constraints apply to the same
  quantity (detect provably impossible combinations)

#### 11.2 Internal strategy

The baseline internal strategy is interval propagation over the constraint
graph. This is:

- Linear in the number of constraints per pass
- Typically converges in a small number of fixed-point iterations
- Sufficient for most bound-based and linear constraints
- Cheap enough to run on every compilation

For constraints involving logical connectives (`implies`, `or`) or nonlinear
operations, the compiler may fall back to conservative approximation or defer to
runtime.

The internal strategy is not user-facing. Users write predicates; the compiler
reasons about them as best it can.

#### 11.3 Runtime interaction

Where static proof is not possible, the compiler emits runtime checks or
penalties:

- In `simulate` mode: runtime assertions that raise on violation
- In `train` mode: differentiable penalty losses (soft constraint enforcement)
- In both modes: diagnostic reporting for constraint violations

The compiler may elide runtime checks where static proof succeeds.

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

#### 13.3 Future backends

**PyTorch**: structurally similar to JAX. `torch.Tensor` operations,
`torch.autograd` for differentiation. Straightforward to add.

**Rust (simulate/inference)**: no autodiff needed. Emits a Rust module with
`step()` and `rollout()`. Fast, deployable, embeddable (WASM, embedded systems,
web services). Well-scoped project.

**Rust + PyO3 hybrid (train)**: Rust for the forward pass, exposed via PyO3,
with a `jax.custom_vjp` wrapper for backprop. Provides Rust-speed forward
passes with JAX-native gradient computation. The compiler emits both the Rust
code and the JAX VJP wrapper.

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
  Linear → direct solve. Small polynomial → symbolic if possible. General →
  numerical.
- **`newton`**: force Newton-Raphson for a specific SCC.
- **`fixed_point`**: force fixed-point iteration (may be more autodiff-friendly
  for some systems).
- **`analytical`**: force analytical solution (fails if the compiler cannot
  derive one).

#### 14.2 Other configuration

Other backend-specific options:

- Conditioning-aware path blending (on/off, blend sharpness)
- Constraint enforcement mode (project vs penalize)
- Debug mode (extra runtime checks and diagnostics)

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
experiment.assume_initial("water")
experiment.assume_initial("carbon")
experiment.learn_slot("controller")  # provide a known controller

artifact = experiment.compile(backend="jax")
```

No observations, no loss helpers. The same structural model produces a different
compiled artifact because the binding changed.

---

## Appendix B: Worked Example — Sperry Hydraulic-Stomatal Model

See `mock_sperry.myco` for the full mock implementation. Key features exercised:

- **Contracts**: `VulnerabilityCurve` with `WeibullVC` and `SigmoidVC`
  implementations
- **Generics**: `XylemSegment<V: VulnerabilityCurve>`,
  `SperryTree<V, N_SOIL, N_CANOPY>`
- **Const generics**: parameterized soil layers and canopy layers (sun/shade)
- **Algebraic loops**: hydraulic flow-pressure coupling, Farquhar A-C_i
  coupling, energy balance T_leaf-E coupling — all discovered automatically by
  the planner
- **Temporal accumulators**: `min` for irreversible cavitation tracking
- **Overdetermined quantities**: supply vs demand transpiration
- **Full-graph slot inputs**: `inputs = [*]` for the stomatal controller
- **Pluggable controllers**: slot can be filled by gain-risk optimization,
  Ball-Berry model, or learned neural network

---

## Appendix C: Implementation Priority

The following is a suggested implementation order based on dependency structure.
Items earlier in the list are prerequisites for items later.

1. **Nodes and types** (sections 2, 3) — the structural core
2. **Units and dimensions** (section 4) — needed by types
3. **Constraint language** (section 5) — needed by types and nodes
4. **Relations and temporal** (section 6) — the equation layer
5. **Slots** (section 7) — declared interfaces
6. **Operation algebra** (section 8) — metadata for all operations
7. **Function registry** (section 9) — user-defined operations
8. **Flattening pass** (section 10) — bridge to the planner
9. **Planning with SCC detection** (section 12) — causal ordering + loop discovery
10. **JAX emitter with solver emission** (section 13) — code generation
11. **Compiler configuration** (section 14) — solver strategy selection
12. **Constraint analysis** (section 11) — static reasoning (can lag)
13. **Binding vocabulary** (section 15) — path-based workflow binding
14. **Modules** (section 1) — namespacing and imports
15. **Learned trajectories** (section 16) — structured latent variables
16. **Study-level training** (section 17) — multi-experiment joint learning
