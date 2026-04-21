# Myco — Specification

**Status.** SKELETON. This document is the structural outline of the
consolidated Myco specification. Section contents are not yet written;
only section headings and one-line overviews are in place. Populate
incrementally; see `spec_dev_notes.md` for consolidation-time decisions
and `v2.1_chunk_reports/` for the drafted material each section will
draw on.

**How to read this spec.** Myco is a domain-specific language for
scientific modeling. A modeler writes two things: a `.myco` file that
declares the world (types, relationships, state evolution, topology
changes), and a Python workflow that supplies values and drives
execution. The compiler bridges them. This specification describes the
`.myco` language surface, the compiler's substrate, the workflow
interface, the standard library, backend abstraction, the open items
still under design, and the developer-experience surfaces that will be
designed after the language and compiler lock.

---

## 0. What Myco Is

Myco is a language for scientific modeling. GPU is the primary execution
target; other backends ship via the trait abstraction in Part V. A
modeler writes a `.myco` file (types, relations, state evolution,
topology change) and a Python workflow (values, data, priors,
observations, training directives). The compiler bridges them.

Five principles.

1. **World-vs-experiment split.** `.myco` claims how the world works;
the workflow supplies what this run measures, assumes, and learns.
Same model, many workflows.

2. **Clean boundary.** No values, fit parameters, or data in `.myco`;
no model declarations in Python.

3. **Compiler does work.** Derivatives, inverses, conditioning,
approximations, projection selection, solver routing. Users write
world-claims; the compiler extracts tractable form.

4. **Structure regularizes.** Shape, units, refinement predicates,
conservation groups, and contracts constrain the program ahead of
runtime. Type correctness eliminates classes of error before the
first numerical step.

5. **Referential truth.** World-claims accumulate monotonically.
Events add facts. No retraction, no tombstoning. Entities do not know
they are dead.

Scope. v2.1 covers the language, compiler substrate, workflow
boundary, standard library, and backend trait. The long-term goal is
a GPU ecosystem simulator with neural controllers, dynamic topology,
and spatial explicitness. v2.1 is a precondition.

Output. The `.myco` file plus the workflow Python fully reproduces a
run. Compiled code belongs to you; inspect it if you want (§22).

### 0.1 Foundational Concepts

Cross-cutting claims referenced by later sections. Each concept is
named here once so that Parts I through VI can invoke it by name
without restating it.

**Conservation laws.** Conserved-group declarations (§3.7) produce
compile-checked invariants that thread through types, relation
equality (§8), event firings (§10), and residual classification
(§18). A conserved group is a compiler property the compiler
enforces. No user annotation suppresses the check; an explicit
`constraint` declaration (§8.1) is required if a relation would
otherwise violate the group.

**Referential truth.** Principle 5, expanded. The monotonicity
machinery lives in §15 (the equational core), §10.5 (`replaces`
semantics), and §16 (adjacent keyed state with its own monotonicity
rules).

**Downward-only cross-scale visibility.** Composite types see their
components. Components do not see their composite. A `Forest`
containing `Tree` entities can read per-tree state. A `Tree` cannot
inquire about the `Forest` it belongs to. Cross-scale coupling uses
explicit composition (§3.3). Inheritance is not in the language.

**Traceability and provenance.** Every e-class merge, rewrite
application, and workflow-injected value carries a provenance record
accessible via `mycoc explain` (§22). Workflow-constant injections
(§17) are tagged separately from compiler rewrites, which are tagged
separately from user-declared equalities. Observations (§13.9) are
layer-2 facts with their own tag. Provenance is durable across plan
serialization.

**Error-reporting philosophy.** Diagnostics split into three tiers
by where the problem surfaces. `mycoc` compile errors catch type,
unit, contract, and structural problems that are visible in `.myco`
alone. Workflow-composition errors (§19.4) catch binding problems
that become visible only once plan meets workflow values. Runtime
errors catch backend-level and numerical problems during simulation.
Each tier has distinct diagnostic conventions and the error heading
names the tier so the user knows which file to inspect.

**Capability errors at workflow composition time.** A specific
class of workflow-tier error. When the compiled plan requires a
backend capability (§31.1) the selected backend does not advertise,
when a bound tensor's shape disagrees with the plan's expected
shape, or when a workflow constant's unit disagrees with its
binding site, the diagnostic surfaces at plan-binding time.
Capability errors carry the capability name, the offending SCC,
and the available fallback modes.

**Three-layer scoping split.** The e-graph substrate has three
layers with distinct monotonicity and scoping rules. Layer 1 is
the equational core: ground terms joined by semantic equality,
append-only merges, globally scoped. Layer 2 is envelope metadata:
distributional, differentiability, invertibility, and observation
facts keyed by e-class identity, append-only, globally scoped.
Layer 3 is adjacent keyed state: per-call error budgets,
approximation-flavor selections, solver intermediates, keyed by
call site or subgraph identity, with its own monotonicity rules
(§16). Part II §15 covers the machinery, and §17 enumerates the
merge sources.

**Determinism and reproducibility.** A given `(plan, workflow
values, run.config.seed)` triple produces the same trajectory on
the same backend version. Across backend versions, numerical
outputs may shift within documented tolerance bounds. Bitwise
equivalence across versions is a stronger property backends may
optionally advertise via capability (§31.1).

**World-vs-experiment split.** Principle 1, named as a cross-cutting
axis. Aleatoric content (stochastic SCCs and the distributional
machinery of §13) lives on the `.myco` side. Epistemic content
(measurements, priors, training directives) lives on the workflow
side and uses the verbs of §24.

**Conversion-graph cost model.** Open. Unit conversions, tensor
reshapes, sparse or dense representation transitions, and
structural-subtype widenings all carry costs that the compiler
should minimize when multiple valid paths exist. The cost model
sits between the type layer and the e-graph rewrite cost model.
Tracked in §35 and scoped to chunk 05 Q7 / chunk 07 Q6.

**Projection-free compiler.** The compiler does not auto-emit
projection operators or solver selection to satisfy a constraint.
`constraint` declarations (§8.1) carry three explicit discharge
paths: compile-time proof via e-graph and refinement reasoning,
runtime projection selected by the workflow via §25's projection-
flavor verbs, or training loss penalty on SCCs classified training
(§20). The compiler surfaces which discharge path each constraint
uses, and the workflow picks among projection flavors when that
path applies. The compiler does not insert projection silently.
This keeps constraint-satisfaction a named modeler decision, never
an implicit compiler behavior.

**Generated code is the product.** The run-time artifact is the
compiled plan plus the workflow bindings that produced it. The
`.myco` source and workflow Python together fully reproduce the
plan under a fixed compiler version (§31.4). Inspection
affordances (§22 `mycoc explain`, plan serialization, provenance
records) let users audit the compiled output and the choices the
compiler made. Myco is a compiled language: the plan is the unit
of execution, the source is the unit of reproduction. Plans are
durable, serializable, and shareable independent of the source
they were compiled from.

---

## 1. Canonical Glossary

The vocabulary used throughout this document. Each term one line.
Terms: `variable`, `relation`, `event`, `controller` (workflow-only),
`initial`, `temporal`, `data contract`, `locus`, `workflow`,
`e-class`, `envelope`, `universal`, `approximate`, `observe`.

---

## Part I — The Language

The surface a modeler writes in `.myco`.

### 2. Modules, Imports, Scope

File-as-module convention. Path-based imports (`use path::to::symbol`).
Visibility rules (public / private / file-local). Scope resolution
rules for names, types, universals, contracts, events. Relationship
to the workflow side: Python imports and `.myco` imports are distinct
systems — the workflow imports `.myco` models, not the other way
around.

### 3. Types

Primitives (`Scalar<U, T = Float64>`, `Tensor<U, shape>` with `Vector`
and `Matrix` as shape-refined aliases). Named types. Generics: val
generics, type generics, named-argument rule for multi-parameter
generics. Structural refinements on matrices (Symmetric, PosDef,
Diagonal, Triangular, Orthogonal).

#### 3.1 Universal Declarations

Module-scope typed names shared across all instances that reference
them. `universal R: Scalar<J_mol_K>` declares a name with a type; the
value is supplied by the workflow via `assume_constant` or
`learn_constant`. CC1: no literal value in `.myco`. Semantics:
universals are "same value for every consumer in this run" — physical
constants, cross-entity shared coefficients. Distinct from ordinary
fields, which vary per instance.

#### 3.2 Refinement Types

Predicate-refined types: `type UnitInterval = Scalar<dimensionless>
where { 0 <= self <= 1 }`. Refinement obligations discharged by
e-graph reasoning where possible, runtime check otherwise. `~`
operator on distributions auto-truncates to a refined target type (§13).

#### 3.3 Newtype and Composite Types

Single-field nominal wrappers (`type Depth: Scalar<m>`) for type
distinction without structural change. Composite record types with
named fields. Named-type comparison rules cross-link §7.

#### 3.4 Node Instantiation

`node name: Type` at module scope creates an entity with identity.
Identity survives timesteps and e-graph merges; events operate on
nodes. Distinct from type aliasing — `node tree: Tree` creates one
Tree entity, not a name for the Tree type. The e-graph instantiates
one identity-tagged class per node.

#### 3.5 Heterogeneous Collections — `impl` and `some`

`impl Contract` and `some T` address orthogonal axes of collection
heterogeneity.

`impl Contract`: **static type heterogeneity, compile-time-known
element-type set.** A collection `Collection<impl Animal>` holds
elements of multiple concrete types that all satisfy `Animal`. The
concrete element-type set is compile-time known (determined by what
types appear at module scope). The compiler monomorphizes per
concrete type, producing one pool per type and compiling each
pool's hot paths against its concrete layout.

`some T`: **runtime sizing, homogeneous element type.** A
collection `Collection<some Cell>` holds elements of a single
concrete type `Cell`, with element count unknown until bound by the
workflow. Storage is dynamically sized; iteration runs a single
monomorphic code path.

The two axes compose. `Collection<some (impl Plant)>` is
statically heterogeneous (element types drawn from a compile-time
set) and dynamically sized (count unknown until workflow binding).
The combination rule follows from each operator's semantics; no
third construct is required.

Together `impl` and `some` positively replace the retired `dyn`
escape. Each operator carries its own compile-time discipline:
`impl`'s static type set drives monomorphization, and `some`'s
runtime sizing drives dynamic-allocation choices.

#### 3.6 Generic Parameter Variance

Variance rules for generic type parameters (chunk 07 Q4):
covariant / contravariant / invariant positions. Subtyping discipline
for named types + refinements + conservation-group hierarchies.

#### 3.7 Conservation Groups

`type Mass : Scalar<kg> { conserved }` marks a parent type whose
named-type children (e.g., `FishMass`, `DetritusMass`) share
conservation semantics. Consequences:

1. Cross-sibling arithmetic forbidden unless an explicit `convert`
   exists.
2. Bare `convert FishMass <-> DetritusMass` permitted between siblings
   (relabel only, no conversion body).
3. Events that destroy instances must route conserved fields
   somewhere; unaccounted mass is a compile error (§10).
4. Compiler auto-generates junction balance relations from `diverg()`
   usage on conserved flux fields (§11); overridable with
   `replaces balance(flux_field)`.
5. Bare-convert sibling merges create magnitude equivalence in the
   e-graph (§17 merge source — named-type conversion).

Tier 2 sub-questions deferred: scoped conservation, boundary-flux
interaction, field-level conservation.

#### 3.8 Scalar and Tensor Reconciliation

Open: whether `Scalar<U>` is formally sugar for `Tensor<U, ()>`
(shape-zero tensor) or a distinct primitive with coercion rules
(chunk 05 Q6). The unification is attractive — it lets structural
refinements, convert variants, and envelope flavors live on a single
hierarchy. Chunk 05 carries the resolution.

Collections (§12) and tensors are orthogonal primitives. A
`Collection<T>` is a homogeneous, unordered-or-keyed aggregation of
entities — membership, iteration, aggregation (§12.1). A `Tensor<U, S>`
is a shaped numerical object — multi-axis indexing, linear-algebra
primitives, structural subtypes (§3.9). The two do not nest into
each other by default: a collection of scalars is not automatically
a vector, and a tensor axis is not automatically a collection. User-
defined conversions exist (e.g. `to_tensor` aggregating a collection
of refined scalars into a dense vector), but they are explicit.
This orthogonality keeps the semantics of `for` / aggregation
(§12) decoupled from the semantics of matrix / tensor operations.

The `convert` facility (§5.1) extends to tensors for a bounded set
of operations: **reshape** between compatible shape specifications
(total element count preserved), **sparse ↔ dense** representation
changes on the same structural type, and **structural-subtype
widening** (e.g. `Matrix<_, Diagonal>` → `Matrix<_, Symmetric>`
throws away structural information without changing values). Out of
scope for `convert`: **numeric precision changes** (float32 ↔ float64
are a backend concern, §31), **storage-order / layout changes**
(row-major ↔ column-major, also backend), and **device residency**
(host ↔ GPU, backend). The split keeps `convert` about meaning-
preserving lossless transforms at the type layer and leaves
representation-level tuning to the backend trait.

#### 3.9 Matrix Structural Subtype Lattice

Matrix structural properties are type-level predicates that form a
lattice under meet (structural intersection). They drive stdlib
primitive dispatch (§30) — e.g. `solve` chooses triangular
substitution, Cholesky back-substitution, or general LU based on the
structural subtype of its first argument.

| structural type | meaning | Cholesky-eligible |
|---|---|---|
| `Symmetric` | `A = Aᵀ` | no (symmetry alone insufficient) |
| `PositiveDefinite` | `xᵀ A x > 0` for all `x ≠ 0` | yes |
| `PositiveSemiDefinite` | `xᵀ A x ≥ 0` | pivoted Cholesky |
| `UpperTriangular` / `LowerTriangular` | one triangle zero | N/A — direct solve |
| `Diagonal` | off-diagonals zero | trivial |
| `Orthogonal` | `A · Aᵀ = I` | N/A — inverse is transpose |
| `Sparse` | substantial zero pattern | representation concern |
| `Banded<b>` | entries zero outside bandwidth `b` | banded Cholesky |

Meet composition is explicit: `PositiveDefinite ∧ Symmetric =
PositiveDefinite` (since `PositiveDefinite` supertraits `Symmetric`
in the standard real-matrix setting); `Diagonal ∧ PositiveDefinite`
yields a diagonal with strictly positive entries, which admits a
trivial Cholesky (`L = √diag(A)`). The lattice is closed under
the meet of any pair that is algebraically compatible; incompatible
pairs (e.g. `UpperTriangular ∧ LowerTriangular` outside of `Diagonal`)
produce `Diagonal` by compile-time reduction or a compile error if
the context requires a strict non-diagonal type.

Dispatch rule: `solve(A, b)` with `A: Matrix<_, LowerTriangular>`
calls triangular substitution directly; `A: PositiveDefinite` routes
through Cholesky; `A: Orthogonal` uses `Aᵀ · b`. The compiler walks
the lattice to pick the tightest applicable specialization.

Deferred to chunk 05:

- **Heterogeneous-unit matrices** (B5). `Matrix<_, _>` with entries
  carrying different units per row or column — e.g. a Jacobian with
  mixed dimensions — is the chunk-05 gating question. The lattice
  above assumes scalar-valued entries in a single unit system; how
  it extends to heterogeneous-unit matrices (and which subtypes like
  `Symmetric` even make sense when units differ across the diagonal)
  is the open resolution.
- **Shape refinements** (§3 generics). Fixed-shape `Matrix<N, M>` vs
  dynamic-shape `Matrix<?, ?>` interaction with the subtype lattice.
  Fixed-shape in refinement syntax, dynamic in runtime-bound, compiler
  enforces compatibility at binding time.
- **Envelope flavors for matrix quantities**. Whether matrix-valued
  quantities participate in the layer-2 envelope metadata system
  (§17) in the same way scalars do, or need specialized envelope
  machinery. Parallel to the MVN Cholesky intermediate (§13.6 Z10)
  but for general matrix-valued terms.
- **Sparse representation choice**. `CSR` vs `CSC` vs `COO` vs
  `block-sparse` — the structural property `Sparse` is an abstract
  marker; the concrete storage is a backend-level choice tracked
  in chunk 06 alongside device-layout concerns.

### 4. Values and Literal Policy

Zero literal numerics in value position. Three exception positions:
unit definitions, affine conversion bodies, structural positions
(shape tuples, indices, generic-parameter definitions). All numeric
values enter from the workflow. See `spec_dev_notes.md` for the
derivation.

#### 4.1 CC1 Diagnostic Surface

Violations surface as `mycoc` compile errors with a consistent
diagnostic shape. The error identifies the literal, the position
kind that was rejected (value position vs structural position), and
the canonical workflow verb that would supply the value instead.
For a literal appearing in an expression that binds to a universal,
the diagnostic names the universal and points to `assume_constant`
or `assume_series` (§24). For a literal in a relation body, the
diagnostic points to the governing variable and suggests lifting
the value to a universal plus a workflow binding. The wording keeps
CC1 enforcement actionable instead of cryptic.

### 5. Units

Base units, derived units, affine conversions, dimensional algebra,
unit-generic types.

#### 5.1 Convert Declarations — Four Variants

Unit and named-type conversions come in four forms:

- `convert A <-> B` (bidi, bare): declares A and B as same-magnitude
  aliases. Required for conservation-group siblings (§3.7). No body.
- `convert A -> B` (one-way, bare): declares A tagged-as-B in one
  direction. Lossy relabel.
- `convert A <-> B { body }` (bidi, parameterized): both directions
  specified in the body; compiler verifies inverse consistency
  (§5.2).
- `convert A -> B { body }` (one-way, parameterized): single-
  direction conversion with arbitrary expression.

#### 5.2 Round-Trip Verification (O2.1)

Parameterized `<->` converts obligate the compiler to verify inverse
consistency. Verification runs bounded counterexample search within
the participating types' refinement bounds. Counterexample found →
compile error with offending value; exhausted bound → accept.

#### 5.3 The `value_in` Operator

`value_in(unit)` extracts the raw numeric magnitude of a quantity in
a named unit. Example: `temperature.value_in(celsius)` pulls the
celsius magnitude from a `Scalar<kelvin>`. Use positions: interop
with unit-naive stdlib atoms, external-library arguments. Units of
the argument must be dimensionally compatible with the receiver.

### 6. Functions

`fn` declarations with parametric generics. Body composition. Contracts
apply to functions using the same composable machinery used for types
and distribution families (see §7). Stdlib atoms (`exp`, `log`, `sin`,
`sqrt`, …) declare capability contracts like `Invertible<_>`,
`Differentiable`, `Monotone`; these drive e-graph rewrites (see §17
merge sources). User functions carry no property-declaration surface
— the compiler derives properties from body composition plus stdlib
atom declarations. No annotation blocks, no `#[...]` attributes.

### 7. Contracts

Contract declaration. Multi-contract satisfaction (`: A + B + C`).
Supertraits (`contract B : A`). Contracts apply uniformly to types,
functions, and distribution families. Named-type comparison rules.

#### 7.1 Parameterized Contracts

Contracts take type parameters: `Invertible<T>` (invertible fn with
inverse type T), `Convert<From, To>` (conversion capability),
`Distribution<U>` (distribution over units U). Parameters thread
through supertrait chains and satisfaction checks. Capability
contracts on stdlib atoms (§6) and distribution families (§27) are
the principal users.

#### 7.2 Capability Contracts

Capability contracts carry compiler-actionable facts. Distribution-
side chain (root `Distribution<U>`, supertraits `AffineSelfClosed`,
`SumSelfClosed`, `ProductSelfClosed`, `ScaleSelfClosed`,
`SmoothTransformable`, `ReparameterizedSampleable`) drives Tier A
closed-form routing (§13). Function-side (`Invertible<_>`,
`Differentiable`, `Monotone`) drives function-inverse rewrites
(§17 merge source 5) and `deriv` / `condition_of` intrinsics (§14).
Satisfaction is composable: a contract `C : A + B` lifts A's and B's
facts through the supertrait chain without restatement.

#### 7.3 Supertraits

`contract B : A` declares B as a refinement of A. Every B-satisfier
is also an A-satisfier. Supertrait chains compose; diamond
inheritance resolved by contract identity (same supertrait reached
by two paths contributes one obligation, not two).

#### 7.4 Multi-Contract Coherence

Satisfaction of `T : A + B` requires disjoint obligations for A and
B, or matching obligations where they overlap. Conflicting
obligations (A requires `fn foo(x: U) -> V`, B requires
`fn foo(x: U) -> W` with `V ≠ W`) emit a coherence error naming
both supertraits.

### 8. Relations and Equality

Relations as world-claims. Overdetermination is not an error; closure
policies combine competing claims. Policies Y1-Y6 including
un-deferred `condition_weighted` (backed by `condition_of`
Levels I-III). Merge semantics.

#### 8.1 `constraint` Declarations

Inequality or logical obligations the modeler asserts must hold.
Distinct from `relation` (equational merge) in that constraints
don't merge e-classes; they restrict the admissible solution set.
Discharge paths: compile-time proof via e-graph + refinement
reasoning, runtime projection (workflow-selected flavor, §25),
or training loss penalty (SCCs classified training, §20).

#### 8.2 `let` Bindings in Relation Bodies

Local aliases for subexpression reuse inside a relation body.
Compile-time only; not a new variable, not a new state field.
Example: `let flux = k * (psi_soil - psi_leaf); d(water) = flux -
transpiration`. Two roles: readability for multi-term equations,
and e-class hinting (binding signals "same e-class both places,"
which the e-graph honors via structural equality).

#### 8.3 `if` / `else` vs `where` in Relation Bodies

Two distinct constructs. `if C then A else B` introduces a runtime
branch: the relation's semantics depends on the truth of `C` at
each evaluation. `where x is T` is compile-time narrowing: the
subsequent body is type-checked under the assumption that `x`
inhabits `T`, and the branch is selected structurally (no runtime
test on the value). `if` preserves the e-graph's merge obligations
on both arms; `where` specializes the arm to a narrower type.

#### 8.4 `for` Loops in Relation Bodies

Compile-time unfolding. `for node in collection: ...` expands to
one relation per element at compile time; the loop variable is
not a runtime iterator. Distinct from runtime iteration in
collections (§12). The collection must be statically known (shape-
generic OK; runtime-sized `some` collections disallowed here).

#### 8.5 Inline Relation and Constraint Sugar

Terse forms for single-line claims attached to field or type
declarations. `x: Scalar<m> = y + z` desugars to a named relation
on `x`. `x: Scalar<m> where { x > 0 }` desugars to a refinement
constraint. The longhand block forms (`relation { ... }`,
`constraint { ... }`) remain available; the sugar is purely
ergonomic.

#### 8.6 Overdetermination: System-Level Classification

When a residual block has more equational claims than unknowns, the
compiler classifies the system three ways before any closure policy
applies:

1. **Redundant (consistent).** One or more claims are derivable from
   the others. The system agrees with itself. Closure policies
   (§8.7) apply here.
2. **Provably inconsistent.** Symbolic elimination yields a
   contradiction (`0 = 5`). Hard compile error; no policy applies.
3. **Conditionally inconsistent.** Consistency depends on runtime
   parameter values. Compiler emits a runtime assertion that fires
   before the solver.

Closure policies operate only on the redundant case. Inconsistent
cases are failures, not approximation choices.

#### 8.7 Closure Policies Y1-Y6

User-facing handlers for redundant overdetermination. Selected per
residual block at workflow composition time. Variants:

- **Y1 `weighted_average(c₁, …, c_N)`** — arithmetic mean.
- **Y2 `soft_select(preference_list, sharpness)`** — differentiable
  soft-pick.
- **Y3 `hard_select(preference_list)`** — deterministic
  non-differentiable pick.
- **Y4 `condition_weighted`** — weights candidates by numerical
  conditioning; backed by `condition_of` Levels I-III (§14).
- **Y5** — user-defined policy (§8.8).
- **Y6 `C(N,M)` enumeration** — combinatorial case for N > M+1.
  Compiler enumerates C(N,M) maximal square subsystems, solves
  each, checks consistency across solutions. Warns on
  combinatorial blowup threshold.

#### 8.8 Y5: User-Defined Closure Policies

A Y5 policy is an ordinary `.myco` function satisfying the
closure-policy interface: inputs are the candidate values (one per
competing claim) plus user-supplied hyperparameters; output is a
single forward value of the same type. Users register a Y5 policy
by name; workflows select it per residual block via the same
mechanism as Y1-Y6. The compiler inlines the fn body into the
extraction pipeline — Y5 policies participate in differentiation
and e-graph reasoning like any other fn.

#### 8.9 Smoothing as a Model Claim

Smoothing is a modeler choice, not a compiler-injected
approximation. The stdlib provides `smooth_max`, `smooth_abs`,
`smooth_step`, and related helpers; the modeler writes them
explicitly in relation bodies where non-smooth operators
(`max`, `abs`, piecewise step) would break differentiability or
solver assumptions. Runtime `where x is T` clauses are the
type-level counterpart (structural narrowing rather than
numerical smoothing). The compiler does not auto-smooth; users
write what they want.

#### 8.10 Generated-Defaults and Obligation Keys

When the compiler auto-generates a relation (e.g., junction balance
from `diverg()` on a conserved flux field, §3.7; boundary condition
stubs from geometry, §11), the generated relation carries a named
obligation key like `balance(axial_flux)`. The modeler overrides
the default by writing `replaces balance(axial_flux): <body>` in
the type body. Gives users a targeted hook to override compiler
decisions without disabling generation globally. Primary consumers:
junction balance, boundary conditions, auto-synthesized
conservation relations.

### 9. State and Time

`initial:` and `temporal:` blocks live in type bodies. Module-scope
only for truly cross-entity relations. `d(x) = expr` for ODE form,
`step(x) = expr` for discrete-update form. No `[t+1]` subscript
surface.

#### 9.1 `dt` Provision

`dt` is not a reserved name in `.myco`, not a universal, not a
special verb. Two cases:

- **`d(x) = expr` (ODE form):** `dt` is not referenced in the
  model. The compiler (or the backend-selected integrator) owns
  integration step size.
- **`step(x) = expr` (discrete form):** tick cadence is a normal
  workflow binding via `assume_constant("config.dt", ...)` or
  `assume_series(...)`. No `bind_dt` verb.

Time itself (`t`) is not a universal either; temporal indexing
produces distinct e-graph ground terms (`y[1]`, `y[2]`, …) with
structural relations between them (§16).

#### 9.2 Per-Path Uniqueness After Generic Expansion

A generic event or relation (`event<T: Species>(…)`) expands at
compile time to one concrete instance per T-satisfier (cartesian
product over all generic parameters). Each expansion path must
yield a unique obligation key; duplicate keys across paths are a
compile error, not a closure-policy situation. Overdetermination
and underdetermination analyses run on the fully expanded
constraint set, so uniqueness is a pre-analysis hygiene check.

### 10. Dynamic Topology and Events

`event` declarations for topology change. Referential-truth semantics:
things do not know they are dead. Events add facts; no tombstoning, no
retraction.

#### 10.1 Firing-Order Policy

When multiple events match at the same timestep, firing order
is a simulation parameter set at workflow composition — not
language syntax. Default: declaration order (lexical). Workflow
override via run-config. Within a single event type, all valid
firings execute in parallel (GPU-batched).

#### 10.2 Generic Event Cartesian-Product Expansion

`event<T: Contract>(…)` expands at compile time to one concrete
event per T-satisfier. Multi-parameter generic events
(`event<T: A, U: B>`) expand over the cartesian product of
satisfier sets. Each expanded path has its own obligation key
(§9.2) and participates in firing-order dispatch (§10.1)
independently.

#### 10.3 Cross-Container Events

An event that touches entities from different container types
resolves its scope via the **nearest-common-ancestor rule**:
the event binds at the lowest type that contains all affected
entities. If no common ancestor exists, compile error. This
keeps event scope minimal and prevents accidentally lifting an
event to module scope.

#### 10.4 Within-Event Tiebreaking

A single event type expands (§10.2) to N firings per tick; all
fire concurrently. Under referential-truth semantics, concurrent
firings fall into three cases:

1. **Structurally identical facts.** The e-graph merges them
   once. Idempotent; no tiebreak needed.
2. **Conflicting writes on conserved fields.** Caught at compile
   time by the junction-balance obligation (§3.7). Not a runtime
   concern.
3. **Legitimately overdetermined residual at the next tick.**
   Handled by closure policies (§8.7).

No additional within-event ordering construct is exposed. Order
across different event types is §10.1; within a single type,
parallelism is the default and the three cases above cover every
outcome.

#### 10.5 `replaces` and Monotonicity

A `replaces <obligation_key>` declaration (§8.10) overrides a
compiler-generated default relation by suppressing its emission,
not by retracting a fact after the fact. The e-graph never
contains both the default and the override simultaneously. This
preserves the monotonicity invariant.

The harder case — a user-written `event` that logically retracts
a prior user claim — remains open and is tracked in §35 Other
Opens. In v2.1, events only add facts; `replaces` applies only
to compiler-generated defaults, not arbitrary prior claims.

### 11. Geometry and Locus

Horse/fly composition pattern for spatial frames. `bind_topology` at
workflow time for concrete meshes. `on locus:` clause applies
symmetrically to `relation` and `temporal`.

#### 11.1 Spatial Operators

Compiler-recognized operators on locus-scoped fields:

- `grad(f)` — gradient of a scalar field; yields a vector field
  on the same locus.
- `diverg(v)` — divergence of a vector field; yields a scalar.
  `diverg` on a conserved flux field drives auto-synthesized
  junction balance (§3.7, §11.8).
- `laplace(f)` — Laplacian; `diverg(grad(f))`.
- `normal_grad(f)` — gradient dotted with the outward normal;
  defined on boundary sub-loci only.
- `trace(f, boundary)` — restriction of `f` to the named
  boundary sub-locus.

Operators are stdlib functions with capability contracts (§7.2).
Relations like `laplace(f) = diverg(grad(f))` fire as e-graph
rewrites from stdlib declarations; users never annotate them.

#### 11.2 Boundary Conditions

Boundary conditions are `requires` blocks on boundary sub-loci.
Three standard forms:

- **Dirichlet** — `requires: f = g`. Fixes the field value.
- **Neumann** — `requires: normal_grad(f) = g`. Fixes the normal
  flux.
- **Robin** — `requires: a * f + b * normal_grad(f) = g`. Linear
  combination.

Each `requires` block lowers to a projection, elimination, or
residual constraint depending on the solver path selected at
workflow composition (§25). A locus with boundary geometry and
no `requires` blocks is underdetermined; the compiler emits no
default boundary condition (silence is not a free Neumann zero).

#### 11.3 Stdlib Geometries

| Name | Dim | Topology | Typical Use |
|---|---|---|---|
| `Line1D` | 1 | interval | roots, stems, cylindrical cross-sections |
| `Rectangle2D` | 2 | rectangle | leaf surfaces, soil patches |
| `Ball3D` | 3 | solid ball | fruit, nodules, root cells |
| `RootedTree` | 1 (branching) | tree | plant hydraulic networks, vasculature |
| `MetricGraph` | 1 (branching) | general graph | river networks, mycelia |
| `BranchingManifold` | n | recursive | fractal / self-similar structures |

Each geometry exposes named sub-loci: `interior`, `boundary`,
junction classes (where applicable). Composition via the
horse/fly pattern (§11.4) lets richer entities reuse these
primitives without inheritance.

#### 11.4 Horse-and-Fly Composition

A horse owns geometry; flies are embedded entities located
against that geometry via an embedding field on the horse. Flies
do not own coordinates; position is a horse-side field indexed
by fly identity. Keeps geometry with the entity that has it;
many fly types share one horse without inheritance.

Example. A `Tree` (horse) owns a `RootedTree` geometry. `LeafPatch`
instances (flies) attach to tree nodes via an embedding field on
the tree. The tree's hydraulic relations live on the tree locus;
the patches' gas-exchange relations live on patch loci. Cross-
scale visibility is downward only: the tree sees its patches;
patches do not see the tree. Composition, not inheritance.

#### 11.5 Discretization Configuration

A geometry becomes a mesh at workflow composition. `bind_topology`
supplies discretization: mesh resolution, element type (FDM /
FVM / FEM basis), refinement policy, boundary identification.
The compiler receives a concrete mesh and lowers spatial
operators against it. The compiler does not auto-refine or adapt
— mesh is a workflow decision.

#### 11.6 Compiler Discretization Defaults

If `bind_topology` does not specify a discretization, the
compiler picks per-geometry defaults documented in the stdlib
reference. Indicatively: `Line1D` → uniform N-node grid (N is
still workflow-supplied); `Rectangle2D` → regular M×N grid;
`RootedTree` → one node per structural vertex with no interior
refinement. Defaults are conservative — the program compiles,
but accuracy targets for scientific applications typically
require explicit override. The default is a smoke-test
affordance, not a production recommendation.

#### 11.7 Edge-Interior vs Locus-Scoped Fields

A field attached to a 1D edge declares itself one of two ways:

- **Locus-scoped** — one value per edge instance, no position
  dependence. No discretization. Example: `edge.conductance:
  Scalar<...>`.
- **Edge-interior** — a function of the interior coordinate on
  the edge. Discretizes during lowering. Example:
  `edge.water(x)`. Spatial operators (§11.1) act on edge-
  interior fields only.

Modelers choose per field based on whether gradients along the
edge matter physically. Mixing is allowed: the same edge can
carry a locus-scoped conductance alongside an edge-interior
water potential. The two declaration styles are not convertible
to each other and do not merge in the e-graph.

#### 11.8 Default Junction Conditions

Where edges meet at a junction, the default condition is
**balance only**: the sum of conserved fluxes across the
junction equals zero (§3.7 consequence 4, auto-synthesized from
`diverg()` on a conserved flux field). Continuity of non-flux
fields is **not** assumed by default. Different edges at a
junction may carry different scalar values unless the modeler
writes an explicit `requires: left.f = right.f`.

Rationale. What conservation forces (balance) is free; what
modeling choice imposes (continuity) is opt-in. This matches
the conservation-first posture throughout the language and
prevents silent assumptions about field matching across
junctions.

#### 11.9 Embedding Fields Are Regular Fields

Flies attach to a horse (§11.4) via ordinary field declarations,
not a dedicated `embed` or `in` construct. A `Tree` carrying a
`LeafPatch` collection with per-patch attachment position uses
standard field syntax on the horse side: `patch_position:
Position`. The horse/fly composition is a pattern, not a
language primitive. The language has no embedding keyword.

#### 11.10 Geometry Coefficients via `requires`

Material properties attached to geometry (conductivity,
diffusivity, elastic modulus) enter via `requires` blocks on
the locus, not a `hint` keyword or parameter list. Example:
`requires: conductivity = <workflow-bound coefficient>` on the
locus body. The same construct that attaches boundary
conditions (§11.2) attaches coefficients — one attachment
surface, not two.

#### 11.11 Standard Locus Vocabulary

Four keywords are used inside locus bodies and geometry
declarations:

- `boundary` — named boundary sub-locus. Carries boundary
  conditions (§11.2) and the target of `trace` and `normal_grad`
  (§11.1).
- `chart` — coordinate chart reference. Used when a locus needs
  explicit parameterization for operators that depend on
  coordinate choice.
- `metric` — metric tensor for non-Euclidean geometries.
- `requires` — attachment of constraints, boundary conditions,
  or material coefficients (§11.2, §11.10).

No other geometry-level keywords are introduced in v2.1. New
standard geometries ship via stdlib (§11.3), not via new
keywords.

### 12. Collections and Iteration

`impl Contract` (heterogeneous element type, static monomorphization)
vs `some` (runtime sizing). Iteration patterns. Aggregation lowering.
Narrowing with `where x is T`.

#### 12.1 Aggregation Primitives

Named stdlib aggregations over collections:

- `sum(xs)`, `product(xs)` — arithmetic. Units-aware;
  conservation-group-aware (§3.7 blocks cross-sibling sums
  without an explicit `convert`).
- `any(xs)`, `all(xs)` — boolean.
- `count(xs)` — cardinality, `Scalar<dimensionless>`.
- `argmin(xs)`, `argmax(xs)` — handle of the extremal element;
  see §12.2 for the heterogeneous case.

Aggregations compose under stdlib-declared e-graph rewrites
(linearity, distributivity, `sum(map(f, xs))` fusions). v2.1 has
no user-declared aggregation surface — new aggregations ship via
stdlib, matching the `.myco`-has-no-annotation-surface stance.
Soft and weighted variants (softmax, weighted_sum) are tracked
in §35 Other Opens pending collection-aggregation syntax lock.

#### 12.2 Tagged Handles for Heterogeneous `argmax`

`argmax` over an `impl Contract` collection returns a tagged
handle, not a bare index. The handle carries `(pool_identity,
intra_pool_index)` because different concrete types live in
separate compile-time pools (§3.5, §12.5). Users match on the
handle to recover the concrete type and reach type-specific
fields. `argmax` over a homogeneous `some` collection returns a
plain index.

The IR-level sum type for tagged handles is the compiler's
internal machinery; surface syntax is the same `argmax` call in
both cases. The type of the returned handle depends on the
collection's static element-type structure.

#### 12.3 Empty-Collection Defaults

Aggregations behave on empty collections as follows:

- `sum(empty) = 0`, `product(empty) = 1`, `count(empty) = 0`.
- `any(empty) = false`, `all(empty) = true`.
- `argmin(empty)`, `argmax(empty)` are a **compile error**.

Identity-element defaults on `sum`/`product`/`any`/`all`/`count`
enable algebraic rewrites without branch logic. `argmin` and
`argmax` have no identity element, so the compiler rejects
empty-reachable calls at compile time; the caller must
statically prove non-emptiness or guard with a `count > 0`
check that the compiler can refine against.

#### 12.4 Bind-Time vs Event-Time Dynamism

Two distinct sources of collection-size change:

- **Bind-time dynamism.** Collection membership is fixed when
  `bind_topology` and the `assume_*` verbs run. After workflow
  composition the collection size is static. Lowers with a true
  runtime size N; no N-max slot machinery.
- **Event-time dynamism.** Events (§10) add or retire members at
  runtime. Requires N-max slot allocation and alive-mask
  lowering (§21). N-max is declared at the collection's
  declaration site; overflow is a runtime error.

The distinction is visible at declaration time: an `impl
Contract` or `some` collection with no events that mutate it
is bind-time; one that events touch is event-time. Compiler
diagnoses ambiguous cases at compile time.

#### 12.5 Per-Type Pool Desugaring

An `impl Contract` collection desugars at compile time to one
homogeneous pool per concrete satisfier type. Iteration fuses
across pools: `for x in xs: body(x)` expands to one
monomorphized loop per pool. Cross-pool aggregations (`sum`,
`argmax`, etc.) compose the per-pool results under stdlib
rewrites. Preserves static monomorphization behind a
heterogeneous-iteration surface — users see one collection, the
compiler sees N pools.

#### 12.6 Iteration Styles

Three iteration surfaces, selected by collection kind:

- **Index-style** — `for i in 0..N: xs[i]`. Explicit index;
  works on any sized collection.
- **Iterator-style** — `for x in xs: …`. Element binding;
  preferred for readability; compiles to index-style.
- **Graph-neighborhood-style** — `for n in node.neighbors: …`.
  Iterates a topological adjacency exposed by the locus.
  Pending finalization of the geometry-side neighbor-query
  surface (§11 geometry vocabulary still open).

All three are compile-time constructs. Runtime iteration
behavior is an artifact of lowering (§21), not a user-visible
distinction.

#### 12.7 Filtering with `where x is T`

`where x is T` narrows an iteration to elements inhabiting `T`.
Reuses the type-narrowing machinery from §8.3. Structural
filter, not a runtime predicate:

```
for tree in trees where tree is OldGrowth:
  biomass += tree.trunk_mass
```

The filter selects a compile-time-known subset of an `impl
Contract` pool; the body of the loop is monomorphized against
`OldGrowth`. Runtime predicates use `if` inside the body.
Combines cleanly with aggregation: `sum(tree.trunk_mass for tree
in trees where tree is OldGrowth)`.

### 13. Probabilistic Programming

`~` as layer-2 distributional metadata, not an equality merge.
Aleatoric/epistemic split. Tier A/B/C routing (exact closed-form /
approximate rewrite / opaque PPL handoff). Independence via structural
identity; no naked correlation. Cholesky reparameterization.

#### 13.1 Aleatoric and Epistemic Uncertainty

Two distinct sources of uncertainty. Same `~` surface; the
distinction is structural position:

- **Aleatoric** — world-randomness. The quantity genuinely
  fluctuates across realizations (measurement noise,
  environmental stochasticity). `~` appears inside `temporal:`
  or event scope. Realized via sampling; does not reduce with
  more data.
- **Epistemic** — parameter uncertainty. A fixed-but-unknown
  value the modeler does not know. `~` appears at module scope
  or in `initial:`. Reduces with observation via Bayesian
  update; participates in training.

SCC classification (§20) threads the two: aleatoric variables
enter the stochastic SCC class; epistemic latents enter the
training class. The same `~` operator, routed differently by
where it lives.

#### 13.2 Tier A / B / C Dispatch

Three tiers of `~` resolution, tried in order per stochastic
SCC at compile time:

1. **Tier A — Exact closed-form.** Capability contracts on
   distribution families (§7.2, §27) advertise algebraic
   closures (`AffineSelfClosed`, `SumSelfClosed`,
   `ProductSelfClosed`, `ScaleSelfClosed`,
   `SmoothTransformable`, `ReparameterizedSampleable`). When a
   transformation matches a closure contract, the result is
   another member of the family with analytically computed
   parameters. Closed-form always wins.
2. **Tier B — Approximate rewrite.** When Tier A does not
   close, approximate-block rewrites (Delta method,
   Fenton-Wilkinson, CLT, block-maxima → GEV; §15) apply if
   the user's `approximate` block permits the relevant error
   class. Envelope metadata records the approximation used.
3. **Tier C — Opaque PPL handoff.** No closed form, no
   user-permitted approximation. The SCC ships to the
   backend's PPL handler (§31). Samples come back; no envelope
   facts about the parametric form.

The compiler records its chosen tier per SCC; inspection
surfaces (§22) show which tier each stochastic SCC landed on.

#### 13.3 Automatic Marginalization

When a latent variable has no downstream observation and the
integral is closed-form, the compiler eliminates it by
marginalization without user directive. Criteria:

- The latent's family has a closed-form marginal available via
  capability contract (e.g., marginalizing σ² out of
  Normal-InverseGamma yields a Student-t posterior).
- No relation outside the marginalized pair references the
  latent's sampled value.

The marginalized form lives as an envelope fact on the
resulting parent distribution. Failed marginalization falls
through to Tier B/C dispatch (§13.2). Users who want to forbid
a particular marginalization attach an `observe`-style tether
that keeps the latent's value in scope.

#### 13.4 SDE Convention: Itô vs Stratonovich

SDE draws carry an integration-convention generic:
`x ~ BrownianMotion<Ito>(...)` vs
`x ~ BrownianMotion<Stratonovich>(...)`. The convention is a
type parameter on the stochastic family, not a global setting.
The compiler uses it to route drift/diffusion rewrites
correctly. Default is `Ito`. Mismatched conventions within one
SCC are a compile error; the compiler does not auto-convert.
Cross-scope consistency is the user's call — one `.myco` file
may contain both conventions at different loci.

#### 13.5 Independence via Structural Identity

Two stochastic draws are independent iff their e-classes are
distinct. `x ~ Normal(μ, σ)` and `y ~ Normal(μ, σ)` on separate
lines produce two e-classes and are independent. A shared
intermediate (`let z ~ Normal(μ, σ); x = z; y = z`) produces
one e-class: x and y are the same draw, fully correlated.

There is no naked correlation surface. No `Cov(x, y) = ρ`, no
`correlate(x, y)`. Correlated structures are built by sharing
upstream distributions or by declaring a joint family (MVN,
Mixture, `JointDistribution` in chunk 08) that bakes the
correlation in. The mechanism matches the three-layer
principle: equational identity of e-classes is the only
language-level handle on independence.

#### 13.6 Cholesky Reparameterization (Z10)

An MVN draw `x ~ MultivariateNormal(μ, Σ)` reparameterizes to
`x = μ + L @ ε` where `L L^T = Σ` and `ε ~ Normal(0, I)`. The
Cholesky factor L is the compiler's canonical intermediate
for MVN machinery:

- Samples come from standard-normal draws plus a matrix
  multiply. Gradients flow through L.
- Positive-definiteness of Σ is encoded structurally by L's
  positive-diagonal refinement — no runtime PD check.
- Observations against x translate to observations against L
  and ε via the affine relationship; likelihood flows back to
  the training loss through matrix-calculus rewrites.

L can be supplied directly by the workflow
(`learn_constant` on L with positive-diagonal refinement) or
derived from a specified Σ at compile time. Non-MVN joints
that structurally factor as affine-in-noise trigger the same
pattern via `ReparameterizedSampleable` (§7.2).

#### 13.7 Field Sampling with `.at()`

For distributions returning structured samples (joints, named-
field-valued), `.at("field_name")` extracts a named field:

```
joint_sample ~ JointDistribution(...)
height = joint_sample.at("height")
```

`.at()` accesses participate in e-graph identity: the same
`.at("height")` on the same sample collapses to one e-class
(so the field value is consistent everywhere it is read).
`.at()` on a missing field is a compile error — the family
declares its named fields statically. This is the only joint-
sample field-access surface; no tuple destructuring, no
positional index access.

#### 13.8 Observation Injection and Likelihood Back-Propagation

`observe(data, x ~ D)` injects observed data into a
stochastic SCC. Mechanism:

1. The observed value becomes an envelope fact on the e-class
   of x (layer 2 of the three-layer split; §16). The e-class
   itself is not merged with a constant.
2. Downstream relations that read x's sampled value see the
   observation; downstream samples are conditioned on it.
3. Likelihood `D.log_pdf(data)` contributes to the SCC's loss
   during training emission (§25); back-propagation through
   the model graph reaches learnable upstream parameters.

The critical distinction from `identify` (§17 merge source 4)
is that `observe` does not make `x = data` equationally. It
narrows the distribution, not the value. The same x elsewhere
in the model stays stochastic — the observation is information,
not an equation.

#### 13.9 Observed Samples as Envelope Facts

`observe` attaches layer-2 distributional metadata; it does
not introduce a new e-graph merge source. The envelope fact
says "this e-class has observed data attached" — it narrows
the distribution and drives likelihood contribution (§13.8),
but the equational core (layer 1) is unchanged.

Consequence: observations compose with other envelope facts
(refinement bounds, capability advertisements, tolerance
envelopes) without equational conflict. The enumeration in
§17 remains eight sources — the probabilistic `observe`
verb is not the ninth. This preserves the layering principle
of §16: layer 1 is monotonic equational merges only; layer 2
carries distributional and tolerance metadata; observations
live there.

Terminology. §17 source #2 ("workflow constant injection")
and the probabilistic `observe` verb share the colloquial name
"observation" but are distinct mechanisms: constant injection
collapses an e-class with a literal (layer 1); `observe`
attaches a distributional fact (layer 2). The distinction is
by layer, not by spelling.

#### 13.10 Tier 2 PPL Lock

The 2026-04 Tier 2 PPL design lock extended the core `~`
mechanism to cover the remaining probabilistic-programming
surfaces without committing surface syntax for all of them:

- **Coupling machinery (B4).** Joint distributions whose
  components share structural dependencies. Declared via the
  joint family definition, not via imperative conditioning
  calls. Syntax deferred to chunk 08.
- **Joint declaration syntax (B2).** Surface for user-defined
  joint families with multiple named fields. Deferred to
  chunk 08. `.at()` (§13.7) is the access pattern once the
  syntax lands.
- **Higher-order distributions.** Distributions over
  functions (Gaussian processes, etc.) route through kernel
  machinery (§28) rather than the parametric Tier 1 list.

The lock closes "does this primitive have a home?" without
freezing every keyword. Tier 1 primitives (§27) remain the
v2.1 ship surface; Tier 2 primitives land in chunk 08 and
§28.

### 14. Compiler Intrinsics

`deriv`, `integrate`, `condition_of` (Levels I symbolic / II algorithmic
/ III runtime), `loss_of`. What each intrinsic means, what the compiler
guarantees about it, how it interacts with the e-graph.

#### 14.1 `condition_of` — Levels I, II, III

`condition_of(expr)` returns a conditioning estimate for an
expression. Three levels of evaluation, tagged in the return
type so downstream code can distinguish:

- **Level I — Symbolic.** Closed-form condition number derived
  from the e-graph's algebraic structure (e.g., condition of
  a triangular solve against its diagonal). Available when the
  expression's conditioning is itself a closed-form function
  of the inputs.
- **Level II — Algorithmic.** Condition number of a specific
  algorithm realizing the expression (e.g., Gaussian
  elimination's condition when applied to a given matrix),
  selected by the compiler's lowering decisions.
- **Level III — Runtime.** Numerically computed at execution
  time. Fallback when neither symbolic nor algorithmic form
  is available.

The mode is tagged in the return; `condition_of(expr).level`
surfaces which tier the compiler chose. Algorithmic-vs-problem
duality: Level I is the *problem's* conditioning (intrinsic to
the math); Level II is the *algorithm's* conditioning (depends
on lowering choice). The two can diverge, and `condition_of`
makes the distinction inspectable. Primary consumer: the Y4
`condition_weighted` closure policy (§8.7).

#### 14.2 `loss_of` — Named-Field Return

`loss_of(residual)` returns a struct of named loss components,
not a scalar. Fields cover the residual's loss sources:

- `data_fit` — likelihood / observation mismatch terms.
- `constraint_violation` — projection/penalty terms from
  `constraint` blocks (§8.1) not discharged at compile time.
- `regularization` — prior log-densities on learned parameters.

Users select components by name for training (§25) — e.g.,
`bind_loss(loss_of(residual).data_fit + 0.1 *
loss_of(residual).regularization)`. Aggregation to a scalar is
the workflow's call. The compiler does not auto-sum; scalar
loss is a workflow composition, not a language default.

#### 14.3 `integrate` — Domain, Units, E-Graph

`integrate(f, x, domain)` returns the integral of `f(x)` over
`domain`. Semantic commitments:

- **Domain.** A locus or interval with endpoints (or a full
  locus surface). Non-compact domains require explicit
  treatment via `limit` or truncation — no implicit
  compactification.
- **Units.** Result units are `[f] · [x]`. Integrating a
  `Scalar<kg/m>` over `Scalar<m>` yields `Scalar<kg>`. The
  unit algebra is mechanical; `integrate` participates in
  the same unit reconciliation as arithmetic.
- **E-graph interaction.** Integration-by-parts fires as a
  stdlib rewrite when capability contracts permit
  (`Differentiable` on the integrand's factors). Definite
  integrals with closed-form antiderivatives collapse to
  the antiderivative evaluation; others remain as
  symbolic `integrate` terms until lowering chooses a
  quadrature.

`integrate` is distinct from SDE-style stochastic integration
(§13.4), which has its own Itô/Stratonovich convention.

### 15. Approximate Blocks

The 2×2 matrix of approximation flavors: (lossy-model vs
lossy-tolerance) × (univariate vs bivariate). Syntax, semantics,
envelope consequences.

#### 15.1 Block Syntax

An `approximate` block authorizes one specific lossy rewrite
for a named scope. Fields:

```
approximate {
  under:           <rewrite-id or rewrite-family-id>
  tolerance_class: <entry-wise | operator-norm | spectral | structural>
  error_bound:     <expression in the input quantities>
  body:            <the expression the approximation scopes over>
  where:           <optional predicate narrowing applicability>
}
```

- `under` names which specific approximation is permitted
  (Delta method, Fenton-Wilkinson, CLT, a named smoothing).
- `tolerance_class` declares how error is measured (§16.4).
- `error_bound` is the user's commitment to acceptable error
  magnitude; the compiler rejects the rewrite if its
  certified bound exceeds this.
- `body` scopes the rewrite to a specific expression or
  residual block.
- `where` optionally gates applicability on input conditions
  (e.g., `where: variance / mean^2 < 0.1` for Delta-method
  linearization).

Blocks compose by nesting. Outside a block's `body`, the
authorized rewrite does not fire. No global `approximate`
scope exists; approximation is always explicitly chosen.

#### 15.2 Auto-Derived Lossiness (Four Sources)

The compiler derives an expression's lossiness from four
cumulative sources:

1. **Stdlib atom contracts.** `log(exp(x)) = x` is lossless via
   `Invertible` on both; `atan2(sin, cos)` is lossy unless
   refined.
2. **Approximation-block declarations.** Every active
   `approximate` block contributes its declared error class
   to the expressions it scopes.
3. **Numeric type choices.** `Float64` arithmetic carries
   unit-in-last-place rounding; `Rational` is exact (with
   termination caveats, §26). The compiler's Tier-C backend
   dispatch can force precision-loss rewrites.
4. **Backend emulation paths.** If a backend lacks a capability
   (capability-advertising, §31) and the workflow permits
   emulation fallback, emulation's error class enters the
   derivation.

The compiler reports the aggregate lossiness per expression
via inspection surfaces (§22). The four sources are
independent contributions; lossiness is a lattice join over
them, not a single authoritative source.

#### 15.3 Three-Tier Lossiness Cut

For diagnostics and Tier B dispatch (§13.2), lossiness groups
into three tiers:

- **Lossless.** Equational rewrites only; no numerical error
  beyond the base numeric type. `log(exp(x)) = x` under
  `Invertible`, stdlib identity rewrites.
- **Lossy-model.** Modeler-chosen approximations — smoothing
  helpers (§8.9), closed-form statistical approximations
  (Delta method, CLT, Fenton-Wilkinson). The model itself is
  an approximation; the compiler surfaces which one.
- **Lossy-tolerance.** Numerical tolerance intrinsic to the
  solve: floating-point rounding, quadrature truncation,
  iteration-convergence tolerance. Independent of modeler
  intent; bounded by the backend and the residual's
  conditioning.

The cut lets diagnostics say "this output is exact modulo
floating-point" vs "this output uses a Delta-method
linearization the modeler authorized" vs "this output is
a tolerance-gated iterative solve." Three different trust
postures, surfaced distinctly.

---

## Part II — Compiler Substrate

What the compiler sees and manipulates.

**Hierarchical SCC decomposition.** The compiler's central
structural operation is decomposing the relation graph into
strongly connected components (SCCs) at multiple scales. The top-
level decomposition partitions the full model into SCCs over
variables; each SCC becomes a residual block under §18's
classification. Within each SCC, the compiler may further
decompose. Tier A stochastic closed-form SCCs (§13.2) may nest
within deterministic SCCs. Tier B lossy-model SCCs may contain
Tier A subcomponents. Numerical solve SCCs may nest around
stochastic kernels (§13.8's observation ingestion reaches into the
surrounding SCC). Decomposition proceeds until every SCC is either
a single-verb residual block (solve, sample, project) or fails to
decompose further.

The tiered nesting lets the compiler dispatch different solvers
per level. A deterministic outer iteration wrapping an inner
stochastic sampler is routine; the outer iteration does not need
to know what the inner SCC does, only that the inner SCC commits
to an output e-class. Each SCC carries its own classification
(§18), residual flavor (§19), and tolerance envelope (§16.4). The
decomposition is the bridge between the e-graph's global equational
substrate (§16) and the per-block solver dispatch of Parts II-III.

### 16. The E-Graph

The e-graph as the internal equality substrate. Three-layer split:
(1) equational core, (2) envelope metadata attached to e-classes,
(3) adjacent keyed state (timesteps, events, identity-tagged copies).

#### 16.1 Three-Layer Scoping Split

The e-graph is structured as three concentric layers. Each layer
has its own modification rules and its own consumers. Every
downstream section in Part II assumes this layering; the
principle is restated in §0 as a structural commitment.

1. **Equational core (layer 1).** Union-find of e-classes under
   merge equalities. Monotonic (§16.2). The eight merge sources
   (§17) all write here. Relation equations, `identify`,
   stdlib rewrites, conversion-group merges.

2. **Envelope metadata (layer 2).** Facts attached to e-classes
   that narrow or qualify the class without merging it with
   another. Refinement bounds, distributional metadata from
   `~` (§13.8), capability advertisements from contracts
   (§7.2), observed samples (§13.9), tolerance envelopes
   (§16.4). Monotonic in aggregate (facts compose; none
   retract).

3. **Adjacent keyed state (layer 3).** Structures indexed by
   temporal subscript, event firing, or identity tag, but
   holding e-class references internally. `y[1]`, `y[2]`,
   …; per-event copies; identity-tagged instances. The layer
   is a dispatch table; per-key updates are independent and
   do not interact equationally with other keys except via
   explicit relations (`step(y) = expr` writes `y[t+1]` from
   `y[t]`).

Layer choice is how a construct participates. Merges write
layer 1; contracts and observations write layer 2; timesteps
and events index layer 3. Downstream consumers read the layer
relevant to their task; diagnostics surface which layer a
fact lives on (§22).

#### 16.2 Monotonicity Invariant

The equational core is append-only. Once two e-classes merge,
they stay merged; once an envelope fact attaches, it stays
attached. No retraction, no tombstoning, no rollback. This is
the substrate-level version of referential truth (§0 principle
5): world-claims accumulate; they do not unwrite.

Consequences:

- `replaces` (§8.10, §10.5) suppresses default generation; it
  does not retract an already-emitted fact.
- Events add facts; they do not remove prior e-classes. Dead
  entities continue to exist equationally; their absence from
  subsequent ticks is a layer-3 keyed-state fact, not a
  layer-1 deletion.
- Envelope metadata compositions must be closed under join —
  two facts about the same e-class combine into a single
  stronger fact, never replacing either.

Operationally: the e-graph does not need rollback machinery or
undo logs. Compilation is a left-to-right accumulation; the
final state is the union of every fact ever claimed.

#### 16.3 Envelope Ownership

Envelope facts (layer 2) have three classes of writer, one
class of reader, and no invalidator:

**Writers.**
- **Stdlib contracts.** Capability advertisements (`Invertible`,
  `Differentiable`, `AffineSelfClosed`, etc.) attach on type
  or family declaration.
- **Compiler rewrites.** Tier B approximations, refinement
  inference from relation bodies, conservation-group
  induction from `{ conserved }`.
- **`observe` verb (workflow).** Attaches observation envelope
  facts at workflow composition time (§13.8, §13.9).

**Readers.**
- **Tier A/B dispatch** (§13.2) consumes capability facts to
  select closed-form or approximate routing.
- **Extraction pipeline** (§19) reads refinement and tolerance
  facts to choose projection flavors.
- **Diagnostics / `mycoc explain`** (§22) reads every envelope
  fact and surfaces provenance.

**Invalidators.** None. The monotonicity invariant (§16.2)
forbids retraction; envelope facts are as permanent as
equational merges. If a fact conflicts with a later fact, the
compiler emits a coherence error rather than silently
preferring one.

#### 16.4 Envelope Flavors

Tolerance envelopes (a subclass of envelope facts) carry a
flavor declaring how error is measured. Four flavors:

- **Entry-wise.** Error bound applies independently per
  element. Used for tolerance-class statements about scalar
  fields, component-wise vector results.
- **Operator-norm.** Error measured by induced matrix norm
  (spectral radius, Frobenius, etc.). Used for
  matrix-valued approximations where worst-case singular
  behavior matters.
- **Spectral.** Error bound on eigenvalue / singular-value
  behavior specifically. Used when the downstream consumer
  cares about spectral properties (stability analysis,
  conditioning).
- **Structural.** Error bound on combinatorial or pattern
  properties (sparsity pattern preserved, positive-definiteness
  preserved). Zero-numerical-tolerance flavor — either the
  structural property holds or the rewrite does not apply.

Each flavor has its own composition rule: entry-wise bounds
compose by summation under triangle inequality; operator-norm
by sub-multiplicativity; spectral by Weyl-style inequalities;
structural by set intersection. `approximate` blocks (§15.1)
declare flavor in `tolerance_class`; Tier B rewrites
(§13.2) route via flavor to the appropriate approximation
family.

### 17. Equality-Introducing Machinery

Eight enumerated merge sources: explicit relation equations,
workflow constant injection, algebraic rewrites, `identify`,
stdlib-declared function inverses (via capability contracts on
fns; see §6), named-type conversion, closure-policy co-membership,
unit-preserving rewrites. The 2×3 faithfulness × orientation matrix
covering `convert`, `identify`, `approximate`, relation `=`.
Unified rewrite-predicate language.

Terminology note. "Workflow constant injection" is the merge
source by which a workflow-bound numeric constant (`provider.bind`,
`assume_constant`, `bind_known_constants`) collapses the e-class
of a model variable with the e-class of a literal value. This is
distinct from the probabilistic `observe` verb (§13.8), which
attaches distributional metadata as an envelope fact and is not
a merge source. Two mechanisms, one unfortunately-similar name;
the distinction is by layer (§16.1), not by spelling.

#### 17.1 The Eight Merge Sources — Prose

The e-graph's equational core (layer 1 of the three-layer split,
§16.1) accepts merges from exactly eight sources. Each source
has a declaration surface, a trigger condition, and a
faithfulness posture (§17 preamble matrix).

1. **Explicit relation equations.** A `relation { x = expr }` or
   inline `x = expr` asserts an equation; the compiler merges
   the e-class of `x` with the e-class of `expr`. The canonical
   user-visible source.
2. **Workflow constant injection.** `assume_constant`,
   `bind_known_constants`, and related workflow verbs (§24)
   collapse a model variable with a literal value supplied by
   the workflow. Mechanism: at workflow composition the binding
   becomes an equation `variable = <literal>` and fires as
   source 1. Distinct from the probabilistic `observe` verb,
   which writes layer 2 (§13.8, §13.9).
3. **Algebraic rewrites.** Commutativity, associativity,
   distributivity, identity elements, and similar ring-algebra
   rewrites fire from stdlib declarations on arithmetic
   operators. They introduce merges between structurally
   different but equivalent terms (`a + b = b + a`).
4. **`identify` declarations.** `identify x = y` asserts two
   already-declared entities are the same thing (§17.2).
   Merges their e-classes. Distinct from relation `=`, which
   asserts an equation that holds.
5. **Stdlib-declared function inverses.** Capability contracts
   on stdlib atoms (`Invertible<inv=log>` on `exp`) fire
   rewrites like `log(exp(x)) = x` on qualifying input
   domains. The user has no annotation path; derivation is
   compiler-side (§17.3).
6. **Named-type conversion.** `convert A <-> B` (bare or
   parameterized) injects equality between the A- and B-tagged
   e-classes. Bare converts in conservation groups (§3.7)
   produce sibling-magnitude merges.
7. **Closure-policy co-membership.** Y-group closure policies
   (§8.7) that combine multiple candidate claims into one
   forward value produce co-membership merges at the residual
   level — the merged result is one e-class whose contributors
   are tracked as provenance, not independent equations.
8. **Unit-preserving rewrites.** Dimensional algebra on
   unit-tagged expressions (§5) simplifies under unit-preserving
   equalities (`3 m * 4 m = 12 m²`, `x kg / x kg = 1`). These
   are lossless by construction and always fire.

The eight are enumerated because downstream tooling
(diagnostics, `mycoc explain`, provenance reporting) needs to
know which source produced any given merge. Source tags travel
with merges through the e-graph.

#### 17.2 `identify` vs Relation `=`

Both produce e-class merges, but the user-facing semantics
differ:

- **Relation `=`.** "This equation holds in this world." The
  equation participates in normal overdetermination analysis
  (§8.6), closure policies (§8.7), and solving. Multiple
  relations on the same variable can create a redundant-
  consistent residual to be closed.
- **`identify x = y`.** "x and y are the same thing." No
  equation, no residual. The compiler treats them as a single
  entity from the declaration onward. No closure-policy
  consequences; no redundancy check (identity is idempotent).

Use `identify` when two names refer to the same object
(refactoring, alias establishment, renaming). Use `=` when two
expressions are equal in value but conceptually distinct
things. Surface-level: `identify` lives at module scope or
inside type bodies; relation `=` lives inside relation bodies.

#### 17.3 Function Inverses via Stdlib Capability Contracts

Function-inverse merges fire from stdlib-declared capability
contracts on atoms, not from user annotations. `exp` declares
`Invertible<inv=log, domain=Real>`; the e-graph then fires
`log(exp(x)) = x` wherever `x: Real` holds structurally (and
symmetrically for `exp(log(x)) = x` on `x: Positive`).

The user has no annotation path to declare a function
invertible. Option B (§6) commits this: stdlib carries
capability contracts; user functions have no property-
declaration surface. If a user function needs inverse
recognition the compiler cannot derive, the user refactors
the function into structurally composable pieces using stdlib
atoms with the requisite contracts.

Consequence: the function-inverse rewrite catalog is
inspectable from the stdlib alone. Users cannot extend it by
annotation; they extend it by composition.

#### 17.4 Unified Rewrite-Predicate Language

All merge sources use one predicate language for expressing
guards. A rewrite predicate can reference:

- Refinement predicates on participating types (`x: Scalar<m>
  where { x > 0 }`).
- Capability satisfaction (`D : Distribution + AffineSelfClosed`).
- Structural shape (generic arity, tensor rank, contract
  satisfaction).
- Unit algebra (dimensional matching).

Predicates are compile-time only; runtime-observed values do
not drive rewrites. The unified language means a Tier B
approximate rewrite (§13.2) uses the same predicate form as a
stdlib algebraic rewrite, which uses the same form as a
`convert` body. One surface, one discharge procedure
(e-graph reasoning with refinement + contract lookup).

#### 17.5 Rewrite-Rule Groups A-Y

Rewrite rules are organized into lettered groups by category.
The compiler and stdlib commit to the grouping for inspection,
debugging, and approximate-block referencing. Representative
groups:

- **A — Algebraic** — commutativity, associativity,
  distributivity, identity elements.
- **E — Equality / merge** — source-specific rewrites
  following the eight-source enumeration.
- **Y — Closure-policy** — the Y1-Y6 policies (§8.7).
- **Z — Distribution-family** — conjugate posteriors, affine
  closures (§27).

The complete A-Y catalog is large and belongs in an appendix,
not §17 prose. The appendix is tracked in §34 Chunk-Slotted
Work and will ship with the stdlib reference; chunk 04 already
commits partial enumeration. Approximate blocks (§15.1)
reference rules by group letter in their `under:` field.

#### 17.6 Baseline Rewrite Partition

Rewrites partition into **default-on** and **default-off**
buckets:

- **Default-on.** Fire unconditionally whenever their
  predicate (§17.4) holds. Includes: relation-`=` merges,
  algebraic rewrites (A-group), stdlib function-inverse
  rewrites (E-group), named-type conversion, unit-preserving
  rewrites, `identify`, `assume_constant` injections. All
  lossless or modeler-asserted.
- **Default-off.** Fire only inside an authorizing
  `approximate` block (§15.1). Includes: Tier B statistical
  approximations (Delta method, CLT, Fenton-Wilkinson),
  smoothing substitutions (`max` → `smooth_max`), numerical-
  tolerance rewrites that exceed the default precision.

The partition is what gives `.myco` its conservative default
posture — a model compiles with zero authorized
approximations if the modeler wrote none, and any lossy
rewrite is traceable to a specific block. Default-off
rewrites fire one-at-a-time, scoped to the block's `body`;
they do not compose across blocks without explicit nesting.

### 18. The Type Graph

STUB — chunk 07 pending. The type graph is a separate substrate from
the expression e-graph, carrying named-type relations (subtyping,
conversion, conservation-group membership, refinement implications).
Its interaction with the e-graph (how named-type conversions inject
expression-level merges, how refinement obligations translate to
rewrite predicates) is the chunk 07 deliverable.

### 19. Residual Graph (Projection)

The residual graph as a user-facing diagnostic view projected from
the e-graph. Extraction decisions and what they yield. How
diagnostics reference which view.

#### 19.1 Extraction Cost Model

Residual extraction from the e-graph optimizes against a
**multi-dimensional cost vector**, not a single scalar. Cost
dimensions:

- **Precision.** Aggregate lossiness class (§15.3): lossless
  preferred over lossy-model preferred over lossy-tolerance.
- **Latency.** Estimated floating-point cost, memory bandwidth,
  backend-specific kernel availability.
- **Memory.** Peak allocation, intermediate buffer count.
- **Approximation class.** Which `approximate` blocks (§15.1)
  the extraction activates, if any.

Extraction returns a Pareto front in the cost space by default;
workflow configuration selects a specific point
(latency-first, precision-first, or weighted). No default
scalar weighting — the compiler does not assume one dimension
dominates.

Consequence: the same e-graph yields different residuals under
different workflow policies. The residual graph is a projection
*parameterized by cost preference*, not a canonical form.

#### 19.2 Residual ↔ E-Graph Projection Mechanics

The extractor walks the e-graph top-down, choosing one
representative term per e-class subject to the cost vector
(§19.1). Open items tracked in §35 (Tier 0 Phase 2 Q3):

- **Root set.** How the extractor identifies which e-classes
  anchor the residual (variables the workflow binds plus
  output quantities referenced by `observe`).
- **Sharing policy.** When two paths through the e-graph reach
  the same e-class, the extractor must decide whether to emit
  one shared binding or inline the term twice. Currently leans
  share-always; the performance tradeoff pends backend
  codegen decisions.
- **Envelope carriage.** Which layer-2 facts propagate into the
  residual as runtime assertions, which stay compile-time-only.

The mechanism is stable in broad strokes; the specific
heuristics are chunk 04 Tier 0 Phase 2 work and remain open.

#### 19.3 Residual Classification

Residual nodes receive classifications that pivot lowering and
diagnostics. Two orthogonal axes:

**Four-way SCC classification (§20).** Each residual SCC is
tagged `static` / `dynamic` / `stochastic` / `training`. The
tag determines lowering strategy and backend dispatch. This is
the compiler's primary classification.

**Three-way overdetermination classification (§8.6).**
Independently, each residual's equation set is classified
`redundant` (closure policies apply) / `provably inconsistent`
(hard compile error) / `conditionally inconsistent` (runtime
assertion). This classification gates whether closure policy
selection is even meaningful.

A single residual carries both tags. Diagnostics surface the
pair: "this residual is a stochastic SCC with conditionally
inconsistent equations," for example.

#### 19.4 Saturation Termination and Rewrite Scheduling

The e-graph applies rewrites until saturation or termination
bound. Scheduling and termination guarantees:

- **Default-on rewrites** (§17.6) are applied to fixed point.
  The subset is designed to terminate: algebraic rewrites are
  confluent under standard orientations; unit-preserving
  rewrites reduce complexity; stdlib capability contracts are
  oriented (`log(exp(x)) → x`, not the reverse).
- **Default-off rewrites** fire only under authorizing
  `approximate` blocks and each within a block has an explicit
  error bound and a `where:` guard. Within a block, scheduling
  is round-robin over active rewrites up to the authorized
  error budget.
- **Scheduling priority.** Merges from explicit relation `=`
  and `identify` (sources 1 and 4, §17.1) fire first;
  algebraic and unit-preserving rewrites next; conversion and
  closure-policy last. Order affects extraction choice but
  not correctness; the final e-graph is determined by the
  rewrite set, not the order.
- **Termination bound.** An absolute rewrite-count cap
  (workflow-configurable) prevents pathological
  non-terminating cases. Hitting the bound is a compile
  warning, not an error; the partial e-graph still extracts
  a residual. Practical models do not approach the bound.

Non-confluent rewrite sets (rare; only possible via
`approximate` blocks that introduce oriented lossy rewrites in
both directions) are a compile error detected at block
elaboration, before saturation runs.

### 20. SCC Decomposition and Component Classification

After constraint collection, the compiler decomposes the residual
graph into strongly-connected components. Each SCC receives a four-
way classification: **static** (fully resolved pre-run), **dynamic**
(timestepped), **stochastic** (distributional; requires sampling or
closed-form marginalization), **training** (gradient-optimized).
Classification pivots lowering, training emission, and backend
dispatch.

### 21. Lowering

N-max / alive-mask lowering for dynamic topology. `y[t]` and `y[t+1]`
as distinct ground terms (no per-timestep or template e-graph).
Handoff to the backend.

#### 21.1 Static vs Dynamic Module Classification

A `.myco` module is classified at compile time, before SCC
decomposition:

- **Static module.** No events (§10), no temporal relations
  (`d()` / `step()`, §9). Lowers to a single-pass
  computation; no runtime loop, no alive mask. Typical
  shape: a standalone constraint-satisfaction or
  algebraic-expression program.
- **Dynamic module.** At least one event or temporal
  relation. Lowers to a timestepped runtime loop with event
  dispatch and collection-mask management.

Static modules skip dynamic lowering machinery entirely.
Classification is a module-level fact; a dynamic module
remains dynamic even if only one of its SCCs is actually
time-dependent.

#### 21.2 Four-Way SCC Lowering Targets

Each SCC's class (§20) determines its lowering target.
The four targets are distinct compilation paths:

- **Static SCCs.** Resolved at compile time where possible
  (constant folding, unit simplification, algebraic
  extraction); otherwise a single evaluation emitted in
  the pre-loop prelude. No per-tick cost.
- **Dynamic SCCs.** Per-tick computation in the runtime
  loop body. Intra-SCC ordering resolved by the residual
  graph's topology; values at tick t depend on values at
  tick t-1 via explicit temporal terms (§21.3).
- **Stochastic SCCs.** Lowered to backend PPL primitives
  (§31) or an explicit sampler. Tier A closed-form
  marginals resolve at compile time; Tier B approximate
  rewrites pre-materialize their error-bounded form; Tier
  C hands off opaquely (§13.2).
- **Training SCCs.** Lowered to a gradient-producing
  computation. Loss exposure per residual (§25) enables
  workflow-selected scalar combinations; differentiability
  propagates through contained stdlib atoms via their
  `Differentiable` contracts (§7.2).

Class dominance: an SCC inherits the most expensive class
among its members. A stochastic variable inside an
otherwise dynamic SCC promotes the whole SCC to stochastic.
The compiler diagnoses the promotion at classification time
so the modeler can decide whether to split the SCC
structurally (by refactoring) or accept the promotion.

#### 21.3 `y[t]` and `y[t+1]` as Ground Terms

Temporal indexing produces distinct e-graph ground terms,
not a templated family. `y[1]`, `y[2]`, `y[3]` are three
different e-classes; temporal relations (`step(y) = expr`
writes `y[t+1]` from `y[t]`; `d(y) = expr` encodes a
derivative relation between adjacent ticks) connect them.
The e-graph does not "template" over time — there is no
symbolic `y[t]` class that specializes at runtime.

Consequences:

- Merges on `y[5]` are permanent (§16.2 monotonicity) but
  do not propagate to `y[6]` except through `step(y)` or
  `d(y)`.
- Closure policies (§8.7) applied at one tick do not
  commit later ticks to the same policy — each tick's
  residuals are independently classified.
- Temporal indexing composes cleanly with event-time
  topology (§12.4): an event retiring an entity at tick t
  leaves every prior `entity[s < t]` ground term valid in
  the e-graph.

Lowering maps each ground term to a backend storage slot.
For a bounded-time run with T ticks, the backend allocates
T slots per temporal field; streaming runs use rolling
buffers sized to the maximum temporal-lookback depth the
module references.

#### 21.4 N-max Slots and Alive Masks

Event-time collections (§12.4) lower to a fixed-capacity
array plus an alive mask.

- **N-max selection.** The collection declares an N-max
  capacity at its declaration site. Workflow override via
  `bind_topology` (§24) is permitted up to a
  compile-enforced ceiling.
- **Alive mask.** One Boolean per slot, stored as a packed
  bitmap (or SIMD-lane-aligned on GPU backends).
  Iteration primitives (§12.6) gate kernel lanes via the
  mask; dead slots contribute no work without introducing
  divergent branches.
- **Allocation.** Events that create entities claim the
  next free slot (free list maintained at runtime).
  Allocation is O(1) amortized; deterministic under a
  given workflow seed.
- **Retirement.** Events that retire entities flip the
  alive bit. Under monotonicity (§16.2), the entity's
  e-classes continue to exist equationally; the alive
  mask is a layer-3 adjacent-keyed-state fact, not an
  e-graph deletion. Dead entities "do not know they are
  dead" (§0 principle 5).
- **Overflow.** Exceeding N-max is a runtime error with a
  specific diagnostic. The workflow chose the capacity;
  the compiler does not silently grow the buffer.
  Workflow tooling can inspect alive-slot high-water marks
  across runs to calibrate N-max before production.

### 22. Plan Inspection

The compiled program is an output artifact. Reproducibility is
guaranteed by `.myco` plus workflow Python together. Compiled code
is inspectable via `mycoc explain` (and related CLI surfaces, §36)
for users who want to audit the plan, debug behavior, or verify
compilation choices. Inspection is a debugging affordance.

---

## Part III — Workflow Interface

The boundary between `.myco` and Python.

### 23. The `.myco` ↔ Python Boundary

`.myco` declares structure; Python supplies values and drives
execution. The compiler does not auto-emit projection or solver
selection; those are workflow choices (§0.1 projection-free
compiler). All numeric values (physical constants, fit parameters,
data series, initial conditions, topology, observations) cross
this boundary.

#### 23.1 Runtime `where` at Workflow Composition

`where` appears at three layers, each with its own semantics:

- **§8.3 Compile-time type narrowing.** `where x is T` in a
  relation body narrows x's type for the subsequent
  expression.
- **§12.7 Collection filter.** `for x in xs where x is T`
  filters iteration to T-inhabiting elements.
- **Workflow composition gate (this subsection).** A
  workflow binding may attach a `where` predicate that
  gates the binding's application. Example:
  `assume_constant("config.dt", 0.01, where=scenario ==
  "high_res")`. The predicate evaluates at composition,
  not at runtime; the compiled artifact carries only the
  selected bindings.

The three uses share the keyword but live at three different
layers — compile, iteration, composition. Context
disambiguates; diagnostics name the layer when the keyword
appears ambiguously.

#### 23.2 Multi-Binding Compilation

One `.myco` compiles once to a plan; many workflows bind the
same plan under different value configurations.

- **Plan.** Compile emits a plan parameterized by its binding
  surface: which constants, series, topology, controllers,
  priors, and observations the plan accepts.
- **Instantiation.** Each workflow supplies concrete values
  for the parameterized surface via §24 verbs. The compiled
  artifact is shared across workflows; binding is cheap.
- **Callable weight reuse.** Trained weights on callables
  attached via `bind_controller` (§24.1) persist across
  workflows that bind the same controller's contract.
  Calibration on one dataset transfers to prediction on
  another without recompilation.

This is the reuse story that makes `.myco` valuable beyond
single-run scripts. The compiler's job is to produce a plan
that accepts many workflows; the workflow's job is to bind
values that make the plan concrete for this run.

#### 23.3 Cross-Study Callable Reuse via Plain Contracts

Callables cross study boundaries by conforming to plain
contracts (§7). The "data contract" kind is retired (see
anti_spec.md); callables advertise their output type's
contract, and workflows accepting that contract can bind the
callable.

Example. A controller trained in study A outputs values
satisfying `PhotosynthesisRate : Scalar<μmol_CO2_m2_s> +
Positive`. A workflow in study B that consumes
`PhotosynthesisRate` can bind the same trained callable,
provided study B's required input contract matches the
callable's declared input contract. Contract satisfaction is
checked at workflow composition; mismatches surface as §23.4
composition errors.

The mechanism handles the "train once, reuse" story without a
separate contract kind or a stateful cross-workflow runtime.
The shared artifact is trained weights plus a plain contract
— no extra machinery.

#### 23.4 Error Tiers: Compile vs Workflow Composition

Errors surface at two distinct layers:

- **`mycoc` compile errors.** Structural problems in the
  `.myco`: type mismatches, missing contracts, unresolved
  universals, undischargeable relations, conservation
  violations (§3.7), provable inconsistency (§8.6 case 2),
  coherence errors from contract conflict (§7.4). Detected
  before any workflow binds; the plan cannot be produced.
- **Workflow composition errors.** Problems visible only
  once bindings arrive: capability mismatches (backend
  does not advertise a required capability, §31), shape
  mismatches (bound tensor disagrees with plan's expected
  shape), contract violations on bound callables, N-max
  ceiling exceeded at `bind_topology` (§21.4), missing
  required binding. Detected by workflow composition; the
  plan exists but cannot run.

Both tiers emit user-directed diagnostics. Tooling
distinction: `mycoc check` catches tier-1 errors; workflow
composition surfaces tier-2. Runtime errors (numerical
divergence, overflow, solver non-convergence) are a third
tier that this spec does not address normatively — they live
in backend and deployment surfaces.

### 24. Eight Workflow Verbs

`assume_constant`, `assume_series`, `learn_constant`, `learn_initial`,
`learn_trajectory`, `bind_controller`, `bind_topology`, `observe`. For
each verb: what it binds, when it fires, gradient-flow implications.

#### 24.1 `bind_controller` — Contract I/O Specification

`bind_controller(path, fn, input_contract, output_contract)`
attaches a Python callable to a named `.myco` site. Both
contracts are plain contracts (§7); there is no separate
"data contract" kind (retired to anti_spec.md; subsumed
2026-04-21).

- **Path.** Names the binding site in the `.myco` model.
  One path per controller instance; multi-binding is
  supported (§23.2) through the same mechanism other
  verbs use.
- **`fn`.** The Python callable. Typically a neural net
  module, but any callable that conforms to the declared
  contracts works.
- **Input contract.** Types the controller reads from its
  scope. Names fields, units, refinements the controller
  requires. Compiler checks at workflow composition that
  the named fields exist in scope at the binding site.
- **Output contract.** Types the controller returns.
  Capability obligations on the output (e.g.,
  `Differentiable`, `Positive`, refinement bounds) drive
  downstream gradient flow and admissibility.

Controllers are purely workflow concept. No `.myco` keyword
introduces a controller; the binding is the only mechanism.
This retires the `slot` / `learn_slot` machinery and the
transparent-heuristic ABI (anti_spec.md).

#### 24.2 `bind_controller` — Gradient-Flow Semantics

Controllers usually wrap differentiable components (neural
nets with learnable weights). Gradient semantics:

- **Parameter registration.** The controller's internal
  learnable parameters register with the training loss at
  workflow composition. The workflow decides whether this
  particular run trains the controller (`learn_`) or
  freezes it (`assume_`); the choice is per-run, not per-
  controller.
- **Backward pass.** Loss gradients from `observe` (§13.8)
  flow through the model graph to the controller's output,
  into the controller's parameters, via the backend's AD
  facility (§31). The compiler treats the controller as a
  differentiable black box — it advertises
  `Differentiable` on its output contract; implementation
  is the backend's business.
- **Opaque-fn fallback.** Controllers without
  `Differentiable` are opaque: no gradient flows back, the
  parameters cannot be learned in the current run. Useful
  for fixed heuristics or non-differentiable routines
  (decision trees, symbolic rules) that replace a prior
  hard-coded behavior.
- **Cross-run weight persistence.** Trained weights
  persist across runs that bind the same callable (§23.3).
  A controller trained in one workflow is available as
  frozen in a later workflow by binding the same trained
  instance.

The controller is the seam where neural machinery attaches to
the scientific model. Gradient flow at this seam supports the
"neural controllers replacing heuristics" research direction;
opaque-fn fallback supports interop with non-differentiable
legacy code.

#### 24.3 `bind_topology` and §11 Geometry

`bind_topology(path, geometry, discretization=...)` is the
workflow-side counterpart to the `.myco` geometry declarations
of §11. The verb supplies:

- **Concrete mesh.** The specific discretization the run uses
  (resolution, element type FDM / FVM / FEM, refinement
  policy). Defaults from §11.6 apply if the workflow does not
  specify; production runs typically override.
- **Boundary identification.** Which physical sub-loci the
  named boundary regions of §11.2 correspond to in this run.
- **Material coefficients.** Workflow-supplied values for any
  `requires` coefficient blocks declared in the `.myco` locus
  (§11.10).
- **Event-time capacity.** Optional override of N-max for
  event-time collections embedded in the locus (§21.4), up to
  the compile-enforced ceiling.

`bind_topology` fires at workflow composition. The compiler
receives a concrete mesh and lowers spatial operators against
it. The verb is the only path by which geometry becomes
executable — `.myco` declares the locus structure, the verb
materializes a specific instance.

#### 24.4 Future Verbs Beyond the Eight

Positive statement of v2.1 scope: the eight verbs listed in
§24 preamble are the complete workflow-composition surface
for v2.1. No additional verbs ship in the first release.

Candidate future additions tracked for post-v2.1:

- **`bind_known_constants`** — batch form for binding many
  physical constants at once from a workflow-side table.
- **`bind_parameters`** — batch binding for empirical-fit
  parameter vectors (e.g., a full parameter sweep).
- **`assume_prior`** — explicit prior-distribution binding
  distinct from `learn_constant`, for cases where the user
  wants to specify a prior without declaring the constant
  as learned.

Each is deferred because the eight verbs cover the shipped
use cases and adding surface without concrete demand risks
coupling to specific workflow idioms. Revisit when Tier 2
PPL (§13.10) and chunk 08 lock; some may subsume into
existing verbs by that point.

#### 24.5 Run-Config and Workflow Configuration Surface

Run-config is the non-binding configuration the workflow
supplies at composition. Distinct from the eight verbs: run-
config does not bind model values; it configures how the
compiled plan executes.

Representative fields:

- `run.config.seed` — RNG seed for stochastic SCCs.
- `run.config.backend` — backend selection and its
  capability-fallback mode (error / host / emulate, §31).
- `run.config.verbosity` — diagnostics level.
- `run.config.dt` — when referenced via `assume_constant`
  in a discrete-time model (§9.1).
- `run.config.profile` — execution-profile hints (batch
  size, memory budget).

Run-config fields are referenced from workflow verbs as
strings (`assume_constant("run.config.dt", 0.01)`); the
compiler does not bake them into the plan beyond the
binding surface. Different runs of the same plan can use
different run-config without recompilation.

### 25. Training Emission

How the compiler emits gradient-trainable code for SCCs classified as
training (§20). Warm-start semantics (initial values from
`assume_constant`, or priors from `learn_constant`). Projection-
flavor selection (`hard_clip` / `sigmoid` / `soft_clip`) chosen by
the workflow. Per-residual loss exposure: users attach losses to
named residuals. Constraint enforcement strategy: compile-time
discharge where possible, runtime projection otherwise.

---

## Part IV — Standard Library

What ships with Myco.

### 26. Numeric Types

`Scalar<U, T = Float64>` with explicit `T` parameter and `Float64`
default. `Rational` for exact constant folding (with termination
caveats). `BigFloat`. Default-compatibility constraints.

#### 26.1 Numeric Representation Hierarchy

`Scalar<U, T>` takes an explicit numeric representation parameter
T. The v2.1 stdlib provides:

| T | Role | Notes |
|---|---|---|
| `Bool` | two-valued logic | consumed by boolean relations, predicates, alive masks |
| `Integer` | arbitrary-precision integers | exact; GPU-incompatible for arbitrary precision |
| `Rational` | exact rationals | §26.3 termination caveat; GPU-incompatible |
| `Float32` | IEEE single | backend-dependent availability |
| `Float64` | IEEE double | default; universal backend support |
| `BigFloat` | arbitrary-precision floats | exact rounding semantics; GPU-incompatible |
| `Complex` | complex numbers | v2.1 scope, representation and contracts open (§35) |

Forward-mode AD is not a user-facing representation in v2.1.
Backends own AD (§31); dual numbers would duplicate what the
backend already provides. Retired to anti_spec.md.

Default `T = Float64` is per-Scalar, not module-wide. Mixing
T within one expression is forbidden without explicit
`convert T1 -> T2` (§26.2).

#### 26.2 Default-Compatibility Constraints on T

The `T` parameter in `Scalar<U, T>` must satisfy a base
`Numeric` contract hierarchy:

- **Ring closure** (`Plus`, `Minus`, `Times`) — the four
  arithmetic operators close within T.
- **Total ordering** (`Compare`) — required for `min`,
  `max`, sort, `argmin`, `argmax`. Complex T does not
  satisfy total ordering; stdlib functions requiring it
  accept only ordered T.
- **Zero and one identity elements** — required for
  empty-collection defaults (§12.3), algebraic rewrites
  (§17.1 source 3).
- **Backend representability** — the run's backend must
  advertise support for T. Mismatch surfaces as a
  workflow-composition error (§23.4).

Mixed-T arithmetic is a compile error; the user must write
`convert T1 -> T2` explicitly. This makes numerical behavior
predictable: `Scalar<m, Float32>` and `Scalar<m, Float64>`
do not silently promote. Conversion `Float32 -> Float64` is
lossless; `Float64 -> Float32` emits the standard lossy-
tolerance envelope (§15.3).

#### 26.3 Rational Termination Caveat

`Rational` arithmetic is exact but unbounded. Numerator and
denominator grow with each non-trivial operation; iterated
exact arithmetic can blow up representation size. Two
compile-time guards:

- **Unbounded-loop warning.** `Rational`-typed state inside
  a temporal relation (`d` or `step`, §9) emits a compile
  warning. Warning, not error — some applications
  legitimately use `Rational` in bounded iterations (short
  exact simulations, verification runs).
- **GPU-incompatibility surface.** `Rational` has no GPU
  representation under any current backend (§31). Using
  `Rational` in an SCC that targets a GPU backend is a
  workflow-composition error. Same caveat applies to
  arbitrary-precision `Integer` and `BigFloat`; tracked
  collectively in §35 Other Opens.

`Rational` is useful for exact unit-conversion factors,
compile-time algebraic constant folding, and short bounded
computations where exactness matters. It is rarely the right
runtime representation for production models.

### 27. Distribution Families (Z-group)

Tier 1 univariate continuous families (19): Normal, LogNormal, Uniform,
Beta, Gamma, Exponential, ChiSquared, Cauchy, Student-t, Laplace,
HalfNormal, HalfCauchy, InverseGamma, Lévy, Weibull, Pareto, Fréchet,
Gumbel, GEV. Tier 1 discrete: Bernoulli, Categorical, Poisson,
NegBinomial, Hypergeometric. Tier 1 multivariate (gated on B5):
MultivariateNormal, Dirichlet, Multinomial. Meta-families: `Truncated<D>`,
`Mixture<D₁,…,D_N | weights>`. Conjugate-posterior rewrites.
Tier B approximate rewrites: Delta method, Fenton-Wilkinson, CLT,
block-maxima → GEV.

#### 27.1 Tier 1 Distribution Families — Table

Tier 1 families ship as capability-tagged stdlib declarations
(§7.2). Capability columns use shorthand: **D** =
`Distribution<U>`, **A** = `AffineSelfClosed`, **S** =
`SumSelfClosed`, **P** = `ProductSelfClosed`, **Sc** =
`ScaleSelfClosed`, **ST** = `SmoothTransformable`, **R** =
`ReparameterizedSampleable`, **Conj(X)** = conjugate to family X.

**Univariate continuous (19).**

| Family | Support | Parameters | Capabilities |
|---|---|---|---|
| `Normal` | ℝ | `μ`, `σ` | D, A, S, ST, R |
| `LogNormal` | ℝ₊ | `μ`, `σ` | D, P, ST |
| `Uniform` | `[a, b]` | `a`, `b` | D, R |
| `Beta` | `[0, 1]` | `α`, `β` | D, Conj(Bernoulli), Conj(Binomial) |
| `Gamma` | ℝ₊ | `α`, `β` | D, S (shared β), Conj(Poisson) |
| `Exponential` | ℝ₊ | `λ` | D, S (n-fold → Gamma), R |
| `ChiSquared` | ℝ₊ | `k` | D, S (shared k degrees) |
| `Cauchy` | ℝ | `x₀`, `γ` | D, S |
| `StudentT` | ℝ | `ν`, `μ`, `σ` | D |
| `Laplace` | ℝ | `μ`, `b` | D, ST |
| `HalfNormal` | ℝ₊ | `σ` | D, Sc |
| `HalfCauchy` | ℝ₊ | `γ` | D, Sc |
| `InverseGamma` | ℝ₊ | `α`, `β` | D, Conj(Normal variance) |
| `Lévy` | ℝ₊ | `μ`, `c` | D |
| `Weibull` | ℝ₊ | `λ`, `k` | D |
| `Pareto` | `[xₘ, ∞)` | `xₘ`, `α` | D |
| `Fréchet` | ℝ₊ | `α`, `s`, `m` | D |
| `Gumbel` | ℝ | `μ`, `β` | D, R (via `-log(-log U)`) |
| `GEV` | ℝ (domain-dependent) | `μ`, `σ`, `ξ` | D, block-maxima limit |

**Discrete (5).**

| Family | Support | Parameters | Capabilities |
|---|---|---|---|
| `Bernoulli` | `{0, 1}` | `p` | D, Conj(Beta) |
| `Categorical` | `{0 … K-1}` | `p[K]` | D |
| `Poisson` | ℕ | `λ` | D, Conj(Gamma) |
| `NegBinomial` | ℕ | `r`, `p` | D |
| `Hypergeometric` | `[max(0, n-(N-K)), min(n, K)]` | `N`, `K`, `n` | D |

**Multivariate (3, gated on B5).**

| Family | Support | Parameters | Capabilities |
|---|---|---|---|
| `MultivariateNormal` | ℝᵈ | `μ`, `Σ` | D, A, R (Cholesky, §13.6) |
| `Dirichlet` | simplex Δᵈ⁻¹ | `α[d]` | D, Conj(Multinomial) |
| `Multinomial` | `Σⱼ xⱼ = n` | `n`, `p[K]` | D, Conj(Dirichlet) |

B5 (matrix heterogeneous-unit resolution, chunk 05) gates
how `Σ` carries units in the multivariate group — per-row-unit
matrices vs globally-scalar-unit matrices. Resolution upstream
of final MVN shipping.

Meta-families (`Truncated<D>`, `Mixture<D₁,…,Dₙ | weights>`),
conjugate-posterior rewrites, and Tier B approximate rewrites
are covered in subsequent subsections.

#### 27.2 Meta-Families: `Truncated<D>` and `Mixture`

Two meta-families wrap base Tier 1 distributions (§27.1) to
produce new compositional distributions.

**`Truncated<D>` — interval truncation.** `Truncated<Normal>(μ,
σ, lo, hi)` restricts `Normal(μ, σ)` to the interval `[lo,
hi]` and renormalizes. Applies to any univariate D that
satisfies `Distribution<U>`. Capabilities: inherits D's
capabilities minus closures broken by truncation
(`AffineSelfClosed` is generally lost; `ReparameterizedSampleable`
survives via inverse-CDF sampling). Refinement types
(§3.2) interact cleanly: `x ~ Truncated<Normal>(0, 1, 0, 1)`
auto-satisfies `UnitInterval`.

**`Mixture<D₁, …, Dₙ | weights>` — weighted combination.** A
mixture of n component distributions with non-negative weights
summing to 1. Components can be distinct families; shared-
support requirement is enforced structurally. Weights are
themselves values — workflow-supplied (`assume_constant` or
`learn_constant`). Capabilities: `Mixture` is a `Distribution`
but closes under fewer algebraic operations than its
components; specifically, `AffineSelfClosed` survives only
when every component satisfies it.

Both meta-families compose: `Mixture<Truncated<Normal>(…),
Truncated<Normal>(…)>` is a legitimate Tier 1 construction.
Nesting depth is bounded only by backend handoff costs.

#### 27.3 Conjugate-Posterior Rewrite Catalog

The stdlib ships an enumerated catalog of conjugate-posterior
rewrites. Each rewrite fires from capability-contract
`Conj(X)` declarations on Tier 1 families (§27.1 table).

| Prior | Likelihood | Posterior |
|---|---|---|
| `Beta(α, β)` | `Bernoulli(p)` with n draws, k successes | `Beta(α + k, β + n − k)` |
| `Beta(α, β)` | `Binomial(n, p)` single draw k | `Beta(α + k, β + n − k)` |
| `Gamma(α, β)` | `Poisson(λ)` with n draws summing s | `Gamma(α + s, β + n)` |
| `Normal(μ₀, σ₀²)` | `Normal(μ, σ²)` known σ, n draws mean x̄ | `Normal((σ² μ₀ + n σ₀² x̄)/(σ² + n σ₀²), (σ₀² σ²)/(σ² + n σ₀²))` |
| `InverseGamma(α, β)` | `Normal(μ, σ²)` known μ, n draws, sum-sq s | `InverseGamma(α + n/2, β + s/2)` |
| `Dirichlet(α)` | `Multinomial(n, p)` counts c | `Dirichlet(α + c)` |

The catalog is closed for v2.1 — additional conjugate pairs
that modelers need are either derivable via `Truncated` /
`Mixture` composition or route to Tier 2 (chunk 08). The
rewrites fire automatically when the compiler detects a
matching `~` structure; no user directive is required.

#### 27.4 Extended Capability Table

For Tier A dispatch (§13.2), the compiler needs more than
the core capability tags (§27.1). The extended per-family
table records:

| Column | Meaning |
|---|---|
| `support` | the domain on which density is non-zero |
| `log_pdf` | closed-form log density availability |
| `moments` | closed-form mean, variance, higher moments |
| `reparam` | reparameterization trick form, if any |
| `sampling` | direct / inverse-CDF / rejection / backend-only |
| `entropy` | closed-form differential entropy |
| `kl_div` | closed-form KL divergence against same-family pairs |

The full extended table lives in the stdlib reference, not in
this spec. What's normative here is which columns exist and
that every Tier 1 family populates them. Missing entries are
interpreted as "not closed-form"; the compiler falls through
to Tier B or Tier C for the missing capability.

#### 27.5 Tier Ordering

Tiers are the PPL scoping axis distinct from the distribution-
family catalog:

- **Tier 1** — ships in v2.1. The 27 families in §27.1 plus
  the two meta-families in §27.2, with capability contracts
  and closed-form rewrites (§27.3) wired in. Includes three
  multivariate members (MVN, Dirichlet, Multinomial), with
  MVN using the Cholesky reparameterization locked in §13.6.
- **Tier 2** — partial. The multivariate subset that admits a
  factorized representation or a closed-form reparameterization
  ships in Tier 1 (MVN via Cholesky, Dirichlet/Multinomial via
  conjugacy). The genuinely joint subset — declarations that
  introduce coupling structure directly (B2 syntax), correlated-
  sample coupling machinery (B4), copulas, Wishart / InverseWishart
  / LKJ (gated on B5 heterogeneous-unit matrix resolution), and
  higher-order distributions routing through kernel machinery
  (§28) — remains **open** pending chunk 08 design. Framing is
  "in scope for v2.x, machinery not yet locked," not "deferred to
  a future version." Tracked in §35 Other Opens.
- **Tier 3** — open. Non-parametric and process-valued families
  (Gaussian Process, Dirichlet Process, Chinese Restaurant
  Process, Pitman-Yor, Indian Buffet Process, Beta Process). No
  formal tier boundary has been drawn. GPs are expected to route
  through §28 Kernels rather than through a distribution-family
  catalog entry, but whether non-parametric families share that
  routing, require a distinct mechanism, or are treated as Tier C
  (opaque PPL handoff) is an open question. Tracked in §35.
- **Tier A / B / C** — dispatch tiers (§13.2), orthogonal to
  Tier 1/2/3. A = exact closed-form, B = approximate rewrites
  (Delta, Fenton-Wilkinson, CLT, block-maxima → GEV), C =
  opaque PPL handoff.

"Tier 1 ships" is the positive commitment. "Tier 2 partial /
Tier 3 open" are explicit open design questions, not deferrals
to a future Myco version — they belong inside the v2.x design
envelope and block shipping only of the specific families that
need their machinery. Tier A/B/C are about dispatch, not about
what exists — a Tier 1 family can dispatch to any of A/B/C
depending on the transformation applied to it.

### 28. Kernels

Chunk 03 unified-machinery thread is pending e-graph substrate lock;
the surface shape below is committed, internal substrate not.

#### 28.1 Kernels as Functions with Capability Contracts

Kernels are ordinary functions from pairs of locus points to scalars:
`fn k(x: Point<L>, y: Point<L>) -> Scalar<U>`. No separate `kernel`
keyword, no separate type kind. Kernel-ness is a property of the
function that the compiler verifies from body composition plus
capability contracts on atoms, mirroring how function invertibility
and differentiability are derived (§7.2, §6, Anti-Spec "user-declared
fn invertibility / differentiability / domain"). The relevant
capability contracts:

- `PositiveDefinite` — guarantees the Gram matrix
  `K_{ij} = k(x_i, x_j)` is PSD for any finite point set. Required
  for use as a Gaussian Process covariance kernel.
- `Stationary` — guarantees `k(x, y) = k̃(x − y)` for some `k̃`.
  Implies translation invariance on the ambient locus.
- `Isotropic` — guarantees `k(x, y) = k̂(‖x − y‖)` for some `k̂`.
  Supertrait `Stationary` plus rotation invariance.

Stdlib kernels — Matérn (ν = 1/2, 3/2, 5/2, ∞), squared-exponential
(RBF), rational-quadratic, Wendland compact-support — satisfy all
three. Non-stationary kernels (e.g. polynomial `k(x, y) = (x · y + c)^d`,
Brownian `k(x, y) = min(x, y)`) satisfy `PositiveDefinite` but not
`Stationary`. The usual operations on kernels preserve the contracts:
sum preserves `PositiveDefinite` and `Stationary`, product preserves
`PositiveDefinite`, scaling by a positive scalar preserves both, and
radial wrapping (`k̂(‖·‖)`) elevates `Stationary` to `Isotropic`.
These closure rules are how the compiler derives kernel contracts
from composition without user property-declaration surface.

#### 28.2 Ambient-Locus via Composition

Kernels take `Point<L>` arguments, where the locus `L` is ambient and
fixed by where the kernel is called, not by a per-kernel declaration.
This avoids kernel families that only work on one space — e.g.
squared-exponential is usable on any `L` that admits a norm, and the
compiler picks up the norm from the locus definition (§11.1). A
kernel that requires a specific structure (e.g. spherical kernels
requiring `L = Sphere`) expresses that via a contract on the locus,
not via a specialized kernel type.

Composite kernels compose ambient-locus the same way any other
function composes. `k_sum = k_a + k_b` is well-formed iff `k_a` and
`k_b` share an ambient locus; the compiler checks this. Product
kernels on product loci (`L = L_x × L_y`) are written
`k((x1, y1), (x2, y2)) = k_x(x1, x2) * k_y(y1, y2)` and the
PositiveDefiniteness closure rule covers them.

#### 28.3 Kernel Sparsity and Integration — Deferred to Chunk 03

Two kernel-adjacent concerns are deferred:

- **Sparse / compact-support kernel representation.** Wendland and
  compactly-supported Matérn variants produce sparse Gram matrices,
  and the backend representation for sparse kernel matrices (`CSR`,
  `block-sparse`, `k-nearest`) is a matrix-layer concern that belongs
  in chunk 05 (B5). The kernel surface itself is locus-agnostic about
  this; sparsity falls out of matrix assembly, not kernel definition.
- **Kernel integration operators.** Convolution, integration against
  a measure, and the various ways kernels interact with stochastic
  integrals (for e.g. GP posterior predictives, kernel density
  estimates) are chunk 03 concerns. The stdlib ships the kernel
  functions themselves; operators that combine kernels with
  integration machinery wait for the e-graph substrate lock to avoid
  committing to a representation that the e-graph cannot efficiently
  normalize.

Until those unlocks, kernels in v2.1 support direct evaluation,
function composition, and use as GP covariances via opaque PPL
handoff (§13.2, Tier C). Non-opaque GP handling routes through the
kernel stdlib when chunk 03 lands.

### 29. Units Library

SI base units (m, kg, s, A, K, mol, cd). Common SI-derived units
(N, Pa, J, W, C, V, Ω, Hz, etc.) via derived-unit algebra (§5).
Standard affine conversions between equivalent SI-derived spellings.
Dimensionless-ratio handling.

Domain-specific unit libraries (ecophysiology, chemistry, astronomy,
finance, etc.) are **out of scope** for Myco core: they ship as
distributable Myco packages that import the core units library and
add domain-specific units, refinements, and conversion declarations
on top. This keeps the core stdlib narrow and keeps domain expertise
under the domain's own project maintenance.

### 30. Matrix and Tensor Primitives (STUB)

Chunk 05 (B5 heterogeneous-unit resolution) is the design venue for
the underlying type layer; this section commits only the stdlib
function surface. Type content (structural subtypes, shape refinements,
envelope interaction) lives in §3.9 per the chunk 05 scope decision.

The matrix / tensor stdlib ships the linear-algebra primitives that
the rest of the spec depends on by name — in particular, the Cholesky
factorization used in MVN reparameterization (§13.6, Z10) and the
kernel Gram-matrix machinery (§28). Committed primitives:

- `cholesky(A)` — lower-triangular factor `L` such that `L · Lᵀ = A`
  for `A: Matrix<_, PositiveDefinite>`. Returns `Matrix<_, LowerTriangular>`.
- `lu(A)` — `(L, U, P)` with `P · A = L · U`, for square invertible `A`.
- `qr(A)` — `(Q, R)` with `A = Q · R`, `Q` orthogonal, `R` upper
  triangular. Works on rectangular `A` (`m × n`, `m ≥ n`).
- `svd(A)` — `(U, Σ, Vᵀ)` with `A = U · Σ · Vᵀ`, `Σ` diagonal with
  nonnegative entries. Works on general rectangular `A`.
- `eigen(A)` — eigenvalue / eigenvector pair for square `A`. Real-
  symmetric specialization returns real eigenvalues and orthonormal
  eigenvectors; general case defers to complex eigenvalues pending
  §26.1 `Complex` lock.
- `solve(A, b)` — linear solve for `A · x = b`. Dispatches on the
  structural subtype of `A` (triangular solve, Cholesky back-
  substitution, general LU) via the §3.9 lattice.
- `inverse(A)` — direct inversion for documentation and small cases;
  the compiler rewrites `inverse(A) · b` to `solve(A, b)` by default
  to avoid explicit inversion in numeric code.
- `det(A)` — determinant. On `Matrix<_, Triangular>` this reduces to
  diagonal product; on general `A` it routes through LU.

Each primitive carries a capability contract that records what its
output satisfies structurally (see §3.9). The primitives are opaque
at the e-graph layer — their invariants are declared by contract,
not derived from body composition — because they wrap backend
linear-algebra kernels (BLAS / LAPACK / cuBLAS equivalents via the
Part V backend trait).

---

## Part V — Backend Abstraction (STUB)

Pending chunk 06 design completion. Specific trait shape and open forks
tracked separately; this part is normative in scope only.

### 31. Backend Trait Surface

The backend is an abstraction: Myco compiles plans against a trait
surface, not a specific runtime. Multiple backends can satisfy the
trait (burn-style tensor stacks, JAX-alikes, CPU reference
implementations). The compiler emits against the trait; the workflow
selects a concrete backend at run time (§24 verbs).

The minimum trait API covers four responsibilities — numerical
execution, automatic differentiation, PPL handoff, and opaque-
callable runtime — plus a capability-advertising mechanism that lets
the compiler and workflow negotiate what a particular backend
supports. The subsections below commit the shape; concrete signatures
land in chunk 06.

#### 31.1 Capability Advertising and Fallback Modes

Backends advertise capabilities (e.g. `supports_complex`,
`supports_forward_ad`, `supports_hamiltonian_monte_carlo`,
`supports_sparse_matmul`) and the compiler verifies required
capabilities at plan-binding time. When a required capability is
missing, the compiler consults the workflow's fallback policy:

- **`error`** — fail at plan-binding time with a capability-mismatch
  diagnostic (workflow-composition error tier, §19.4). Conservative
  default.
- **`host`** — route the offending subgraph to a host-side reference
  implementation. Correctness preserved at the cost of device-host
  traffic. Useful for CPU-only families (e.g. `Rational` arithmetic,
  §26).
- **`emulate`** — substitute an approximate or slower algorithm that
  the backend does support (e.g. dense solve in place of a missing
  sparse solve, finite-difference AD in place of missing forward AD).
  The substitution enters the approximation-error layer (§16 adjacent
  keyed state) so its effect on guarantees is tracked.

Fallback mode is set per-run via `run.config.backend` (§24.5);
workflows can also scope fallback to specific capabilities.

#### 31.2 PPL Handoff Protocol

Tier C stochastic SCCs (§13.2) ship to the backend's PPL handler
as opaque log-density problems. The handoff is a protocol, not a
library call: the backend receives a sampling / inference task
described by a standard serialized form (log-density callable,
parameter shape and bounds, observation data, inference kind — MCMC,
VI, importance, etc.), runs inference with backend-native machinery,
and returns samples plus diagnostics. Samples come back without
envelope facts about the parametric form (§13 recommits this);
downstream code treats them as opaque draws.

#### 31.3 Opaque-Callable Runtime

`bind_controller` (§24.1) hands the compiler a Python callable (a
learned function, typically a neural network). The backend provides
the runtime that calls back into Python-land during simulation,
threads gradients back through Python for training emission (§25),
and manages any memory / device-residency needed for the interop.
The opaque-callable runtime sits at the backend ↔ workflow boundary;
the compiler does not see the callable's interior, only its advertised
input / output contract.

#### 31.4 Backend Versioning

Backend implementations are versioned on their own cadence. The trait
surface is versioned by Myco. A given plan binds against a specific
trait-surface version; compatible backend versions advertise which
trait versions they implement. Breaking changes to the trait surface
are rare and require a major-version bump; within a trait version,
backend implementations can evolve freely. The plan cache (§20)
keys on `(plan, trait_version, backend_identity)` so upgrading
backends invalidates the cache correctly.

#### 31.5 Stochastic E-Class Serialization

Stochastic e-classes (§13 distributional metadata in the envelope)
need to cross the trait boundary when Tier C SCCs hand off to the
backend's PPL. The serialization: e-class identity, parametric form
recorded in envelope metadata (family, parameters, shape), current
layer-1 equational-core term, capability requirements, observation
constraints (§13.9). This is the wire format the PPL handoff protocol
(§31.2) uses. The compiler owns the serialization; backends own the
deserialization and any backend-specific canonicalization after
receipt.

#### 31.6 No Primary-Backend Commitment

Myco does not privilege any single backend. The trait-surface design
treats backends symmetrically: a burn-style Rust tensor stack, a
JAX-alike, a PyTorch-alike, and CPU reference implementations are
all first-class. The compiler emits trait-targeting code; capability
advertising (§31.1) lets each backend declare what it supports
honestly, and the workflow-side `run.config.backend` selects which
one a given run uses. Earlier design drafts privileged a specific
Python ecosystem backend; v2.1 retires that framing in favor of the
trait-based approach.

### 32. Open Backend Items

AD ownership fork (Myco-owned / backend-delegate / hybrid — leans
hybrid). PPL protocol specifics (message schema, inference-kind
enumeration). Gradient-flow semantics for `bind_controller`
callables.

#### 32.1 Mixed-Backend Policy

Open question: should a single Myco run be permitted to span multiple
backends, or must each run commit to exactly one? Arguments for
single-backend-per-run: simpler capability negotiation, no cross-
backend data marshalling, deterministic reproducibility across runs.
Arguments for mixed: allow a specialized backend for one SCC (e.g. a
PPL-specialized backend for a Tier C stochastic SCC) while a
general-purpose backend handles the rest.

Current lean: single-backend-per-run. If a workflow needs specialized
handling for one SCC, the intended escape hatch is to run the
specialized SCC in isolation and pass its samples / outputs into the
main run via workflow-layer glue, rather than to implement
cross-backend marshalling in the compiler. Not yet locked; chunk 06.

#### 32.2 First Concrete Backend

Which backend is implemented first — a burn-style Rust tensor stack,
a NumPy reference implementation, or a JAX-alike. Open. Affects
ergonomics of the first end-to-end demos but does not change the
trait-surface design, since the trait is backend-agnostic by
construction.

---

## Part VI — Known Open Items

Carried forward explicitly so they are not silently committed during
consolidation.

### 33. Design Blockers

- **B1.** Opaque log_pdf stdlib policy.
- **B2.** Joint declaration syntax.
- **B4.** Coupling machinery.
- **B5.** Matrix heterogeneous-unit resolution.
- **B6.** Backend abstraction (see Part V).

### 34. Chunk-Slotted Work

- **Chunk 05** — matrix details (heterogeneous units, envelope flavors,
  subtype lattice, shape refinements, scalar reconciliation).
- **Chunk 06** — backend abstraction.
- **Chunk 07** — type-graph ↔ e-graph bridge.
- **Chunk 08** — B2 + B4 joint syntax / coupling.
- **Chunk 03** — kernels, resume after substrate lock.

### 35. Other Opens

`replaces` obligation retraction (monotonicity tension with the
e-graph; cross-refs §8.10 declaration, §10.5 semantics, §15
equational-core monotonicity, §16 adjacent-keyed-state monotonicity).
Tier 0 Phase 2 Q3 (residual ↔ e-graph relationship) and Q4 (envelope
ownership). Literal-constants diagnostic surface (CC1 enforcement
messages; shape in §4.1). GPU-incompatibility of BigFloat and
Rational (cross-refs §26.1 numeric table, §26.3 Rational termination
caveat, §31.1 backend fallback modes). Conversion-graph cost
model. **Chunk 04 carryovers:** O4.1 `replaces` obligation
retraction (rewrite group W1 in Appendix C; three candidate
semantics still open). O4.3 per-residual training emission (CC3
cross-cut: overconstrained relations must survive extraction with
original names so training can expose per-residual loss terms;
tension with strict algebraic collapse; §20 rewrite group O1).
O4.6 heterogeneous `argmax` tagged handles (closure-policy
extensibility for collections with tagged alternatives). O4.7
event-driven topology mutation (incremental saturation strategy
when events add nodes, edges, or locus structure mid-run). O4.8
spatial operator lowering (rewrite group P1 architectural call:
e-graph rewrite versus pre-e-graph codegen; geometry chunk 01
cross-ref). Backend AD ownership (Part V §32, listed separately for
visibility). Macros (dropped from v2.1 surface; revisit if concrete
boilerplate pain emerges). `softmax` and weighted-sum aggregation
surface (stdlib primitive vs user-composed from `exp` + `sum`; collection-
aggregation syntax pending zip/alignment semantics lock; Y2 `soft_select`
already uses softmax internally in §8.7, so the shape is known but
the ergonomic surface is not). **Complex numeric representation in scope
for v2.1** — Riley-confirmed that `Complex` ships; open items are which
contracts it satisfies (not totally ordered, so `Compare`-dependent
stdlib functions exclude it; which of `Plus` / `Minus` / `Times` /
`Divide` close; interaction with units in `Scalar<U, Complex>`), backend
support commitments, and whether `Complex` forms a separate `Numeric`
sub-hierarchy or lives alongside `Float`. **Controller-interface
affordances in the Python layer** — general-system question: what hooks
does Myco need to expose so workflows can cleanly implement patterns
like taxonomic embeddings, context injection, per-category modulation,
FiLM-style conditioning? Not FiLM specifically; the meta-question of
which controller-binding surfaces belong in the language / stdlib vs
which are workflow idioms the user builds on their own. The goal is to
avoid baking Riley-specific project patterns into the language while
still exposing enough machinery that workflow authors can implement
them cleanly against `bind_controller` (§24.1). **Tier 2 distribution
machinery** — joint-declaration syntax (B2), coupling / correlated-
sample machinery (B4), copulas, Wishart / InverseWishart / LKJ (gated
on B5), higher-order distribution routing through kernels. In scope
for v2.x but not yet locked; chunk 08 is the intended design venue.
The multivariate subset that admits factorization or closed-form
reparameterization (MVN, Dirichlet, Multinomial) already ships in
Tier 1 so this open does not block the common cases. **Tier 3
distribution machinery** — non-parametric and process-valued families
(Gaussian Process, Dirichlet Process, Chinese Restaurant Process,
Pitman-Yor, Indian Buffet Process, Beta Process). Open question
whether these share §28 kernel routing (likely for GPs), require a
distinct process-family mechanism, or are treated as Tier C opaque
PPL handoff. No formal tier boundary drawn; design not yet scoped to
a chunk.

---

## Part VII — Developer Experience (Deferred)

Outside the language and compiler proper, but on the roadmap. Deferred
until Parts I–IV are locked. Listed here so the surfaces aren't
forgotten during consolidation.

### 36. Command-Line Interface

The `myco` CLI: compile, run, check, fmt, explain, and related
subcommands. Flags, exit codes, output conventions.

### 37. Dependency Management and Package Registry

How `.myco` packages declare dependencies on each other. Version
resolution. Package registry layout and publishing workflow. Lockfile
format. Interaction with the Python workflow layer's package system
(distinct but adjacent).

### 38. Editor Tooling

Language server (LSP). VS Code extension. Tree-sitter grammar. Syntax
highlighting, diagnostics, hover, goto-definition, refactoring
affordances.

### 39. Documentation Generation and Website

Docstring conventions. Doc generator for user-defined types, contracts,
events, universals. Website layout: language reference, tutorials, API
docs, examples.

### 40. Agent / LLM Integration

Agent skills for writing, reviewing, and validating `.myco` models.
Harness support for running Myco-aware agents. Conventions so LLMs can
reason about the language correctly (canonical examples, known
anti-patterns, diagnostic interpretation).

---

## Appendices

### Appendix A — Reserved Keywords and Syntactic Surface

The `.myco` surface reserves the following keywords. Reserved keywords
cannot be used as user identifiers and will emit a `mycoc` parse error
if encountered in identifier position.

**Declaration keywords.** `type`, `node`, `universal`, `fn`,
`contract`, `relation`, `constraint`, `event`, `geometry`, `locus`,
`chart`, `topology`, `metric`, `domain`, `convert`, `use`.

**Type-former keywords.** `Scalar`, `Tensor`, `Vector`, `Matrix`,
`Collection`, `impl`, `some`, `where`.

**Body-form keywords.** `let`, `if`, `else`, `for`, `in`, `trace`,
`identify`, `requires`, `replaces`, `conserved`.

**Stochastic operator.** `~` (distribution-binding operator;
stochastic relation). Unit generics use `<Ito>`, `<Stratonovich>`
as contract-parameter keywords on `~`.

**Reserved but not yet assigned semantics.** `self` (reserved for
refinement-predicate body use and future module-instance use).
`match` (reserved for future pattern-matching surface).

**Structural punctuation.** `::` (path separator), `->` / `<->`
(convert-direction arrows), `<=`, `>=`, `<`, `>`, `==`, `!=`,
`=` (relation-equality and binding use site-determined by
context), `|` (currently unassigned, reserved for future
pattern or pipe use).

**Stdlib-reserved identifiers.** The stdlib atom namespace reserves
`exp`, `log`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `sqrt`,
`abs`, `sign`, `floor`, `ceil`, `round`, `min`, `max`, `sum`,
`prod`, `mean`, `std`, `var`, `solve`, `invert`, `deriv`,
`integrate`, `condition_of`, `value_in`, plus the distribution-
family names enumerated in §27. User functions shadow stdlib atoms
at the user's own module scope; stdlib dispatch preempts at the
global scope.

The full list is normative as of v2.1 lock. Additions are a
breaking change to the parse surface and follow the source-
language stability process (to be designed post-build).

### Appendix B — Grammar / EBNF Summary

Open. A normative EBNF summary of the `.myco` surface will appear
here once the surface is stable enough to commit to a grammar.
The concrete form is a production per construct in §2-§14 (types,
values, units, functions, contracts, relations, constraints, events,
geometry, stdlib calls, workflow-boundary syntax). Placeholder for
a later pass.

### Appendix C — Rewrite Catalog (A–Y)

Enumerates the rewrite rules the compiler applies over the e-graph
substrate (§16, §17). Organized into 25 lettered groups. Each group
carries a faithfulness tag (strict / fuzzy-model / fuzzy-tolerance /
one-way / N-way extraction / forbidden) and an orientation tag (bidi /
uni). Rules marked **LOCKED** ship in v2.1. Rules marked **OPEN**
are intended for v2.1 but pending resolution of a named design item.
Cross-cutting flags (CC1-5) appear in-line; see §0.1 for their normative
disposition.

Merge-source correspondence: the eight merge sources of §17 are
canonical shapes; the A–Y catalog enumerates the concrete rule surface.
Every rule below routes through one of the eight sources.

---

**Strict / lossless rewrites (bidirectional unless marked).**

**A. Ring/field axioms.** Load-bearing for canonical-evaluator
selection, residual simplification, and SCC invariance. LOCKED.

- A1. `x + 0 → x`, `0 + x → x`
- A2. `x * 1 → x`, `1 * x → x`
- A3. `x * 0 → 0`, `0 * x → 0` (gated: 0 dimensionless unless dimension
  matches per §5)
- A4. `x - x → 0`, `x / x → 1` (latter gated on `x ≠ 0` via envelope
  bounds)
- A5. Associativity of `+` and `*`
- A6. Commutativity of `+` and `*`
- A7. Distributivity `x*(y+z) ↔ x*y + x*z` (extraction picks expanded
  vs factored form by cost)
- A8. `-(-x) → x`
- A9. `x^0 → 1` (gated `x ≠ 0`), `x^1 → x`
- A10. `x^a * x^b → x^(a+b)`, `(x^a)^b → x^(a*b)` (needed for Arrhenius
  canonicalization)

**B. Constant folding.** LOCKED.

- B1. Literal arithmetic and transcendentals at identity points
  (`2+3→5`, `exp(0)→1`, `log(1)→0`, `sin(0)→0`, `cos(0)→1`, `sqrt(1)→1`)
- B2. Universal bound to literal after workflow binding (e.g. `R →
  8.314` once `assume_constant` fires). Per the CC1 literal-numerics
  lock (§4) the value enters from the workflow; no literal appears in
  `.myco` value position.

**C. Unit / dimensional normalization.** Base-SI internal representation;
dimension-exponent arithmetic. LOCKED.

- C1. Literal-with-unit to base SI: `0 degC ↔ 273.15 K`, `0.75 MPa ↔
  750000 Pa`
- C2. `to_base(to_base(x)) → to_base(x)` (idempotence)
- C3. Dimension exponent vector arithmetic on `*`, `/`, `sqrt`, integer
  powers
- C4. Dimensionless 0 / 1 collapse across unit signatures

**D. Named-type normalization.** Convert equalities at the magnitude
level, name-preserving arithmetic. LOCKED.

- D1. Bare `convert FishMass <-> DetritusMass` treats them as same-
  magnitude in the e-graph when both share a conserved parent (§3.7)
- D2. Bidirectional `convert` installs both forward and backward
  rewrites
- D3. Inverse convert round-trip: `convert(convert(x)) → x` when verified
  per O2.1 round-trip checker (§5.2)
- D4. Same-named-type addition preserves the name: `CarbonPool +
  CarbonPool → CarbonPool` (uni; name cannot be re-inferred if stripped)
- D5. Named + anonymous-matching-dimension addition preserves the name
  (uni; name-join semilattice with anonymous as bottom)

**E. Function-inverse round-trip elimination.** Requires declared or
registered inverse. LOCKED.

- E1. For declared-bijective `f` with explicit inverse: `f⁻¹(f(x)) → x`,
  `f(f⁻¹(y)) → y` (gated on envelope bounds proving input in `f`'s
  declared domain)
- E2. Built-in inverse pairs: `exp(log(x)) → x` (gated `x > 0`),
  `log(exp(x)) → x` (always)

**F. Geometry-specific strict merge.** Scalar-field seam identification.
LOCKED; vector/tensor seams OPEN (§35 geometry chunk 01).

- F1. `identify phi=0 <-> phi=2*pi` merges scalar-field e-classes at
  the seam

**G. Transcendental simplifications (gated).** LOCKED.

- G1. `exp(a)*exp(b) → exp(a+b)`, `log(a*b) → log(a)+log(b)` (gated
  `a,b > 0`), `exp(a)^b → exp(a*b)` (Arrhenius canonicalization)
- G2. Trig fundamentals: `sin(0) → 0`, `cos(0) → 1`, `tan(0) → 0`;
  Pythagorean `sin(x)² + cos(x)² → 1`
- G3. Idempotency of lossy ops: `abs(abs(x)) → abs(x)`, `abs(-x) →
  abs(x)`, `min(x,x) → x`, `max(x,x) → x`

**H. Aggregate / collection identities.** Linearity rules for
reductions. LOCKED.

- H1. `sum(0) → 0`, `product(1) → 1`, `sum(x+y) → sum(x)+sum(y)`,
  `sum(c*x) → c*sum(x)` when `c` is loop-invariant
- H2. Empty-collection: `sum(empty)→0`, `product(empty)→1`,
  `any(empty)→false`, `all(empty)→true`, `count(empty)→0`

**I. Conditional rewrites.** Constraint-analysis-gated. LOCKED.

- I1. `if true then a else b → a`; `if false then a else b → b`;
  `if c then a else a → a`

**J. Temporal invariant (forbidden merge, not a rewrite).** LOCKED.

- J1. `x[t]` and `x[t-1]` never merge across timesteps even if
  numerically equal at runtime. Distinct ground terms per referential-
  truth (§0 principle 5, §16.2 monotonicity).

---

**Fuzzy / tolerance-gated rewrites (uni unless marked).**

**K. Kernel truncation.** The headline fuzzy rewrite from §28 kernels.

- K1. `K(a,b) → 0` when `distance(a,b) > L_char` for compact-support
  kernels (Gaussian beyond ±3σ, Matérn, spline compact support).
  Turns O(N²) integrals into O(N·k). LOCKED.
- K2. Separable decomposition: `K((x₁,y₁),(x₂,y₂)) → K_x(x₁,x₂) *
  K_y(y₁,y₂)` when declared or inferred. OPEN (§35, kernels chunk 03;
  bidi when exact, uni when approximate).
- K3. Low-rank `K → U·Vᵀ` (truncated SVD, Nyström, random Fourier
  features). OPEN (chunk 03; speculative — kernels report does not
  enumerate, but §28 machinery must accommodate).

**L. Smoothing rewrites.** User-written smooth forms only; `where` is
never silently smoothed (§8.3 runtime `where` lock).

- L1. `smooth_min(a, b, large_sharpness) → min(a, b)` when sharpness
  exceeds tolerance. LOCKED. Reverse direction (`min → smooth_min`)
  forbidden per "no silent smoothing."
- L2. `where p then a else b → a*sigmoid(k*p) + b*(1-sigmoid(k*p))`
  only in user-written smooth form, never auto-fired. OPEN (depends on
  smoothing-surface finalization; §8.3, §8.9).

**M. Series / linearization.** First-order expansions and asymptotic
truncation. OPEN (§35 envelope machinery).

- M1. First-order Taylor `f(x) → f(x₀) + f'(x₀)*(x-x₀)` around declared
  operating point
- M2. Drop higher-order terms when envelope bounds their contribution
  below tolerance

**N. Numerical quadrature substitution.** Every PDE passes through
this. OPEN (§35, kernels chunk 03).

- N1. `integrate(f, var, lo, hi) → quadrature_n(...)` for user-tunable
  `n` when symbolic integration fails

**O. Training-time consistency-loss substitution.** Mode-conditional.
OPEN (§35, chunk 04 O4.3 per-residual training emission).

- O1. In train mode, overconstrained `lhs = rhs` becomes `loss += w *
  (lhs - rhs)²`

**P. Mesh discretization (continuous → discrete).** Tolerance-gated by
mesh resolution `h`. OPEN (geometry chunk 01 P1; architectural call
between e-graph rewrite and pre-e-graph codegen).

- P1. `grad(field) → fd_stencil`, `laplacian(field) → 5-point or 9-point
  stencil`, etc.

**Q. Probabilistic truncation / marginalization.** Interacts with `~`
(§13). OPEN (§35, stochastic rewriting semantics).

- Q1. Latent-discrete-with-finite-support → `logsumexp_i[...]` auto-
  marginalization
- Q2. Continuous distribution + refinement-type bound → truncated
  distribution with normalized log-pdf

---

**One-way / directional rewrites.**

**R. Lossy-function simplification.** Forward only under bound
tightening. LOCKED.

- R1. `abs(x) → x` when envelope proves `x ≥ 0`
- R2. `max(a,b) → a` when envelope proves `a ≥ b` (same pattern for
  `min`)
- R3. `floor`, `relu`, `clamp` — forward only under bound tightening;
  never invertible

**S. Opaque function applications.** No reverse rewrite. LOCKED.

- S1. `f(x)` where `f` is opaque — forward edge only; no recovery of
  `x` from `f(x)`
- S2. `bind_controller`-attached callable: `g(inputs) → output` forward
  only (black box, §24.1)

**T. One-way convert.** Explicit user-declared non-invertible transform.
LOCKED.

- T1. `convert Plaintext -> Ciphertext { ... }` installs forward rewrite
  only

**U. Named-type stripping under arithmetic.** Required for type
checking; directional because names cannot be re-inferred. LOCKED.

- U1. `LeafArea * CarbonFlux → anonymous Scalar<umol_s>`
  (multiplication strips the name)
- U2. `CarbonPool / CarbonPool → anonymous Scalar<ratio>` (same-type
  division strips the name)
- U3. `Temperature - Temperature → anonymous Scalar<K>` (affine
  subtraction breaks named-type symmetry)

**V. Observation injection.** Ground-truth data pinning (§13.9).
LOCKED.

- V1. `observe(path, data)` installs `path = data` as a forward
  observation factor; data does not get rewritten by inferred
  constraints

**W. Obligation retraction.** Deletion, not rewrite. OPEN (chunk 04
O4.1, cross-ref §8.10, §10.5, §15, §16, §35).

- W1. `relation X on locus replaces balance(axial_flux): ...` retracts
  the compiler-generated `balance(axial_flux)` at the named locus and
  substitutes the user equation

**X. Structural-predicate-gated strict.** Strict/lossless but gated on
a structural predicate, not value bounds. LOCKED (O4.2 resolved
2026-04-20).

- X1. Pole L'Hopital: at any mesh node coinciding with declared locus
  pole, rewrite `laplacian(f)` from naive `1/sin(θ)` form to the
  L'Hopital limit. Also characterizes `identify`-induced merges gated
  on coordinate predicates.

---

**N-way rewrites (closure policies — extraction-time, not rewrite-
time).**

**Y. Closure policies.** Extraction strategies for an e-class with
multiple equally-valid evaluators (§8.7). User picks via closure
config.

- Y1. `weighted_average(c1,...,cN) → mean` (arithmetic mean of candidate
  outputs). LOCKED.
- Y2. `soft_select(preference_list, sharpness) → Σ softmax(rank_i /
  sharpness) * candidate_i`. LOCKED.
- Y3. `hard_select(preference_list)` picks highest-ranked by name;
  non-differentiable (rejected in train mode unless discarded paths
  have no learned parameters upstream). LOCKED.
- Y4. `condition_weighted`: uses `condition_of(·)` intrinsic to weight
  candidates by well-conditionedness. LOCKED (un-deferred 2026-04-20,
  closes O4.5).
- Y5. User-defined custom policy: any `.myco` function taking
  candidates plus hyperparameters, returning a forward value.
  Extensibility surface. LOCKED.
- Y6. General `C(N,M)` enumeration for overconstrained blocks
  (`N > M+1`): planner enumerates all maximal square subsystems; policy
  receives the set. OPEN (combinatorial-blowup warning threshold
  pending; §35).

---

**Summary table by faithfulness × orientation.**

| faithfulness | bidi | uni | total |
|---|---|---|---|
| Strict | ~24 (A1-10, B1-2, C1-4, D1-3, E1-2, F1, G1-3, H1-2, I1) | ~4 (D4-5, X1, J1 forbidden) | ~28 |
| Fuzzy-model | — | ~2 (L1-2) | 2 |
| Fuzzy-tolerance | ~7 (K1-3, M1-2, N1, Q1-2) | ~3 (O1, P1, M2) | ~10 |
| One-way (lossless uni) | — | ~11 (R1-3, S1-2, T1, U1-3, V1, W1) | ~11 |
| N-way extraction | — | ~6 (Y1-6) | 6 |
| Forbidden | 1 (J1 temporal) | — | 1 |

Grand total approximately 58 rules, depending on sub-rule counting.

**Cross-cutting items (flags, not rewrites).** CC1-5 are absorbed
into normative spec text: CC1 literal-numerics (§4, §4.1), CC2 sanity
inverses (§5.2 round-trip), CC3 per-residual training emission (§20;
open as O4.3), CC4 stochastic `~` rewrite blank (§13.8 resolved
2026-04-20), CC5 pole L'Hopital category (§16.4 resolved 2026-04-20 as
X-group).

---
