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

**Summary.** Myco is a language for scientific modeling with a GPU-first
execution target. A modeler writes a `.myco` file (types, relations,
state, topology) and a Python workflow (values, data, priors,
observations, training directives). The compiler bridges them under
five principles: world-vs-experiment split, clean boundary, compiler
does work, structure regularizes, referential truth.

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

Scope. Myco is a general language for scientific modeling. This spec
covers the language, compiler substrate, workflow boundary, standard
library, and backend trait. Domain libraries (ecosystem simulators,
circuit models, population dynamics, and so on) are external to the
core and ship as ordinary Myco modules against the surface defined
here.

Output. The `.myco` file plus the workflow Python fully reproduces a
run. Compiled code belongs to you; inspect it if you want (§22).

### 0.1 Foundational Concepts

**Summary.** Cross-cutting claims named once so later sections can
invoke them without restating: conservation laws, referential truth,
downward-only visibility, traceability, error-reporting tiers,
capability-error surfacing, three-layer e-graph scoping, determinism,
world-vs-experiment axis, conversion-graph cost model, projection-
free compiler, and generated-code-is-the-product.

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

**Summary.** The vocabulary used throughout the spec. Each term is
defined once here and referenced by name elsewhere.

The vocabulary used throughout this document. Each term one line.
Terms: `variable`, `bound variable`, `free variable`, `relation`,
`binding verb`, `event`, `controller` (workflow-only), `initial`,
`temporal`, `node`, `locus`, `geometry`, `domain`, `workflow`,
`e-class`, `envelope`, `universal`, `SCC`, `residual graph`,
`~` (distribution operator), `impl`, `some`, `` `approximate` `` block,
`observe`.

---

## Part I — The Language

**Summary.** The surface a modeler writes in `.myco`: modules and
imports, the type system (primitives, generics, refinements), node
declarations, functions, contracts, events, and the static checks the
compiler runs before any code is generated.

### 2. Modules, Imports, Scope

**Summary.** File-as-module convention with path-based imports
(`use path::to::symbol`) and name resolution for types, universals,
contracts, and events. Re-exports thread symbols through the importing
module's path. The import graph is a DAG; circular imports are a
compile error. Python imports and `.myco` imports are separate
systems; the workflow imports `.myco` models, not the reverse.

File-as-module convention. Path-based imports
(`use path::to::symbol`). Scope resolution rules for names, types,
universals, contracts, events.

Re-export. A module that imports a symbol may re-expose it under its
own path; downstream imports resolve against the importing module's
path, not the source. Re-exports make a module's external surface
independent of where each name was first declared.

Import graph. The `.myco` module import graph is a directed acyclic
graph. Circular imports are a compile error at module load; the
diagnostic names the cycle.

Relationship to the workflow side: Python imports and `.myco` imports
are distinct systems; the workflow imports `.myco` models, not the
other way around.

Package-level dependencies (cross-spore imports, version resolution,
workspace layout) are a separate concern from file-as-module scoping
and are covered in `v2.1_chunk_reports/10_package_dependencies.md`.
Within a `.myco` file, `use hydraulics::...` resolves to whatever
spore the enclosing `myco.toml` declares as `hydraulics`; the `use`
form itself does not change between intra-spore and cross-spore
imports.

### 3. Types

**Summary.** The static type system. Primitives
(`Scalar<U, T = Float64>`, `Tensor<U, shape>`, with `Vector` and
`Matrix` as shape-refined aliases), named types, universals, val and
type generics with the named-argument rule, and the structural
refinement lattice on matrices (Symmetric, PosDef, Diagonal,
Triangular, Orthogonal).

#### 3.1 Universal Declarations

**Summary.** Module-scope typed names (`universal R: Scalar<J_mol_K>`)
that every consumer in a run shares. Value comes from the workflow via
`assume_constant` or `learn_constant`; CC1 forbids literals in `.myco`.

Module-scope typed names shared across all instances that reference
them. `universal R: Scalar<J_mol_K>` declares a name with a type; the
value is supplied by the workflow via `assume_constant` or
`learn_constant`. CC1: no literal value in `.myco`. Semantics:
universals are "same value for every consumer in this run": physical
constants, cross-entity shared coefficients. Distinct from ordinary
fields, which vary per instance.

#### 3.2 Refinement Types

**Summary.** Predicate-refined types such as
`type UnitInterval = Scalar<dimensionless> where { 0 <= self <= 1 }`.
Obligations discharge at compile time via e-graph reasoning where
possible, at runtime otherwise. The `~` operator auto-truncates a
distribution to a refined target type (§13).

Predicate-refined types: `type UnitInterval = Scalar<dimensionless>
where { 0 <= self <= 1 }`. Refinement obligations discharged by
e-graph reasoning where possible, runtime check otherwise. `~`
operator on distributions auto-truncates to a refined target type (§13).

#### 3.3 Newtype and Composite Types

**Summary.** Nominal wrappers (`type Depth: Scalar<m>`) for type
distinction without structural change, plus composite record types
with named fields. "Atomic" means the type is a leaf of the
containment tree rather than a single-field wrapper; a composite is
atomic at sites that do not decompose its fields. Named-type
comparison rules: `=` and ordering between two distinct named types
require an explicit `convert` to share scope; the compiler rejects
cross-type arithmetic and comparison without it.

Nominal wrappers (`type Depth: Scalar<m>`) distinguish a name from
its underlying representation without changing structure. Composite
record types declare named fields. A nominal wrapper is atomic
(a containment leaf); a composite is atomic at every site that does
not decompose its fields.

Named-type comparison. Relations that write `=` (or any ordering
operator) between two values of distinct named types require an
explicit `convert` declaration (§5.1) between those types; otherwise
the relation is a compile error. Bare `convert` sibling-relabels
under a conservation group (§3.7) satisfy this requirement without
a conversion body. Comparisons between a named type and its
underlying representation are never implicit; the modeler must name
which side of the wrapper the comparison lives on.

#### 3.4 Node Instantiation

**Summary.** `node name: Type` at module scope creates an entity with
durable identity. Identity survives timesteps and e-graph merges;
events operate on nodes. Distinct from type aliasing; the e-graph
instantiates one identity-tagged class per node.

`node name: Type` at module scope creates an entity with identity.
Identity survives timesteps and e-graph merges; events operate on
nodes. Distinct from type aliasing: `node tree: Tree` creates one
Tree entity, not a name for the Tree type. The e-graph instantiates
one identity-tagged class per node.

#### 3.5 Heterogeneous Collections — `impl` and `some`

**Summary.** Two orthogonal operators for collection heterogeneity.
`impl Contract` gives static type heterogeneity over a compile-time-
known element-type set; `some T` gives homogeneous elements with
runtime sizing. They compose: `Collection<some (impl Plant)>` is
statically heterogeneous and dynamically sized.

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

**Summary.** Variance rules for generic type parameters:
covariant / contravariant / invariant positions. Subtyping discipline
for named types, refinements, and conservation-group hierarchies.
Full treatment tracked in chunk 07 Q4.

Variance rules for generic type parameters (chunk 07 Q4):
covariant / contravariant / invariant positions. Subtyping discipline
for named types + refinements + conservation-group hierarchies.

Scalar-value generics. A generic parameter may itself be a typed
scalar (e.g., `LOW: Scalar<U>`) rather than a type. Scalar-value
generics participate in refinements and shape-tuple positions and
are bound at compile time through the same workflow verbs as
ordinary universals (§4 exception classes cover their declaration
sites).

#### 3.7 Conservation Groups

**Summary.** `type Mass : Scalar<kg> { conserved }` marks a parent
type whose named-type children share conservation semantics. Cross-
sibling arithmetic is forbidden without explicit convert; destructive
events must route conserved fields somewhere; the compiler auto-
generates junction balance relations from `diverg()` on conserved
flux fields.

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

**Summary.** How `Scalar<U>` relates to `Tensor<U, ()>`, how
collections relate to tensor axes, and which transformations live in
`convert` versus the backend trait. Collections and tensors are
orthogonal primitives. `convert` handles meaning-preserving tensor
transforms (reshape, sparse↔dense, structural widening); precision,
layout, and device residency belong to the backend.

Open: whether `Scalar<U>` is formally sugar for `Tensor<U, ()>`
(shape-zero tensor) or a distinct primitive with coercion rules
(chunk 05 Q6). The unification is attractive: it lets structural
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

**Summary.** Matrix structural properties (Symmetric, PosDef,
Diagonal, Triangular, Orthogonal, Sparse, Banded) are type-level
predicates forming a lattice under meet. They drive stdlib primitive
dispatch: `solve` picks triangular substitution, Cholesky, or general
LU from the structural subtype of its first argument.

Matrix structural properties are type-level predicates that form a
lattice under meet (structural intersection). They drive stdlib
primitive dispatch (§30), for example `solve` chooses triangular
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

#### 3.10 Sum Types / Enums (STUB)

**Summary.** Tagged sum types (enums) are a core composite-type form
alongside newtype and record. They capture **structural
polymorphism** — a field that is one of several shapes — where
contracts capture **behavioral polymorphism**. Variants may be unit,
positional, or struct-like; matches are exhaustive; the compiler
picks compile-time specialization (when the discriminant is static
after workflow binding) or a runtime discriminant-tagged kernel
(when dynamic). Enums compose with contracts; variant fields may
themselves be contract-typed. Stdlib ships at least `Prior<T>`
(number-or-distribution), `Maybe<T>`, and `Result<T, E>`. Exact
syntax, pattern-matching power, event-triggered variant transitions
(FSMs), lifted-arithmetic sugar through `Prior<T>`, and workflow
binding surface are open.

Four independent pressures motivate enums as a single mechanism:
number-or-distribution materialization of the same model, Mode B
heterogeneous contract dispatch across a population (chunk 08,
chunk 09), finite state machines in dynamic topology, and
Option/Result at the workflow boundary. Contracts alone cannot
cover these cases without hiding the `~` operator from the PPL
machinery (§13) or collapsing structural differences that the
compiler needs to see.

Full design lives in `v2.1_chunk_reports/11_sum_types_enums.md`.
This subsection is a stub; detailed prose lands when chunk 11 closes.

**Summary.** Zero literal numerics in value position (CC1). Three
exception positions where literals are allowed: unit definitions,
affine conversion bodies, and structural positions (shape tuples,
indices, generic parameters). All numeric values enter through the
workflow.

Zero literal numerics in value position. Three exception positions:
unit definitions, affine conversion bodies, structural positions
(shape tuples, indices, generic-parameter definitions). All numeric
values enter from the workflow. See `spec_dev_notes.md` for the
derivation.

Mathematical constants. π, e, and similar fixed reals are ordinary
stdlib-declared identifiers (`universal pi: Scalar<dimensionless>`,
`universal e: Scalar<dimensionless>`). They receive no CC1 carve-out:
they are universals like any other, and a workflow binds their
numeric values at compile time through the same mechanism as any
other constant. The stdlib ships default bindings so users do not
write them by hand.

Workflow bindings enter the e-graph as equalities. A workflow
constant supplied at compile time merges an observation-tagged
equality between the universal's e-class and a literal term in the
B2 rewrite layer (§17). Numeric values therefore participate in
rewriting and extraction without appearing in `.myco` source.

#### 4.1 CC1 Diagnostic Surface

**Summary.** CC1 violations surface as `mycoc` compile errors with a
consistent shape: identify the literal, name the rejected position
kind, and point to the canonical workflow verb (`assume_constant`,
`assume_series`) that would supply the value instead.

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

**Summary.** Base units, derived units, affine conversions,
dimensional algebra, and unit-generic types. The `convert` declaration
(four variants), round-trip verification, and `value_in` operator are
the modeler surface. Unit-normalization rewrites live in the e-graph
machinery (§17, Appendix C group C); §5 covers the declaration surface
and the modeler-facing invariants.

#### 5.0 Unit System Fundamentals

**Summary.** `base_unit` introduces an orthogonal dimension axis.
`Scalar<U>` is the unit-parameterized quantity primitive. Derived units
are products, quotients, and scalar multiples of existing units.
Internally, all computation uses base-SI magnitudes; declared units are
a presentation layer. No implicit unit inference: every `Scalar<U>`
must have its unit established syntactically or by workflow binding.

A `base_unit` declaration introduces a new orthogonal axis in the
dimension exponent vector. Example:

```myco
base_unit meter
base_unit second
base_unit kilogram
```

`Scalar<U>` is the built-in parameterized type for "a real number
measured in unit U". Derived units are defined as products, quotients,
and scalar multiples of existing units:

```myco
unit meter_per_second = meter / second
unit pascal = kilogram / (meter * second^2)
```

**Base-unit storage invariant.** Internally, all computation happens in
base units. Declared units are a user-facing layer; the compiler never
stores a pascal-magnitude and a kilogram-per-meter-second^2-magnitude
as distinct objects.

**No implicit unit inference.** The compiler does not infer units from
context. Every `Scalar<U>` must have its unit established either
syntactically or by workflow binding; a bare numeric literal in a
unit-typed position is a compile error.

**Expression-level unit annotation.** A parenthesized expression
followed by a unit name attaches the unit to the result:

```myco
(0.1579 + 0.0017 * T_c) mol_m2_s
```

This is syntactic sugar for multiplying by the unit's base-SI scale
factor. The result has dimension `mol / (m^2 * s)`.

Unit-normalization rewrites (literal-with-unit to base SI, dimension
exponent arithmetic, dimensionless collapse) are e-graph rules, not §5
machinery (§17, Appendix C group C).

#### 5.1 Convert Declarations — Four Variants

**Summary.** Four forms of unit/named-type conversion: `<->` or `->`
crossed with bare or parameterized-body. Bare forms declare same-
magnitude aliases or one-way relabels; parameterized forms carry
bodies the compiler verifies for inverse consistency.

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

**Summary.** Parameterized `<->` converts obligate the compiler to
verify inverse consistency via bounded counterexample search within
the participating types' refinement bounds. Counterexample is a
compile error; exhausted bound accepts.

Parameterized `<->` converts obligate the compiler to verify inverse
consistency. Verification runs bounded counterexample search within
the participating types' refinement bounds. Counterexample found is a
compile error with the offending value; exhausted bound accepts.

#### 5.3 The `value_in` Operator

**Summary.** `value_in(unit)` extracts the raw numeric magnitude of a
quantity in a named unit (`temperature.value_in(celsius)`). Returns
`Scalar<dimensionless>`. Use positions: interop with unit-naive stdlib
atoms and external-library arguments. Unit must be dimensionally
compatible with the receiver. Because internal storage is always in
base units (§5.0), `value_in` is the only path to a named-unit
magnitude; no other operator exposes this conversion.

`value_in(unit)` extracts the raw numeric magnitude of a quantity in
a named unit. Example: `temperature.value_in(celsius)` pulls the
celsius magnitude from a `Scalar<kelvin>`. The return type is
`Scalar<dimensionless>`. Use positions: interop with unit-naive stdlib
atoms, external-library arguments. The argument unit must be
dimensionally compatible with the receiver.

#### 5.4 Affine Unit Semantics

**Summary.** Affine units (Celsius, gauge pressure) have an offset
relative to their absolute counterpart. Multiplication and division of
affine quantities require conversion to the absolute unit first.
Subtraction of two affine quantities yields a base-unit difference.
No silent coercion occurs; the compiler rejects disallowed forms.

Temperature in Celsius is an affine unit: its zero point differs from
Kelvin's. Arithmetic on affine quantities follows these rules:

- `20°C * 2` is not `40°C`. Multiplication by a dimensionless scalar
  requires converting to Kelvin first: `(20°C.to_base() * 2).value_in(celsius)`.
- `20°C - 5°C` yields `15 K`, not `15°C`. Subtracting two affine
  quantities of the same affine unit produces a base-unit difference
  (the offsets cancel).
- Adding an affine quantity to a base-unit difference is defined:
  `20°C + 5 K` is `25°C`.
- Adding two affine quantities directly is a compile error.

The compiler enforces these rules statically. No silent coercion
converts between affine and absolute forms.

#### 5.5 Workflow-Boundary Unit Parameter

**Summary.** External data enters with a declared unit via
`assume_series(..., unit='K')`. The workflow layer converts from the
declared unit to base units at the binding boundary. See §24 for the
full workflow-verb inventory.

External data sources are unit-naive (raw floats, CSV columns). The
`assume_series` verb accepts a `unit` keyword argument naming the
unit in which the data is expressed:

```python
experiment.assume_series('atm.temperature', data_in_kelvin, unit='K')
experiment.assume_series('atm.pressure', data_in_mpa, unit='MPa')
```

When the dimension of the declared unit matches the declared type of
the bound field, the workflow layer converts to base units at the
binding boundary. A dimension mismatch is an error at composition
time. See §24 for the full workflow-verb inventory and gradient-flow
implications of `assume_series`.

### 6. Functions

*Open (pending application).* The design lock in chunk 08 bans user
`fn` declarations in favor of parameterized relations; contract
methods become required parameterized relations; kernels become
parameterized relations. The prose in this section still describes
the prior `fn`-as-first-class surface and is stale relative to the
locked design. Canonical reference:
`planning/v2/v2.1_chunk_reports/08_relation_fix_whoops.md`. Tracked
in §34.

**Summary.** `fn` declarations with parametric generics. Contracts
apply to functions via the same composable machinery used for types
and distributions. Stdlib atoms declare capability contracts like
`Invertible<_>`, `Differentiable`, `Monotone` that drive e-graph
rewrites; user functions have no property-declaration surface. The
compiler derives function properties from body composition plus
stdlib atom declarations. Functions are also the extensibility
surface for closure policies (§8.7).

`fn` declarations with parametric generics. Body composition. Contracts
apply to functions using the same composable machinery used for types
and distribution families (see §7). Stdlib atoms (`exp`, `log`, `sin`,
`sqrt`, …) declare capability contracts like `Invertible<_>`,
`Differentiable`, `Monotone`; these drive e-graph rewrites (see §17
merge sources). User functions carry no property-declaration surface;
the compiler derives properties from body composition plus stdlib
atom declarations. No annotation blocks, no `#[...]` attributes.

Kernels are ordinary `.myco` functions that accept two point
arguments and return a scalar; there is no separate `kernel` keyword
or kernel kind.

#### 6.1 Generic Functions

**Summary.** Functions may be generic over contracts, including unit
contracts. A generic function monomorphizes per instantiation at the
boundary where the generic is concretized.

A unit-polymorphic function uses a contract bound on the type
parameter:

```myco
fn arrhenius<U: Unit>(rate_25: Scalar<U>, activation_energy: Scalar<joule_per_mol>, T: Scalar<kelvin>) -> Scalar<U> {
    rate_25 * exp(-activation_energy / (R * T))
}
```

The compiler monomorphizes `arrhenius` once per distinct unit
instantiation at each call site where the generic `U` is concretized
to a specific unit. The body is type-checked against the declared
contract bound; calls that cannot satisfy `U: Unit` are a compile
error.

#### 6.2 Compiler Roles for `fn` Bodies

**Summary.** The compiler treats a `fn` body as source material for
several analyses. User functions require no annotation to participate.

What the compiler does with a `fn` body:

- **Dimensional analysis.** Unit-checks every subexpression in the
  body. A dimension mismatch in the body is a compile error.
- **Symbolic differentiation.** Bodies participate in `deriv`
  lowering: the compiler symbolically differentiates the body
  expression using stdlib-atom capability contracts (`Differentiable`,
  `Invertible<_>`) to produce the A-group rewrites (§17, Appendix C
  group A).
- **Solver emission.** Bodies enter the e-graph as rewrite candidates.
  The compiler may apply B-group and E-group rewrites to a function
  call when the called function's stdlib atoms carry the necessary
  contracts.

**Closure-policy extensibility.** Functions are the extensibility
surface for closure policies (§8.7, policy Y5). Any `.myco` function
that accepts a candidate-value collection and user hyperparameters
and returns a forward value qualifies as a user-defined custom policy.

**User recourse when the compiler cannot infer an inverse.** If the
compiler cannot derive an inverse for a `fn` body, refactor the
monolithic function into smaller composable pieces whose inverses the
compiler can infer from stdlib capability contracts; see `Invertible<_>`
(§7).

### 7. Contracts

**Summary.** Contracts are the single abstraction mechanism in Myco:
declaration, multi-contract satisfaction (`: A + B + C`), and
supertraits (`contract B : A`). Contracts apply uniformly to types,
functions, and distribution families. Parameterized and capability
variants carry compiler-actionable facts.

Contracts apply uniformly to types, functions, and distribution
families. Contract declaration. Multi-contract satisfaction
(`: A + B + C`). Supertraits (`contract B : A`). Named-type
comparison rules. Contract bodies are restricted to typed field
obligations and supertraits; `initial:`, `temporal:`, `d(x) = ...`,
`step(x) = ...`, and relation bodies are not valid in a contract
declaration (see §9).

#### 7.1 Parameterized Contracts

**Summary.** Contracts take type parameters (`Invertible<T>`,
`Convert<From, To>`, `Distribution<U>`). Parameters thread through
supertrait chains and satisfaction checks. Principal users are
capability contracts on stdlib atoms (§6) and distribution families
(§27).

Contracts take type parameters: `Invertible<T>` (invertible fn with
inverse type T), `Convert<From, To>` (conversion capability),
`Distribution<U>` (distribution over units U). Parameters thread
through supertrait chains and satisfaction checks. Capability
contracts on stdlib atoms (§6) and distribution families (§27) are
the principal users.

#### 7.2 Capability Contracts

**Summary.** Capability contracts carry compiler-actionable facts.
The distribution-side chain drives Tier A closed-form PPL routing;
the function-side chain (`Invertible<_>`, `Differentiable`,
`Monotone`) drives function-inverse rewrites and the `deriv` /
`condition_of` intrinsics. Satisfaction is composable through
supertrait chains.

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

**Summary.** `contract B : A` declares B as a refinement of A. Every
B-satisfier is also an A-satisfier. Supertrait chains compose;
diamond inheritance resolves by contract identity (one obligation
per supertrait, not one per path).

`contract B : A` declares B as a refinement of A. Every B-satisfier
is also an A-satisfier. Supertrait chains compose; diamond
inheritance resolved by contract identity (same supertrait reached
by two paths contributes one obligation, not two).

#### 7.4 Multi-Contract Coherence

**Summary.** `T : A + B` requires disjoint obligations or matching
obligations where A and B overlap. Conflicting obligations (same
name, incompatible signature across A and B) are a hard error at
**contract declaration time**, not impl time. `contract C : A + B`
with an unresolvable same-name collision between A and B is
rejected immediately, before any type tries to impl C. Contract
authors must rename to eliminate the collision.

Satisfaction of `T : A + B` requires disjoint obligations for A and
B, or matching obligations where they overlap. Two kinds of
collision can arise, with different resolution rules:

1. **Same obligation, reached by two supertrait paths** (ordinary
   diamond). `contract C : A + B` where both `A` and `B` supertrait
   a common `Root`, and `Root` declares obligation `foo`. This
   contributes **one** obligation per §7.3 (contract identity,
   not path identity). No conflict.

2. **Different obligations colliding on name** (the hard case).
   `A` declares obligation `foo: ... -> V`; `B` independently
   declares obligation `foo: ... -> W` with `V ≠ W` (or any
   incompatible signature). These are distinct obligations that
   happen to share a name. Myco rejects this at the declaration
   site of the contract that supertraits both: `contract C : A + B`
   emits a coherence error naming `foo`, both supertraits, and both
   signatures. Contract authors rename at the point of divergence.

The alternative policies (supertrait-order precedence with silent
shadow, or per-impl disambiguation syntax) were considered and
rejected. Silent shadow violates the "no surprises" posture; per-
impl disambiguation pushes the fix downstream to impl authors who
did not introduce the collision.

No same-name collisions across `A + B` ever reach the impl author.
By the time `T : A + B` is satisfiable, all obligations are
uniquely named.

#### 7.5 Default Implementations

**Summary.** A contract may supply a default body for an obligation.
The default applies only when the implementing type does not supply
its own. A type-supplied definition always takes precedence; defaults
never override a type-provided obligation.

A contract obligation may carry a default body that composes from
other obligations on the same contract:

```myco
contract Comparable {
    fn magnitude(self) -> Scalar<dimensionless>

    fn smaller_than(self, other: Self) -> Bool {
        // default: compare along the magnitude axis
        self.magnitude() < other.magnitude()
    }
}

type Mass : Comparable {
    grams: Scalar<gram>

    fn magnitude(self) -> Scalar<dimensionless> {
        value_in(self.grams, gram)
    }

    fn smaller_than(self, other: Self) -> Bool {
        self.grams < other.grams   // type-supplied; default is ignored
    }
}

type Energy : Comparable {
    joules: Scalar<joule>

    fn magnitude(self) -> Scalar<dimensionless> {
        value_in(self.joules, joule)
    }

    // no fn smaller_than supplied; compiler uses contract default
}
```

The fallback rule is unconditional: if the implementing type
provides the obligation (by name and compatible signature), the
type body wins and the contract default is not used. If the type
body omits the obligation, the contract default fills it. Contracts
never re-examine whether a type-supplied body is "better"; the
type author is the authority on their own type.

### 8. Relations and Equality

**Summary.** Relations are world-claims that the compiler treats as
equational merges. Overdetermination is not an error: after a system-
level classification (redundant / provably inconsistent /
conditionally inconsistent), closure policies Y1-Y6 combine competing
claims. `constraint` declarations carry inequality obligations with
three explicit discharge paths.

Relations as world-claims. Overdetermination is not an error; closure
policies combine competing claims. Policies Y1-Y6 including
un-deferred `condition_weighted` (backed by `condition_of`
Levels I-III). Merge semantics.

#### 8.1 `constraint` Declarations

**Summary.** Inequality or logical obligations the modeler asserts.
Unlike relations, constraints do not merge e-classes; they restrict
the admissible solution set. Three discharge paths: compile-time
proof, runtime projection (workflow-selected flavor), or training
loss penalty on training-classified SCCs.

Inequality or logical obligations the modeler asserts must hold.
Distinct from `relation` (equational merge) in that constraints
don't merge e-classes; they restrict the admissible solution set.
Discharge paths: compile-time proof via e-graph + refinement
reasoning, runtime projection (workflow-selected flavor, §25),
or training loss penalty (SCCs classified training, §20).

#### 8.2 `let` Bindings in Relation Bodies

**Summary.** Local aliases for subexpression reuse inside a relation
body. Compile-time only: not a new variable, not a state field. Two
roles: readability for multi-term equations, and e-class hinting
(same binding means same e-class).

Local aliases for subexpression reuse inside a relation body.
Compile-time only; not a new variable, not a new state field.
Example: `let flux = k * (psi_soil - psi_leaf); d(water) = flux -
transpiration`. Two roles: readability for multi-term equations,
and e-class hinting (binding signals "same e-class both places,"
which the e-graph honors via structural equality).

#### 8.3 `if` / `else` vs `where` in Relation Bodies

**Summary.** Two distinct constructs with different semantics.
`if C then A else B` is a runtime branch on the truth of `C`;
`where x is T` is compile-time structural narrowing under the
assumption that `x` inhabits `T`. `if` keeps e-graph merge
obligations on both arms; `where` specializes to the narrower type.

Two distinct constructs. `if C then A else B` introduces a runtime
branch: the relation's semantics depends on the truth of `C` at
each evaluation. `where x is T` is compile-time narrowing: the
subsequent body is type-checked under the assumption that `x`
inhabits `T`, and the branch is selected structurally (no runtime
test on the value). `if` preserves the e-graph's merge obligations
on both arms; `where` specializes the arm to a narrower type.

#### 8.4 `for` Loops in Relation Bodies

**Summary.** Compile-time unfolding. `for node in collection: ...`
expands to one relation per element at compile time; the loop
variable is not a runtime iterator. Collection must be statically
known (shape-generic is OK; runtime-sized `some` collections are
disallowed here).

Compile-time unfolding. `for node in collection: ...` expands to
one relation per element at compile time; the loop variable is
not a runtime iterator. Distinct from runtime iteration in
collections (§12). The collection must be statically known (shape-
generic OK; runtime-sized `some` collections disallowed here).

#### 8.5 Inline Relation and Constraint Sugar

**Summary.** Terse forms for single-line claims on field or type
declarations. `x: Scalar<m> = y + z` desugars to a named relation;
`x: Scalar<m> where { x > 0 }` desugars to a refinement constraint.
The longhand block forms remain available; the sugar is purely
ergonomic.

Terse forms for single-line claims attached to field or type
declarations. `x: Scalar<m> = y + z` desugars to a named relation
on `x`. `x: Scalar<m> where { x > 0 }` desugars to a refinement
constraint. The longhand block forms (`relation { ... }`,
`constraint { ... }`) remain available; the sugar is purely
ergonomic.

#### 8.6 Overdetermination: System-Level Classification

**Summary.** Before any closure policy applies, the compiler
classifies an overdetermined residual block three ways: redundant
(consistent; policies apply), provably inconsistent (hard compile
error), or conditionally inconsistent (runtime assertion before the
solver). Closure policies operate only on the redundant case.

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

**Summary.** Six user-facing handlers for redundant overdetermination:
`weighted_average`, `soft_select`, `hard_select`, `condition_weighted`,
user-defined (Y5), and `C(N,M)` enumeration. Selected per residual
block at workflow composition time.

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

**Summary.** A Y5 policy is an ordinary `.myco` function satisfying
the closure-policy interface: candidate values plus hyperparameters
in, a single forward value out. The compiler inlines the fn body into
the extraction pipeline, so Y5 policies participate in differentiation
and e-graph reasoning like any other fn.

A Y5 policy is an ordinary `.myco` function satisfying the
closure-policy interface: inputs are the candidate values (one per
competing claim) plus user-supplied hyperparameters; output is a
single forward value of the same type. Users register a Y5 policy
by name; workflows select it per residual block via the same
mechanism as Y1-Y6. The compiler inlines the fn body into the
extraction pipeline; Y5 policies participate in differentiation
and e-graph reasoning like any other fn.

#### 8.9 Smoothing as a Model Claim

**Summary.** Smoothing is a modeler choice, not a compiler-injected
approximation. The stdlib provides `smooth_max`, `smooth_abs`,
`smooth_step`, and related helpers; the modeler writes them
explicitly where non-smooth operators would break differentiability
or solver assumptions. The compiler does not auto-smooth.

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

**Summary.** Compiler-generated relations (junction balance, boundary
condition stubs, conservation synthesis) carry named obligation keys
like `balance(axial_flux)`. Modelers override with
`replaces balance(axial_flux): <body>` for targeted overrides
without disabling generation globally.

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

**Summary.** `initial:` and `temporal:` blocks in type bodies.
Module-scope relations reserved for truly cross-entity cases. ODE
form `d(x) = expr` and discrete form `step(x) = expr`; no `[t+1]`
subscript surface. `dt` is not a reserved name and temporal indexing
produces distinct e-graph ground terms.

`initial:` and `temporal:` blocks live in type bodies. Module-scope
only for truly cross-entity relations. `d(x) = expr` for ODE form,
`step(x) = expr` for discrete-update form. No `[t+1]` subscript
surface.

**Type bodies vs. contract bodies.** `initial:` and `temporal:`
blocks, `d(x) = ...` equations, and `step(x) = ...` equations appear
only in type bodies. Contracts are structural: a contract body may
declare typed field obligations and supertraits, nothing more. A
contract cannot carry initialization or evolution semantics because
contracts describe what a type exposes, not how that type evolves over
time. Any attempt to write `initial:` or `temporal:` inside a contract
body is a compile error (see §7 for the cross-link statement).

#### 9.1 `dt` Provision

**Summary.** `dt` is not reserved, not a universal, not a special
verb. For ODE form the compiler (or integrator) owns step size; for
discrete form tick cadence is an ordinary workflow binding via
`assume_constant("config.dt", ...)`. Time `t` is not a universal
either.

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

**Summary.** Generic events and relations expand at compile time to
one instance per satisfier (cartesian product over generic
parameters). Duplicate obligation keys across expanded paths are a
compile error, not a closure-policy situation. Overdetermination
analysis runs on the fully expanded constraint set.

A generic event or relation (`event<T: Species>(…)`) expands at
compile time to one concrete instance per T-satisfier (cartesian
product over all generic parameters). Each expansion path must
yield a unique obligation key; duplicate keys across paths are a
compile error, not a closure-policy situation. Overdetermination
and underdetermination analyses run on the fully expanded
constraint set, so uniqueness is a pre-analysis hygiene check.

### 10. Dynamic Topology and Events

**Summary.** `event` declarations mutate the simulation graph
structure. Referential-truth semantics: entities do not know they are
dead, events add facts, no tombstoning, no retraction. Firing order,
generic expansion, cross-container events, and `replaces` /
monotonicity live here.

`event` declarations for topology change. Referential-truth semantics:
things do not know they are dead. Events add facts; no tombstoning, no
retraction.

#### 10.1 Firing-Order Policy

**Summary.** Firing order for multiple matching events is a
simulation parameter at workflow composition, not language syntax.
Default is declaration order; workflow overrides via a Python-side
scheduling policy. Within a single event type, all valid firings
execute in parallel (GPU-batched). Three stdlib policies ship
(priority-based, random-with-seed, FIFO); the exact Python API
signature lives in §24.

When multiple events match at the same timestep, firing order
is a simulation parameter set at workflow composition, not
language syntax. Default: declaration order (lexical). Workflow
override via a Python-side scheduling policy that sees the
pending firings and the current state and returns an ordered list.
Three stdlib policies ship:

- **Priority-based.** Events declare numeric priorities;
  scheduler orders by descending priority.
- **Random-with-seed.** Scheduler draws a permutation from a
  seeded RNG for reproducibility.
- **FIFO.** Order of queueing; useful for streaming / arrival-
  order semantics.

Users may supply custom Python policies. The Python API signature
for scheduling policies is a workflow-layer concern and lives in
§24 (workflow verbs); §10 commits only to the contract that such
a policy exists and the three stdlib policies above. Keeps the
`.myco`-side story focused on event semantics rather than Python
surface.

Within a single event type, all valid firings execute in parallel
(GPU-batched); the scheduling policy operates between event types
at the same tick.

#### 10.2 Generic Event Cartesian-Product Expansion

**Summary.** Generic events expand at compile time to one concrete
event per satisfier combination (cartesian product over multiple
generic parameters). Each expanded path has its own obligation key
and participates in firing-order dispatch independently.

`event<T: Contract>(…)` expands at compile time to one concrete
event per T-satisfier. Multi-parameter generic events
(`event<T: A, U: B>`) expand over the cartesian product of
satisfier sets. Each expanded path has its own obligation key
(§9.2) and participates in firing-order dispatch (§10.1)
independently.

#### 10.3 Cross-Container Events

**Summary.** Events touching entities from different container types
resolve scope via the nearest-common-ancestor rule: the event binds
at the lowest type containing all affected entities. No common
ancestor is a compile error. Keeps event scope minimal.

An event that touches entities from different container types
resolves its scope via the **nearest-common-ancestor rule**:
the event binds at the lowest type that contains all affected
entities. If no common ancestor exists, compile error. This
keeps event scope minimal and prevents accidentally lifting an
event to module scope.

#### 10.4 Within-Event Tiebreaking

**Summary.** Concurrent firings of a single event type fall into
three cases: structurally identical facts (e-graph merges them),
conflicting writes on conserved fields (caught by junction balance),
or legitimately overdetermined next-tick residual (closure policies).
No within-event ordering construct is exposed.

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

**Summary.** `replaces <obligation_key>` overrides a compiler-
generated default by suppressing its emission, not by retracting a
fact. The e-graph never contains both the default and the override
simultaneously, preserving monotonicity. User-written retraction of
prior user claims remains open (tracked in §35).

A `replaces <obligation_key>` declaration (§8.10) overrides a
compiler-generated default relation by suppressing its emission,
not by retracting a fact after the fact. The e-graph never
contains both the default and the override simultaneously. This
preserves the monotonicity invariant.

The harder case of a user-written `event` that logically retracts
a prior user claim remains open and is tracked in §35 Other
Opens. Events only add facts; `replaces` applies only
to compiler-generated defaults, not arbitrary prior claims.

### 11. Geometry and Locus

**Summary.** Spatial framing. Horses own geometry, flies are embedded
entities located against that geometry. `bind_topology` supplies
concrete meshes at workflow time. Standard locus vocabulary
(`boundary`, `chart`, `metric`, `requires`), stdlib geometry catalog,
spatial operators (`grad`, `diverg`, `laplacian`, `curl`,
`normal_grad`, `trace`), and boundary conditions via `requires`.

Horse/fly composition pattern for spatial frames. `bind_topology` at
workflow time for concrete meshes. `on locus:` clause applies
symmetrically to `relation` and `temporal`.

#### 11.1 Spatial Operators

**Summary.** Stdlib-recognized spatial operators on locus-scoped
fields: `grad`, `diverg`, `laplacian`, `curl`, `normal_grad`,
`trace`, `limit_from`. `diverg` on a conserved flux field drives
auto-synthesized junction balance. Operators are stdlib axioms with
capability contracts; users do not annotate them. Dimension-
dependent signatures (e.g., `curl`) dispatch at the axiom level via
case-on-val-generic in the return type.

Compiler-recognized operators on locus-scoped fields:

- `grad(f)` — gradient of a scalar field; yields a vector field
  on the same locus.
- `diverg(v)` — divergence of a vector field; yields a scalar.
  `diverg` on a conserved flux field drives auto-synthesized
  junction balance (§3.7, §11.8).
- `laplacian(f)` — Laplace operator; `diverg(grad(f))`.
- `curl(F)` — dimension-dependent signature:
  `Vec<U> over Domain<G>` with `G.dim == 2` yields
  `Scalar<U/length>`; with `G.dim == 3` yields
  `Vec<U/length>`. `G.dim ∉ {2, 3}` is a compile error at the
  call site.
- `normal_grad(f)` — gradient dotted with the outward normal;
  defined on boundary sub-loci only.
- `trace(f, boundary)` — manifold restriction: the value of
  field `f` restricted to the named boundary sub-locus.
  Standard PDE trace operator.
- `limit_from(f, junction, edge)` — one-sided directional limit:
  the value of `f` as the junction is approached along a
  specified incident edge. Defined on `MetricGraph` /
  `RootedTree` junctions where the field may be discontinuous
  across incident edges.

Operators are stdlib axioms with capability contracts (§7.2).
Relations like `laplacian(f) = diverg(grad(f))` fire as e-graph
rewrites from stdlib declarations; users never annotate them.

**Dimension dispatch in axiom return positions.** `curl` is the
first operator whose return type depends on a val generic carried
by the input domain (`G.dim`). The dispatch pattern mirrors
`solve`'s dispatch on matrix structural subtype (§3.9, §30): the
stdlib declaration enumerates per-dimension cases, and the compiler
picks the applicable one at the call site based on the input's
generic parameters. User code generic in dimension may reach for
`curl` under a `where G.dim in {2, 3}` clause; monomorphization
produces distinct specialized bodies per dimension with the
appropriate return type. The formalization of case-on-val-generic in
axiom return positions is a small extension tracked as an open in
§35 (chunk 11 cross-cut, since sum types / pattern-matching at the
type level touch the same family of concerns).

#### 11.2 Boundary Conditions

**Summary.** Boundary conditions are `requires` blocks on boundary
sub-loci. Three standard forms (Dirichlet, Neumann, Robin) lower to
projection, elimination, or residual constraints based on
workflow-selected solver path. No defaults: a boundary without
`requires` is underdetermined.

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

**Summary.** Standard geometries use authoritative mathematical
names without dimensional suffixes. Solid regions: `Interval`,
`Rectangle`, `Disk`, `Box`, `Ball`. Manifolds: `Circle` (S¹),
`Sphere` (S²). Networks: `RootedTree`, `MetricGraph`,
`BranchingManifold`. Each exposes named sub-loci (`interior`,
`boundary`, junction classes). Horse/fly composition lets richer
entities reuse primitives without inheritance.

| Name | Dim | Kind | Typical Use |
|---|---|---|---|
| `Interval` | 1 | solid | roots, stems, cylindrical cross-sections |
| `Circle` | 1 | manifold (S¹) | periodic loops, azimuthal coordinates |
| `Rectangle` | 2 | solid | leaf surfaces, soil patches |
| `Disk` | 2 | solid | circular regions, polar-coord domains |
| `Sphere` | 2 | manifold (S²) | closed surfaces, radiative hemispheres |
| `Box` | 3 | solid | rectangular volumes, voxel domains |
| `Ball` | 3 | solid | fruit, nodules, root cells |
| `RootedTree` | 1 (branching) | network | plant hydraulic networks, vasculature |
| `MetricGraph` | 1 (branching) | network | river networks, mycelia |
| `BranchingManifold` | n | recursive | fractal / self-similar structures |

Naming rule: solid regions and manifolds use their standard
mathematical names without dimensional suffixes. `Sphere` is the
2-manifold S² (the surface of a 3-ball), distinct from `Ball`
(the 3D solid region). `Disk` is the 2D solid region, distinct
from `Circle` (the 1-manifold S¹, the loop boundary of a disk).
The solid-vs-manifold distinction is load-bearing; using `Sphere`
interchangeably for the surface and the solid region is a
category error the compiler rejects.

Coordinate-system parameterization lives on the `as` clause, not
as separate geometry types. `Disk as (r, θ)` expresses a disk in
polar coordinates; `Ball as (r, θ, φ)` expresses a ball in
spherical coordinates. There is no `Polar` or `Spherical`
geometry type in stdlib; those are coord conventions on solid
regions.

Each geometry exposes named sub-loci: `interior`, `boundary`,
junction classes (where applicable). Composition via the
horse/fly pattern (§11.4) lets richer entities reuse these
primitives without inheritance.

#### 11.4 Horse-and-Fly Composition

**Summary.** A horse owns geometry; flies are embedded entities
located against that geometry via an embedding field on the horse.
Flies do not own coordinates. Many fly types share one horse without
inheritance. Cross-scale visibility is downward-only.

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

Identifications on a geometry (§17 X2, geometry-body `identify`
declarations) apply to fields defined over the geometry. Fly
positions reference geometric points but do not inherit identity
collapse across fly identities: two flies at identified positions
sit at the same geometric point but remain distinct fly entities
unless the modeler writes an explicit merge.

#### 11.5 Discretization Configuration

**Summary.** Geometry becomes a mesh at workflow composition via
`bind_topology`: resolution, element type (FDM/FVM/FEM), refinement
policy, boundary identification. Compiler receives a concrete mesh;
no auto-refinement or adaptation.

A geometry becomes a mesh at workflow composition. `bind_topology`
supplies discretization: mesh resolution, element type (FDM /
FVM / FEM basis), refinement policy, boundary identification.
The compiler receives a concrete mesh and lowers spatial
operators against it. The compiler does not auto-refine or adapt;
mesh is a workflow decision.

#### 11.6 Compiler Discretization Defaults

**Summary.** If `bind_topology` omits discretization, the compiler
picks per-geometry defaults (uniform grids, one node per structural
vertex). Defaults are conservative smoke-test affordances; production
use typically requires explicit override.

If `bind_topology` does not specify a discretization, the
compiler picks per-geometry defaults documented in the stdlib
reference. Indicatively: `Line1D` uses a uniform N-node grid (N is
still workflow-supplied); `Rectangle2D` uses a regular M×N grid;
`RootedTree` uses one node per structural vertex with no interior
refinement. Defaults are conservative; the program compiles,
but accuracy targets for scientific applications typically
require explicit override. The default is a smoke-test
affordance, not a production recommendation.

#### 11.7 Edge-Interior vs Locus-Scoped Fields

**Summary.** A 1D edge field is either locus-scoped (one value per
edge, no position dependence) or edge-interior (a function of the
interior coordinate, discretized during lowering). Spatial operators
act on edge-interior fields only. The two styles do not merge.

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

**Summary.** Junction default is balance only: conserved-flux sums
to zero, auto-synthesized from `diverg()`. Continuity of non-flux
fields is not assumed; modelers opt in with explicit
`requires: left.f = right.f`. Conservation forces balance for free;
continuity is a modeling choice.

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

**Summary.** Flies attach to a horse via ordinary field declarations,
not a dedicated `embed` or `in` construct. Horse/fly composition is
a pattern, not a language primitive. No embedding keyword.

Flies attach to a horse (§11.4) via ordinary field declarations,
not a dedicated `embed` or `in` construct. A `Tree` carrying a
`LeafPatch` collection with per-patch attachment position uses
standard field syntax on the horse side: `patch_position:
Position`. The horse/fly composition is a pattern, not a
language primitive. The language has no embedding keyword.

#### 11.10 Geometry Coefficients via `requires`

**Summary.** Material properties (conductivity, diffusivity,
elastic modulus) enter via `requires` blocks on the locus, not a
`hint` keyword or parameter list. The same construct that attaches
boundary conditions attaches coefficients; one attachment surface.

Material properties attached to geometry (conductivity,
diffusivity, elastic modulus) enter via `requires` blocks on
the locus, not a `hint` keyword or parameter list. Example:
`requires: conductivity = <workflow-bound coefficient>` on the
locus body. The same construct that attaches boundary
conditions (§11.2) attaches coefficients; one attachment
surface, not two.

#### 11.11 Standard Locus Vocabulary

**Summary.** Four keywords inside locus bodies and geometry
declarations: `boundary` (named sub-locus), `chart` (coordinate
chart reference), `metric` (metric tensor for non-Euclidean
geometries), `requires` (attachment surface for constraints, BCs,
coefficients). New standard geometries ship via stdlib, not new
keywords.

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

No other geometry-level keywords are introduced. New standard
geometries ship via stdlib (§11.3), not via new keywords.

### 12. Collections and Iteration

**Summary.** Collections via `impl Contract` (heterogeneous element
type, static monomorphization) and `some` (runtime sizing).
Iteration patterns, aggregation primitives (`sum`, `product`, `any`,
`all`, `count`, `argmin`, `argmax`), and narrowing with
`where x is T`. Aggregations are stdlib-only.

`impl Contract` (heterogeneous element type, static monomorphization)
vs `some` (runtime sizing). Iteration patterns. Aggregation lowering.
Narrowing with `where x is T`.

#### 12.1 Aggregation Primitives

**Summary.** Named stdlib aggregations: `sum`, `product`, `any`,
`all`, `count`, `argmin`, `argmax`. Units-aware and conservation-
group-aware. Compose under stdlib-declared e-graph rewrites
(linearity, distributivity, `sum(map(f, xs))` fusions). No
user-declared aggregation surface.

Named stdlib aggregations over collections:

- `sum(xs)`, `product(xs)` — arithmetic. Units-aware;
  conservation-group-aware (§3.7 blocks cross-sibling sums
  without an explicit `convert`).
- `any(xs)`, `all(xs)` — boolean.
- `count(xs)` — cardinality, `Scalar<dimensionless>`.
- `argmin(xs)`, `argmax(xs)` — handle of the extremal element;
  see §12.2 for the heterogeneous case.

Aggregations compose under stdlib-declared e-graph rewrites
(linearity, distributivity, `sum(map(f, xs))` fusions). There is
no user-declared aggregation surface — new aggregations ship via
stdlib, matching the `.myco`-has-no-annotation-surface stance.
Soft and weighted variants (softmax, weighted_sum) are tracked
in §35 Other Opens pending collection-aggregation syntax lock.

#### 12.2 Tagged Handles for Heterogeneous `argmax`

**Summary.** `argmax` over `impl Contract` returns a tagged handle
`(pool_identity, intra_pool_index)` since concrete types live in
separate compile-time pools. `argmax` over homogeneous `some`
returns a plain index. Surface syntax is the same in both cases.

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

**Summary.** Aggregations with identity elements use them on empty
collections (`sum = 0`, `product = 1`, `all = true`, etc.). `argmin`
and `argmax` have no identity, so empty-reachable calls are compile
errors; callers must prove non-emptiness or guard.

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

**Summary.** Two sources of collection-size change. Bind-time
dynamism fixes membership at workflow composition (lowers with
runtime size N, no N-max). Event-time dynamism mutates at runtime
(requires N-max slot allocation and alive-mask lowering).

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

**Summary.** `impl Contract` collections desugar to one homogeneous
pool per concrete satisfier. Iteration fuses across pools into one
monomorphized loop per pool; cross-pool aggregations compose per-pool
results. Users see one collection; the compiler sees N pools.

An `impl Contract` collection desugars at compile time to one
homogeneous pool per concrete satisfier type. Iteration fuses
across pools: `for x in xs: body(x)` expands to one
monomorphized loop per pool. Cross-pool aggregations (`sum`,
`argmax`, etc.) compose the per-pool results under stdlib
rewrites. Preserves static monomorphization behind a
heterogeneous-iteration surface: users see one collection, the
compiler sees N pools.

#### 12.6 Iteration Styles

**Summary.** Three compile-time iteration surfaces: index-style
(`for i in 0..N`), iterator-style (`for x in xs`), and graph-
neighborhood-style (`for n in node.neighbors`). All are compile-time
constructs; runtime iteration is a lowering artifact.

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

**Summary.** `where x is T` narrows iteration to elements inhabiting
`T`, reusing the §8.3 type-narrowing machinery. Structural filter on
an `impl Contract` pool with body monomorphized against `T`; not a
runtime predicate. Runtime predicates use `if` inside the body.

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

**Summary.** `~` is layer-2 distributional metadata, not an equality
merge. Aleatoric/epistemic split, Tier A/B/C routing (closed-form /
approximate rewrite / opaque PPL handoff), independence via
structural identity (no naked correlation), and Cholesky
reparameterization for MVN. Observation machinery and joint-
sample field access via `.at()`.

`~` as layer-2 distributional metadata, not an equality merge.
Aleatoric/epistemic split. Tier A/B/C routing (exact closed-form /
approximate rewrite / opaque PPL handoff). Independence via structural
identity; no naked correlation. Cholesky reparameterization.

**Analytical-first ordering.** The compiler attempts closed-form
envelope propagation before approximate rewrite, and approximate
rewrite before sampling handoff. Tier A runs to exhaustion, then
Tier B under authorized `approximate` blocks runs to exhaustion,
then Tier C hands residual SCCs to the backend PPL. Sampling is
the language of last resort, not first resort. Capability contracts
on distribution families (§27) are load-bearing precisely because
every closure contract (`AffineSelfClosed`, `SumSelfClosed`,
`ProductSelfClosed`, `ScaleSelfClosed`, `SmoothTransformable`,
`ReparameterizedSampleable`) is a compile-time affordance for
keeping the graph symbolic. The pushforward rewrite (Appendix C
Z11) extends analytical carry-through across invertible
differentiable maps whose closure contracts do not otherwise match.

#### 13.1 Aleatoric and Epistemic Uncertainty

**Summary.** Same `~` surface, two kinds of uncertainty
classified by graph position plus workflow bindings. Aleatoric
`~` applies to measured/observed quantities tethered to data
via `observe` or to `~` inside temporal/event scope; realized
via sampling, does not reduce with data. Epistemic `~` applies
to unknown constants not observed per time-step; reduces via
Bayesian update and participates in training. The classification
is compiler-derived, not user-annotated.

Two distinct sources of uncertainty. Same `~` surface; the
compiler derives the classification from two static signals:
whether the LHS e-class has observation data attached (workflow
`observe`; §13.8) and where the `~` appears in the model
structure.

- **Aleatoric** — world-randomness. The quantity genuinely
  fluctuates across realizations (measurement noise,
  environmental stochasticity). Applies when the LHS is a
  measured/observed quantity tethered to data, or when the `~`
  appears inside `temporal:` or event scope. Realized via
  sampling; does not reduce with more data.
- **Epistemic** — parameter uncertainty. A fixed-but-unknown
  value the modeler does not know. Applies when the quantity
  is an unknown constant not observed per time-step (module
  scope, `initial:`, or any `~` whose LHS is neither data-bound
  nor in temporal/event scope). Reduces with observation via
  Bayesian update; participates in training.

The classification is compiler-derived, not user-annotated.
The user writes `~` uniformly; the compiler inspects graph
position plus workflow bindings to assign aleatoric vs
epistemic. SCC classification (§20) threads the two: aleatoric
variables enter the stochastic SCC class; epistemic latents
enter the training class. The same `~` operator, routed
differently based on derived classification.

#### 13.2 Tier A / B / C Dispatch

**Summary.** Three tiers tried in order per stochastic SCC. Tier A
uses capability-contract-advertised closed forms (affine, sum,
product, scale closures). Tier B applies user-permitted approximate
rewrites (Delta, Fenton-Wilkinson, CLT, GEV). Tier C hands the SCC
to the backend's opaque PPL.

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

**Summary.** When a latent has no downstream observation and its
marginal is closed-form (via capability contract), the compiler
eliminates it without user directive. Failed marginalization falls
through to Tier B/C. Users forbid specific marginalization by
tethering the latent with an observation.

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

**Summary.** SDE draws carry an integration-convention type
parameter (`BrownianMotion<Ito>` vs `<Stratonovich>`), not a global
setting. Default is `Ito`. Mismatched conventions within one SCC
are a compile error; cross-scope consistency is the user's call.

SDE draws carry an integration-convention generic:
`x ~ BrownianMotion<Ito>(...)` vs
`x ~ BrownianMotion<Stratonovich>(...)`. The convention is a
type parameter on the stochastic family, not a global setting.
The compiler uses it to route drift/diffusion rewrites
correctly. Default is `Ito`. Mismatched conventions within one
SCC are a compile error; the compiler does not auto-convert.
Cross-scope consistency is the user's call; one `.myco` file
may contain both conventions at different loci.

#### 13.5 Independence via Structural Identity

**Summary.** Two stochastic draws are independent iff their
e-classes are distinct. No naked correlation surface: correlated
structures are built by sharing upstream distributions or declaring
a joint family (MVN, Mixture). E-class identity is the only
language-level handle on independence.

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

**Summary.** `x ~ MultivariateNormal(μ, Σ)` reparameterizes to
`x = μ + L @ ε` with `L L^T = Σ` and `ε ~ Normal(0, I)`. The
Cholesky factor L is the compiler's canonical MVN intermediate;
positive-definiteness of Σ is encoded by L's positive-diagonal
refinement, removing the runtime PD check.

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

**Summary.** `.at("field_name")` extracts named fields from joint
/ named-field samples. Same `.at(...)` on the same sample collapses
to one e-class for consistency. `.at()` on a missing field is a
compile error. No tuple destructuring or positional indexing.

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

**Summary.** `observe(data, x ~ D)` attaches observed data as a
layer-2 envelope fact on x's e-class (no equational merge with the
data). Downstream samples condition on it; `D.log_pdf(data)` adds
to the SCC's training loss. Distinct from `identify`: observation
narrows the distribution, not the value.

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

**Summary.** `observe` attaches layer-2 metadata, not a new merge
source. The eight sources in §17 remain eight; layer-1 equational
core is untouched. Observations compose with other envelope facts
(refinement bounds, capability advertisements, tolerance envelopes)
without equational conflict.

`observe` attaches layer-2 distributional metadata; it does
not introduce a new e-graph merge source. The envelope fact
says "this e-class has observed data attached"; it narrows
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

**Summary.** Core `~` extends to cover remaining PPL surfaces
(coupling machinery B4, joint declaration syntax B2, higher-order
distributions) without freezing every keyword. Tier 1 primitives
(§27) are the ship surface; Tier 2 primitives land in chunk 08 and
§28.

The Tier 2 PPL design lock extended the core `~`
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
current ship surface; Tier 2 primitives land in chunk 08 and
§28.

### 14. Compiler Intrinsics

**Summary.** The intrinsics the compiler surfaces to modelers:
`deriv`, `integrate`, `condition_of` (Levels I symbolic / II
algorithmic / III runtime), and `loss_of` (named-field return).
Each intrinsic has defined e-graph interaction and documented
guarantees.

`deriv`, `integrate`, `condition_of` (Levels I symbolic / II algorithmic
/ III runtime), `loss_of`. What each intrinsic means, what the compiler
guarantees about it, how it interacts with the e-graph.

#### 14.1 `condition_of` — Levels I, II, III

**Summary.** `condition_of(expr)` returns a conditioning estimate at
one of three levels: symbolic (Level I, problem-intrinsic), algorithmic
(Level II, lowering-dependent), or runtime (Level III, numerically
computed). The level is tagged on the return. Primary consumer: Y4
`condition_weighted` closure policy.

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

The mode is tagged in the return; `condition_of(expr).mode`
surfaces which tier the compiler chose. Algorithmic-vs-problem
duality: Level I is the *problem's* conditioning (intrinsic to
the math); Level II is the *algorithm's* conditioning (depends
on lowering choice). The two can diverge, and `condition_of`
makes the distinction inspectable. Primary consumer: the Y4
`condition_weighted` closure policy (§8.7).

#### 14.2 `loss_of` — Named-Field Return

*Open.* Field inventory overlaps `cost_of` (§14) and the §19.1
extraction cost vector. Three surfaces use the word "cost" with
three different field sets and no cross-reference. Unification
tracked in chunk 12:
`planning/v2/v2.1_chunk_reports/12_cost_field_unification.md`.

**Summary.** `loss_of(residual)` returns a struct of named components
(`data_fit`, `constraint_violation`, `regularization`), not a scalar.
Users aggregate by name at training emission. The compiler does not
auto-sum; scalar loss is a workflow composition.

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

**Summary.** `integrate(f, x, domain)` returns the integral of `f(x)`
over `domain`. Unit algebra is mechanical (`[f] · [x]`). Integration-
by-parts fires as a stdlib rewrite; closed-form antiderivatives
collapse at compile time. Distinct from SDE stochastic integration.

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

**Summary.** `approximate` blocks authorize specific lossy rewrites
for a named scope with declared tolerance class and error bound. The
compiler derives expression lossiness from four cumulative sources
(atom contracts, approximation declarations, numeric types, backend
emulation) and cuts it into three tiers: lossless, lossy-model,
lossy-tolerance.

The 2x2 matrix of approximation flavors: (lossy-model vs
lossy-tolerance) x (univariate vs bivariate). Syntax, semantics,
envelope consequences.

#### 15.1 Block Syntax

**Summary.** `approximate` blocks carry `under` (named rewrite),
`tolerance_class`, `error_bound`, `body` scope, and optional `where`
predicate. Blocks compose by nesting; outside a block's `body` the
authorized rewrite does not fire. No global `approximate` scope.

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

**Summary.** Expression lossiness is a lattice join over four
sources: stdlib atom contracts, approximation-block declarations,
numeric type choices, and backend emulation paths. The compiler
reports the aggregate via inspection surfaces.

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
them, not a single authoritative source. The four sources are
the *origin* axis of lossiness; the *accounting* axis —
where in the compile stack the lossiness is quantified — is
the five-layer stack in §15.4. The two axes are orthogonal: a
single rewrite carries both a source label (one of four) and
a layer label (one of five).

#### 15.3 Three-Tier Lossiness Cut

**Summary.** Lossiness groups into three tiers for diagnostics and
Tier B dispatch: lossless (equational rewrites only), lossy-model
(modeler-chosen approximations), and lossy-tolerance (numerical
tolerance intrinsic to the solve). Each tier is surfaced distinctly
in diagnostics.

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

#### 15.4 Five-Layer Lossiness Accounting

**Summary.** Lossiness is quantified at five layers of the
compile stack: syntactic, distributional-envelope, structural-
identification, seam-state, and extraction-cost. Orthogonal to
the four-source origin taxonomy (§15.2); each rewrite carries
both a source label and a layer label. The layer axis tells
diagnostics *where the distortion is booked*; the source axis
tells them *why it happened*.

Lossiness accounting layers:

- **Layer 0 — Syntactic.** Distortions visible in the surface
  form without consulting envelope facts. Name-stripping
  rewrites (U-group) and operator-form substitutions at
  singularity sites (X1 pole L'Hopital). No numerical loss;
  information loss shows up as names that no longer
  round-trip.
- **Layer 1 — Equational.** Merges in the equational core
  (§16.1 layer 1). Strict by construction (monotonicity, §16.2).
  No lossiness at this layer by design; lossy-model rewrites
  (L, M, Q groups) must be authorized by an `approximate`
  block before they may produce layer-1 effects.
- **Layer 2 — Distributional envelope.** Distortions
  quantified in the distributional metadata layer (§16.1 layer
  2). Tier B approximations (Delta, Fenton-Wilkinson, CLT,
  GEV) live here. Admissibility projections (`hard_clip`,
  `sigmoid`, `soft_clip`) distort the distribution and are
  accounted at this layer.
- **Layer 3 — Adjacent keyed state.** Distortions localized
  to specific seams, events, or identity-indexed state
  (§16.1 layer 3). Identify-seam propagation (X2) and event-
  scoped rewrites (O-group, W-group) book their effects
  here. The layer-3 record carries provenance back to the
  declaring construct.
- **Layer 4 — Extraction cost.** Distortions that manifest
  only at residual-projection time: cost-vector-guided
  extraction picks one among multiple valid representations
  (Y-group closure policies, cost-struct tradeoffs §19.1).
  The rewrite itself is layer-1 or layer-2 lossless; the
  *choice* among equivalents carries accounting only when
  extraction commits to one.

**Worked example.** `hard_clip(x, 0, inf)` at a positivity
bound. Source axis (§15.2): source-1 projection (pre/post-
processing collapse onto a half-line). Layer axis (this
section): layer 2 distributional-envelope (distorts x's
distribution; equational core is untouched because the
projection is a user-authorized pre/post-processing operator,
not a layer-1 merge). One rewrite, two independent labels.

Diagnostic surfaces (§22) render both axes; the layer axis
tells the reader *where* to look for the distortion's
bookkeeping, the source axis tells them *why* it was
introduced.

---

## Part II — Compiler Substrate

**Summary.** What the compiler sees and manipulates: the e-graph
substrate with its three-layer scoping split, the eight equality-
introducing machinery sources, the type graph, the residual graph
projection, hierarchical SCC decomposition, lowering targets, and
plan-inspection affordances.

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

**Summary.** The e-graph is Myco's internal equality substrate,
structured as three concentric layers: an equational core (ground
terms under monotonic merge), envelope metadata keyed by e-class
identity, and adjacent keyed state for per-call solver state and
timestep/event-tagged copies. Each layer has its own monotonicity
and ownership rules.

The e-graph as the internal equality substrate. Three-layer split:
(1) equational core, (2) envelope metadata attached to e-classes,
(3) adjacent keyed state (timesteps, events, identity-tagged copies).

#### 16.1 Three-Layer Scoping Split

**Summary.** Three concentric layers: equational core (union-find
under monotonic merge, one per-run instance), envelope metadata
(facts keyed by e-class narrowing without merging), adjacent keyed
state (temporal/event/identity-indexed structures holding e-class
references). Merge sources write layer 1; contracts and observations
write layer 2; timesteps and events index layer 3.

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

**Summary.** Append-only. Merged e-classes stay merged; attached
envelope facts stay attached. No retraction, tombstoning, or
rollback. `replaces` suppresses default generation, not emitted
facts. Events add facts; dead entities continue to exist
equationally. Compilation is a left-to-right accumulation.

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

**Summary.** Envelope facts have three writers (stdlib contracts,
compiler rewrites, workflow `observe`), four readers (Tier A/B
dispatch, extraction pipeline, diagnostics, plan inspection), and
no invalidator. Conflicting facts emit a coherence error rather
than silent preference.

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

**Summary.** Tolerance envelopes carry one of four flavors:
entry-wise (per-element), operator-norm (induced matrix norm),
spectral (eigenvalue/singular-value behavior), or structural
(combinatorial / pattern-preserving). Each flavor has its own
composition rule; `approximate` blocks declare flavor in
`tolerance_class`.

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

**Summary.** Eight enumerated merge sources for the equational core:
explicit relation equations, workflow constant injection, algebraic
rewrites, `identify`, stdlib-declared function inverses, named-type
conversion, closure-policy co-membership, unit-preserving rewrites.
Unified rewrite-predicate language, A-Z rule groupings
(Appendix C), `identify` vs relation `=` distinction.

Eight enumerated merge sources: explicit relation equations,
workflow constant injection, algebraic rewrites, `identify`,
stdlib-declared function inverses (via capability contracts on
fns; see §6), named-type conversion, closure-policy co-membership,
unit-preserving rewrites. The 2x3 faithfulness x orientation matrix
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

#### 17.1 The Eight Authorization Sources — Prose

**Summary.** Exactly eight authorization sources write — directly
or via authorized rewrite classes — to the equational core:
explicit relation equations, workflow constant injection, algebraic
rewrites, `identify`, stdlib-declared function inverses, named-type
conversion, closure-policy co-membership, unit-preserving rewrites.
Some authorize direct merges; others authorize a rewrite class
whose merges fire when a predicate matches. Source tags travel
with merges for diagnostics and provenance.

The e-graph's equational core (layer 1 of the three-layer split,
§16.1) accepts merges from exactly eight authorization sources.
Each source has a declaration surface, a trigger condition, and a
faithfulness posture (§17 preamble matrix).

Sources split into two mechanisms:

- **Direct writers.** The declaration site produces a layer-1
  merge immediately when parsed. Sources 1, 2, 3, 7, 8 (relation
  equations, workflow constant injection, algebraic rewrites,
  closure-policy co-membership, unit-preserving rewrites).
- **Rewrite-class authorizers.** The declaration installs a
  rewrite class (or a Layer-3 site record, §16.1) whose merges
  fire later when a structural or site predicate matches.
  Sources 4, 5, 6 (`identify` via Layer-3 site records
  consumed by X2; stdlib-declared function inverses via
  capability contracts fed into E-group rewrites; named-type
  conversion via bidirectional rewrite installation).

Both mechanisms are first-class authorization sources; the
distinction is purely operational (when does the layer-1 merge
appear). Downstream tooling reads source tags uniformly.

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
   already-declared entities are the same thing (§17.2). Module-
   scope aliases produce a direct layer-1 merge. Geometry-body
   `identify coord_a <-> coord_b` declarations install a Layer-3
   site record (§16.1) keyed on the locus path; Appendix C X2
   consumes the record to emit layer-1 merges for fields over
   the geometry, tagged with the site's identity. Distinct from
   relation `=`, which asserts an equation that holds.
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

**Summary.** Both produce e-class merges but differ in user-facing
semantics. `identify x = y` means "these are the same thing": no
residual, no closure-policy consequences, idempotent. Relation `=`
is a world-claim equation that participates in overdetermination
and closure policies.

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
things. Surface-level: `identify` lives at module scope, inside
type bodies, or inside geometry bodies (where it declares
quotient-space identifications; Appendix C X2); relation `=`
lives inside relation bodies.

Idempotency of `identify` is a property of the resulting merge,
not of the declaration. Two geometry-body `identify` declarations
that produce the same layer-1 merge deduplicate at the merge
level (the e-class is merged once) but both persist in provenance:
diagnostics (§22) surface every declaration that contributed,
even when all produced the same merge. This keeps
`mycoc explain` honest when a modeler writes two `identify`
calls intending to state different facts that happen to collapse
to the same layer-1 equation.

#### 17.3 Function Inverses via Stdlib Capability Contracts

**Summary.** Function-inverse merges fire from stdlib-declared
capability contracts (`Invertible<inv=log, domain=Real>` on `exp`),
not user annotations. Users extend the catalog by composition, not
declaration. The inverse catalog is inspectable from stdlib alone.

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

**Summary.** All merge sources share one predicate language for
guards: refinements, capability satisfaction, structural shape, and
unit algebra. Compile-time only; runtime values do not drive
rewrites. One surface, one discharge procedure.

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

#### 17.5 Rewrite-Rule Groups A-Z

**Summary.** Rewrite rules are organized into 26 lettered groups A-Z
for inspection, debugging, and `approximate` block referencing.
Representative groups: A (algebraic), E (equality/merge), Y
(closure-policy), Z (distribution-family). Full catalog lives in
Appendix C.

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

The complete A-Z catalog is large and belongs in an appendix,
not §17 prose. The appendix is tracked in §34 Chunk-Slotted
Work and will ship with the stdlib reference; chunk 04 already
commits partial enumeration. Approximate blocks (§15.1)
reference rules by group letter in their `under:` field.

#### 17.6 Baseline Rewrite Partition

**Summary.** Rewrites partition into default-on (fire when predicate
holds: relation-`=`, algebraic, stdlib inverses, conversion, unit,
`identify`, constant injection) and default-off (fire only inside an
authorizing `approximate` block). Gives `.myco` its conservative
default: a model compiles with zero authorized approximations if the
modeler wrote none.

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

**Summary.** The type graph is a separate substrate from the expression
e-graph, carrying named-type relations (subtyping, conversion,
conservation-group membership, refinement implications). Its precise
interaction with the e-graph (how conversions inject merges, how
refinement obligations translate to rewrite predicates) remains open
pending chunk 07.

STUB, chunk 07 pending. The type graph is a separate substrate from
the expression e-graph, carrying named-type relations (subtyping,
conversion, conservation-group membership, refinement implications).
Its interaction with the e-graph (how named-type conversions inject
expression-level merges, how refinement obligations translate to
rewrite predicates) is the chunk 07 deliverable.

### 19. Residual Graph (Projection)

**Summary.** The residual graph is a user-facing diagnostic view
projected from the e-graph via cost-vector-guided extraction. It is
not a canonical form: different workflow cost preferences yield
different residuals. Subsections cover the cost model, projection
mechanics, residual classification, and saturation scheduling.

The residual graph as a user-facing diagnostic view projected from
the e-graph. Extraction decisions and what they yield. How
diagnostics reference which view.

#### 19.1 Extraction Cost Model

*Open.* Cost-vector fields here overlap the §14 O2.4 `cost_of`
inventory and the §14.2 `loss_of` fields. Three surfaces, three
divergent field sets, no cross-reference today. Unification
tracked in chunk 12:
`planning/v2/v2.1_chunk_reports/12_cost_field_unification.md`.

**Summary.** Residual extraction optimizes against a multi-dimensional
cost vector (precision, latency, memory, approximation class), not a
single scalar. Extraction returns a Pareto front; workflow
configuration selects a point. The same e-graph can yield different
residuals under different policies.

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

**Summary.** The extractor walks the e-graph top-down, choosing one
representative term per e-class under the cost vector. The broad
mechanism (root set from workflow-bound variables and observed
quantities, share-always preference, envelope propagation) is stable;
specific heuristics remain open under Tier 0 Phase 2 work.

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

**Summary.** Residuals carry two orthogonal classifications: the
four-way SCC tag (static, dynamic, stochastic, training) drives
lowering and backend dispatch, while the three-way overdetermination
tag (redundant, provably inconsistent, conditionally inconsistent)
gates closure-policy meaning. Diagnostics surface the pair.

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

**Summary.** Default-on rewrites saturate; default-off rewrites fire
only inside authorizing `approximate` blocks with an error budget.
Explicit relation and `identify` merges fire first, then algebraic
and unit-preserving, then conversion and closure-policy. An absolute
rewrite-count cap guards against pathological cases.

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

**Summary.** The compiler decomposes the residual graph into
strongly-connected components and assigns each SCC a four-way class:
static, dynamic, stochastic, or training. The class pivots lowering
strategy, training emission, and backend dispatch.

After constraint collection, the compiler decomposes the residual
graph into strongly-connected components. Each SCC receives a four-
way classification: **static** (fully resolved pre-run), **dynamic**
(timestepped), **stochastic** (distributional; requires sampling or
closed-form marginalization), **training** (gradient-optimized).
Classification pivots lowering, training emission, and backend
dispatch.

### 21. Lowering

**Summary.** Lowering compiles the residual graph into a backend
artifact. Static and dynamic modules take distinct paths; each SCC
lowers per its class; dynamic topology uses N-max slots plus an alive
mask; temporal indexing produces distinct ground terms rather than a
template. Subsections detail each mechanism.

N-max / alive-mask lowering for dynamic topology. `y[t]` and `y[t+1]`
as distinct ground terms (no per-timestep or template e-graph).
Handoff to the backend.

#### 21.1 Static vs Dynamic Module Classification

**Summary.** Modules classify as static (no events, no temporal
relations; single-pass lowering) or dynamic (at least one event or
temporal relation; timestepped runtime loop). Classification happens
before SCC decomposition and is module-level, so static modules
skip dynamic lowering machinery entirely.

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

**Summary.** Each SCC class lowers through a distinct path: static
SCCs fold or prelude-evaluate; dynamic SCCs emit per-tick body code;
stochastic SCCs route to PPL tiers A, B, or C; training SCCs emit
gradient-producing computation with per-residual loss exposure. An
SCC inherits the most expensive class among its members, with
diagnostics on promotion so the modeler can refactor or accept.

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

**Summary.** Each tick's value of a temporal field is a distinct
ground term in the e-graph, not a template instance. Temporal
relations (`step`, `d`) connect adjacent ticks; merges at one tick
do not propagate across time; each tick's residuals are independently
classified. Backend storage maps T ticks to T slots (bounded runs)
or a rolling buffer sized to max lookback (streaming runs).

Temporal indexing produces distinct e-graph ground terms,
not a templated family. `y[1]`, `y[2]`, `y[3]` are three
different e-classes; temporal relations (`step(y) = expr`
writes `y[t+1]` from `y[t]`; `d(y) = expr` encodes a
derivative relation between adjacent ticks) connect them.
The e-graph does not "template" over time; there is no
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

**Summary.** Event-time collections lower to a fixed-capacity array
plus an alive-bit mask. N-max is declared at the collection, optionally
overridden by `bind_topology`. Allocation claims free slots in O(1);
retirement flips the bit but leaves equational history intact;
overflow is a runtime error, not silent growth.

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

**Summary.** The compiled program is an output artifact, not the
source of truth: reproducibility rests on `.myco` plus workflow
Python together. The `mycoc explain` CLI exposes the compiled plan
for auditing, debugging, and verifying compilation choices.
Inspection is a debugging affordance, not a required step.

The compiled program is an output artifact. Reproducibility is
guaranteed by `.myco` plus workflow Python together. Compiled code
is inspectable via `mycoc explain` (and related CLI surfaces, §36)
for users who want to audit the plan, debug behavior, or verify
compilation choices. Inspection is a debugging affordance.

---

## Part III — Workflow Interface

**Summary.** Part III defines the boundary between `.myco` and the
Python workflow that drives it: the compiler declares structure,
Python supplies values, initial conditions, topology, and observations.
Covers the eight workflow verbs, training emission, and how the
boundary keeps the compiler projection-free.

The boundary between `.myco` and Python.

### 23. The `.myco` ↔ Python Boundary

**Summary.** `.myco` declares structure; Python supplies values and
drives execution. The compiler stays projection-free: solver choice,
projection flavor, and numeric configuration all cross from Python.
Subsections cover runtime `where`, multi-binding compilation,
cross-study callable reuse, and the two error tiers.

`.myco` declares structure; Python supplies values and drives
execution. The compiler does not auto-emit projection or solver
selection; those are workflow choices (§0.1 projection-free
compiler). All numeric values (physical constants, fit parameters,
data series, initial conditions, topology, observations) cross
this boundary.

**Dumb-data Python layer.** Python never sees `.myco` types as
Python classes. The compiled artifact exposes a node catalog (path,
declared type shape, binding role, units); Python verbs (`bind`,
`observe`, `run`) operate over those path names, not over spore-
specific symbols. Spore authors ship one artifact (`.myco` sources
plus `myco.toml`); there is no Python mirror package. The Python
library grows along one axis only — generic data primitives — not
along the shape of any particular model. Locked in
`v2.1_chunk_reports/09_workflow_data_layer.md`; exact syntax for
node paths, the typing of the catalog, and the observe output-format
menu remain open (§35).

#### 23.1 Runtime `where` at Workflow Composition

**Summary.** The `where` keyword spans three layers: compile-time
type narrowing, collection iteration filter, and workflow
composition gate. Context disambiguates; diagnostics name the layer.
The composition-gate form evaluates at binding time, so the compiled
artifact carries only the selected bindings.

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
layers: compile, iteration, composition. Context disambiguates;
diagnostics name the layer when the keyword appears ambiguously.

#### 23.2 Multi-Binding Compilation

**Summary.** One `.myco` compiles once to a parameterized plan;
many workflows bind the same plan under different value
configurations. Trained controller weights persist across runs that
bind the same callable, so calibration on one dataset transfers to
prediction on another without recompilation.

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

**Summary.** Callables cross study boundaries by conforming to plain
contracts: output contract advertises what the callable produces,
input contract advertises what it consumes, and any workflow whose
surface matches can bind the trained instance. No separate "data
contract" kind, no stateful cross-workflow runtime; the shared
artifact is trained weights plus a plain contract.

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
The shared artifact is trained weights plus a plain contract;
no extra machinery.

#### 23.4 Error Tiers: Compile vs Workflow Composition

**Summary.** Errors split into two tiers: `mycoc` compile errors
(structural problems visible without bindings) and workflow
composition errors (problems visible only once bindings arrive, like
shape mismatches, contract violations, or N-max ceiling overrun).
Runtime errors form a third tier that lives in backend surfaces, not
this spec.

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
tier that this spec does not address normatively; they live
in backend and deployment surfaces.

### 24. Eight Workflow Verbs

**Summary.** Eight verbs form the workflow-composition surface:
`assume_constant`, `assume_series`, `learn_constant`, `learn_initial`,
`learn_trajectory`, `bind_controller`, `bind_topology`, `observe`.
Each verb binds a specific surface with specific gradient-flow
implications. Subsections detail controllers, topology binding,
future candidates, and run-config.

`assume_constant`, `assume_series`, `learn_constant`, `learn_initial`,
`learn_trajectory`, `bind_controller`, `bind_topology`, `observe`. For
each verb: what it binds, when it fires, gradient-flow implications.

#### 24.1 `bind_controller`, Contract I/O Specification

**Summary.** `bind_controller(path, fn, input_contract,
output_contract)` attaches a Python callable to a named `.myco`
site. Both contracts are plain contracts; there is no separate
"data contract" kind. The controller is a purely workflow concept,
with no `.myco` keyword introducing it.

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

#### 24.2 `bind_controller`, Gradient-Flow Semantics

**Summary.** Controllers register their learnable parameters with
the training loss at composition. Gradient flow happens via the
backend's AD: the compiler treats the controller as a differentiable
black box advertising `Differentiable` on its output contract. Non-
differentiable controllers fall back to opaque. Trained weights
persist across runs that bind the same callable.

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
  differentiable black box: it advertises
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

**Summary.** `bind_topology` is the workflow counterpart to `.myco`
geometry declarations: it supplies a concrete mesh, boundary
identification, material coefficients, and optional event-time
capacity override. Fires at workflow composition and is the only
path by which declared geometry becomes executable.

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
executable: `.myco` declares the locus structure, the verb
materializes a specific instance.

#### 24.4 Future Verbs Beyond the Eight

**Summary.** The eight verbs are the complete workflow-composition
surface for this release. Candidate future additions
(`bind_known_constants`, `bind_parameters`, `assume_prior`) are
tracked but deferred because the existing verbs cover shipped use
cases. Revisit if later work creates concrete demand.

Positive statement of scope: the eight verbs listed in the
§24 preamble are the complete workflow-composition surface
for this release. No additional verbs ship in the first release.

Candidate future additions tracked for later:

- **`bind_known_constants`.** Batch form for binding many
  physical constants at once from a workflow-side table.
- **`bind_parameters`.** Batch binding for empirical-fit
  parameter vectors (e.g., a full parameter sweep).
- **`assume_prior`.** Explicit prior-distribution binding
  distinct from `learn_constant`, for cases where the user
  wants to specify a prior without declaring the constant
  as learned.

Each is deferred because the eight verbs cover the shipped
use cases and adding surface without concrete demand risks
coupling to specific workflow idioms. Revisit when Tier 2
PPL (§13.10) and chunk 08 lock; some may subsume into
existing verbs by that point.

#### 24.5 Run-Config and Workflow Configuration Surface

**Summary.** Run-config is the non-binding configuration the
workflow supplies at composition: seed, backend, verbosity, dt
(when used), profile hints. Distinct from the eight verbs since
run-config does not bind model values; it configures how the
compiled plan executes. Different runs of one plan can use
different run-config without recompilation.

Run-config is the non-binding configuration the workflow
supplies at composition. Distinct from the eight verbs: run-
config does not bind model values; it configures how the
compiled plan executes.

Representative fields:

- `run.config.seed`. RNG seed for stochastic SCCs.
- `run.config.backend`. Backend selection and its
  capability-fallback mode (error / host / emulate, §31).
- `run.config.verbosity`. Diagnostics level.
- `run.config.dt`. Referenced via `assume_constant` in a
  discrete-time model (§9.1).
- `run.config.profile`. Execution-profile hints (batch
  size, memory budget).

Run-config fields are referenced from workflow verbs as
strings (`assume_constant("run.config.dt", 0.01)`); the
compiler does not bake them into the plan beyond the
binding surface. Different runs of the same plan can use
different run-config without recompilation.

### 25. Training Emission

**Summary.** Training SCCs compile to gradient-trainable code with
warm-start semantics drawn from `assume_constant` initial values or
`learn_constant` priors. Workflow selects projection flavor
(`hard_clip`, `sigmoid`, `soft_clip`). Per-residual loss exposure
lets users attach losses to named residuals; constraint enforcement
discharges at compile time where possible, otherwise at runtime.

How the compiler emits gradient-trainable code for SCCs classified as
training (§20). Warm-start semantics (initial values from
`assume_constant`, or priors from `learn_constant`). Projection-
flavor selection (`hard_clip` / `sigmoid` / `soft_clip`) chosen by
the workflow. Per-residual loss exposure: users attach losses to
named residuals. Constraint enforcement strategy: compile-time
discharge where possible, runtime projection otherwise.

---

## Part IV — Standard Library

**Summary.** Part IV covers what ships with Myco: numeric types,
distribution families, kernels, units, and matrix/tensor primitives.
Domain-specific units and models stay out of core and ship as
distributable packages on top of the stdlib.

What ships with Myco.

### 26. Numeric Types

**Summary.** `Scalar<U, T = Float64>` takes an explicit numeric
representation parameter with `Float64` as default. Seven reps ship:
`Bool`, `Integer`, `Rational`, `Float32`, `Float64`, `BigFloat`,
`Complex`. `T` must satisfy a base `Numeric` contract hierarchy;
mixed-T arithmetic is forbidden without explicit conversion.

`Scalar<U, T = Float64>` with explicit `T` parameter and `Float64`
default. `Rational` for exact constant folding (with termination
caveats). `BigFloat`. Default-compatibility constraints.

#### 26.1 Numeric Representation Hierarchy

**Summary.** The stdlib provides seven representations for the `T`
parameter: `Bool`, `Integer`, `Rational`, `Float32`, `Float64`,
`BigFloat`, and `Complex`. `Float64` is the per-Scalar default, not
module-wide. Forward-mode AD is not exposed as a user-facing
representation since backends own AD.

`Scalar<U, T>` takes an explicit numeric representation parameter
T. The stdlib provides:

| T | Role | Notes |
|---|---|---|
| `Bool` | two-valued logic | consumed by boolean relations, predicates, alive masks |
| `Integer` | arbitrary-precision integers | exact; GPU-incompatible for arbitrary precision |
| `Rational` | exact rationals | §26.3 termination caveat; GPU-incompatible |
| `Float32` | IEEE single | backend-dependent availability |
| `Float64` | IEEE double | default; universal backend support |
| `BigFloat` | arbitrary-precision floats | exact rounding semantics; GPU-incompatible |
| `Complex` | complex numbers | in scope, representation and contracts open (§35) |

Forward-mode AD is not a user-facing representation.
Backends own AD (§31); dual numbers would duplicate what the
backend already provides. Retired to anti_spec.md.

Default `T = Float64` is per-Scalar, not module-wide. Mixing
T within one expression is forbidden without explicit
`convert T1 -> T2` (§26.2).

#### 26.2 Default-Compatibility Constraints on T

**Summary.** `T` must satisfy a base `Numeric` hierarchy: ring
closure, total ordering (where applicable; Complex is exempt), zero
and one identity elements, and backend representability. Mixed-T
arithmetic is a compile error and requires explicit `convert`;
`Float32 -> Float64` is lossless, the reverse is lossy-tolerance.

The `T` parameter in `Scalar<U, T>` must satisfy a base
`Numeric` contract hierarchy:

- **Ring closure** (`Plus`, `Minus`, `Times`). The four
  arithmetic operators close within T.
- **Total ordering** (`Compare`). Required for `min`,
  `max`, sort, `argmin`, `argmax`. Complex T does not
  satisfy total ordering; stdlib functions requiring it
  accept only ordered T.
- **Zero and one identity elements.** Required for
  empty-collection defaults (§12.3), algebraic rewrites
  (§17.1 source 3).
- **Backend representability.** The run's backend must
  advertise support for T. Mismatch surfaces as a
  workflow-composition error (§23.4).

Mixed-T arithmetic is a compile error; the user must write
`convert T1 -> T2` explicitly. This makes numerical behavior
predictable: `Scalar<m, Float32>` and `Scalar<m, Float64>`
do not silently promote. Conversion `Float32 -> Float64` is
lossless; `Float64 -> Float32` emits the standard lossy-
tolerance envelope (§15.3).

#### 26.3 Rational Termination Caveat

**Summary.** `Rational` arithmetic is exact but unbounded, so
numerator and denominator can grow without limit in iterated
operations. Two compile-time guards: a warning (not error) for
`Rational` state inside temporal relations, and a workflow-composition
error for `Rational` in GPU-backed SCCs. Same constraints apply to
arbitrary-precision `Integer` and `BigFloat`.

`Rational` arithmetic is exact but unbounded. Numerator and
denominator grow with each non-trivial operation; iterated
exact arithmetic can blow up representation size. Two
compile-time guards:

- **Unbounded-loop warning.** `Rational`-typed state inside
  a temporal relation (`d` or `step`, §9) emits a compile
  warning. Warning, not error: some applications
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

**Summary.** Tier 1 ships 19 univariate continuous, 5 discrete, and 3
multivariate families, plus the `Truncated<D>` and `Mixture` meta-
families. Conjugate-posterior rewrites are enumerated as a closed
catalog. Tier B approximate rewrites (Delta, Fenton-Wilkinson, CLT,
block-maxima to GEV) fire under `approximate` blocks. Tier 1, 2, 3
scope the family catalog; Tier A, B, C are the orthogonal dispatch axis.

Tier 1 univariate continuous families (19): Normal, LogNormal, Uniform,
Beta, Gamma, Exponential, ChiSquared, Cauchy, Student-t, Laplace,
HalfNormal, HalfCauchy, InverseGamma, Lévy, Weibull, Pareto, Fréchet,
Gumbel, GEV. Tier 1 discrete: Bernoulli, Categorical, Poisson,
NegBinomial, Hypergeometric. Tier 1 multivariate (gated on B5):
MultivariateNormal, Dirichlet, Multinomial. Meta-families: `Truncated<D>`,
`Mixture<D₁,…,D_N | weights>`. Conjugate-posterior rewrites.
Tier B approximate rewrites: Delta method, Fenton-Wilkinson, CLT,
block-maxima → GEV.

**The `Distribution<U>` contract.** Every Tier 1 and Tier 2
distribution family implements the `Distribution<U>` capability
contract. The contract has three required methods and a set of
optional capability sub-contracts that advertise algebraic
closures used by Tier A dispatch (§13.2).

Required methods:

- `sample(params) -> Scalar<U>` — draw one realization.
  Required for Tier C opaque handoff and for Tier B rewrites
  that reduce to sampling at specific call sites. Backend-
  owned; the `.myco` signature is the contract surface only.
- `log_pdf(params, x: Scalar<U>) -> Scalar<unitless>` — log
  density at `x`. Required for likelihood contributions (§13.8
  `observe`), training emission (§25), and Tier A closed-form
  posterior construction. Stdlib atoms for Tier 1 families
  supply closed forms; user-defined distributions compose
  `log_pdf` from stdlib atoms.
- `pdf(params, x: Scalar<U>) -> Scalar<unitless>` — density at
  `x`, provided as a convenience. May be derived from `log_pdf`
  (default) or given directly when closed-form density avoids
  a log/exp round-trip.

Optional capability sub-contracts (advertised on the family
declaration; see §7.2 and §27.1 table):

- `AffineSelfClosed` — `a * X + b` is in the same family with
  analytically computed parameters.
- `SumSelfClosed` — `X + Y` for independent same-family draws
  is in the family (possibly under shared-parameter
  constraints).
- `ProductSelfClosed` — same, under multiplication.
- `ScaleSelfClosed` — scalar scaling preserves the family.
- `SmoothTransformable` — smooth differentiable transformation
  admits a Tier B delta-method rewrite; see Appendix C Z-group.
- `ReparameterizedSampleable` — sampling via a differentiable
  transform of a base noise source (e.g., MVN via Cholesky;
  §13.6, Appendix C Z10).
- `Conj(X)` — conjugate prior to family X. Fires the
  conjugate-posterior rewrite catalog (§27.3).

User-defined distributions implement `Distribution<U>` by
supplying the three required methods (composed over stdlib
atoms; chunk 08 bans user-declared capability annotations).
The compiler derives which optional sub-contracts hold when
possible; when it cannot, the user-defined family routes to
Tier C. This is the only extensibility path — no annotation
surface for advertising closures.

#### 27.1 Tier 1 Distribution Families, Table

**Summary.** Tier 1 families ship as capability-tagged stdlib
declarations with Distribution, Affine/Sum/Product/ScaleSelfClosed,
SmoothTransformable, ReparameterizedSampleable, and Conj(X) tags.
Multivariate subset (MVN, Dirichlet, Multinomial) is gated on B5
matrix heterogeneous-unit resolution for how `Σ` carries units.

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

**Summary.** Two meta-families wrap Tier 1 distributions to produce
new compositional distributions. `Truncated<D>` restricts a univariate
D to an interval and renormalizes; `Mixture` combines n components
under non-negative weights summing to 1. Both inherit a subset of
their components' capabilities and compose with each other.

Two meta-families wrap base Tier 1 distributions (§27.1) to
produce new compositional distributions.

**`Truncated<D>`, interval truncation.** `Truncated<Normal>(μ,
σ, lo, hi)` restricts `Normal(μ, σ)` to the interval `[lo,
hi]` and renormalizes. Applies to any univariate D that
satisfies `Distribution<U>`. Capabilities: inherits D's
capabilities minus closures broken by truncation
(`AffineSelfClosed` is generally lost; `ReparameterizedSampleable`
survives via inverse-CDF sampling). Refinement types
(§3.2) interact cleanly: `x ~ Truncated<Normal>(0, 1, 0, 1)`
auto-satisfies `UnitInterval`.

**`Mixture<D₁, …, Dₙ | weights>`, weighted combination.** A
mixture of n component distributions with non-negative weights
summing to 1. Components can be distinct families; shared-
support requirement is enforced structurally. Weights are
themselves values, workflow-supplied (`assume_constant` or
`learn_constant`). Capabilities: `Mixture` is a `Distribution`
but closes under fewer algebraic operations than its
components; specifically, `AffineSelfClosed` survives only
when every component satisfies it.

Both meta-families compose: `Mixture<Truncated<Normal>(…),
Truncated<Normal>(…)>` is a legitimate Tier 1 construction.
Nesting depth is bounded only by backend handoff costs.

#### 27.3 Conjugate-Posterior Rewrite Catalog

**Summary.** The stdlib enumerates a closed catalog of conjugate-
posterior rewrites covering Beta-Bernoulli/Binomial, Gamma-Poisson,
Normal-Normal (known variance), InverseGamma-Normal (known mean),
and Dirichlet-Multinomial. The rewrites fire automatically when the
compiler detects a matching `~` structure, no user directive
required.

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

The catalog is closed for this release; additional conjugate pairs
that modelers need are either derivable via `Truncated` /
`Mixture` composition or route to Tier 2 (chunk 08). The
rewrites fire automatically when the compiler detects a
matching `~` structure; no user directive is required.

#### 27.4 Extended Capability Table

**Summary.** Tier A dispatch needs extra capability columns beyond the
core tags: support, log_pdf, moments, reparam, sampling, entropy,
kl_div. The full table lives in the stdlib reference; this spec is
normative only about which columns exist. Missing entries are "not
closed-form" and fall through to Tier B or Tier C.

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

**Summary.** Tier 1 ships: 27 families plus two meta-families with
capability contracts and conjugate-rewrite wiring. Tier 2 is partial:
the factorized or closed-form-reparameterizable subset ships in Tier
1; the genuinely joint subset (B2 coupling syntax, copulas, Wishart
variants) is open pending chunk 08. Tier 3 (non-parametric) is open.
Tier 1/2/3 orders the family catalog; Tier A/B/C orders dispatch and
is orthogonal.

Tiers are the PPL scoping axis distinct from the distribution-
family catalog:

- **Tier 1.** Ships in this release. The 27 families in §27.1
  plus the two meta-families in §27.2, with capability contracts
  and closed-form rewrites (§27.3) wired in. Includes three
  multivariate members (MVN, Dirichlet, Multinomial), with
  MVN using the Cholesky reparameterization locked in §13.6.
- **Tier 2.** Partial. The multivariate subset that admits a
  factorized representation or a closed-form reparameterization
  ships in Tier 1 (MVN via Cholesky, Dirichlet/Multinomial via
  conjugacy). The genuinely joint subset: declarations that
  introduce coupling structure directly (B2 syntax), correlated-
  sample coupling machinery (B4), copulas, Wishart / InverseWishart
  / LKJ (gated on B5 heterogeneous-unit matrix resolution), and
  higher-order distributions routing through kernel machinery
  (§28), remains **open** pending chunk 08 design. Framing is
  "in scope for this design envelope, machinery not yet locked,"
  not "deferred to a future version." Tracked in §35 Other Opens.
- **Tier 3.** Open. Non-parametric and process-valued families
  (Gaussian Process, Dirichlet Process, Chinese Restaurant
  Process, Pitman-Yor, Indian Buffet Process, Beta Process). No
  formal tier boundary has been drawn. GPs are expected to route
  through §28 Kernels rather than through a distribution-family
  catalog entry, but whether non-parametric families share that
  routing, require a distinct mechanism, or are treated as Tier C
  (opaque PPL handoff) is an open question. Tracked in §35.
- **Tier A / B / C.** Dispatch tiers (§13.2), orthogonal to
  Tier 1/2/3. A = exact closed-form, B = approximate rewrites
  (Delta, Fenton-Wilkinson, CLT, block-maxima → GEV), C =
  opaque PPL handoff.

"Tier 1 ships" is the positive commitment. "Tier 2 partial /
Tier 3 open" are explicit open design questions, not deferrals
to a future Myco version: they belong inside the current design
envelope and block shipping only of the specific families that
need their machinery. Tier A/B/C are about dispatch, not about
what exists: a Tier 1 family can dispatch to any of A/B/C
depending on the transformation applied to it.

### 28. Kernels

**Summary.** Kernels are ordinary functions from locus point pairs
to scalars, with kernel-ness expressed via capability contracts
(`PositiveDefinite`, `Stationary`, `Isotropic`) rather than a separate
type kind. Stdlib ships Matérn, RBF, rational-quadratic, and Wendland;
composition rules preserve contracts. Sparsity and integration
operators are deferred to chunk 03.

Chunk 03 unified-machinery thread is pending e-graph substrate lock;
the surface shape below is committed, internal substrate not.

#### 28.1 Kernels as Functions with Capability Contracts

**Summary.** Kernels are plain functions `fn k(x: Point<L>, y:
Point<L>) -> Scalar<U>` with no separate keyword or type kind.
Kernel-ness comes from capability contracts on atoms:
`PositiveDefinite`, `Stationary`, `Isotropic`. Standard operations
(sum, product, scaling, radial wrapping) preserve contracts via
closure rules, so the compiler derives kernel properties from
composition without user property-declaration surface.

Kernels are ordinary functions from pairs of locus points to scalars:
`fn k(x: Point<L>, y: Point<L>) -> Scalar<U>`. No separate `kernel`
keyword, no separate type kind. Kernel-ness is a property of the
function that the compiler verifies from body composition plus
capability contracts on atoms, mirroring how function invertibility
and differentiability are derived (§7.2, §6, Anti-Spec "user-declared
fn invertibility / differentiability / domain"). The relevant
capability contracts:

- `PositiveDefinite`. Guarantees the Gram matrix
  `K_{ij} = k(x_i, x_j)` is PSD for any finite point set. Required
  for use as a Gaussian Process covariance kernel.
- `Stationary`. Guarantees `k(x, y) = k̃(x − y)` for some `k̃`.
  Implies translation invariance on the ambient locus.
- `Isotropic`. Guarantees `k(x, y) = k̂(‖x − y‖)` for some `k̂`.
  Supertrait `Stationary` plus rotation invariance.

Stdlib kernels, Matérn (ν = 1/2, 3/2, 5/2, ∞), squared-exponential
(RBF), rational-quadratic, Wendland compact-support, satisfy all
three. Non-stationary kernels (e.g. polynomial `k(x, y) = (x · y + c)^d`,
Brownian `k(x, y) = min(x, y)`) satisfy `PositiveDefinite` but not
`Stationary`. The usual operations on kernels preserve the contracts:
sum preserves `PositiveDefinite` and `Stationary`, product preserves
`PositiveDefinite`, scaling by a positive scalar preserves both, and
radial wrapping (`k̂(‖·‖)`) elevates `Stationary` to `Isotropic`.
These closure rules are how the compiler derives kernel contracts
from composition without user property-declaration surface.

#### 28.2 Ambient-Locus via Composition

**Summary.** Kernels take `Point<L>` arguments where `L` is ambient at
the call site, not declared per-kernel. Kernel families that require
specific locus structure express it via a locus contract, not a
specialized kernel type. Product loci compose via `(x1,y1), (x2,y2)`
arguments with the PositiveDefiniteness closure rule applied.

Kernels take `Point<L>` arguments, where the locus `L` is ambient and
fixed by where the kernel is called, not by a per-kernel declaration.
This avoids kernel families that only work on one space; e.g.
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

#### 28.3 Kernel Sparsity and Integration, Deferred to Chunk 03

**Summary.** Two kernel-adjacent concerns defer to chunk 03: sparse /
compact-support representation (belongs in matrix assembly, not
kernel definition) and kernel integration operators (convolution,
measures, stochastic integrals). Until unlock, kernels support
direct evaluation, function composition, and use as GP covariances
via Tier C opaque handoff.

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

Until those unlocks, kernels support direct evaluation,
function composition, and use as GP covariances via opaque PPL
handoff (§13.2, Tier C). Non-opaque GP handling routes through the
kernel stdlib when chunk 03 lands.

### 29. Units Library

**Summary.** The core units library ships SI base units, common
SI-derived units via derived-unit algebra, standard affine
conversions between equivalent spellings, and dimensionless-ratio
handling. Domain-specific unit libraries (ecophysiology, chemistry,
finance) stay out of core and ship as distributable packages on top.

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

**Summary.** Section 30 commits only the stdlib function surface for
linear algebra (cholesky, lu, qr, svd, eigen, solve, inverse, det);
the underlying type-layer design lives in §3.9 pending chunk 05. Each
primitive wraps backend kernels and is opaque at the e-graph layer,
with invariants declared by capability contract.

Chunk 05 (B5 heterogeneous-unit resolution) is the design venue for
the underlying type layer; this section commits only the stdlib
function surface. Type content (structural subtypes, shape refinements,
envelope interaction) lives in §3.9 per the chunk 05 scope decision.

The matrix / tensor stdlib ships the linear-algebra primitives that
the rest of the spec depends on by name, in particular the Cholesky
factorization used in MVN reparameterization (§13.6, Z10) and the
kernel Gram-matrix machinery (§28). Committed primitives:

- `cholesky(A)`. Lower-triangular factor `L` such that `L · Lᵀ = A`
  for `A: Matrix<_, PositiveDefinite>`. Returns `Matrix<_, LowerTriangular>`.
- `lu(A)`. `(L, U, P)` with `P · A = L · U`, for square invertible `A`.
- `qr(A)`. `(Q, R)` with `A = Q · R`, `Q` orthogonal, `R` upper
  triangular. Works on rectangular `A` (`m × n`, `m ≥ n`).
- `svd(A)`. `(U, Σ, Vᵀ)` with `A = U · Σ · Vᵀ`, `Σ` diagonal with
  nonnegative entries. Works on general rectangular `A`.
- `eigen(A)`. Eigenvalue / eigenvector pair for square `A`. Real-
  symmetric specialization returns real eigenvalues and orthonormal
  eigenvectors; general case defers to complex eigenvalues pending
  §26.1 `Complex` lock.
- `solve(A, b)`. Linear solve for `A · x = b`. Dispatches on the
  structural subtype of `A` (triangular solve, Cholesky back-
  substitution, general LU) via the §3.9 lattice.
- `inverse(A)`. Direct inversion for documentation and small cases;
  the compiler rewrites `inverse(A) · b` to `solve(A, b)` by default
  to avoid explicit inversion in numeric code.
- `det(A)`. Determinant. On `Matrix<_, Triangular>` this reduces to
  diagonal product; on general `A` it routes through LU.

Each primitive carries a capability contract that records what its
output satisfies structurally (see §3.9). The primitives are opaque
at the e-graph layer: their invariants are declared by contract, not
derived from body composition, because they wrap backend linear-
algebra kernels (BLAS / LAPACK / cuBLAS equivalents via the Part V
backend trait).

---

## Part V — Backend Abstraction (STUB)

**Summary.** Part V specifies the abstraction by which Myco compiles
plans against a trait surface (numerical execution, AD, PPL handoff,
opaque-callable runtime, capability advertising) rather than a
specific runtime. Specific trait signatures and open forks land in
chunk 06; this part is normative in scope only.

Pending chunk 06 design completion. Specific trait shape and open forks
tracked separately; this part is normative in scope only.

### 31. Backend Trait Surface

**Summary.** The backend is a trait the compiler targets: numerical
execution, automatic differentiation, PPL handoff, opaque-callable
runtime, plus capability advertising. Multiple backends (Rust tensor
stacks, JAX-alikes, CPU reference implementations) satisfy the trait
symmetrically; the workflow selects a concrete backend at run time.

The backend is an abstraction: Myco compiles plans against a trait
surface, not a specific runtime. Multiple backends can satisfy the
trait (burn-style tensor stacks, JAX-alikes, CPU reference
implementations). The compiler emits against the trait; the workflow
selects a concrete backend at run time (§24 verbs).

The minimum trait API covers four responsibilities, numerical
execution, automatic differentiation, PPL handoff, and opaque-
callable runtime, plus a capability-advertising mechanism that lets
the compiler and workflow negotiate what a particular backend
supports. The subsections below commit the shape; concrete signatures
land in chunk 06.

#### 31.1 Capability Advertising and Fallback Modes

**Summary.** Backends advertise capabilities (complex support,
forward AD, HMC, sparse matmul) and the compiler verifies required
ones at plan-binding time. Three fallback modes handle mismatches:
`error` (fail), `host` (route to host-side reference), `emulate`
(substitute approximate algorithm and enter approximation-error
layer). Fallback is per-run via `run.config.backend`.

Backends advertise capabilities (e.g. `supports_complex`,
`supports_forward_ad`, `supports_hamiltonian_monte_carlo`,
`supports_sparse_matmul`) and the compiler verifies required
capabilities at plan-binding time. When a required capability is
missing, the compiler consults the workflow's fallback policy:

- **`error`.** Fail at plan-binding time with a capability-mismatch
  diagnostic (workflow-composition error tier, §19.4). Conservative
  default.
- **`host`.** Route the offending subgraph to a host-side reference
  implementation. Correctness preserved at the cost of device-host
  traffic. Useful for CPU-only families (e.g. `Rational` arithmetic,
  §26).
- **`emulate`.** Substitute an approximate or slower algorithm that
  the backend does support (e.g. dense solve in place of a missing
  sparse solve, finite-difference AD in place of missing forward AD).
  The substitution enters the approximation-error layer (§16 adjacent
  keyed state) so its effect on guarantees is tracked.

Fallback mode is set per-run via `run.config.backend` (§24.5);
workflows can also scope fallback to specific capabilities.

#### 31.2 PPL Handoff Protocol

**Summary.** Tier C stochastic SCCs ship to the backend's PPL as
opaque log-density problems via a protocol (not a library call).
Serialized form: log-density callable, parameter shape and bounds,
observation data, inference kind. The backend returns samples plus
diagnostics; returned samples carry no parametric envelope facts
and are treated downstream as opaque draws.

Tier C stochastic SCCs (§13.2) ship to the backend's PPL handler
as opaque log-density problems. The handoff is a protocol, not a
library call: the backend receives a sampling / inference task
described by a standard serialized form (log-density callable,
parameter shape and bounds, observation data, inference kind: MCMC,
VI, importance, etc.), runs inference with backend-native machinery,
and returns samples plus diagnostics. Samples come back without
envelope facts about the parametric form (§13 recommits this);
downstream code treats them as opaque draws.

#### 31.3 Opaque-Callable Runtime

**Summary.** The backend supplies the runtime that calls back into
Python during simulation for `bind_controller` callables, threads
gradients through Python for training emission, and manages memory
and device-residency for interop. The compiler sees only the
callable's advertised input and output contract, not its interior.

`bind_controller` (§24.1) hands the compiler a Python callable (a
learned function, typically a neural network). The backend provides
the runtime that calls back into Python-land during simulation,
threads gradients back through Python for training emission (§25),
and manages any memory / device-residency needed for the interop.
The opaque-callable runtime sits at the backend ↔ workflow boundary;
the compiler does not see the callable's interior, only its advertised
input / output contract.

#### 31.4 Backend Versioning

**Summary.** Backend implementations version on their own cadence;
the trait surface is versioned by Myco. Plans bind a specific trait
version; compatible backends advertise which versions they implement.
Breaking trait changes require a major bump. The plan cache keys on
`(plan, trait_version, backend_identity)` so upgrades invalidate
correctly.

Backend implementations are versioned on their own cadence. The trait
surface is versioned by Myco. A given plan binds against a specific
trait-surface version; compatible backend versions advertise which
trait versions they implement. Breaking changes to the trait surface
are rare and require a major-version bump; within a trait version,
backend implementations can evolve freely. The plan cache (§20)
keys on `(plan, trait_version, backend_identity)` so upgrading
backends invalidates the cache correctly.

#### 31.5 Stochastic E-Class Serialization

**Summary.** Tier C handoff serializes stochastic e-classes across
the trait boundary: e-class identity, envelope parametric metadata
(family, parameters, shape), layer-1 equational term, capability
requirements, observation constraints. The compiler owns
serialization; backends own deserialization and any backend-specific
canonicalization post-receipt.

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

**Summary.** Myco does not privilege any backend: Rust tensor
stacks, JAX-alikes, PyTorch-alikes, and CPU reference implementations
are all first-class against the trait. Capability advertising lets
each backend declare honestly what it supports; the workflow selects
the concrete backend via `run.config.backend`.

Myco does not privilege any single backend. The trait-surface design
treats backends symmetrically: a burn-style Rust tensor stack, a
JAX-alike, a PyTorch-alike, and CPU reference implementations are
all first-class. The compiler emits trait-targeting code; capability
advertising (§31.1) lets each backend declare what it supports
honestly, and the workflow-side `run.config.backend` selects which
one a given run uses. Earlier design drafts privileged a specific
Python ecosystem backend; the current design retires that framing
in favor of the trait-based approach.

### 32. Open Backend Items

**Summary.** Open items in the backend design: AD ownership (Myco-
owned, backend-delegate, hybrid; leans hybrid), PPL protocol
specifics (message schema, inference-kind enumeration), gradient-
flow semantics for `bind_controller`, the mixed-backend policy, and
the first concrete backend choice.

AD ownership fork (Myco-owned / backend-delegate / hybrid, leans
hybrid). PPL protocol specifics (message schema, inference-kind
enumeration). Gradient-flow semantics for `bind_controller`
callables.

#### 32.1 Mixed-Backend Policy

**Summary.** Whether a single run can span multiple backends is
open. Current lean is single-backend-per-run: if a workflow needs
specialized handling for one SCC, the intended escape hatch is
workflow-layer glue rather than cross-backend marshalling in the
compiler. Not yet locked; chunk 06.

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

**Summary.** Which backend ships first (Rust tensor stack, NumPy
reference, JAX-alike) is open. Affects ergonomics of the first
end-to-end demos but does not change the trait-surface design.

Which backend is implemented first, a burn-style Rust tensor stack,
a NumPy reference implementation, or a JAX-alike. Open. Affects
ergonomics of the first end-to-end demos but does not change the
trait-surface design, since the trait is backend-agnostic by
construction.

---

## Part VI — Known Open Items

**Summary.** Part VI enumerates open design items (B-tagged blockers,
chunk-slotted work, and a catalog of smaller opens) carried forward
explicitly so they are not silently committed during consolidation.
Covers matrix heterogeneous-unit resolution, backend AD ownership,
joint distribution syntax, residual-graph mechanics, and more.

Carried forward explicitly so they are not silently committed during
consolidation.

### 33. Design Blockers

**Summary.** Five named B-blockers remain open: B1 opaque log_pdf
policy, B2 joint declaration syntax, B4 coupling machinery, B5
matrix heterogeneous-unit resolution, B6 backend abstraction.

- **B1.** Opaque log_pdf stdlib policy.
- **B2.** Joint declaration syntax.
- **B4.** Coupling machinery.
- **B5.** Matrix heterogeneous-unit resolution.
- **B6.** Backend abstraction (see Part V).

### 34. Chunk-Slotted Work

**Summary.** Outstanding design chunks: chunk 05 matrix details,
chunk 06 backend abstraction, chunk 07 type-graph to e-graph bridge,
chunk 08 joint syntax and coupling, chunk 03 kernels (resumes after
substrate lock), chunk 11 sum types / enums.

- **Chunk 05.** Matrix details (heterogeneous units, envelope flavors,
  subtype lattice, shape refinements, scalar reconciliation).
- **Chunk 06.** Backend abstraction.
- **Chunk 07.** Type-graph ↔ e-graph bridge.
- **Chunk 08.** B2 + B4 joint syntax / coupling; user-`fn` ban and
  parameterized-relation lock (design resolved, §6 / §7 / §8 prose
  pending application). Canonical reference:
  `planning/v2/v2.1_chunk_reports/08_relation_fix_whoops.md`.
- **Chunk 03.** Kernels, resume after substrate lock.
- **Chunk 11.** Sum types / enums. Motivation and shape locked
  (§3.10 stub); exact syntax, pattern-matching power, event-
  triggered variant transitions, lifted-arithmetic sugar, and
  workflow binding surface open. Resolves the Mode B open in §35
  and the number-or-distribution materialization question.
- **Chunk 12.** Cost-field struct unification across `cost_of`
  (§14), `loss_of` (§14.2), and the §19.1 extraction cost vector.
  Three divergent field sets, no cross-reference today. Subsumes
  the §35 "Memory as a `cost_of` field" open. Canonical reference:
  `planning/v2/v2.1_chunk_reports/12_cost_field_unification.md`.

### 35. Other Opens

**Summary.** Catalog of smaller open items: `replaces` obligation
retraction under monotonicity, residual-to-e-graph mechanics, CC1
diagnostics, GPU-incompatibility of exact numeric types, chunk 04
carryovers (per-residual loss, heterogeneous `argmax`, event-driven
topology, spatial operator lowering), Complex contracts, controller-
interface affordances, and Tier 2/Tier 3 distribution machinery.

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
visibility). Macros (dropped from the current surface; revisit if
concrete boilerplate pain emerges). `softmax` and weighted-sum
aggregation surface (stdlib primitive vs user-composed from `exp` +
`sum`; collection-aggregation syntax pending zip/alignment semantics
lock; Y2 `soft_select` already uses softmax internally in §8.7, so
the shape is known but the ergonomic surface is not).

**Complex numeric representation in scope.** Riley-confirmed that
`Complex` ships; open items are which contracts it satisfies (not
totally ordered, so `Compare`-dependent stdlib functions exclude it;
which of `Plus` / `Minus` / `Times` / `Divide` close; interaction
with units in `Scalar<U, Complex>`), backend support commitments,
and whether `Complex` forms a separate `Numeric` sub-hierarchy or
lives alongside `Float`.

**Controller-interface affordances in the Python layer.** General-
system question: what hooks does Myco need to expose so workflows
can cleanly implement patterns like taxonomic embeddings, context
injection, per-category modulation, FiLM-style conditioning? Not
FiLM specifically; the meta-question of which controller-binding
surfaces belong in the language / stdlib vs which are workflow
idioms the user builds on their own. The goal is to avoid baking
project-specific patterns into the language while still exposing
enough machinery that workflow authors can implement them cleanly
against `bind_controller` (§24.1).

**Tier 2 distribution machinery.** Joint-declaration syntax (B2),
coupling / correlated-sample machinery (B4), copulas, Wishart /
InverseWishart / LKJ (gated on B5), higher-order distribution
routing through kernels. In scope for the current design envelope
but not yet locked; chunk 08 is the intended design venue. The
multivariate subset that admits factorization or closed-form
reparameterization (MVN, Dirichlet, Multinomial) already ships in
Tier 1 so this open does not block the common cases.

**Tier 3 distribution machinery.** Non-parametric and process-valued
families (Gaussian Process, Dirichlet Process, Chinese Restaurant
Process, Pitman-Yor, Indian Buffet Process, Beta Process). Open
question whether these share §28 kernel routing (likely for GPs),
require a distinct process-family mechanism, or are treated as
Tier C opaque PPL handoff. No formal tier boundary drawn; design
not yet scoped to a chunk.

**Memory as a `cost_of` field.** The `cost_of(expr)` extraction-cost
struct (§14, §19.1) carries the five O2.4 fields `compute`,
`approximation`, `condition`, `truncation`, `discretization`. An
earlier §19.1 draft listed `memory` as a separate dimension; O2.4
dropped it. Open: is peak allocation a first-class sixth field of
`cost_of`, or a backend-specific annotation exposed through a
separate surface? Informs whether `run.config.extraction_policy`
weights a six-field or five-field vector.

**CC5 site-gated strict rewrites: data path resolved.** CC5 locks
both category and data path for identify-seam merges and pole
L'Hopital rewrites. Category (locked 2026-04-20): site-gated strict
(Appendix C X). Data path (locked 2026-04-22): Layer-3 adjacent
keyed state (§16.1) mediates firing. Appendix C X splits into X1
(pole L'Hopital / removable-singularity operator substitution) and
X2 (identify / quotient-induced value equality). A geometry-body
`identify coord_a <-> coord_b` declaration produces a Layer-3 site
record keyed on the locus path (e.g., `seam@SphereSurface.azimuth`)
carrying the glue map, site predicate, and declaration provenance.
X2 consults the record: for field expressions on the geometry
whose coordinates match the predicate, X2 emits a Layer-1 merge
tagged with the site's identity. Cross-geometry pollution is
impossible by construction — a site record on one geometry cannot
be consulted for fields on another. Layer 3's role as dispatch
table for per-key state (§16.1) extends cleanly to site-keyed
records; no new layer mechanism introduced.

**Envelope-narrowing promotion: partition labeling.** A default-off
rewrite whose declared `error_bound` evaluates to zero over the
e-class envelope is promoted to default-on for that class (§16.3,
§17.6). Open: for cost accounting and diagnostics, does the promoted
rewrite move into the default-on bucket (so its
`cost_of().approximation` contribution drops to zero and it
disappears from the approximation-class ledger), or does it stay in
the default-off bucket with a fire-unconditionally-in-this-context
flag? The first is cleaner algebraically; the second preserves the
bookkeeping trail for a reader looking up a `Float64 -> Float32`
conversion in the approximation ledger. Affects §19.1 extraction-
cost accounting and §22 `explain` surfaces.

**Approximation cost composition.** Two lossy-model rewrites applied
within the same extracted plan are not in general independent — they
may reinforce, partially cancel, or compose non-linearly. Current
§19.1 draft implicitly sums `approximation` contributions. Open
whether conservative summation is the locked policy (sound but loose
bound), whether a richer algebra is needed for cases where stdlib
rewrites carry known non-independence annotations, or whether the
extractor should surface a warning when multiple lossy rewrites stack
on the same expression. Affects §17 rewrite-rule cost annotation
schema and §19.1 extraction-cost accounting.

**Condition cost representation for multi-output operations.** The
`condition` field of `cost_of` is scalar-valued in O2.4. Matrix
solves, eigenproblems, and other multi-output operations carry
richer conditioning structure than a scalar captures — the §17.1
tolerance classes (entry-wise, operator-norm, spectral, structural)
are the right shape. Open whether `condition` stays scalar and the
extra structure is recorded out-of-band, or becomes a sum-type over
the tolerance classes, or splits into separate fields per class.
Affects §14 `cost_of` signature and §19.1 extraction-cost
accounting.

**Stdlib canonical inventory.** The set of stdlib atoms (fn) and
stdlib-shipped parameterized relations is referenced throughout the
spec but not enumerated in one place. Deferred to a dedicated chunk
that locks: the full list of axiomatic primitives (`exp`, `log`,
`sin`, `cos`, `sqrt`, arithmetic, `smooth_max`, etc.), the
classification of each surface (fn vs parameterized relation), the
capability contracts and abstract cost tags for each, and the
classification of distributions (`log_pdf` / `sample`) and kernels.
Cross-refs §6, §7, §13.8, §14, §28, §30.

**Mode B: per-instance heterogeneous contract binding.** Chunk 08
pins three modes for pluggable behavior: Mode A (concrete type),
Mode B (contract-typed field, heterogeneous across instances of a
population), Mode C (generic type parameter, homogeneous within a
type instantiation). Mode B is only usable if `.myco` has a
mechanism for declaring that different instances of the same
population can carry different contract implementations, since the
Python dumb-data layer cannot drive per-instance type dispatch
(chunk 09 principle). Resolution path: chunk 11 (sum types / enums,
§3.10 stub) introduces tagged unions as the core mechanism; a
contract-typed variant field inside an enum lets a population carry
mixed VC families or any other contract-bound heterogeneity, with
the compiler picking compile-time specialization when the
discriminant is static and a runtime discriminant-tagged kernel
when per-instance. Open items live in chunk 11: the exact syntax,
event-triggered variant transitions (FSM / life-stage dynamic
topology), workflow binding surface for enum-typed fields, and
whether v2.1 ships the full mechanism or a minimum viable subset.
Cross-refs chunk 08 (three modes), chunk 09 (dumb-data Python),
chunk 11 (sum types), §3.10 (enum stub), §7 (contracts), §12
(collections / populations).

**Package dependency story.** Vocabulary is locked (`spore` for
packages, `hypha` for the CLI, `myco.toml` manifest, `myco.lock`
lockfile) and the overall shape follows Cargo + uv conventions
(chunk 10). Resolver algorithm, version semantics (what counts as
a breaking change for a parameterized relation, a contract, or a
capability shift), feature model, build-script / codegen surface,
workspace ↔ Python interaction, cross-spore relation visibility
(`pub(crate)`-style private relations), registry story, and
platform / backend metadata in the manifest are all open. None of
this blocks the core language lock; full spec-level prose is
deferred post-v2.1 per chunk 10. Cross-refs §2, §36, §37.

**Event scheduling-policy Python API signature.** §10.1 commits to
the contract (a Python-side policy orders competing firings; three
stdlib policies ship: priority, random-with-seed, FIFO) but defers
the exact Python API signature to §24 (workflow verbs) since it is
a workflow-layer concern. Open: the canonical signature for custom
policies (e.g., `policy(pending_firings, state) -> List[Firing]`
vs. a class-based interface with explicit hook methods), how custom
policies interact with determinism and reproducibility guarantees,
and the exact menu of state the policy sees. Should be resolved
when §24 workflow verbs are fleshed out during Phase 1 batch 5
(§20-§24 audit).

---

## Part VII — Developer Experience (Deferred)

**Summary.** Part VII names developer-experience surfaces outside the
language and compiler proper: CLI, dependency management, editor
tooling, doc generation, agent/LLM integration. Deferred until Parts
I-IV lock; listed to keep the surfaces from being forgotten.

Outside the language and compiler proper, but on the roadmap. Deferred
until Parts I–IV are locked. Listed here so the surfaces aren't
forgotten during consolidation.

### 36. Command-Line Interface

**Summary.** The `myco` CLI spans compile, run, check, fmt, explain,
and related subcommands, with flag conventions, exit codes, and
output formats yet to lock.

The `myco` CLI: compile, run, check, fmt, explain, and related
subcommands. Flags, exit codes, output conventions.

### 37. Dependency Management and Package Registry

**Summary.** How `.myco` packages declare dependencies, resolve
versions, publish, and lock. Interacts with but stays distinct from
the Python workflow layer's package system.

How `.myco` packages declare dependencies on each other. Version
resolution. Package registry layout and publishing workflow. Lockfile
format. Interaction with the Python workflow layer's package system
(distinct but adjacent).

### 38. Editor Tooling

**Summary.** Editor-side surfaces: a language server (LSP), VS Code
extension, tree-sitter grammar, and the full syntax-highlighting,
diagnostics, hover, goto-definition, and refactoring affordances.

Language server (LSP). VS Code extension. Tree-sitter grammar. Syntax
highlighting, diagnostics, hover, goto-definition, refactoring
affordances.

### 39. Documentation Generation and Website

**Summary.** Docstring conventions, a doc generator for user-defined
types, contracts, events, and universals, and a website layout
covering language reference, tutorials, API docs, and examples.

Docstring conventions. Doc generator for user-defined types, contracts,
events, universals. Website layout: language reference, tutorials, API
docs, examples.

### 40. Agent / LLM Integration

**Summary.** Agent skills for writing, reviewing, and validating
`.myco` models, harness support for running Myco-aware agents, and
conventions (canonical examples, anti-patterns, diagnostic
interpretation) so LLMs can reason about the language correctly.

Agent skills for writing, reviewing, and validating `.myco` models.
Harness support for running Myco-aware agents. Conventions so LLMs can
reason about the language correctly (canonical examples, known
anti-patterns, diagnostic interpretation).

---

## Appendices

### Appendix A — Reserved Keywords and Syntactic Surface

**Summary.** Appendix A enumerates the reserved keyword surface of
`.myco`: declaration keywords, type-formers, body forms, the
stochastic operator, not-yet-assigned reservations, structural
punctuation, and stdlib-reserved identifiers. Additions to this list
are a breaking change to the parse surface.

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

The full list is normative as of the current lock. Additions are a
breaking change to the parse surface and follow the source-
language stability process (to be designed post-build).

### Appendix B — Grammar / EBNF Summary

**Summary.** Placeholder for the normative EBNF summary of the
`.myco` surface. Lands once the surface is stable enough to commit
to a grammar (production per construct across §2 through §14).

Open. A normative EBNF summary of the `.myco` surface will appear
here once the surface is stable enough to commit to a grammar.
The concrete form is a production per construct in §2-§14 (types,
values, units, functions, contracts, relations, constraints, events,
geometry, stdlib calls, workflow-boundary syntax). Placeholder for
a later pass.

### Appendix C — Rewrite Catalog (A–Z)

**Summary.** Appendix C is the concrete rule surface of the e-graph
rewrite system: 26 lettered groups (A through Z), each tagged with a
faithfulness class (strict, fuzzy-model, fuzzy-tolerance, one-way,
N-way extraction, forbidden, distribution-family) and an orientation
(bidi, uni). LOCKED rules ship now; OPEN rules pend a named design
item. Every rule routes through one of the eight §17 authorization
sources.

Enumerates the rewrite rules the compiler applies over the e-graph
substrate (§16, §17). Organized into 26 lettered groups. Each group
carries a faithfulness tag (strict / fuzzy-model / fuzzy-tolerance /
one-way / N-way extraction / forbidden / distribution-family) and an
orientation tag (bidi / uni). Rules marked **LOCKED** ship now. Rules
marked **OPEN** are in scope for the current design envelope but pend
resolution of a named design item. Cross-cutting flags (CC1-5) appear
in-line; see §0.1 for their normative disposition.

Authorization-source correspondence: the eight authorization sources
of §17.1 are canonical shapes; the A–Z catalog enumerates the concrete
rule surface. Every rule below routes through one of the eight
sources.

**Catalog closure.** The A–Z catalog is closed for v2.1. New rewrite
rules are not expressible in `.myco`; the compiler is not a
user-facing rewrite-authoring surface. Post-v2.1 extensibility lands
via a Rust-side plugin system invoked from workflow, not via new
in-language keywords or annotations. Additions to the catalog
proceed by spec edit and compiler release, on the same cadence as
stdlib atoms.

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

- V1. `observe(path, data)` attaches observed data as a layer-2
  envelope fact on `path`'s e-class (§13.8, §13.9); `log_pdf(data)`
  contributes to the training loss (§25). Not an equational merge:
  `path` is not rewritten to `data` in layer 1, and the same `path`
  elsewhere remains stochastic. Data is never rewritten by inferred
  constraints.

**W. Obligation retraction.** Deletion, not rewrite. OPEN (chunk 04
O4.1, cross-ref §8.10, §10.5, §15, §16, §35).

- W1. `relation X on locus replaces balance(axial_flux): ...` retracts
  the compiler-generated `balance(axial_flux)` at the named locus and
  substitutes the user equation

**X. Site-gated strict.** Strict/lossless but gated on a site or
geometric predicate, not value bounds. LOCKED (O4.2 resolved
2026-04-20; data path locked 2026-04-22 as Layer-3 mediated).

- X1. Pole L'Hopital (removable-singularity operator substitution).
  At any mesh node coinciding with a declared locus pole, rewrite
  `laplacian(f)` from naive `1/sin(θ)` form to the L'Hopital limit.
  Operator form changes; value equality is incidental. LOCKED.
- X2. Identify (quotient-induced value equality). A geometry-body
  `identify coord_a <-> coord_b` declaration installs a Layer-3 site
  record (§16.1) keyed on the locus path (e.g.,
  `seam@SphereSurface.azimuth`) carrying the glue map, site predicate,
  and declaration provenance. X2 consults the record: for field
  expressions on the geometry whose coordinates match the predicate,
  X2 emits a Layer-1 merge to the coordinate-translated counterpart,
  tagged with the site's identity. Cross-geometry pollution is
  impossible by construction (site records are owned by the
  geometry). Supports non-identity glue maps (Möbius-style orientation
  flips, lens-space identifications) via the record's glue-map field.
  LOCKED.

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

**Distribution-family rewrites (Tier A closed-form propagation).**

**Z. Distribution-family rewrites.** Analytical envelope propagation
through stochastic e-classes. Fires from capability contracts on Tier
1 families (§27.1). Each rule is strict in the distributional sense
(closed-form parametric image) and monotonic in envelope accumulation
(§16.2). Tier A analytical-first ordering (§13 preamble) depends on
this group.

- Z1. Affine closure: `a * X + b` for `X ~ D` with `D :
  AffineSelfClosed` produces another `D`-draw with analytically
  derived parameters. LOCKED. Normal, Cauchy, MVN satisfy.
- Z5. Exp/log transform: `X ~ Normal(μ, σ)` → `exp(X) ~
  LogNormal(μ, σ)` and the inverse. LOCKED.
- Z10. MVN Cholesky reparameterization: `X ~ MultivariateNormal(μ,
  Σ)` rewrites to `X = μ + L @ ε` with `L L^T = Σ` and `ε ~
  Normal(0, I)` (§13.6). LOCKED.
- Z11. Pushforward under invertible differentiable map: for
  `f : Scalar<U_X> -> Scalar<U_Y>` satisfying `Invertible + Differentiable`
  (both advertised via stdlib capability contracts; §7.2, §17.3)
  and `X ~ D_X`, the image `Y = f(X)` carries a distributional
  envelope fact computed by change-of-variables:
  `log_pdf_Y(y) = log_pdf_X(f⁻¹(y)) - log |det J_f(f⁻¹(y))|`.
  Produces a `Distribution<U_Y>` envelope fact on `Y`'s e-class
  without routing to Tier B/C when the Jacobian determinant
  simplifies symbolically. Falls through to Tier B (delta method
  via `SmoothTransformable`) when the Jacobian does not simplify,
  and to Tier C when neither route applies. LOCKED. Bridges
  invertibility machinery (§17.3) to distributional envelope
  machinery without a new mechanism.

Intermediate Z-numbers (Z2-Z4, Z6-Z9) are reserved for
conjugate-posterior rewrites (§27.3 catalog) and approximate
closures (Tier B: Delta, Fenton-Wilkinson, CLT, block-maxima →
GEV). The enumeration is closed for v2.1.

---

**Summary table by faithfulness × orientation.**

| faithfulness | bidi | uni | total |
|---|---|---|---|
| Strict | ~24 (A1-10, B1-2, C1-4, D1-3, E1-2, F1, G1-3, H1-2, I1) | ~5 (D4-5, X1, X2, J1 forbidden) | ~29 |
| Distribution-family | ~3 (Z1, Z5, Z10) | ~1 (Z11) | ~4 |
| Fuzzy-model | — | ~2 (L1-2) | 2 |
| Fuzzy-tolerance | ~7 (K1-3, M1-2, N1, Q1-2) | ~3 (O1, P1, M2) | ~10 |
| One-way (lossless uni) | — | ~11 (R1-3, S1-2, T1, U1-3, V1, W1) | ~11 |
| N-way extraction | — | ~6 (Y1-6) | 6 |
| Forbidden | 1 (J1 temporal) | — | 1 |

Grand total approximately 63 rules, depending on sub-rule counting
and on how many Z-slots (Z2-Z4, Z6-Z9) the v2.1 conjugate-posterior
enumeration ultimately occupies.

**Cross-cutting items (flags, not rewrites).** CC1-5 are absorbed
into normative spec text: CC1 literal-numerics (§4, §4.1), CC2 sanity
inverses (§5.2 round-trip), CC3 per-residual training emission (§20;
open as O4.3), CC4 stochastic `~` rewrite blank (§13.8 resolved
2026-04-20), CC5 site-gated strict rewrites (§17, Appendix C X):
category and data path resolved 2026-04-22 — X1 pole L'Hopital
(removable-singularity operator substitution) and X2 identify
(quotient-induced value equality), site-indexed via Layer-3 adjacent
keyed state with provenance tagging; cross-geometry pollution
impossible by construction.

---
