# Myco — Specification

**Status.** WORKING DRAFT. This document is the current consolidation
target for the Myco specification. Some sections are still stubs or
carry explicit open items, but normative prose in this file supersedes
older planning drafts unless an open item says otherwise.

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
(§20). A conserved group is a compiler property the compiler
enforces. No user annotation suppresses the check; a model that would
otherwise violate the group must satisfy the emitted obligation
explicitly or state the relevant constraint (§8.1).

**Referential truth.** Principle 5, expanded. The monotonicity
machinery lives in §15 (the equational core), §8.11 / §10.5
(obligation fulfillment), and §16 (adjacent keyed state with its own
monotonicity rules).

**Downward-only cross-scale visibility.** Composite types see their
components. Components do not see their composite. A `Forest`
containing `Tree` entities can read per-tree state. A `Tree` cannot
inquire about the `Forest` it belongs to. Cross-scale coupling uses
explicit composition (§3.3). Inheritance is not in the language.

**Traceability and provenance.** Every e-class merge, rewrite
application, and workflow-injected value carries a provenance record
accessible via `hypha explain` (§22). Workflow-value injections
(§17) are tagged separately from compiler rewrites, which are tagged
separately from user-declared equalities. Observations (§13.9) are
layer-2 facts with their own tag. Provenance is durable across plan
serialization.

**Error-reporting philosophy.** Diagnostics split into three tiers
by where the problem surfaces. `hypha` compile errors catch type,
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
shape, or when a workflow source's unit disagrees with its
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
side and uses the source model of §24.

**Conversion-graph semantics / cost boundary.** Conversion legality
and conversion realization are separate. Unit conversions, tensor
reshapes, sparse or dense materialization, and structural-refinement
widenings enter the type graph as semantic edges with witnesses,
faithfulness, and obligations; extraction / lowering records whether
a legal edge realizes as a view, copy, kernel, host route, backend
materialization, or other costed plan. A legal conversion may still be
expensive or unsupported for a selected backend; an illegal conversion
never becomes legal because it is cheap.

**Projection-free compiler.** The compiler does not auto-emit
projection operators or solver selection to satisfy a constraint.
`constraint` declarations (§8.1) carry three explicit discharge
paths: compile-time proof via e-graph and refinement reasoning,
runtime projection selected by the workflow via §25's projection-
flavor policy, or training-objective penalty on SCCs classified training
(§20). The compiler surfaces which discharge path each constraint
uses, and the workflow picks among projection flavors when that
path applies. The compiler does not insert projection silently.
This keeps constraint-satisfaction a named modeler decision, never
an implicit compiler behavior.

**Artifacts and generated code.** Four artifacts have distinct roles.
The source bundle is `.myco` plus workflow Python. The plan is the
canonical, serializable IR emitted by the compiler. Backend-emitted
code is the generated product users may inspect, own, and run. A run
record is a plan plus concrete sources, evidence, backend, seed, and
version data. The source bundle reproduces the plan under a fixed
compiler version (§31.4); the plan emits backend code; the run record
reproduces output. Inspection affordances (§22 `hypha explain`, plan
serialization, provenance records) let users audit the compiled
output and the choices the compiler made.

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
`observe`, `regime boundary`, `regime surface`, `crossing policy`,
`relaxation`, `catalog`, `catalog entry`, `node path`, `facet path`,
`selector`, `existence domain`, `adapter`, `residual site`,
`residual realization`, `selected handle`, `Selected<T>`,
`SelectedSite`, `TopologyDelta`, `TopologyVersion`, `capability profile`,
`resolved run lock`, `SpatialOperatorSite`, `WeakFormSite`,
`ResidualFormSite`, `TransferSite`, `DiscreteOperatorSite`,
`realization provider`, `evidence grade`, `ConditionRecord`,
`promoted_exact_in_context`.

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
refinement lattice on matrices (Symmetric, PositiveDefinite, Diagonal,
Triangular, Orthogonal).

#### 3.1 Universal Declarations

**Summary.** Module-scope typed names (`universal R: Scalar<J_mol_K>`)
that every consumer in a run shares. Value comes from the workflow via
`Constant` or `Trainable` sources; CC1 forbids literals in `.myco`.

Module-scope typed names shared across all instances that reference
them. `universal R: Scalar<J_mol_K>` declares a name with a type; the
value is supplied by the workflow via `bind(path, Constant(...))` or
`bind(path, Trainable(...))`. CC1: no literal value in `.myco`. Semantics:
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
require an explicit `convert` to share scope. Named-type arithmetic
follows the Appendix C name-preservation / stripping rules; cross-
wrapper arithmetic without a stdlib rule or explicit `convert` is
rejected.

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

Named semantic quantity types preserve their name only through
stdlib-declared name-preserving operations. Same-named addition is
name-preserving; multiplication, division, affine subtraction, and
cross-wrapper arithmetic strip to the underlying unit result or require
an explicit `convert` (§17 Appendix C groups D/U). This is the pattern
used by stdlib angle quantities: `Angle` and `Phase` are nominal
wrappers over `Scalar<rad>`, not new unit syntax.

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

**Summary.** Generic parameters are invariant by default. Parameter
relationships can authorize explicit conversions, rewrites,
obligations, or dispatch, but they do not silently substitute one
instantiated type for another.

Generic parameters are invariant by default. `Tensor<m, shape>` is not
silently a subtype of `Tensor<Length, shape>`; `Tensor<U, (3,)>` is
not silently a subtype of `Tensor<U, (1, 3)>`; `Collection<some T>` is
not a hidden `Collection<T, N>`; and `impl Contract` does not erase to
a runtime trait object. Relationships across parameters must be named
by the relevant system: unit conversion edges, shape / index
bijections, refinement facts with evidence, contract satisfaction and
monomorphization, or `ShapePhase` facts for runtime sizing.

Parameter relationships may discharge rewrite guards, select lowering
or dispatch paths, or produce obligations / diagnostics. They do not
silently change the value's instantiated type.

Scalar-value generics. A generic parameter may itself be a typed
scalar (e.g., `LOW: Scalar<U>`) rather than a type. Scalar-value
generics participate in refinements and shape-tuple positions and
are bound at compile time through the same workflow source model as
ordinary universals (§4 exception classes cover their declaration
sites).

#### 3.7 Conservation Groups

**Summary.** `type Mass : Scalar<kg> { conserved }` marks a parent
type whose named-type children share conservation semantics. Cross-
sibling arithmetic is forbidden without explicit convert; destructive
events must satisfy conservation-route obligations; `diverg()` on
conserved flux fields emits junction flux-condition obligations.

`type Mass : Scalar<kg> { conserved }` marks a parent type whose
named-type children (e.g., `FishMass`, `DetritusMass`) share
conservation semantics. Consequences:

1. Cross-sibling arithmetic forbidden unless an explicit `convert`
   exists.
2. Bare `convert FishMass <-> DetritusMass` permitted between siblings
   (relabel only, no conversion body).
3. Events that create, destroy, split, merge, or move instances must
   satisfy `event_conservation_route(<group>)` obligations for affected
   conserved fields; unaccounted mass is a compile or workflow
   composition error (§10.5).
4. Compiler emits `flux_condition(flux_field)` obligation sites from
   `diverg()` usage on conserved flux fields (§11). A package may
   provide the default candidate `balance_zero(flux_field)`; users
   write explicit `fulfills flux_condition(flux_field)` relations or
   temporal blocks when the junction leaks, stores, or otherwise
   follows a nonzero balance law.
5. Bare-convert sibling merges create magnitude equivalence in the
   e-graph (§17 merge source — named-type conversion).

Tier 2 sub-questions deferred: scoped conservation and field-level
conservation. Boundary and junction interactions use the obligation
ledger (§8.11, §11.2, §11.8).

#### 3.8 Scalar, Tensor, and Shape Expressions

**Summary.** `Scalar<U>` is the normative source spelling for rank-0
tensor values and elaborates to `Tensor<U, ()>`. `Vector` and
`Matrix` are rank-refined tensor aliases. Collections relate to
tensor axes only through explicit extraction. `convert` handles
meaning-preserving tensor isomorphisms / materializations /
widenings, while backend layout remains a backend fact. Shapes are
structural expressions over dimensions, axes, products, partitions,
and provider-bound quantities. They may appear in type parameters,
refinement predicates, stdlib primitive contracts, and diagnostics;
they are not runtime model values. The spec defines the broad
shape-expression AST now, while the guaranteed solver subset is
intentionally staged.

`Scalar<U>` is formally sugar for `Tensor<U, ()>`. Users should write
`Scalar<U>` for ordinary rank-0 quantities; compiler internals record
one unified tensor substrate:

```text
Scalar<U> := Tensor<U, ()>
Vector<U, n> := Tensor<U, (n,)>
Matrix<U, m, n> := Tensor<U, (m, n)>

shape(scalar_value) = ()
rank(shape(scalar_value)) = 0
```

Diagnostics preserve the humane spelling: a temperature field is
reported as `Scalar<kelvin>` unless the diagnostic is specifically
about shape reasoning. There is no implicit `Scalar <-> Tensor0`
conversion edge because there are not two semantic types to convert
between. Unit envelopes, distribution facts, derivative facts,
tensor facts, and `convert` / `approximate` metadata all attach to
the same underlying value.

Shape expressions are compile-time / plan-time structural metadata.
They describe tensor extent and compatibility; they do not introduce
ordinary numeric values, stochastic quantities, or relation-level
unknowns. They may appear only in structural positions: type
parameters, `val` generic constraints, refinement predicates,
stdlib primitive contracts, and inspection / diagnostics.

The represented shape language has four layers:

- **`DimExpr`.** Dimension expressions: positive integer structural
  literals, `val` generics, axis lengths (`len(row_axes(A))`),
  provider-bound dimensions, arithmetic (`+`, `-` where nonnegative,
  `*`, exact division / divisibility), `min` / `max` where a stdlib
  primitive explicitly requires them, and named topology-derived
  counts such as `num_vertices(topology)`.
- **`ShapeExpr`.** Tuples of `DimExpr` plus shape transforms:
  indexing (`shape(A)[i]`), `rank(shape)`, `product(shape)`,
  `sum(shape)`, `transpose_shape(shape)`, `concat`, `slice`,
  `insert_axis`, `remove_axis`, `flatten`, `reshape_to`, and
  block partitions.
- **`ShapeConstraint`.** Equalities, inequalities / bounds,
  divisibility, product equality, matmul compatibility, reshape
  compatibility, broadcast / stack compatibility where the stdlib
  primitive defines that behavior, and block-partition compatibility.
- **`ShapePhase`.** Evidence for when a dimension is known:
  `static`, `provider_validated`, `runtime_bounded`, or
  `dynamic_unknown`.

The represented language is intentionally broader than the initial
solver. This keeps hard cases expressible before every inference rule
exists. v2.1's guaranteed automatic solver subset covers tuple
equality, rank, indexing, product equality, transpose, concat / stack,
and simple affine dimension expressions where variables match
syntactically. Represented but not necessarily automatically solved:
floor / exact-division formulas for convolution-like operators,
arbitrary nonlinear arithmetic, dynamic topology dimensions, ragged
row lengths, and general block algebra.

Shape facts reuse the §3.9 fact evidence statuses (`proven`,
`refuted`, `conditional`, `obligation`, `provider_validated`,
`backend_reported`, `unknown`). A primitive may require a particular
shape phase in addition to a shape equation. For example, a backend
that specializes code by static shape may require `static` dimensions,
while a runtime-sized backend may accept `provider_validated` or
`runtime_bounded` dimensions for the same mathematical operation.

Examples:

```text
shape(A) = (m, k)
shape(B) = (k, n)
shape(A * B) = (m, n)

shape(flatten(A)) = (product(shape(A)),)
product(shape(old)) = product(new_shape)

shape(blocked) = (m1 + m2, n1 + n2)
shape(stacked) = insert_axis(shape(item), axis=0, count=batch)
```

Dynamic topology uses shape phases rather than pretending runtime
sizes are static:

```text
shape(field_over_vertices) = (num_vertices(topology),)
phase(num_vertices(topology)) = provider_validated
```

Ragged or irregular structures usually surface as sparsity /
topology facts rather than dense tensor shapes: row nonzero counts,
offset-array shapes, index-array shapes, and zero-pattern facts (§3.9).

Collections (§12) and tensors are orthogonal primitives. A
`Collection<T>` is a homogeneous, unordered-or-keyed aggregation of
entities — membership, iteration, aggregation (§12.1). A `Tensor<U, S>`
is a shaped numerical object — multi-axis indexing, linear-algebra
primitives, structural facts / refinements (§3.9). The two do not
nest into each other by default: a collection of scalars is not
automatically a vector, and a tensor axis is not automatically a
collection.

Bridging is explicit through collection-axis extraction. A stdlib
extraction relation may gather a field from a collection into a
vector or matrix only when it names the entity ordering, axis
identity, field path, unit law, and missing / inactive-entry policy.
For example:

```text
collect_to_vector(leaves, key: leaf_id, field: temperature, out: temp_vec)

axis(temp_vec, 0) = leaves ordered by leaf_id
entry_unit(temp_vec[i]) = kelvin
provenance(temp_vec) = collected_from(leaves.temperature)
```

The extracted tensor is a numerical object with an axis identity
derived from the collection. The source collection remains an entity
aggregation with membership and iteration semantics. This
orthogonality keeps `for` / aggregation (§12) decoupled from matrix /
tensor operations while still letting models intentionally assemble
linear-algebra objects from entity state.

Dynamic topology and tensor shapes use regime-boundary semantics
(§8.10) plus `ShapePhase` evidence. Topology-changing events emit a
`TopologyDelta`; applying the delta to the current `TopologyVersion`
produces the next version. The runtime may apply that transition by
capacity masks, event-time replanning, dynamic keyed state, or a later
backend-specific handler, but the semantic object is always the
versioned topology, not in-place shape mutation inside an SCC solve:

- **`static`.** Shape known from source / generics.
- **`provider_validated`.** Shape known after workflow materializes a
  topology or dataset before planning.
- **`runtime_bounded`.** Fixed maximum shape with alive masks,
  zero-pattern facts, or capacity records. Tensor shape is stable;
  active set changes. This is the `CapacityMask` handler and requires
  backend capability for masked capacity arrays and any needed masked
  AD / sparse update operations.
- **`event_replan`.** A topology-changing event creates a new
  topology version and a new member of an SCC family. The executor
  stops at the event boundary, applies the topology delta, recomputes
  axes / facts / sparsity / obligations, and re-lowers or dispatches
  a cached plan for the new topology fingerprint. This is the
  `EventReplan` handler.
- **`dynamic_keyed`.** Axis sets are runtime maps keyed by entity IDs.
  This is the `DynamicKeyed` handler and is a valid Myco semantic mode
  for CPU / host execution and dynamic sparse runtimes. Compiled
  accelerator backends must advertise direct support or the workflow
  must explicitly authorize host / replan routing.
- **`dynamic_unknown`.** Shape is not sufficiently bounded or keyed
  for a selected backend / handler. Planning reports an unmet
  obligation or asks the workflow to choose a crossing policy (§24.6).

An SCC may not silently change tensor shape in the middle of one
solve step. Shape-changing events cross a regime boundary and must
use one of the explicit handlers above (`CapacityMask`,
`EventReplan`, `DynamicKeyed`, or a later backend-specific handler).
Incremental saturation and plan-cache reuse are implementation
optimizations only; they must produce the same symbolic plan as a
fresh compile from the source bundle, workflow intent config, and
event history. CPU reference execution is semantics-complete for
`dynamic_keyed`; JIT / GPU backends advertise which modes they lower
without host fallback.

The `convert` facility (§5.1) extends to tensors only for
meaning-preserving isomorphisms, materializations, and widenings:

- **Reshape / flatten.** A tensor reshape is legal only when the
  shape solver proves equal cardinality and the conversion names an
  index bijection, either a stdlib canonical map or an explicit map
  in the conversion body. The bijection transports axes, entry-unit
  laws, zero-pattern facts, and provenance through the new shape.
  Equal element count alone is insufficient when axis identity or
  matrix provenance cannot be mapped.
- **Sparse / dense materialization.** Sparse-to-dense materializes
  known zeros and preserves the same mathematical object. Dense-to-
  sparse requires an explicit target pattern plus a proven or
  provider-validated `zero_pattern` fact for every entry outside the
  pattern. Thresholded sparsification or over-approximate sparsity is
  an `approximate` block, not `convert`.
- **Structural-refinement widening.** A conversion may forget facts
  without changing values, e.g. `Diagonal<U, n>` to
  `Symmetric<U, n>` or `PositiveDefinite<U, n>` to
  `PositiveSemiDefinite<U, n>`. Narrowing to a stronger refinement
  creates an obligation; it does not grant the fact.

Out of scope for `convert`: **numeric precision downcasts**
(authorized via `approximate`, §26.2), **storage-order / layout
changes** (`CSR`, `CSC`, row-major, column-major), **device
residency** (host ↔ GPU), and matrix role relabels. Those are
handled by approximation policy, backend / provider facts, or the
matrix fact engine (§3.9). The split keeps `convert` about meaning at
the type layer and leaves representation-level tuning to the backend
trait.

#### 3.9 Matrix Facts and Structural Refinements

**Summary.** Matrix intelligence is carried by compiler-facing graph
facts, not by user-marked matrix roles. Users write ordinary contracts,
relations, constraints, and workflow bindings; the compiler derives,
validates, or reports facts such as shape, axes, entry-unit laws,
symmetry, definiteness, rank, sparsity, conservation, construction
provenance, and backend representation. Stdlib primitives consume
required facts (§30). If a required fact is unknown, the result is an
unmet obligation with diagnostics, not a semantic fallback.

Matrix-shaped values remain ordinary tensors: `Tensor<U, shape>`,
with `Vector` and `Matrix` as shape-refined aliases (§3). Myco does
not add a separate user-facing ontology of matrix roles such as
`LinearMap<State, Residual>` or `Covariance<Obs>`. Those names may
exist as documentation vocabulary, stdlib relation patterns, or
inspection labels, but the source of truth is the graph fact set.

Pure field-set contracts already provide the axis-signature vocabulary
needed for heterogeneous matrix accounting:

```myco
contract Obs {
    temp: Scalar<kelvin>
    pressure: Scalar<pascal>
}
```

A matrix whose rows and columns are indexed by `Obs` can carry facts
such as `row_axes(A) = Obs`, `col_axes(A) = Obs`, and
`entry_unit(A[temp, pressure]) = kelvin * pascal`. No `basis`
declaration or role annotation is added.

Canonical matrix facts include:

- **Shape / axis / unit facts.** `rank(A)`, `shape(A)`,
  `square(A)`, `row_axes(A)`, `col_axes(A)`,
  `compatible_axes(A, x, b)`, `entry_unit(A[i,j])`,
  `entry_unit_law(A)`, `factorable_unit_law(A)`, and
  `dimensionless_under_scaling(A, scaling)`.
- **Construction / provenance facts.** `jacobian_of(J, residuals,
  variables)`, `hessian_of(H, scalar, variables)`,
  `gradient_of(g, scalar, variables)`, `covariance_of(Sigma, x)`,
  `precision_of(Lambda, x)`, `correlation_of(R, x)`,
  `gram_of(K, kernel, points)`, `metric_of(G, domain)`,
  `incidence_of(B, graph)`, `laplacian_of(L, graph_or_geometry)`,
  `mass_matrix_of(M, discretization)`, and
  `stiffness_matrix_of(K, operator)`.
- **Structural facts.** `symmetric(A)`, `skew_symmetric(A)`,
  `hermitian(A)`, `diagonal(A)`, `scalar_diagonal(A)`,
  `upper_triangular(A)`, `lower_triangular(A)`, `orthogonal(A)`,
  `unitary(A)`, `permutation(A)`, `projection(A)` (`A*A = A`),
  `involution(A)` (`A*A = I`), and `normal(A)`.
- **Definiteness / spectral facts.** `positive_definite(A)`,
  `positive_semidefinite(A)`, `negative_definite(A)`,
  `negative_semidefinite(A)`, `indefinite(A)`,
  `eigenvalue_bounds(A)`, `singular_value_bounds(A)`,
  `spectral_radius_bound(A)`, `condition_bound(A)`,
  `coercive(A)`, and `elliptic(A)`.
- **Rank / subspace facts.** `rank_value(A)`, `full_rank(A)`,
  `full_row_rank(A)`, `full_col_rank(A)`, `nullspace_dim(A)`,
  `left_nullspace_dim(A)`, `row_space(A)`, `col_space(A)`,
  `range_space(A)`, `kernel_basis(A)`, and
  `constraint_redundancy(A)`.
- **Sparsity / pattern facts.** `zero_pattern(A)`, `sparse(A)`,
  `banded(A, width)`, `block_sparse(A, blocks)`,
  `block_diagonal(A, blocks)`, `block_triangular(A, blocks)`,
  `tridiagonal(A)`, `stencil_pattern(A)`, `local_coupling(A)`,
  and `nearest_neighbor_coupling(A)`. These are mathematical or
  pattern facts, not storage formats.
- **Operator / conservation facts.** `self_adjoint(A)`,
  `adjoint_pair(A, B)`, `conservative_operator(A)`,
  `row_sum_zero(A)`, `col_sum_zero(A)`, `mass_preserving(A)`,
  `energy_preserving(A)`, `dissipative(A)`,
  `monotone_operator(A)`, `m_matrix(A)`, `graph_laplacian(A)`,
  `incidence_matrix(A)`, and `divergence_gradient_pair(D, G)`.
- **Positivity / stochastic facts.** `nonnegative_entries(A)`,
  `nonpositive_offdiagonal(A)`, `diagonally_dominant(A)`,
  `strictly_diagonally_dominant(A)`, `row_stochastic(A)`,
  `col_stochastic(A)`, `doubly_stochastic(A)`,
  `substochastic(A)`, and `markov_generator(A)`.
- **Numerical / approximation facts.** `discretization_order(A)`,
  `truncation_error_bound(A)`, `roundoff_sensitivity(A)`,
  `approximation_source(A)`, `mesh_dependent(A)`,
  `timestep_dependent(A)`, `scaling_policy(A)`, and
  `preconditioner_for(P, A)`.
- **Backend / representation facts.** `preferred_layout(A)`,
  `supports_csr(A)`, `supports_dense(A)`,
  `supports_block_sparse(A)`, `backend_kernel_available(op, A)`,
  `estimated_memory(A)`, and `estimated_compute(A)`. These facts
  belong to lowering / provider records, not source-language storage
  declarations.

Facts carry evidence status:

- `proven` — derived from Myco relations, e-graph rewrites, stdlib
  laws, or construction provenance.
- `refuted` — contradicted by graph facts or validation evidence.
- `conditional` — true under named preconditions.
- `obligation` — required by a constraint, refinement, or primitive
  use, but not yet discharged.
- `provider_validated` — checked against workflow-bound data at
  composition time.
- `backend_reported` — discovered by lowering, profiling, or backend
  capability inspection.
- `unknown` — expressible in the fact vocabulary but not established.

`constraint positive_definite(A)` creates an obligation; it does not
grant `positive_definite(A)`. The compiler may prove it, validate it
against a provider-bound matrix, preserve it as a runtime check where
the enclosing construct allows checks, or report it as unmet.

Operations consume facts. `cholesky(A)` consumes `square(A)`,
`symmetric(A)` or `hermitian(A)`, `positive_definite(A)`, and
`factorable_unit_law(A)`. If any required fact is unknown, planning
reports the missing fact and does not silently choose a different
semantic path.

Matrix structural properties are therefore facts/refinements rather
than closed enum cases. The operative model is an implication lattice
over compiler facts, not a finite taxonomy of matrix kinds. A fact is
below another fact when the first entails the second under named
domain assumptions. Meet combines compatible facts and normalizes
their consequences; join keeps only facts common to all alternatives,
or records conditional alternatives when the branch condition remains
visible.

Every matrix fact record carries:

- **Predicate and parameters.** For example `positive_definite(A)`,
  `banded(A, width)`, `zero_pattern(A, pattern)`, or
  `entry_unit_law(A, law)`.
- **Domain.** Real vs complex scalar setting, square vs rectangular
  shape, axis identities, unit / scaling policy, and any construction
  preconditions that make the implication valid.
- **Evidence.** Relation provenance, e-graph rewrite, stdlib
  primitive contract, provider validation, backend capability report,
  or conditional proof.
- **Status.** One of the evidence states above (`proven`, `refuted`,
  `conditional`, `obligation`, `provider_validated`,
  `backend_reported`, `unknown`).

The stdlib may expose names such as `PositiveDefinite`,
`PositiveSemiDefinite`, `Symmetric`, `Diagonal`, `LowerTriangular`,
and `Orthogonal` as readable refinement names, but those names lower
to facts and obligations. They are not user-granted proof labels.
User-defined named refinements may bundle constraints, but satisfying
the refinement still requires derived, validated, or conditional
facts.

Core shipped entailments:

| fact or meet | entailed / normalized facts | notes |
|---|---|---|
| `positive_definite(A)` in the real-matrix setting | `square(A)`, `symmetric(A)`, `positive_semidefinite(A)`, `full_rank(A)`, `invertible(A)`, `lambda_min(A) > 0` | Myco's stdlib `positive_definite` fact is the symmetric / Hermitian linear-algebra notion, not a loose annotation. |
| `positive_semidefinite(A)` in the real-matrix setting | `square(A)`, `symmetric(A)`, `lambda_min(A) >= 0` | Does not imply invertibility or ordinary Cholesky eligibility. |
| `diagonal(A)` | `square(A)`, `upper_triangular(A)`, `lower_triangular(A)`, off-diagonal `zero_pattern(A)`, and `symmetric(A)` in the real setting | Rectangular diagonal-like tensors, if needed later, require a separate fact. |
| `scalar_diagonal(A)` | `diagonal(A)` and all diagonal entries equal | `identity(A)` further entails `positive_definite(A)`, `orthogonal(A)`, and unit-compatible inverse identity facts. |
| `upper_triangular(A) ∧ lower_triangular(A)` | `diagonal(A)` | The meet is not contradictory; it normalizes to the tighter fact. |
| `orthogonal(A)` | `square(A)`, `full_rank(A)`, `invertible(A)`, `inverse(A) = transpose(A)` | `condition_bound(A)=1` is emitted only when the relevant norm and scaling policy are established. |
| `permutation(A)` | `orthogonal(A)`, boolean / nonnegative entries, one-hot row and column patterns | Also emits sparse / zero-pattern facts useful for lowering. |
| `full_rank(A) ∧ square(A)` | `invertible(A)` | Rectangular `full_row_rank` and `full_col_rank` remain distinct and do not imply an inverse. |
| `transpose(A) * A` | `symmetric`, `positive_semidefinite`, and Gram/provenance facts when units and axes are compatible | Upgrades to `positive_definite` only with `full_col_rank(A)`. |
| `graph_laplacian(A) ∧ conservative_operator(A)` | `row_sum_zero(A)` and graph/discretization provenance | Symmetry, PSD, or M-matrix facts require the corresponding undirected / nonnegative / boundary-condition evidence. |

Contradictions are handled by the same fact engine. `positive_definite(A)`
and `singular(A)` on the same e-class refute each other. In the
real setting, `symmetric(A) ∧ skew_symmetric(A)` normalizes to a
zero-matrix fact; if the same e-class also requires
`positive_definite(A)`, the compiler reports an impossible obligation.
Unknown facts stay unknown. They do not authorize fallback lowering.

Primitive propagation is explicit and local:

- `transpose(A)` swaps row / column axes, transposes the
  `entry_unit_law`, flips upper / lower triangular facts, and
  preserves symmetry, diagonal, orthogonality, definiteness, and
  spectral bounds where the named rule applies.
- `A + B` requires compatible shape / axes / entry units. It preserves
  symmetry, diagonal, triangular direction, and shared zero patterns;
  it preserves PSD only by the cone rule when both operands are PSD
  over the same axes and scaling policy.
- `A * B` contracts axes and composes entry-unit laws. It preserves
  triangular direction for same-direction triangular operands and
  orthogonality for products of orthogonal matrices. It does not
  preserve positive definiteness unless a named congruence or
  product rule establishes the fact.
- `inverse(A)` consumes `invertible(A)`. It preserves triangular,
  diagonal, orthogonal, and positive-definite facts under their named
  inverse rules, and emits inverse unit laws.
- Factorizations consume facts and emit facts: `cholesky(A)` consumes
  positive definiteness and factorable units, emits
  `lower_triangular(L)`, `positive_diagonal(L)`, and the exact
  factorization identity.
- Spatial and graph lowerings emit provenance, pattern,
  conservation, and spectral facts only when the geometry,
  discretization, and boundary evidence prove them.

Dispatch rule: `solve(A, b)` with `lower_triangular(A)` calls
triangular substitution directly; `positive_definite(A)` routes
through Cholesky; `orthogonal(A)` uses `Aᵀ · b`. The compiler walks
the fact set to pick the tightest applicable specialization that
preserves the same semantics. If no required fact is established,
planning reports the unmet obligation instead of choosing a
different mathematical operation.

Chunk-05 closure: v2.1 commits this matrix / tensor layer as source
semantics. Backend execution, AD ownership, accelerator support,
layout / device selection, and runtime estimators are chunk-06
concerns; they do not reopen the type, fact, assembly, or obligation
model above.

#### 3.10 Sum Types / Enums

**Summary.** Tagged sum types (`enum`) are a core composite-type form
alongside newtype and record. They capture **structural
polymorphism** — a value that is one of several shapes — where
contracts capture **behavioral polymorphism**. Variants may be unit,
positional, or struct-like. Dispatch uses flat, exhaustive `match`;
there is no wildcard/default arm in the core surface, and enum-typed
values must be narrowed before variant fields are accessed. Ordinary
model bodies narrow with `match`; event bodies may also narrow with an
event `where ... is Variant` guard. The compiler picks compile-time
specialization (when the discriminant is static after workflow
binding) or a runtime
discriminant-tagged kernel (when dynamic). Enums compose with
contracts; variant fields may themselves be contract-typed. Stdlib
ships at least `Prior<S>` (fixed value or distribution over sample
type S), `Option<T>`, and `Result<T, E>`. Event-triggered variant
transitions use event-only `becomes` with full explicit construction
of the next variant. Workflow enum binding uses dumb-data tagged
records, with optional thin Python helpers that produce those records.
`Prior<S>` has no lifted arithmetic or `materialize` sugar in v2.1;
users write the exhaustive `match` explicitly.

Four independent pressures motivate enums as a single mechanism:
number-or-distribution materialization of the same model, Mode B
heterogeneous contract dispatch across a population (chunk 08,
chunk 09), finite state machines in dynamic topology, and
Option/Result at the workflow boundary. Contracts alone cannot
cover these cases without hiding the `~` operator from the PPL
machinery (§13) or collapsing structural differences that the
compiler needs to see.

Full design lives in `v2.1_chunk_reports/11_sum_types_enums.md`.

Declaration syntax:

```myco
enum Prior<S> {
    Fixed(S),
    Random(some Distribution<S>),
}

enum LifeStage {
    Seed { age: Scalar<days> },
    Seedling { age: Scalar<days>, height: Scalar<m> },
    Mature { age: Scalar<days>, height: Scalar<m>, dbh: Scalar<cm> },
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`match` is a body-form construct. It dispatches on one enum-typed
value, binds fields from the chosen variant, and each arm contributes
ordinary body statements such as equations, relation invocations,
constraints, or nested matches:

```myco
match stage {
    Seed { age } => {
        active = false
    }
    Seedling { age, height } => {
        active = true
        canopy_height = height
    }
    Mature { age, height, dbh } => {
        active = true
        canopy_height = height
        dbh_proxy = dbh * dbh
    }
}
```

Matches must be exhaustive over the enum's declared variants. Missing
variants are a type-check error. The core surface has no wildcard
arm, no default arm, no guards, no or-patterns, and no nested pattern
matching beyond destructuring the top-level variant. Those features
can be added later as sugar only if they preserve exhaustiveness and
diagnostic clarity.

`Option<T>` and `Result<T,E>` are ordinary enums. Payload access
requires explicit `match`; there is no implicit unwrap, default value,
or exception-like projection.

Enum fields are not projected implicitly. Outside an explicit
narrowing context, this is invalid even if some variants contain a
`height` field:

```myco
stage.height
```

The model must narrow first:

```myco
match stage {
    Seed { age } => { has_height = false }
    Seedling { height, age } => { has_height = true }
    Mature { height, dbh, age } => { has_height = true }
}
```

If many variants share a meaningful behavioral surface, the modeler
should express that behavior as a contract, or as a relation on the
enum that matches internally. Shared field names alone do not create
a structural projection surface.

Event-triggered variant transitions use `becomes` in event bodies:

```myco
event germinate(p: Plant where p.stage is Seed) {
    p.stage becomes Seedling {
        age: p.stage.age,
        height: germination_height,
    }
}
```

`becomes` is valid only in `event` bodies and always crosses a
regime boundary (§8.10, §10). The event's `where p.stage is Seed`
guard narrows `p.stage` inside the event body so old-variant fields
may be read while constructing the next variant. The new variant must
be fully constructed: every field required by `Seedling` must be
provided, preserved fields must be copied explicitly, and same-name
fields never carry over implicitly. Fields from the old variant that
are not copied leave scope in the next regime. Historical values must
be written explicitly into the new variant or into a separate event /
history record. `becomes` does not satisfy conservation obligations by
itself; ordinary variant transition is separate from `fulfills`.

Workflow binding for enum-typed fields uses catalog-validated tagged
records, not generated Python enum classes:

```python
workflow.bind("growth_rate", {"tag": "Fixed", "value": 0.03})
workflow.bind("stage", {"tag": "Seedling",
                         "fields": {"age": 12.0, "height": 0.08}})
```

Unit variants omit payload, positional variants use `value` (or an
ordered payload record if the variant has multiple positional
fields), and struct-like variants use `fields`. The Python library may
offer helpers such as `myco.variant("Fixed", value=...)`, but helpers
only produce the same dumb-data representation. They do not make
Python classes mirror `.myco` enum types.

`Prior<S>` is explicit-match-only in v2.1. There is no arithmetic
lifting through `Prior<S>` and no stdlib `materialize(prior, out)`
sugar:

```myco
match growth_rate {
    Fixed(r) => {
        d(height) = r * height
    }
    Random(dist) => {
        let r: Scalar<per_day>
        r ~ dist
        d(height) = r * height
    }
}
```

### 4. Values and Literal Numerics

**Summary.** Float literals and unit-qualified numeric literals are
banned in value position (CC1). Bare dimensionless integer literals
are legal via a stdlib desugar to the parametric universal family
`integer<N: val>: Scalar<dimensionless>`, whose default workflow
binding is the parameter itself. Unit definitions, affine conversion
bodies, and structural positions (shape tuples, indices, generic
parameters) remain exception positions for any literal form. All
numeric values enter through the workflow, via the universal-binding
mechanism.

Integer literal desugar. A bare integer literal `N` in dimensionless
value position parses to `integer<N>`, a reference into the stdlib
parametric universal `integer<N: val>: Scalar<dimensionless>`. The
default workflow binding for `integer<N>` is `N` itself, so a user
who never rebinds gets the natural value. Sensitivity analysis or
alternative bindings are available through the standard
universal-binding mechanism (`bind(path, Constant(...))`). See `spec_dev_notes.md`
for the derivation.

Exception positions accept literals directly. Unit definitions,
affine conversion bodies, and structural positions (shape tuples,
indices, generic-parameter definitions) are not value positions;
they are declarations about the type or shape of a quantity, and
CC1 does not apply to them.

Finite matrix assembly is not a CC1 exception. `matrix[[a, b]; [c,
d]]` is a source-level assembly expression from entry expressions
that are already legal graph values. CC1 is checked recursively
inside each entry: float and unit-qualified numeric entries are
rejected, while bare dimensionless integer entries follow the same
`integer<N>` desugar as any other value position. Example:

```myco
A = matrix[[a, b]; [c, d]]          # legal when a,b,c,d are graph values
B = matrix[[1.2, 0.1]; [0.1, 3.4]] # rejected: float literals in value position
```

Mathematical constants. π, e, and similar fixed reals are ordinary
stdlib-declared identifiers (`universal pi: Scalar<dimensionless>`,
`universal e: Scalar<dimensionless>`). They receive no CC1 carve-out:
they are universals like any other, and a workflow binds their
numeric values at compile time through the same mechanism as any
other constant. The stdlib ships default bindings so users do not
write them by hand, and the stdlib may ship additional well-known
dimensionless constants on the same basis. Extending the stdlib
universal catalog (new named constants, parametric families beyond
`integer<N>`) is a workflow-side or compiler-plugin concern, not a
source-language surface concern.

Workflow bindings enter the e-graph as equalities. A workflow
constant supplied at compile time merges a workflow-value-tagged
equality between the universal's e-class and a literal term in the
B2 rewrite layer (§17). Numeric values therefore participate in
rewriting and extraction without appearing in `.myco` source.
`integer<N>` participates on the same footing: the default binding
merges `integer<N>` with the literal `N`, so ring/field axioms
(Appendix C A-group) fire on the literal form after rewrite.

#### 4.1 CC1 Diagnostic Surface

**Summary.** CC1 violations surface as `hypha` compile errors with a
consistent shape: identify the literal, name why it is rejected
(float, unit-qualified, or out-of-position), and point to the
resolution (declare a universal, bind a `Constant` / `Series` source,
or remove the unit annotation for an integer
literal).

Rejection reasons. A float literal (any numeric token containing a
decimal point or scientific-notation exponent) in value position is
rejected: suggest lifting the value to a universal and binding it
from the workflow. A unit-qualified literal (`273.15 K`, `5 MPa`, `0
degC`) in value position is rejected: suggest either moving the
declaration to an affine convert body if that is its role, or
lifting the value to a universal. An integer literal written with a
unit suffix (`1 meter`) in value position is rejected on the same
grounds as the float case; the integer carve-out applies only to
dimensionless use.

Resolutions. The diagnostic points to the canonical resolution:
`bind(path, Constant(...))` or `bind(path, Series(...))` (§24) for a
universal-lifted value, or an affine-convert-body rewrite for a unit-qualified
magnitude that belongs in a conversion. The wording keeps CC1
enforcement actionable instead of cryptic.

### 5. Units

**Summary.** Base units, derived units, named dimensionless units,
affine conversions, dimensional algebra, and unit-generic types. The
`convert` declaration (four variants), round-trip verification, and
`value_in` operator are the modeler surface. Unit-normalization
rewrites live in the e-graph machinery (§17, Appendix C group C); §5
covers the declaration surface and the modeler-facing invariants.

#### 5.0 Unit System Fundamentals

**Summary.** `base_unit` introduces an orthogonal dimension axis.
`Scalar<U>` is the unit-parameterized rank-0 quantity spelling.
Derived units are products, quotients, and scalar multiples of
existing units. Internally, all computation uses base-SI magnitudes;
declared units are a presentation layer. No implicit unit inference:
every `Scalar<U>` must have its unit established syntactically or by
workflow binding.

A `base_unit` declaration introduces a new orthogonal axis in the
dimension exponent vector. Example:

```myco
base_unit meter
base_unit second
base_unit kilogram
```

`Scalar<U>` is the built-in source spelling for "a unit-bearing scalar
quantity measured in unit U"; internally it is the rank-0 tensor
`Tensor<U, ()>` (§3.8). Derived units are defined as products,
quotients, and scalar multiples of existing units:

```myco
unit meter_per_second = meter / second
unit pascal = kilogram / (meter * second^2)
```

Named dimensionless units are allowed when a dimensionless quantity
has important semantic intent. The core example is the SI radian:

```myco
unit rad = dimensionless
```

`rad` has dimension `dimensionless` and participates in ordinary
dimension algebra, but stdlib/compiler facts can keep angle intent
available to trig, geometry, and complex-number atoms while the unit
name is still present. The semantic wrapper lives in the named type
layer, not in a new unit-kind syntax:

```myco
type Angle: Scalar<rad>
type Phase: Scalar<rad>
```

`Angle` is the general semantic quantity type for angles. `Phase` is a
stdlib refined angle quantity for principal complex phase; its chosen
principal interval is a stdlib fact. Arithmetic on semantic quantity
types follows the named-type preservation / stripping rules in §3.3
and Appendix C groups D/U.

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
(a + b * T_c) mol_m2_s
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

- Multiplying a Celsius quantity by a dimensionless scalar does not
  produce another Celsius quantity. Multiplication requires converting
  to the absolute unit first, then converting the result for
  presentation if desired.
- Subtracting one Celsius quantity from another yields a Kelvin
  difference, not a Celsius quantity. Subtracting two affine
  quantities of the same affine unit produces a base-unit difference
  (the offsets cancel).
- Adding an affine quantity to a base-unit difference is defined.
- Adding two affine quantities directly is a compile error.

The compiler enforces these rules statically. No silent coercion
converts between affine and absolute forms.

#### 5.5 Workflow-Boundary Unit Parameter

**Summary.** External data enters with a declared unit via a source
object such as `Series(data, unit='K')`. The workflow layer converts
from the declared unit to base units at the binding boundary. See §24
for the workflow source model.

External data sources are unit-naive (raw floats, CSV columns). The
`Series` source accepts a `unit` keyword argument naming the unit in
which the data is expressed:

```python
experiment.bind('atm.temperature', Series(data_in_kelvin, unit='K'))
experiment.bind('atm.pressure', Series(data_in_mpa, unit='MPa'))
```

When the dimension of the declared unit matches the declared type of
the bound field, the workflow layer converts to base units at the
binding boundary. A dimension mismatch is an error at composition
time. See §24 for the source inventory and gradient-flow implications
of source objects.

### 6. Parameterized Relations and Stdlib Functions

**Summary.** User-declared reusable model structure is expressed as
parameterized relations, not user `fn` declarations. Parameterized
relations are invoked in statement position with all slots explicit;
they may be generic over units, contracts, types, and `val`
parameters. Stdlib expression functions (`exp`, `log`, `sin`,
`sqrt`, arithmetic atoms, etc.) are compiler-owned primitives that
may appear inside expressions and carry capability contracts such as
`Invertible<_>`, `Differentiable`, and `Monotone`.

Myco keeps two surfaces distinct:

- **Parameterized relations.** User-authored, statement-position
  reusable structure. A relation invocation adds obligations and
  equations to the graph; it does not return a value expression.
- **Stdlib functions.** Compiler-owned expression atoms. They may be
  called inside expressions and may carry axiomatic contracts that
  drive rewrites (§17, Appendix C). User code cannot declare new
  expression-position functions.

No annotation blocks, no `#[...]` attributes, no user-declared
function property surface. If a user needs reusable behavior, they
write a parameterized relation with explicit input and output slots.

#### 6.1 Generic Parameterized Relations

**Summary.** Parameterized relations may be generic over contracts,
including unit contracts. A generic relation monomorphizes per
instantiation at the boundary where the generic is concretized.

A unit-polymorphic relation uses a contract bound on the type
parameter and writes into an explicit output slot:

```myco
relation arrhenius<U: Unit>(
    rate_25: Scalar<U>,
    activation_energy: Scalar<joule_per_mol>,
    T: Scalar<kelvin>,
    rate: Scalar<U>,
) {
    rate = rate_25 * exp(-activation_energy / (R * T))
}
```

Invocation is statement-form and all slots are explicit:

```myco
let rate: Scalar<mol_m2_s>
arrhenius(rate_25, activation_energy, canopy.temperature, rate)
```

The compiler monomorphizes `arrhenius` once per distinct unit
instantiation at each call site where `U` is concretized. The body is
type-checked against the declared contract bound; calls that cannot
satisfy `U: Unit` are compile errors.

#### 6.2 Invocation and Method-Style Sugar

**Summary.** A parameterized relation invocation is a statement that
adds graph structure. It is never an expression. Method-style syntax
is only sugar for a relation whose first parameter is the receiver.

Rules:

- `relation_name(a, b, out)` is a statement-form invocation.
- `let out: T` introduces a fresh e-node that a relation may constrain.
- `relation_name(a, b)` cannot appear where an expression is expected.
- `receiver.rel(args..., out)` desugars to `rel(receiver, args..., out)`
  when the relation's first parameter is the receiver type.
- Field access remains parenless; parentheses always mean invocation.

This keeps graph growth explicit: relation calls add constraints,
whereas stdlib functions inside expressions build expression terms.

#### 6.3 Compiler Roles

**Summary.** The compiler treats parameterized relation bodies as
source material for unit checking, e-graph construction, symbolic
differentiation, solver emission, and closure-policy extensibility.

What the compiler does with a relation body:

- **Dimensional analysis.** Unit-checks every subexpression in the
  body. A dimension mismatch is a compile error.
- **E-graph construction.** Every `=` in the body emits a Layer-1
  merge (§16, §17); `constraint` clauses attach Layer-2 envelope
  metadata (§8.1).
- **Symbolic differentiation.** Bodies participate in `deriv`
  lowering through stdlib-atom capability contracts
  (`Differentiable`, `Invertible<_>`) and A-group rewrites (§17,
  Appendix C group A).
- **Solver emission.** Relation bodies enter the residual graph when
  their equations remain unresolved after saturation (§19).

**User recourse when the compiler cannot infer an inverse.** If the
compiler cannot derive an inverse for a relation body, refactor the
monolithic relation into smaller composable pieces whose stdlib atoms
carry the needed capability contracts; see `Invertible<_>` (§7).

### 7. Contracts

**Summary.** Contracts are the single abstraction mechanism in Myco:
declaration, multi-contract satisfaction (`: A + B + C`), and
supertraits (`contract B : A`). Contracts apply uniformly to types,
parameterized relations, stdlib atoms, and distribution families.
Parameterized and capability variants carry compiler-actionable facts.

Contracts apply uniformly to types, parameterized relations, stdlib
atoms, and distribution families. Contract declaration. Multi-contract satisfaction
(`: A + B + C`). Supertraits (`contract B : A`). Named-type
comparison rules. Contract bodies are restricted to typed field
obligations and supertraits; `initial:`, `temporal:`, `d(x) = ...`,
`step(x) = ...`, and relation bodies are not valid in a contract
declaration (see §9).

#### 7.1 Parameterized Contracts

**Summary.** Contracts take type/sample parameters (`Invertible<T>`,
`Convert<From, To>`, `Distribution<S>`). Parameters thread through
supertrait chains and satisfaction checks. Principal users are
capability contracts on stdlib atoms (§6) and distribution families
(§27).

Contracts take type parameters: `Invertible<T>` (invertible stdlib
atom with inverse type T), `Convert<From, To>` (conversion capability),
`Distribution<S>` (distribution over sample type S; scalar families
use `Scalar<U>` as their sample type). Parameters thread through
supertrait chains and satisfaction checks. Capability contracts on
stdlib atoms (§6) and distribution families (§27) are the principal
users.

#### 7.2 Capability Contracts

**Summary.** Capability contracts carry compiler-actionable facts.
The distribution-side chain drives Tier A closed-form PPL routing;
the stdlib-atom chain (`Invertible<_>`, `Differentiable`,
`Monotone`) drives inverse rewrites and the `deriv` /
`condition_of` intrinsics. Satisfaction is composable through
supertrait chains.

Capability contracts carry compiler-actionable facts. Distribution-
side chain (root `Distribution<S>`, supertraits `AffineSelfClosed`,
`SumSelfClosed`, `ProductSelfClosed`, `ScaleSelfClosed`,
`SmoothTransformable`, `ReparameterizedSampleable`) drives Tier A
closed-form routing (§13). Stdlib-atom side (`Invertible<_>`,
`Differentiable`, `Monotone`) drives inverse rewrites
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

**Summary.** A contract may supply a default body for a required
parameterized relation. The default applies only when the implementing
type does not supply its own. A type-supplied definition always takes
precedence; defaults never override a type-provided obligation.

A contract obligation may carry a default body that composes from
other obligations on the same contract:

```myco
contract Comparable {
    relation magnitude(self: Self, out: Scalar<dimensionless>)

    relation smaller_than(self: Self, other: Self, out: Bool) {
        // default: compare along the magnitude axis
        let lhs: Scalar<dimensionless>
        let rhs: Scalar<dimensionless>
        self.magnitude(lhs)
        other.magnitude(rhs)
        out = lhs < rhs
    }
}

type Mass : Comparable {
    grams: Scalar<gram>

    relation magnitude(self: Self, out: Scalar<dimensionless>) {
        out = value_in(self.grams, gram)
    }

    relation smaller_than(self: Self, other: Self, out: Bool) {
        out = self.grams < other.grams   // type-supplied; default is ignored
    }
}

type Energy : Comparable {
    joules: Scalar<joule>

    relation magnitude(self: Self, out: Scalar<dimensionless>) {
        out = value_in(self.joules, joule)
    }

    // no relation smaller_than supplied; compiler uses contract default
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
Levels I-III). Every `=` in a `relation` body introduces a Layer-1
e-class merge in the e-graph substrate (§16, §17 merge source 1).
Merge semantics.

#### 8.1 `constraint` Declarations

**Summary.** Inequality or logical obligations the modeler asserts.
Unlike relations, constraints do not merge e-classes; they restrict
the admissible solution set. Three discharge paths: compile-time
proof, runtime projection (workflow-selected flavor), or training
objective penalty on training-classified SCCs.

Inequality or logical obligations the modeler asserts must hold.
Distinct from `relation` (equational merge) in that constraints
don't merge e-classes; they restrict the admissible solution set.
Each `constraint` obligation attaches as Layer-2 envelope metadata
on the relevant e-class (§16). Discharge paths: compile-time proof
via e-graph + refinement reasoning, runtime projection (workflow-
selected flavor, §25), or training-objective penalty (SCCs classified
training, §20).

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
cases are failures, not approximation choices. The same three-way
classification applies to `where` preconditions on `convert` bodies
(§5): provably true preconditions are elided, provably false
preconditions are compile errors, and undecidable preconditions
emit a runtime assertion.

#### 8.7 Closure Policies Y1-Y6

**Summary.** Six exact user-facing handlers for redundant
overdetermination: `weighted_average`, `soft_select`, `hard_select`,
`condition_weighted`, user-defined (Y5), and exact `C(N,M)`
enumeration. Selected per residual block at workflow composition time.
Workflow-authorized guided subsystem search is an approximate
extraction strategy, not exact Y6.

User-facing handlers for redundant overdetermination. Selected per
residual block at workflow composition time. Variants:

- **Y1 `weighted_average(candidates)`** — closure-policy shorthand for
  arithmetic mean, semantically equivalent to the uniform-weight case of
  stdlib `weighted_average(values, weights)`.
- **Y2 `soft_select(preference_list, sharpness)`** — differentiable
  soft-pick implemented with stdlib `softmax` + `weighted_sum`.
- **Y3 `hard_select(preference_list)`** — deterministic
  non-differentiable pick.
- **Y4 `condition_weighted`** — weights candidates by numerical
  conditioning; backed by `condition_of` Levels I-III (§14).
- **Y5** — user-defined policy (§8.8).
- **Y6 `C(N,M)` exact enumeration** — combinatorial case for N > M+1.
  The exact semantics are exhaustive over all relevant maximal square
  subsystems: the compiler solves each subsystem and checks consistency
  across the solution set.

Y6 implementations may use graph-derived certified reductions before
enumeration when they preserve the same exhaustive semantics. Examples:
collapsing e-graph-equivalent claims while preserving provenance,
decomposing independent residual blocks, proving rank consistency for a
linear or locally-linear block, marking implied claims as dependent, or
compressing symmetric subsystem orbits. Each reduction emits proof /
provenance in the plan trace. If a reduction is not proven equivalent to
exhaustive Y6, it is not an exact Y6 reduction.

Y6 must be costed before enumeration. The compiler records the raw
`choose(N,M)` count, certified reductions, reduced exact count, and
active workflow budget. If the reduced exact count exceeds the active
budget, workflow composition fails unless the workflow explicitly raises
the budget or chooses a different policy. For dynamic collections whose
`N` depends on workflow data or events, Y6 requires a static upper bound
or an emitted runtime guard before expansion.

Workflow-authorized guided subsystem search is separate from exact Y6.
It may use graph priors (conditioning, uncertainty envelopes,
provenance quality, topology locality, symmetry representatives, stale
event state, or other compiler-visible facts) to prioritize promising
subsystems and may terminate early only under explicit workflow budgets
and acceptance criteria. Such a plan is extraction-layer approximation:
it attaches to the relevant `ResidualSite`, contributes to
`cost_of().approximation`, and is surfaced by diagnostics and run
records. It must not be reported as exact Y6.

#### 8.8 Y5: User-Defined Closure Policies

**Summary.** A Y5 policy is a parameterized relation satisfying the
closure-policy interface: candidate values plus hyperparameters in,
one explicit output slot. The compiler inlines the relation body into
the extraction pipeline, so Y5 policies participate in differentiation
and e-graph reasoning like other parameterized relations.

A Y5 policy is an ordinary parameterized relation whose inputs are
the candidate values (one per competing claim) plus user-supplied
hyperparameters, and whose output slot is a single forward value of
the same type. Users register a Y5 policy by name; workflows select
it per residual block via the same mechanism as Y1-Y6. The compiler
inlines the relation body into the extraction pipeline; Y5 policies
participate in differentiation and e-graph reasoning like other
parameterized relations.

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

#### 8.10 Regime Boundaries and Nonsmooth Crossings

**Summary.** A regime boundary is any guard surface where the active
equations, facts, topology, selected branch, or support pattern can
change. Myco records these boundaries explicitly and classifies how
information crosses them. Ordinary derivatives flow inside regimes;
at a boundary the compiler exposes the mathematically valid crossing
information and never invents a smooth gradient.

Regime boundaries are a cross-cutting semantic object, not a special
case of `event`. Sources include:

- `if` / `else` and piecewise relations whose guard depends on a
  continuous quantity.
- `event when:` triggers whose guard crosses from false to true
  (§10.0).
- Dynamic-topology changes that create, delete, or re-key tensor axes
  (§10, §21.4, §30).
- Hard selections such as `min`, `max`, `argmin`, `argmax`,
  `option_argmin`, `option_argmax`, `argmin_all`, and `argmax_all`
  (§12.2).
- Geometry junctions and one-sided locus limits (§11.1, §11.8).
- Compact-support kernels and kernel truncation surfaces (§28.5,
  Appendix C K).
- Contact, active-set, complementarity, saturation, and threshold
  laws expressed in ordinary relations.

A regime boundary record contains:

- **Surface.** The Boolean guard or equality locus, e.g.
  `turgor = 0`, `distance(x, y) = radius`, or
  `pressure_drop = split_threshold`.
- **Active forms.** The relation bodies, topology versions,
  selected branches, fact sets, or support patterns on each side.
- **Continuity class.** One of `smooth`, `value_continuous_kink`,
  `value_jump`, `structural_discontinuity`, `stochastic_discrete`,
  or `unknown`.
- **Derivative class.** One of `ordinary`, `one_sided`,
  `subgradient`, `saltation`, `estimator`, `none`, or `unknown`.
- **Crossing policy.** The authorized way information crosses the
  boundary: strict rejection, within-regime only, one-sided,
  subgradient, saltation / reset sensitivity, stochastic estimator,
  or workflow-authorized relaxation (§24.6).

The default crossing policy is strict. Simulation may evaluate hard
piecewise relations and events according to their source semantics,
but gradient-demanding contexts cannot silently treat a boundary as
smooth. If ordinary differentiation reaches a boundary whose
derivative class is not `ordinary`, the compiler reports the boundary
and the available crossing policies.

Examples:

```text
psi =
  if turgor > 0:
    elastic_curve(turgor)
  else:
    flaccid_curve(turgor)

regime_surface = (turgor = 0)
active_forms = {positive: elastic_curve, nonpositive: flaccid_curve}
continuity_class = value_continuous_kink | value_jump | unknown
derivative_class = one_sided | none | unknown
```

```text
k(x, y) = 0 when distance(x, y) > radius

regime_surface = (distance(x, y) = radius)
active_forms = {inside: kernel_body, outside: zero}
continuity_class = smooth | value_continuous_kink | value_jump
```

Smoothing remains a model claim when written in `.myco` (§8.9). A
workflow may authorize a relaxed plan (§24.6), but that plan is a
surrogate extraction with ledger entries, not a rewrite of the source
model's truth. The compiler's responsibility is to detect regime
boundaries, classify them conservatively, preserve one-sided or
within-regime information where valid, and refuse fake ordinary
gradients.

#### 8.11 Obligation Sites, Default Candidates, and Fulfillments

**Summary.** Compiler and package rules emit named `ObligationSite`s.
Modelers satisfy them with `fulfills <obligation_key>` on relations,
temporal blocks, event effects, or package default candidates. Defaults
are visible candidate fulfillments, not hidden facts; explicit
fulfillments suppress default selection without retracting anything.

An `ObligationSite` is adjacent keyed state owned by the compiler's
plan ledger, not an e-graph equality. It records:

- **Key.** Stable semantic name, such as `flux_condition(axial_flux)`,
  `boundary_condition(pressure)`, or
  `event_conservation_route(Carbon)`.
- **Kind / cardinality.** `exactly_one` for boundary conditions and
  ordinary junction flux laws; `accumulative` for event routes whose
  pieces are summed and checked.
- **Locus / event / guard.** The place where the obligation applies,
  including topology predicates and event guards.
- **Candidates.** Explicit user fulfillments and package-provided
  default candidates.
- **Status.** `unfulfilled`, `fulfilled_explicitly`,
  `fulfilled_by_default`, `suppressed_default`, `conflicting`, or
  `inactive_guard`.

User source names the obligation it satisfies:

```myco
relation leaky_junction on junction
  fulfills flux_condition(axial_flux):
    sum(e in incident_edges,
        orientation(e) * limit_from(axial_flux, junction, e)) = leak_flux

relation root_pressure on terminal where role is Root
  fulfills boundary_condition(pressure):
    limit_from(pressure, terminal) = soil_water_potential
```

For an exactly-one obligation, overlapping explicit fulfillments are a
compile or workflow-composition error; disjoint guards are valid. If no
explicit fulfillment applies, a package default candidate may be
selected only when the workflow policy permits default fulfillments.
If no explicit or permitted default fulfillment applies, the obligation
is unfulfilled and extraction fails before pretending a model exists.

Package defaults are ordinary candidate fulfillments shipped by
stdlib / imported packages, not workflow magic. The canonical junction
default is `balance_zero(flux)` as a candidate for
`flux_condition(flux)`. Boundary conditions have no silent default.
Event conservation routes are accumulative: several event effects may
fulfill the same `event_conservation_route(group)` obligation, and the
ledger checks that the routed total matches the conserved quantity
leaving or entering the source regime.

Package authors mark a candidate as default by prefixing the fulfillment
declaration with `default`. `default` is only legal on a declaration
that also names `fulfills`; it changes ledger priority, not the body
semantics:

```myco
default relation balance_zero(flux) on junction
  fulfills flux_condition(flux):
    sum(e in incident_edges,
        orientation(e) * limit_from(flux, junction, e)) = zero_like(flux)
```

`zero_like(x)` is a stdlib additive-identity constructor for the
shape/unit/type of `x`; it is not a numeric literal value supplied by
the modeler.

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
`bind("config.dt", Constant(...))` or `bind("config.dt", Series(...))`.
Time `t` is not a universal either.

`dt` is not a reserved name in `.myco`, not a universal, not a
special verb. Two cases:

- **`d(x) = expr` (ODE form):** `dt` is not referenced in the
  model. The compiler (or the backend-selected integrator) owns
  integration step size.
- **`step(x) = expr` (discrete form):** tick cadence is a normal
  workflow binding via `bind("config.dt", Constant(...))` or
  `bind("config.dt", Series(...))`. No `bind_dt` verb.

Within a `step(·)` equation, unsubscripted RHS references read the
prior-tick value and the LHS writes the current-tick value.
Consequently, `step(a) = b` and `step(b) = a` together form a swap,
not a cycle, because both RHSs read the pre-tick values of `a` and
`b` before any assignment takes effect.

Both `d(·)` and `step(·)` forms may appear in the same model.
`d(·)` variables are advanced by the integrator between ticks;
`step(·)` variables update at tick boundaries. The compiler composes
the two update disciplines without user-level coordination.

Time itself (`t`) is not a universal either; temporal indexing
produces distinct e-graph ground terms (`y[1]`, `y[2]`, …) with
structural relations between them (§16).

#### 9.2 Per-Path Uniqueness After Expansion

**Summary.** Two expansion sources produce per-path obligation keys:
generic parameter expansion (cartesian product over satisfiers) and
type-body per-instance expansion (one declaration per instantiation
of the type). An obligation key is the canonical fully-qualified path
string (`type_instance.field` with generic parameters bound)
identifying a unique temporal, initial, or relation obligation after
all expansion. Duplicate keys from either source are a compile error.
Overdetermination analysis runs on the fully expanded constraint set.

Two distinct sources produce obligation keys at compile time.

**Generic expansion.** A generic event or relation
(`event<T: Species>(…)`) expands to one concrete instance per
T-satisfier (cartesian product over all generic parameters). Each
expansion path yields one obligation key. Duplicate keys across
expanded paths are a compile error, not a closure-policy situation.

**Type-body per-instance expansion.** A type that declares
`initial:` or `temporal:` blocks expands to one per-instance
declaration per instantiation of that type. If a module-scope
declaration resolves to the same fully-qualified path as a
per-instance expansion, or if two per-instance expansions (via
nested types or multiple instantiation sites) resolve to the same
path, the compiler emits a diagnostic naming both sources. Duplicates
from this source are also a compile error, not a closure-policy
situation.

Overdetermination and underdetermination analyses run on the fully
expanded constraint set after both sources are resolved, so
uniqueness is a pre-analysis hygiene check.

#### 9.3 Initialization

**Summary.** Four mutually exclusive mechanisms initialize the value
of a temporal quantity at the start of a simulation. The compiler
emits a diagnostic for any fully-expanded temporal quantity path that
lacks exactly one of the four. The three non-inline mechanisms are
workflow bindings against source objects (§24).

Every fully-expanded temporal quantity path must have exactly one
initialization mechanism. The four options are:

- **`initial:` block in a `.myco` type body.** The value is fixed at
  compile time as a structural expression (free of numeric literals
  per CC1). This is the inline form: the initialization lives in the
  same `.myco` source as the temporal declaration.

  ```
  type SoilColumn {
    moisture: Scalar<volume_fraction>

    temporal: {
      d(moisture) = infiltration_rate - evaporation_rate
    }

    initial: {
      moisture = moisture_field_capacity
    }
  }
  ```

  Here `moisture_field_capacity` is a universal or workflow-bound
  quantity.

- **`bind(path.initial, Constant(value))`.** A workflow binding that
  injects a fixed constant as the initial value. The path is the
  fully-qualified obligation key plus the `initial` facet. The value
  is workflow-supplied and not written into `.myco` source.

- **`bind(path.initial, Trainable(prior, init=...))`.** A workflow
  binding that declares the initial value as a trainable source,
  initialized from the given prior or initial guess and trained via
  the standard gradient pipeline.

- **`bind(path, Trainable(trajectory=...))`.** A workflow binding
  that declares the full time trajectory as a learned source, not
  just the t=0 slice. This subsumes initialization: the trajectory
  source is responsible for predicting the state at every timestep.

The four mechanisms are mutually exclusive per path. If a path
receives more than one, the compiler emits a diagnostic naming the
conflicting declarations. If a fully-expanded path receives none, the
compiler emits a missing-initialization diagnostic naming the path
and its declaration site. Detailed source semantics for `Constant`
and `Trainable` bindings are in §24.

#### 9.4 Locus-Scoped Temporal Blocks

**Summary.** `temporal name on locus:` is legal by symmetry with
`relation name on locus:` (§11). State evolution that applies only at
a specific locus of a domain is expressible as a locus-scoped
temporal block, separate from the bulk temporal declarations that
govern the domain interior.

The `on locus:` clause applies symmetrically to both `relation` and
`temporal`. A locus-scoped temporal block declares state evolution
equations that fire only at the named locus of the enclosing domain.
The locus mechanism, locus vocabulary, and geometry machinery are
defined in §11.

A common use case is boundary-specific evolution: a soil domain may
have bulk diffusion governed by one `temporal` block in the type body,
and surface evaporation governed by a separate `temporal
surface_drying on top_boundary:` block that applies only at the
domain's top locus. The compiler treats the two blocks as distinct
obligation keys (§9.2) because their paths include the locus
qualifier. No user-level coordination is required to compose them;
the compiler assembles the full update from the resolved keys.

### 10. Dynamic Topology and Events

**Summary.** `event` declarations mutate the simulation graph
structure. Referential-truth semantics: entities do not know they are
dead, events add facts, no tombstoning, no retraction. Enum variant
transitions use event-only `becomes` with explicit next-regime
construction. Firing order, generic expansion, cross-container
events, event-route fulfillments, and monotonicity live here.

`event` declarations for topology change. Referential-truth semantics:
things do not know they are dead. Events add facts; no tombstoning, no
retraction. When an event changes an enum variant, it constructs the
next variant explicitly with `becomes` (§10.6).

#### 10.0 Event Triggers

**Summary.** The `when` clause is the event trigger surface. It
specifies a Boolean-valued condition that must become true for the
event to fire. Semantics are edge-triggered: the event fires at the
moment the condition transitions from false to true, not continuously
while the condition holds.

An event declaration carries an optional `when` clause whose body is
a Boolean-valued expression. The expression may reference fields on
the event participants and on their enclosing container.

```
event seedling_recruit(plot: Plot):
    when: plot.canopy_openness > plot.light_threshold
    -> Tree<SomeConcreteCanopy>
```

The condition is evaluated at each tick against the current state of
the referenced quantities. When the condition is false at tick T and
true at tick T+1, the event fires once at T+1. A condition that
remains true across consecutive ticks does not re-fire. A condition
that falls back to false and then rises again fires a second time at
the second rising edge. One rising transition equals one firing.

A `when` clause with a deterministic threshold (comparing a field to
a workflow-bound universal) produces one firing per rising-edge
crossing per eligible participant group. Probabilistic conditions
(`when: canopy_openness ~ Bernoulli(p)`) are handled under the
aleatoric scope rules of §13.1 and still obey edge-triggered
semantics: the sampled outcome is resolved each tick and an edge is
detected on the resolved Boolean sequence. A `when` guard whose truth
depends on model values is also a regime boundary (§8.10); gradients
inside a pre-event or post-event regime do not automatically cross
the event surface.

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
§24 (workflow source model); §10 commits only to the contract that such
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

**Concrete output type for `impl`-typed collections.** An event that
emits a new entity into a collection typed `[T<impl Contract>; some]`
must name the concrete output type in the event signature. The
compiler requires this so that it can route the newly created entity
to the correct type pool at instantiation time.

```
event oak_recruit(plot: Plot):
    when: plot.canopy_openness > plot.light_threshold
    -> Tree<OakCanopy>
```

A generic event (`event new_tree<C: Canopy>(plot: Plot): -> Tree<C>`)
is the shorthand that expands to one concrete event per in-scope
implementation of `Canopy` (§10.2 cartesian-product rule). Each
expanded variant carries a concrete output type by construction.
Omitting the concrete type when targeting a heterogeneous `impl`
collection is a compile error.

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

#### 10.5 Fulfillment Ledger and Monotonicity

**Summary.** `fulfills <obligation_key>` satisfies an obligation site;
it does not retract or mutate another source statement. Default
candidates are selected or suppressed in the obligation ledger before
residual extraction. The e-graph only receives the selected fulfillment
facts, preserving monotonicity.

A fulfillment declaration (§8.11) answers an `ObligationSite`. For
exactly-one obligations, the ledger selects one active fulfillment:
an explicit user fulfillment if exactly one applies, otherwise a
permitted package default if exactly one applies, otherwise an
unfulfilled or conflicting status. Suppressed defaults remain
inspectable ledger candidates but are not emitted as layer-1 facts.

For accumulative obligations such as
`event_conservation_route(Carbon)`, every active fulfillment contributes
to the ledger total. The compiler checks the aggregate route against
the conserved quantity crossing the event boundary. This supports
split, merge, shedding, storage, and source/sink-style event models
without pretending the event happened in a fixed statement order.

Inside an event body, a named effect block may carry `fulfills`. The
block name is provenance, not ordering:

```myco
event leaf_shed(l: Leaf, d: DetritusPool, a: Atmosphere) {
    l.stage becomes Shed

    route_to_detritus fulfills event_conservation_route(Carbon):
        d.carbon += l.carbon * detritus_fraction

    route_to_atmosphere fulfills event_conservation_route(Carbon):
        a.carbon += l.carbon * respired_fraction
}
```

The two route blocks are accumulated by obligation key. The ledger
checks routed carbon against the carbon removed from `l` by the event
transition; it does not impose a statement-order interpretation.

The harder case of a user-written event that logically retracts a
prior user claim remains out of the core source language and is tracked
in §35 Other Opens. Events add facts; obligation fulfillment selects
which candidate answers a compiler/package obligation, not whether
arbitrary prior claims are deleted.

#### 10.6 Enum Variant Transitions

**Summary.** Event-triggered enum variant transitions use `becomes`.
They are event-only regime-boundary crossings, not ordinary
assignment. The next variant is fully constructed explicitly; no
same-name field carryover occurs. Removed old-variant fields leave
scope unless copied into the next variant or an event/history record.

Enums whose discriminant changes over time participate in the same
regime-boundary model as shape-changing SCCs (§8.10). Source syntax:

```myco
event mature(p: Plant where p.stage is Seedling) {
    p.stage becomes Mature {
        age: p.stage.age,
        height: p.stage.height,
        dbh: initial_dbh,
    }
}
```

Rules:

- `becomes` is valid only in `event` bodies.
- The event guard `where p.stage is Seedling` narrows the old variant
  for the event body.
- The right-hand side names the next variant and supplies every field
  required by that variant.
- Preserved values are ordinary expressions and must be copied
  explicitly.
- Same-name fields never carry over implicitly.
- Fields that existed only on the old variant leave scope after the
  transition.
- Historical values are available only if the model writes them into
  the new variant or a separate event/history record.
- `becomes` does not retract relations and does not satisfy obligations
  implicitly; use `fulfills` when an event effect answers an
  obligation (§10.5).

Lowering may represent a dynamic enum with tags, branches, masks, or
regime-specific plans depending on backend capability, but the Myco
semantics are one event-boundary variant replacement with explicit
field construction.

### 11. Geometry and Locus

**Summary.** Spatial framing. Horses own geometry, flies are embedded
entities located against that geometry. `bind_topology` supplies
concrete meshes at workflow time. Standard locus vocabulary
(`boundary`, `chart`, `metric`, `requires`), stdlib geometry catalog,
spatial operators (`grad`, `diverg`, `laplacian`, `curl`,
`normal_grad`, `trace`), weak / residual forms, and boundary
conditions via `requires`.

Horse/fly composition pattern for spatial frames. `bind_topology` at
workflow time for concrete meshes. `on locus:` clause applies
symmetrically to `relation` and `temporal`.

#### 11.0 Foundations

**Summary.** `geometry` is a first-class language construct. `Domain<G>` is
the composite-type annotation that binds geometric behavior to a declared
geometry. The `as` clause attaches named coordinates, units, and extents to
a domain type. Together these three surfaces express the spatial structure of
a type without encoding coordinate names or units inside the reusable geometry
definition.

`geometry G { ... }` declares a first-class language construct. The block
names the topology class, the coordinate chart, the metric tensor, and any
named loci for a geometric space. Geometries are reusable across many domain
types: the same `Euclidean<Dim = 2>` geometry serves a pasture, an image
domain, or a leaf surface. The geometry body may contain `chart` (local
coordinate binders for internal readability), `topology`, `metric`, `locus`,
`requires` (scalar per-instance coefficients the metric depends on), and
`identify` (periodic seam declarations). Full vocabulary for these keywords
is given in §11.11.

`Domain<G = SomeGeometry>` is an ordinary Myco composite-type annotation that
binds a type's geometric behavior to a declared `geometry`. A horse type
annotated `Domain<G = Euclidean<Dim = 2>>` inherits flat 2D spatial
semantics; all §11.1 spatial operators (`grad`, `diverg`, `laplacian`, `curl`,
`normal_grad`, `trace`) applied to fields on that type route through the bound
geometry's metric. The annotation does not introduce a new kind; it is a
type-system extension point that provides the compiler with the geometric
context needed to lower spatial operators to concrete discretized forms at
workflow composition time.

The `as` clause attaches named physical coordinates, units, and extents to a
domain-typed composite type. Example: a type declared
`Domain<G = Euclidean<Dim = 1>> as (depth: Scalar<meter>)` binds the single
chart coordinate as `depth` with meter units. The mapping is positional: the
first `as` name binds to the first `chart` binder in the referenced geometry,
the second to the second, and so on. The `as` clause is required on every
domain type; the compiler rejects domain types without it. Edge-length units
supplied in `bind_topology` (§11.5) are validated against the `as`-clause
coordinate units and a mismatch is a compile error.

#### 11.1 Spatial Operators

**Summary.** Stdlib-recognized spatial operators on locus-scoped
fields: `grad`, `diverg`, `laplacian`, `curl`, `normal_grad`,
`trace`, `trace_from`, `limit_from`, `jump`, `average`, `normal`,
`normal_traction`, and `test_space`. `diverg` on a conserved flux field
drives `flux_condition` obligations at junctions. Operators are stdlib
axioms with capability contracts; users do not annotate them.
Dimension-dependent signatures (e.g., `curl`) dispatch at the axiom
level via case-on-val-generic in the return type.

Compiler-recognized operators on locus-scoped fields:

- `grad(f)` — gradient of a scalar field; yields a vector field
  on the same locus.
- `diverg(v)` — divergence of a vector field; yields a scalar.
  `diverg` on a conserved flux field emits `flux_condition(v)`
  obligation sites at junctions (§3.7, §11.8).
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
- `trace_from(f, interface, side)` — sided restriction of `f` to a
  codimension-1 interface. `side` must name a side exposed by the
  interface / topology facts. This generalizes one-sided boundary and
  fault / material-interface access beyond graph junctions.
- `limit_from(f, junction, edge)` — one-sided directional limit:
  the value of `f` as the junction is approached along a
  specified incident edge. Defined on `MetricGraph` /
  `RootedTree` junctions where the field may be discontinuous
  across incident edges.
- `jump(f, interface)` — difference between sided traces across an
  interface, with orientation carried by the interface facts.
- `average(f, interface)` — arithmetic or metric-weighted average of
  sided traces across an interface, according to the interface's
  measure / metric facts.
- `normal(interface, side)` — oriented normal vector / covector exposed
  by boundary or interface facts.
- `normal_traction(stress, interface, side)` — stress contracted with
  the oriented interface normal on the requested side.
- `test_space<T>(locus)` — admissible semantic test functions for weak
  / residual statements over a locus. A test space is part of the
  source model's mathematical claim; it does not select a finite
  element basis, quadrature rule, sparse layout, or solver.

Operators are stdlib axioms with capability contracts (§7.2).
Relations like `laplacian(f) = diverg(grad(f))` fire as e-graph
rewrites from stdlib declarations; users never annotate them.

Strong, weak, and interface forms are all source-level mathematics when
written in `.myco`. Strong forms use pointwise spatial operators:

```myco
temporal heat on interior:
    d(T) = diffusivity * laplacian(T) + source
```

Weak / variational forms use ordinary `integrate` plus semantic test
spaces. They state residual claims over all admissible tests; they do
not choose finite basis functions:

```myco
relation heat_weak on interior:
    for test v in test_space<Scalar<dimensionless>>(interior):
        integrate(v * d(T), x, interior)
          = -integrate(dot(grad(v), diffusivity * grad(T)), x, interior)
            + integrate(v * source, x, interior)
```

Interface laws use sided traces, jumps, averages, and normal traction:

```myco
relation fault_law on fault:
    slip = jump(displacement, fault)
    tau_plus = normal_traction(stress, fault, side = plus)
    tau_minus = normal_traction(stress, fault, side = minus)
    tau_plus + tau_minus = 0
    tau_plus = friction_law(slip_rate, state)
```

These constructs let a model state the mathematics that serious PDE
systems actually use: weak residuals, interface conditions, jump laws,
and trace laws. Basis choice, quadrature, numerical flux, matrix-free
action, preconditioner, sparse format, and hardware kernel selection are
realization choices (§31.1, §37.1), not `.myco` source semantics.

**Dimension dispatch in axiom return positions.** `curl` is the
first operator whose return type depends on a val generic carried
by the input domain (`G.dim`). The dispatch pattern mirrors
`solve`'s dispatch on matrix structural facts (§3.9, §30): the
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

**Summary.** Boundary conditions are `requires` blocks or relations
that fulfill `boundary_condition(field)` on boundary sub-loci. Three
standard forms (Dirichlet, Neumann, Robin) lower to projection,
elimination, or residual constraints based on workflow-selected solver
path. No defaults: a boundary without a fulfillment is
underdetermined.

Boundary conditions are `requires` blocks on boundary sub-loci.
Three standard forms:

- **Dirichlet** — `requires: f = g`. Fixes the field value.
- **Neumann** — `requires: normal_grad(f) = g`. Fixes the normal
  flux.
- **Robin** — `requires: a * f + b * normal_grad(f) = g`. Linear
  combination.

Each `requires` block fulfills the corresponding
`boundary_condition(field)` obligation and lowers to a projection,
elimination, or residual constraint depending on the solver path
selected at workflow composition (§25). A locus with boundary geometry
and no fulfillment is underdetermined; the compiler emits no default
boundary condition (silence is not a free Neumann zero).

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

The `Sphere` geometry carries an `identify` seam declaration for its
periodic longitude coordinate (`identify phi = 0 <-> phi = 2 * pi`).
Without `identify`, the compiler would treat the seam as a pair of fake
boundaries and demand boundary conditions there; `identify` tells the
compiler those two coordinate values name the same edge. For v2.1,
`identify` is guaranteed for scalar fields only. Vector and tensor fields
at a seam may require component remapping or orientation flips (for
example, tangent vectors on a non-orientable surface); those transforms
are deferred beyond v2.1 (§35). The `identify` declaration is the surface
expression point in §11; the underlying mechanism is an X2-group rewrite
that installs a Layer-3 site record keyed on the seam locus, from which
Layer-1 merges are derived (§17).

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
no auto-refinement or adaptation. Network topologies (`rooted_tree`,
`metric_graph`) require a data schema described below; manifold
geometries receive mesh resolution via `experiment.compile` instead.

A geometry becomes a mesh at workflow composition. `bind_topology`
supplies discretization: mesh resolution, element type (FDM /
FVM / FEM basis), refinement policy, boundary identification.
The compiler receives a concrete mesh and lowers spatial
operators against it. The compiler does not auto-refine or adapt;
mesh is a workflow decision.

**Schema for `rooted_tree` and `metric_graph`.** When `bind_topology`
is called with a network topology, the supplied data object must
provide:

- **Vertex IDs.** Contiguous non-negative integers starting from zero.
  IDs must be stable across all workflow bindings that reference this
  topology instance; the compiler uses them as canonical indices for
  locus-scoped field arrays and plan inspection output.
- **Edge list.** A list of vertex-ID pairs. For `rooted_tree`, the
  direction implied by each pair is parent-to-child, consistent with
  the anatomical direction away from the root. For `metric_graph`,
  edges are undirected; an explicit `edge_orientation` map may be
  supplied to set the canonical sign convention for oriented operators;
  if omitted the compiler assigns a canonical orientation deterministically.
- **Edge-length units.** Each edge carries a numeric length value with
  units. Those units must match the coordinate units declared in the
  domain type's `as` clause. A unit mismatch is a compile error reported
  at workflow composition.
- **Vertex tags.** A map from vertex ID to key-value metadata. Every tag
  key referenced in a `where`-predicate locus in the geometry (for
  example `on terminal where role = "leaf"`) must appear as a key in the
  supplied tag map for every vertex of the relevant locus class. Missing
  tag coverage is a compile error with a diagnostic naming the undeclared
  key and the locus predicate that requires it.
- **Root vertex** (`rooted_tree` only). A single vertex ID designating
  the root. Required; omission is a compile error.

**Validation.** Missing tag coverage, unit mismatch on edge lengths, gaps
in vertex IDs, missing root (for `rooted_tree`), or cycles in a
`rooted_tree` are all compile errors surfaced at workflow composition, not
at `.myco` parse time.

**Manifold geometries.** `Euclidean<Dim>`, `Interval`, `Rectangle`, `Disk`,
`Sphere`, `Box`, and `Ball` do not use `bind_topology`. Mesh resolution for
these geometries is supplied via `experiment.compile(spatial_config=...)`.
Providing a `bind_topology` call for a manifold domain type is a compile
error.

#### 11.6 Compiler Discretization Defaults

**Summary.** If `bind_topology` omits discretization, the compiler
picks per-geometry defaults (uniform grids, one node per structural
vertex). Defaults are conservative smoke-test affordances; production
use typically requires explicit override.

If `bind_topology` does not specify a discretization, the
compiler picks per-geometry defaults documented in the stdlib
reference. Indicatively: `Interval` uses a uniform N-node grid (N is
still workflow-supplied); `Rectangle` uses a regular M×N grid;
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

**Subdimensional fields (`over` keyword).** A field may vary over
fewer dimensions than its containing domain using the form
`field name: Type over coord`. The named coordinate must appear in the
domain type's `as` clause. The compiler treats the field as a function
of the named coordinate(s) only; the value is constant in the
orthogonal directions. For example, in a 3D `Box` domain with
`as (x: Scalar<meter>, y: Scalar<meter>, z: Scalar<meter>)`, the
declaration `field soil_moisture: Scalar<volume_fraction> over z`
produces a field that varies only with depth while remaining uniform in
the horizontal plane. Spatial operators applied to a subdimensional field
operate in the subspace spanned by the declared coordinates; applying
`grad` orthogonal to the declared coordinates yields zero by definition.
Multiple coordinates may be listed to form a multi-dimensional
sub-field (`over (x, y)` in a 3D domain, for instance).

#### 11.8 Junction Flux Conditions

**Summary.** `diverg()` on a conserved flux emits a
`flux_condition(flux)` obligation at junctions. The stdlib default
candidate is `balance_zero(flux)`: oriented conserved-flux sums equal
zero. Users write `fulfills flux_condition(flux)` for leaks, storage,
or other nonzero junction laws. Continuity of non-flux fields is not
assumed; modelers opt in with explicit `requires: left.f = right.f`.

Where edges meet at a junction, conservation emits a
`flux_condition(flux)` obligation (§3.7 consequence 4). The stdlib may
provide `balance_zero(flux)` as a default candidate: the sum of
conserved fluxes across the junction equals zero. Continuity of
non-flux fields is **not** assumed by default. Different edges at a
junction may carry different scalar values unless the modeler writes an
explicit `requires: left.f = right.f`.

Rationale. What conservation requires is an accounting obligation, not
always a zero-balance equation. The zero-balance law is the common
candidate fulfillment; leak, storage, or source/sink junctions are
equally valid explicit fulfillments. Continuity remains a modeling
choice, which prevents silent assumptions about field matching across
junctions.

**Locus-scoped fulfillments with obligation keys.** When a locus-scoped
relation or temporal block satisfies a compiler/package obligation, it
names the stable semantic key, not a user-chosen relation name:

```myco
relation leaky_junction on junction
  fulfills flux_condition(axial_flux):
    sum(e in incident_edges,
        orientation(e) * limit_from(axial_flux, junction, e)) = leak_flux

temporal junction_storage on junction
  fulfills flux_condition(axial_flux):
    d(junction_water) =
      sum(e in incident_edges,
          orientation(e) * limit_from(axial_flux, junction, e))
```

The key form is `verb(field_name)` where the verb is drawn from the
compiler's obligation vocabulary (`flux_condition`,
`boundary_condition`, `event_conservation_route`). Using stable keys
ensures fulfillment targets are unambiguous across refactoring:
renaming the user relation does not affect which obligation it answers.
Obligation-key semantics are defined in §8.11; monotonicity is in
§10.5.

**Stdlib junction helpers.** `continuous(field)` and
`kirchhoff(potential, flux)` are stdlib convenience functions, not
compiler magic. `continuous(f)` expands to a `requires: left.f = right.f`
continuity condition across all incident edges at a junction.
`kirchhoff(potential, flux)` bundles `continuous(potential)` with a
`balance_zero(flux)` fulfillment candidate, expressing the standard
Kirchhoff pair for a potential-driven network. Users may always write
the explicit trace equations instead; the stdlib helpers are opt-in
shorthand for the common case.

Locus-scoped `temporal name on locus:` blocks follow the same
`on locus:` clause symmetry as locus-scoped relations; they are covered
in §9.4.

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
Iteration patterns, aggregation primitives (`sum`, `product`, `max`,
`min`, `any`, `all`, `count`, `softmax`, `weighted_sum`,
`weighted_average`) and selector primitives (`argmin`, `argmax`,
`option_argmin`, `option_argmax`, `argmin_all`, `argmax_all`).
Aggregations are stdlib-only.

`impl Contract` (heterogeneous element type, static monomorphization)
vs `some` (runtime sizing). Iteration patterns. Aggregation lowering.
Narrowing with `where x is T`.

#### 12.1 Aggregation Primitives

**Summary.** Named stdlib aggregations: `sum`, `product`, `max`,
`min`, `any`, `all`, `count`, `softmax`, `weighted_sum`,
`weighted_average`, plus the selection family `argmin`, `argmax`,
`option_argmin`, `option_argmax`, `argmin_all`, and `argmax_all`.
Units-aware and conservation-group-aware where applicable. Compose
under stdlib-declared e-graph rewrites. No user-declared aggregation
surface.

Named stdlib aggregations over collections:

- `sum(xs)`, `product(xs)` — arithmetic. Units-aware;
  conservation-group-aware (§3.7 blocks cross-sibling sums
  without an explicit `convert`).
- `max(xs)`, `min(xs)` — scalar extrema. Reduce a collection of
  unit-compatible scalars to a scalar of the same unit.
  Differentiability class: subgradient (same as `argmin`/`argmax`;
  see §12.2). For empty-collection behavior, see §12.3.
- `any(xs)`, `all(xs)` — boolean.
- `count(xs)` — number of alive elements, `Scalar<dimensionless>`.
  For event-time (`some`-sized) collections backed by a bitmask-
  liveness array, `count` sums the liveness bits, not the backing-
  array capacity (§12.4).
- `softmax(scores, temperature)` — numerically stable softmax over an
  aligned finite collection of scalar scores. `temperature > 0` is an
  obligation; `score / temperature` must be dimensionless. The result is
  a dimensionless weight collection aligned with `scores`, with
  nonnegative weights whose sum is 1 for nonempty input. Empty input
  returns an empty weight collection.
- `weighted_sum(values, weights)` — sum of aligned value / weight pairs.
  `weights` must be dimensionless and aligned with `values` by the same
  collection axis or an explicit shared key. The result has the same
  unit and value shape as each element of `values`; empty input returns
  the unit-correct zero value.
- `weighted_average(values, weights)` — normalized weighted value
  aggregation, equivalent to `weighted_sum(values, weights) /
  sum(weights)` when the denominator is nonzero. The compiler emits a
  nonzero-denominator obligation unless normalization is proven (for
  example, weights produced by `softmax`).
- `argmin(selector for x in xs)`, `argmax(selector for x in xs)` —
  strict selected handle of the extremal element; require a nonempty
  proof (§12.2).
- `option_argmin(...)`, `option_argmax(...)` — optional selected handle;
  empty input produces `None` (§12.2).
- `argmin_all(...)`, `argmax_all(...)` — collection of all selected
  handles tied at the extremum; empty input produces an empty
  collection (§12.2).

Aggregations compose under stdlib-declared e-graph rewrites
(linearity, distributivity, `sum(map(f, xs))` fusions). There is
no user-declared aggregation surface — new aggregations ship via
stdlib, matching the `.myco`-has-no-annotation-surface stance.
Soft and weighted variants are value aggregations, not entity
selectors. A smooth selector may return a weighted value such as a
soft crown-area aggregate, but it does not fabricate a smooth
`Selected<T>` entity handle.

Alignment for weighted aggregation is semantic, not positional
convenience. Values and weights must share the same finite axis,
collection identity, or explicit key relation. The compiler rejects
implicit index-order matching between unrelated collections. For
event-time collections, alignment respects the same existence-domain /
mask / ragged policy as the collection itself (§12.4, §24.6).

```myco
soft_crown =
    weighted_sum(
        t.crown_area for t in trees,
        weights = softmax(t.height for t in trees, temperature = tau)
    )
```

`softmax` is a stdlib atom with stable log-sum-exp lowering. It may
rewrite to the mathematical `exp(score / temperature) /
sum(exp(...))` form only when the chosen realization preserves the
declared numerical facts; users should not need to hand-roll stable
softmax from `exp` and `sum`.

#### 12.2 Selected Handles and Selector Primitives

**Summary.** Element-selection primitives return `Selected<T>`, an
opaque regime-local reference to an existing collection member. Strict
selectors require nonempty proof; optional selectors return
`Option<Selected<T>>`; all-tie selectors return `[Selected<T>; some]`.
Selection provenance lives in Layer 3 adjacent keyed state, while field
projections from a selected handle are ordinary graph expressions.

`Selected<T>` is a visible stdlib/compiler type constructor for selected
references. Users may write it in relation signatures, but they cannot
construct it manually. A `Selected<T>` is created only by compiler-owned
selector primitives such as `argmax`, `argmin`, `option_argmax`,
`option_argmin`, `argmax_all`, and `argmin_all`.

```myco
contract TreeLike {
    height: Scalar<m>
    crown_area: Scalar<m2>
}

relation dominant_tree(trees: [impl TreeLike; some],
                       out: Option<Selected<TreeLike>>):
    out = option_argmax(t.height for t in trees)
```

Selector return types:

- `argmax(selector for x in xs) : Selected<T>` and
  `argmin(...) : Selected<T>`. Strict; require the compiler to prove
  the input collection nonempty in the active regime.
- `option_argmax(...) : Option<Selected<T>>` and
  `option_argmin(...) : Option<Selected<T>>`. Total; empty input
  produces `None`.
- `argmax_all(...) : [Selected<T>; some]` and
  `argmin_all(...) : [Selected<T>; some]`. Total; empty input produces
  an empty derived view collection containing all elements tied at the
  extremum.

Homogeneous and heterogeneous collections share the same surface. For
`[Oak; some]`, `argmax(o.height for o in oaks)` returns
`Selected<Oak>`, not an `Oak` record. For `[impl TreeLike; some]`, it
returns `Selected<TreeLike>` whose direct field projection is limited to
the contract-common surface:

```myco
let tallest = argmax(t.height for t in trees)

dominant_height = tallest.height       // selector field; equals max height
dominant_crown = tallest.crown_area    // crown area of the height-winner
```

Concrete-type-specific fields require explicit narrowing / matching.
The compiler may lower a heterogeneous selected handle as an internal
tagged reference `(pool_identity, intra_pool_index)`, but that tag is
not source syntax and is not workflow-bindable. `match` on a selected
handle is a type-narrowing match over the collection's static concrete
member set, not enum dispatch; arms must cover every possible concrete
type unless an enclosing narrowing context proves a smaller set:

```myco
match tallest {
    Oak(o) => {
        reproductive_load = o.acorn_load
    }
    Pine(p) => {
        reproductive_load = p.cone_load
    }
}
```

`Option<Selected<T>>` is an ordinary enum value and must be narrowed
before projection:

```myco
let tallest = option_argmax(t.height for t in trees)

match tallest {
    Some(t) => {
        dominant_crown = t.crown_area
    }
    None => {
        dominant_crown = no_tree_crown_area
    }
}
```

`argmax_all` / `argmin_all` return derived view collections over
existing entities, not owned entity collections. They compose with
ordinary aggregations:

```myco
let winners = argmax_all(t.height for t in trees)
winner_count = count(winners)
total_winner_crown = sum(w.crown_area for w in winners)
```

Selection facts are intentionally narrow. For `tallest =
argmax(t.height for t in trees)`, the compiler records that `tallest`
is a member of `trees`, that `tallest.height` is the maximum selector
value, and that every live `t` has `tallest.height >= t.height`.
It does **not** infer that `tallest.crown_area` is a maximum crown area,
nor does it merge `tallest` with candidate entities in the e-graph.

Layer placement:

- Selection identity, input collection, selector expression, empty
  behavior, tie policy, result contract, and existence domain are
  `SelectedSite` records in Layer 3 adjacent keyed state (§16.1).
- Field projections such as `tallest.crown_area` are ordinary Layer-1
  expressions with provenance pointing back to the selected site.
- `Selected<T>` equality is not ordinary relation `=`. Identity tests,
  if needed, use an explicit stdlib predicate such as
  `same_entity(a, b, out: Bool)`.

Tie behavior is deterministic. `argmin` / `argmax` choose the earliest
element in canonical collection order among tied extrema. `argmin_all`
and `argmax_all` choose all tied extrema. Equality for all-tie selection
is exact unless an explicit approximation / relaxation policy is
selected elsewhere; no tolerance is hidden inside the selector.

Differentiability and lifetime:

- The selection family is hard-selection / subgradient-differentiable:
  gradients flow through the currently selected element; winner
  switchover, tie, and empty/nonempty surfaces are regime boundaries
  (§8.10).
- Smooth selection returns aggregate values, not `Selected<T>` handles.
- `Selected<T>` is regime-local by default. Persisting
  `Selected<T>` or `Option<Selected<T>>` across ticks or event
  boundaries is legal only as state/history with an existence domain.
  Later projection requires proof or guard that the referenced entity
  still exists. Capturing a projected value, such as `tallest.height`,
  creates an ordinary boundary-indexed equality claim, not an
  imperative copy.

#### 12.3 Empty-Collection Defaults

**Summary.** Aggregations with identity elements use them on empty
collections (`sum = 0`, `product = 1`, `all = true`, etc.). `max`
returns `-inf` (properly-typed sentinel) and `min` returns `+inf` on
empty collections. Strict `argmin` / `argmax` require nonempty proof;
`option_argmin` / `option_argmax` return `None` on empty input;
`argmin_all` / `argmax_all` return an empty derived collection.

Aggregations behave on empty collections as follows:

- `sum(empty) = 0`, `product(empty) = 1`, `count(empty) = 0`.
- `any(empty) = false`, `all(empty) = true`.
- `max(empty)` returns the additive identity element of the
  extrema lattice: `-inf` (a properly-typed unit-carrying infinity,
  not a numeric literal). `min(empty)` returns `+inf` by the same
  convention. These are the correct identity elements for max/min
  reductions and compose correctly with any subsequent `max`/`min`
  combining step.
- `argmin(empty)`, `argmax(empty)` are a **compile error** unless
  unreachable under the active guard.
- `option_argmin(empty)`, `option_argmax(empty)` return `None`.
- `argmin_all(empty)`, `argmax_all(empty)` return an empty collection.

Identity-element defaults on `sum`/`product`/`any`/`all`/`count`
enable algebraic rewrites without branch logic. `max` and `min`
use `-inf`/`+inf` as their identity elements for the same reason.
Strict `argmin` and `argmax` have no identity element, so the compiler
rejects empty-reachable calls at compile time; the caller must
statically prove non-emptiness or guard with a `count > 0`
check that the compiler can refine against. Library authors who cannot
or do not want to require such proof should expose
`Option<Selected<T>>` via `option_argmin` / `option_argmax` instead.

**Sentinel injection for masked slots.** In collections that use
bitmask-liveness lowering (the GPU-batched array-pool design for
event-time `some`-sized collections; §12.4, §21), aggregation
kernels cannot skip inactive slots directly: on JAX and PyTorch,
`jax.numpy.where`/`torch.where` evaluates both branches regardless
of the condition. The backend emitter therefore injects sentinel
values into inactive slots before reduction: `-inf` for `max`,
`argmax`, `option_argmax`, and `argmax_all` operations, `+inf` for
`min`, `argmin`, `option_argmin`, and `argmin_all` operations. The
handle-returning selectors additionally mask dead slots during the
winner / winner-set reconstruction step so they never return a dead
slot. Users observe only the alive-element semantics; the sentinels are
a lowering artifact.

#### 12.4 Bind-Time vs Event-Time Dynamism

**Summary.** Two sources of collection-size change. Bind-time
dynamism fixes membership at workflow composition (lowers with
runtime size N, no N-max). Event-time dynamism mutates at runtime
(requires N-max slot allocation and alive-mask lowering).

Two distinct sources of collection-size change:

- **Bind-time dynamism.** Collection membership is fixed when
  `bind_topology` and source bindings run. After workflow
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
  Bayesian update; participates in training. Workflow-side
  prior binding for epistemic `~` uses `bind(path, Prior(D))`
  (§24), which attaches a distributional fact to the e-class at
  training or inference time.

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
SCC at compile time. The `Distribution<S>` contract surface
(visible `log_density` relation over sample type S, default-derived
density / `pdf` convenience, backend sampling capabilities, and
optional closure sub-contracts) that makes Tier A dispatch possible
is specified in §27.

1. **Tier A — Exact closed-form.** Capability contracts on
   distribution families (§7.2, §27) advertise algebraic
   closures (`AffineSelfClosed`, `SumSelfClosed`,
   `ProductSelfClosed`, `ScaleSelfClosed`,
   `SmoothTransformable`, `ReparameterizedSampleable`). When a
   transformation matches a closure contract, the result is
   another member of the family with analytically computed
   parameters. Closed-form always wins. Some closure contracts
   apply conditionally on parameter alignment (`SumSelfClosed`
   holds for Gamma only under shared rate parameter, for
   Binomial only under shared success probability); §27.1
   records the per-family conditions. The full Z-group rewrite
   catalog that fires from these contracts is in Appendix C.
2. **Tier B — Approximate rewrite.** When Tier A does not
   close, approximate-block rewrites (Delta method,
   Fenton-Wilkinson, CLT, block-maxima → GEV; §15) apply if
   the user's `approximate` block permits the relevant error
   class. Envelope metadata records the approximation used.
3. **Tier C — Whole-SCC opaque PPL handoff.** No closed form, no
   user-permitted approximation. After Tier A and Tier B have run to
   exhaustion, each unresolved stochastic SCC ships as one inference
   task to the backend's PPL handler (§31). The backend sees the
   whole remaining stochastic SCC, not one factor at a time. Samples
   and diagnostics come back with provenance; no envelope facts about
   the parametric form are granted. Curated stdlib / backend opaque
   stochastic families route to Tier C by default and grant no
   symbolic density, derivative, closure, condition, or independence
   facts unless a visible rewrite or audited backend capability
   supplies that specific fact.

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
a particular marginalization attach workflow evidence that keeps
the latent's value in scope. Markov-structured
discrete latents (HMM-style temporal dependencies) are a
compile error with diagnostic guidance; they require structural
handling (forward-backward, particle filter) as specified in
§28, and do not fall through to Tier C.

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

#### 13.5 Independence via Structural Roots

**Summary.** Separate stochastic roots are independent by structural
default conditional on their visible parents; same e-class means the
same draw; fields from one structured joint root inherit the joint
envelope's dependence facts. No naked correlation surface: correlated
structures are built by sharing upstream stochastic structure or
declaring a joint family (MVN, Mixture).

Separate stochastic roots are independent by structural default
conditional on their visible parent e-classes. `x ~ Normal(μ, σ)` and
`y ~ Normal(μ, σ)` on separate lines produce two roots and are
conditionally independent given `μ` and `σ`; if those parents are
themselves stochastic, any induced marginal dependence flows through
the ordinary graph. A shared intermediate that binds both `x` and `y`
to one draw `z` produces one e-class: x and y are the same draw, fully
correlated. Field projections from one structured joint root are not
separate roots; their dependence is determined by the joint envelope
metadata (§13.7, §13.10).

There is no naked correlation surface. No `Cov(x, y) = ρ`, no
`correlate(x, y)`. Correlated structures are built by sharing
upstream stochastic structure or by declaring a joint family (MVN,
Mixture, structured joint families) that bakes the correlation in.
The mechanism matches the three-layer principle: equality lives in
e-classes, while probabilistic dependence facts live in the
distributional envelope.

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
  the training objective through matrix-calculus rewrites.

L can be supplied directly by the workflow as a `Constant` or
`Trainable` source on L with positive-diagonal refinement, or
derived from a specified Σ at compile time. Non-MVN joints
that structurally factor as affine-in-noise trigger the same
pattern via `ReparameterizedSampleable` (§7.2).

#### 13.7 Structured Joint Samples, `.at()`, and Record-`~` Sugar

**Summary.** Structured joint draws produce one stochastic root with
named field projections. `.at("field_name")` is the canonical field
access. Record-`~` sugar destructures named fields ergonomically and
desugars to the same hidden joint root plus `.at()` projections. No
tuple destructuring or positional indexing.

For distributions returning structured samples (joints, named-
field-valued), `.at("field_name")` extracts a named field from a
single joint root:

```
joint_sample ~ PlantSizeJoint(mu, Sigma)
height = joint_sample.at("height")
diameter = joint_sample.at("diameter")
```

`.at()` accesses participate in e-graph identity: the same
`.at("height")` on the same sample collapses to one e-class
(so the field value is consistent everywhere it is read).
`.at()` on a missing field is a compile error — the family
declares its named fields statically.

The source language also admits record-`~` sugar for the common
case where the modeler wants the named fields directly:

```myco
{ height, diameter } ~ PlantSizeJoint(mu, Sigma)
{ height: h, diameter: d } ~ PlantSizeJoint(mu, Sigma)
```

Both forms desugar to a hidden synthetic joint root plus deterministic
field projections:

```myco
let __joint ~ PlantSizeJoint(mu, Sigma)
height = __joint.at("height")
diameter = __joint.at("diameter")
```

One coupled record-`~` site creates one stochastic root. Field
projections are deterministic reads from that root; the joint family
owns dependence. Record syntax is the only destructuring sugar:
there is no tuple destructuring and no positional index access.

#### 13.8 Observation Injection and Likelihood Back-Propagation

**Summary.** Workflow `observe(path, data)` attaches observed data
as a layer-2 envelope fact on the observed e-class (no equational
merge with the data). Downstream samples condition on it; the
relevant `D.log_density(data, logp)` term adds to the SCC's training
objective. Distinct from `identify`: observation narrows the
distribution, not the value.

`observe` is a workflow verb, not `.myco` source syntax. When a
workflow observes a path whose e-class carries `x ~ D`, the compiler
uses the following mechanism:

1. The observed value becomes an envelope fact on the e-class
   of the observed quantity (layer 2 of the three-layer split; §16). The e-class
   itself is not merged with a constant.
2. Downstream relations that read x's sampled value see the
   observation; downstream samples are conditioned on it.
3. Likelihood `D.log_density(data, logp)` contributes to the SCC's
   objective during training emission (§25); back-propagation through
   the model graph reaches learnable upstream parameters.

The critical distinction from `identify` (§17 merge source 4)
is that workflow observation does not make `x = data` equationally. It
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

Terminology. §17 source #2 ("workflow value injection")
and the probabilistic `observe` verb share the colloquial name
"observation" but are distinct mechanisms: value injection
collapses an e-class with a literal (layer 1); `observe`
attaches a distributional fact (layer 2). The distinction is
by layer, not by spelling.

#### 13.10 Tier 2 PPL Lock

**Summary.** The core Tier 2 PPL blockers are locked: B1 distribution
contract and opaque-family policy, B2 structured joint syntax, and
B4 coupling metadata. Higher-order process priors route through
process-prior and kernel machinery (§28); remaining work is
family-catalog polish, not unresolved core semantics.

The Tier 2 PPL design lock extends the core `~` mechanism to cover
structured stochastic values without adding user annotations or
imperative correlation surfaces:

- **Distribution contract shape / B1.** `Distribution<S>` is over a
  sample type, not only a scalar unit. Visible user-authored
  distributions expose a relation-shaped `log_density(self, sample,
  out)` obligation; `density` / `pdf` is a default-derived
  convenience, and sampling is a backend/runtime capability rather
  than an ordinary user relation. Curated opaque stdlib/backend
  families are Tier-C-first and fact-poor unless a visible rewrite
  or audited backend capability supplies a fact.
- **Joint declaration syntax / B2.** The canonical semantic form is
  one structured joint root plus named `.at()` projections. Record-`~`
  sugar (§13.7) is allowed and desugars to that root; tuple and
  positional destructuring are not part of the language.
- **Coupling machinery / B4.** Coupling lives as joint-envelope
  metadata on the structured stochastic root. Fields from the same
  root are dependent by default unless the joint envelope proves an
  independent partition or dependency graph. Distinct field names do
  not prove independence.
- **Higher-order distributions.** Process-valued priors such as
  Gaussian processes route through the process-prior and kernel
  machinery (§28), not through the parametric Tier 1 list.

Tier 1 primitives (§27) remain the current ship surface. Tier 2
family-catalog details such as copula and Wishart-family capability
tables can now build on the locked mechanics above.

### 14. Compiler Intrinsics

**Summary.** The intrinsics the compiler surfaces to modelers:
`deriv`, `integrate`, `condition_of` (Levels I symbolic / II
algorithmic / III runtime), `cost_of` (planner/extraction economics),
and `objective_terms` (training-objective decomposition). Each
intrinsic has defined e-graph interaction and documented guarantees.

`deriv`, `integrate`, `condition_of` (Levels I symbolic / II algorithmic
/ III runtime), `cost_of`, `objective_terms`. What each intrinsic
means, what the compiler guarantees about it, how it interacts with
the e-graph.

#### 14.1 `condition_of` — Levels I, II, III

**Summary.** `condition_of(expr)` returns a structured
`ConditionRecord` at one of three levels: symbolic (Level I,
problem-intrinsic), algorithmic (Level II, lowering-dependent), or
runtime (Level III, numerically computed). The level is tagged on the
return. Primary consumer: Y4 `condition_weighted` closure policy.

`condition_of(expr)` returns a `ConditionRecord` for an expression. The
record has view slots matching the envelope views (§16.4):

- `entrywise` — per-component sensitivity or amplification facts.
- `norm` — normwise conditioning / perturbation amplification.
- `spectral` — singular-value / eigenvalue conditioning facts.
- `structural` — exact eligibility, scaling, rank, factorization, or
  degeneracy facts that condition interpretation depends on.
- `summary` — optional derived scalar or selected view used for ranking,
  always with provenance naming which view / rule produced it.

A scalar-only operation may populate a one-component `entrywise` record
and a matching `summary`. A scalar summary is never the canonical
condition representation for matrix, tensor, PDE, or multi-output
operations.

Three levels of evaluation are tagged in the return type so downstream
code can distinguish:

- **Level I — Symbolic.** Closed-form conditioning facts derived
  from the e-graph's algebraic structure (e.g., triangular-solve
  conditioning against its diagonal). Available when the expression's
  conditioning is itself a closed-form function of the inputs.
- **Level II — Algorithmic.** Conditioning facts for a specific
  algorithm realizing the expression (e.g., Gaussian elimination's
  pivot-dependent conditioning when applied to a given matrix),
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

Extraction ranking (§19 cost/extraction) consumes Level I and
Level II only. Level III requires runtime numerical computation
and is unavailable to closure policies at extraction time.
Diagnostic surfaces (§22) can expose Level III at post-run
inspection.

The algorithmic-vs-problem duality is concrete in practice.
`(exp(x) - 1) / x` and `expm1(x) / x` compute the same
mathematical value: at small `x` the problem is well-conditioned
(Level I tight), but the naive algorithm suffers catastrophic
cancellation (Level II loose); the `expm1` algorithm holds
Level II tight. For a linear solve `A x = b`, Level I
conditioning is κ(A); Level II depends on the algorithm the
compiler chose: Gaussian elimination tracks pivot quality,
QR tracks the Q factor. The distinction is inspectable at
compile time without running the model.

For a solve, the `ConditionRecord` may hold spectral facts about `A`,
normwise residual-amplification facts for the chosen solve block, and
structural facts such as `positive_definite(A)` or
`used_factorization = cholesky`.

#### 14.2 `cost_of` and `objective_terms`

**Summary.** Chunk 12 resolves the cost/loss naming split:
`cost_of(expr)` is compiler/planner economics for extraction and
diagnostics, while `objective_terms(residual)` is workflow-facing
training-objective decomposition. Neither returns a scalar objective
by default; scalarization is workflow policy.

`cost_of(expr)` returns an extraction-cost record for a compiler
expression or residual candidate. The canonical fields are:

- `compute` — estimated operation count, memory bandwidth pressure,
  and backend-kernel availability as a lowering-time resource cost.
- `memory` — peak allocation and intermediate-buffer pressure.
- `approximation` — contribution from authorized approximate rewrites
  and workflow-authorized extraction approximations such as guided
  closure search (§8.7, §15, §15.6, Appendix C).
- `condition` — structured `ConditionRecord` consumed from
  `condition_of` Levels I/II where available (§14.1).
- `truncation` — finite-support, quadrature, iteration, or finite-
  horizon truncation contribution.
- `discretization` — mesh, timestep, stencil, or sampling-grid
  discretization contribution.

The first two fields are resource economics; the latter four are
faithfulness / numerical-quality economics. Extraction (§19.1) uses
the full record. Approximation diagnostics may project only the
faithfulness fields.

If a baseline default-off rewrite is `promoted_exact_in_context`
(§15.3, §17.6), its contribution to `cost_of().approximation` is zero
for that site. The cost record keeps a provenance pointer to the
promotion proof instead of treating the rewrite as an authorized
approximation.

When multiple authorized approximations affect one extracted candidate,
`cost_of().approximation` composes their terms by §15.6. The default is
a conservative monotone bound; sharper composition requires stdlib,
compiler, or provider evidence.

Guided closure search records its approximation term in
`cost_of().approximation` rather than adding a new canonical cost field.
Its compute and memory effects still contribute to `compute` and
`memory`; its acceptance criteria, searched subsystem count, total
subsystem bound, ordering facts, and `ResidualSite` provenance remain
inside the structured approximation record for diagnostics.

`cost_of().condition` is not a scalar field. It is a `ConditionRecord`
with `entrywise`, `norm`, `spectral`, `structural`, and optional
`summary` slots (§14.1, §16.4). Extraction may use the `summary` slot
for ranking only when a named rule explains how it was derived. If no
summary is available, condition remains a structured cost dimension and
workflow extraction policy decides whether the candidate is admissible.

`objective_terms(residual)` consumes a `ResidualSite` (§19.2) and
returns named training-objective components, not a scalar. Fields cover
the site's training sources:

- `data_fit` — likelihood / observation mismatch terms.
- `constraint_violation` — projection/penalty terms from
  `constraint` blocks (§8.1) not discharged at compile time.
- `regularization` — prior log-densities on learned parameters.

Users select components by name for training (§25) — e.g.,
`objective_terms(residual).data_fit + 0.1 *
objective_terms(residual).regularization`. Aggregation to a scalar is
the workflow's call. The compiler does not auto-sum; scalar objective
construction is workflow composition, not a language default.

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

Weak and residual forms are ordinary integration statements. A `for test`
loop over a `test_space<T>(locus)` is universal quantification over
semantic test functions in the source model. It does not select a
finite-dimensional basis or create a solver hint. Lowering may later
choose finite basis functions, quadrature, matrix assembly, or
matrix-free action through a realization provider (§37.1), with the
chosen discretization recorded as a plan artifact and, when approximate,
in `cost_of().discretization`.

#### 14.4 `deriv` — Symbolic, Algorithmic, Runtime

**Summary.** `deriv(f, x)` returns the derivative of `f` with
respect to `x`. The compiler resolves it through three ordered
lowering modes: symbolic (e-graph closes it at compile time),
algorithmic (compile-time chain-rule expansion via capability
contracts), and runtime (backend autodiff for SCCs the compiler
cannot expand symbolically).

- **Symbolic.** Stdlib atoms carry `Differentiable` capability
  contracts (§7.2); composition rules fire as A-group rewrites
  (§17, Appendix C). `deriv(sin(x), x)` rewrites to `cos(x)`
  at compile time. No runtime cost; the derivative collapses
  entirely in the equational core (Layer 1).
- **Algorithmic.** When the expression composes `Differentiable`
  atoms but symbolic simplification does not terminate (e.g.,
  deeply nested compositions), the compiler emits a structural
  chain-rule expansion using the atom-level derivatives.
  Still compile-time; no runtime AD. Materializes as A-group
  rewrites.
- **Runtime.** When the SCC exceeds a size threshold or contains
  unexpanded closure policies, `deriv` lowers to the backend's
  autodiff facility. Runtime AD is the fallback for large SCCs under
  the hybrid AD boundary (§31); it does not participate in the
  equational core and does not grant symbolic derivative facts unless
  the compiler separately derives the same structure or an audited
  backend capability explicitly certifies it.

The chosen mode is inspectable via `.mode` on the `deriv`
return, matching `condition_of`'s accessor pattern. `deriv`
is valid only on expressions composing `Differentiable`-tagged
atoms.

### 15. Approximate Blocks

**Summary.** `approximate` blocks authorize specific lossy rewrites
for a named scope with declared tolerance class and error bound. The
compiler derives expression lossiness from five cumulative sources
(atom contracts, approximation declarations, numeric types, backend
emulation, and workflow-authorized extraction approximations) and cuts
it into three tiers: lossless, lossy-model, lossy-tolerance.

Approximation flavors organize along two orthogonal axes: a
faithfulness axis (strict / approximate / fuzzy) and an orientation
axis (bidirectional / unidirectional). The 2x3 matrix these axes
define covers every `approximate` block the compiler can authorize.
The strict cell is degenerate (strict rewrites never require an
`approximate` block), so in practice §15 concerns the approximate
and fuzzy rows. Within the fuzzy row, fuzzy-model rewrites (L-group,
M-group lossy-model) carry a modeler-chosen distortion that the
model's equations encode; fuzzy-tolerance rewrites (K-group,
M-group tolerance, Q-group) carry a solver-level numerical tolerance
that is independent of model structure. Appendix C's summary table
organizes the full A-Z catalog by faithfulness x orientation; each
cell has concrete examples there.

The three-tier cut of §15.3 is the trust-posture projection of the
faithfulness axis. Lossless corresponds to the strict row; lossy-model
to fuzzy-model; lossy-tolerance to fuzzy-tolerance. The three-tier
labels are the diagnostic and dispatch-relevant names for those
cells.

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
  (K-group fuzzy-tolerance, M-group, L/Q-group lossy-model,
  Tier B Z-group conjugates; see Appendix C for the closed
  catalog).
- `tolerance_class` declares how error is measured (§16.4).
- `error_bound` is the user's commitment to acceptable error
  magnitude; the compiler rejects the rewrite if its
  certified bound exceeds this.
- `body` scopes the rewrite to a specific expression or
  residual block.
- `where` optionally gates applicability on input conditions
  (e.g., `where: variance / mean^2 < 0.1` for Delta-method
  linearization).

Exactly one of `under` and `tolerance_class` is required per
block. `under` names a specific rewrite and derives the tolerance
class from that rewrite's certification; `tolerance_class` names a
class and leaves the rewrite selection to the compiler subject to
the class. Specifying both is a compile error; specifying neither
is a compile error.

Blocks compose by nesting. Outside a block's `body`, the
authorized rewrite does not fire. No global `approximate`
scope exists; approximation is always explicitly chosen. Nested
or otherwise stacked authorized approximation terms compose by
§15.6.

#### 15.2 Auto-Derived Lossiness (Five Sources)

**Summary.** Expression lossiness is a lattice join over five
sources: stdlib atom contracts, approximation-block declarations,
numeric type choices, backend emulation paths, and workflow-authorized
extraction approximations. The compiler reports the aggregate via
inspection surfaces.

The compiler derives an expression's lossiness from five
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
5. **Workflow-authorized extraction approximations.** If the workflow
   accepts a non-exhaustive extraction strategy such as guided subsystem
   search (§8.7), the accepted search policy, budget, and acceptance
   criteria enter the approximation record. This source is booked at
   Layer 4 (§15.4); it is not a source-language `approximate` block and
   not an exact Y6 reduction.

The compiler reports the aggregate lossiness per expression
via inspection surfaces (§22). The five sources are
independent contributions; lossiness is a lattice join over
them, not a single authoritative source. Sampling parameters
used to empirically estimate error bounds (sample count, seed,
stratification) live workflow-side per CC1; the `.myco`
`approximate` block names the rewrite and bound, and the
workflow's `run.config` surfaces the numerical parameters (§24).

The five sources are the *origin* axis of lossiness; the
*accounting* axis — where in the compile stack the lossiness
is quantified — is the five-layer stack in §15.4. The two axes
are orthogonal: a single rewrite or extraction action carries both a
source label (one of five) and a layer label (one of five).

#### 15.3 Three-Tier Lossiness Cut

**Summary.** Lossiness groups into three tiers for diagnostics and
Tier B dispatch: lossless (equational rewrites only), lossy-model
(modeler-chosen approximations), and lossy-tolerance (run-level
numerical or search tolerance). Each tier is surfaced distinctly in
diagnostics.

For diagnostics and Tier B dispatch (§13.2), lossiness groups
into three tiers:

- **Lossless.** Equational rewrites only; no numerical error
  beyond the base numeric type. `log(exp(x)) = x` under
  `Invertible`, stdlib identity rewrites.
- **Lossy-model.** Modeler-chosen approximations — smoothing
  helpers (§8.9), closed-form statistical approximations
  (Delta method, CLT, Fenton-Wilkinson). The model itself is
  an approximation; the compiler surfaces which one.
- **Lossy-tolerance.** Numerical or search tolerance intrinsic to the
  run's chosen execution / extraction strategy: floating-point rounding,
  quadrature truncation, iteration-convergence tolerance, or
  workflow-authorized guided closure search. Independent of source
  model intent; bounded by the backend, residual conditioning, and
  explicit workflow acceptance criteria.

The cut lets diagnostics say "this output is exact modulo
floating-point" vs "this output uses a Delta-method
linearization the modeler authorized" vs "this output is
a tolerance-gated iterative solve or workflow-budgeted guided
closure search." Three different trust postures, surfaced distinctly.

Envelope metadata (§16, Layer 2) can narrow a rewrite's
effective error class in context. A rewrite that is normally
lossy-tolerance becomes lossless-in-context when the envelope
proves the error bound collapses to zero, for example when an
admissibility bound collapses under a refinement. In that case,
a rewrite that is normally default-off may fire without an explicit
`approximate` declaration, but its baseline partition does not change.
The rewrite trace records `promoted_exact_in_context` with the zero
error-bound proof. It contributes zero to `cost_of().approximation`
while remaining visible to diagnostics. The mechanism is canonical
here; §17.6 carries the corresponding corollary for the rewrite-
predicate language.

#### 15.4 Five-Layer Lossiness Accounting

**Summary.** Lossiness is quantified at five layers of the
compile stack: syntactic, distributional-envelope, structural-
identification, seam-state, and extraction-cost. Orthogonal to
the five-source origin taxonomy (§15.2); each rewrite or extraction
action carries both a source label and a layer label. The layer axis
tells diagnostics *where the distortion is booked*; the source axis
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
  only at residual-projection time: `cost_of`-guided
  extraction picks one among multiple valid representations
  (Y-group closure policies, cost-struct tradeoffs §19.1) or a
  workflow-authorized bounded search accepts a non-exhaustive closure
  result (§8.7). Exact Y6 with certified reductions is lossless; guided
  subsystem search that terminates without an exhaustive proof is booked
  here. Underlying rewrites may still be layer-1 or layer-2 lossless;
  the extraction commitment carries accounting when extraction commits
  to one representative, budget, or accepted approximate search result.

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

#### 15.5 Declaration/Derivation Interaction

**Summary.** When a user declares an error bound and the compiler
derives one independently, three outcomes cover all cases: the
compiler proves a tighter or exact result (user's declaration is
recorded; compiler's result is used); the compiler's derived bound
is looser than the user's but within the declared tolerance
(authorized); or the compiler's derived bound exceeds the user's
declaration (compile error with a diagnostic naming both bounds).

The `error_bound` field in an `approximate` block is a user
commitment. The compiler derives its own bound independently from
the rewrite's certification and the envelope facts at the call
site. Three cases exhaust the interaction:

- **(a) Compiler proves exact.** The compiler's derived bound is
  tighter than or equal to the user's declaration, including the
  degenerate case where the compiler proves the rewrite is exact
  in context. The user's declaration is retained in provenance;
  the compiler's tighter result governs.
- **(b) Compiler within user declaration.** The compiler's derived
  bound is looser than the user's declaration but still within the
  declared tolerance (the derived bound does not exceed the declared
  one). The block is authorized.
- **(c) Compiler disproves declaration.** The compiler's derived
  bound exceeds the user's declaration. The compiler emits a compile
  error naming both bounds and the rewrite in question. The user
  must either widen the `error_bound` or choose a different rewrite.

#### 15.6 Approximation Composition

**Summary.** Stacked approximations compose conservatively by default.
The compiler never assumes cancellation or independence. Stdlib /
compiler rules and evidence-graded provider facts may sharpen the bound;
`hypha explain` reports every approximation term, composition rule, and
looseness warning.

Each authorized approximate rewrite emits an approximation term with:

- rewrite id and source block;
- tolerance class / envelope view (§16.4);
- local certified error bound;
- expression site and downstream expression it affects;
- provenance, including any provider or stdlib evidence.

For an extracted candidate, the compiler first propagates each local
term to the candidate's requested tolerance view using named rules. It
then composes terms in this order:

1. **Exact-zero removal.** Terms from `promoted_exact_in_context`
   rewrites contribute zero but remain visible in provenance.
2. **Declared composition law.** A stdlib, compiler, or evidence-graded
   provider law may compose terms more sharply. Examples include
   Lipschitz propagation, dominance (`max(a, b)` when one bound subsumes
   another), orthogonal / independent RMS composition when independence
   is proven, or a closed-form law for a named approximation family.
3. **Conservative monotone fallback.** If no law applies, comparable
   terms compose by conservative summation in the same tolerance view.
   The compiler never assumes cancellation, anticorrelation, or
   independence from syntax alone.

If terms cannot be converted to a common tolerance view, the composed
`approximation` field remains a structured record rather than a scalar
sum, and extraction diagnostics mark the candidate as carrying
`uncomposed_approximation_terms`. This is still a valid plan when the
workflow accepts the relevant fields, but `hypha explain` must surface
the missing composition law.

Provider-supplied composition laws are ordinary evidence-graded facts
(§37.1). They may sharpen a run's `cost_of().approximation`, but they do
not become global source-level theorems.

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
variables; each SCC becomes a residual block under §20's
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
(§20), residual flavor (§19), and tolerance envelope (§16.4). The
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
(3) adjacent keyed state (event firings, SCC results, semantic site
records, provider bindings, sampling traces, event-trigger flags).

#### 16.1 Three-Layer Scoping Split

**Summary.** Three concentric layers: equational core (union-find
under monotonic merge, one per-run instance), envelope metadata
(facts keyed by e-class narrowing without merging, including
provenance and merge-edge annotations), adjacent keyed state
(event firings, selected-handle sites, spatial / weak / transfer
site records, SCC decomposition results, provider bindings,
sampling traces, event-trigger state). Merge
sources write layer 1; contracts, observations, and backend emulation
write layer 2; event firings, selector sites, and keyed identifiers
index layer 3.

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
   (§16.4), provenance (declaring construct and rewrite trace
   for every envelope fact). Merge-edge annotations (faithfulness
   tag in {strict, fuzzy-model, fuzzy-tolerance,
   distribution-family, one-way}; orientation tag in
   {bidirectional, unidirectional}) are layer-2 content attached
   to the merge edge, not to the merged e-class itself (§15,
   Appendix C). Monotonic in aggregate (facts compose; none
   retract).

3. **Adjacent keyed state (layer 3).** Structures indexed by key
   (event firing, identity tag, selector site, SCC identifier, draw
   ID, provider handle) holding e-class references internally. Per-key
   updates are independent; keys do not interact equationally except
   via explicit relations. Content types:

   - Per-event copies keyed on event firing (§10).
   - Identity-tagged instances of heterogeneous collections.
   - `SelectedSite` records keyed on selector expression identity,
     holding selected-handle provenance, empty behavior, tie policy,
     result contract, existence domain, and lowering metadata (§12.2).
   - `SpatialOperatorSite`, `WeakFormSite`, `ResidualFormSite`, and
     `TransferSite` records keyed on source expression identity and
     topology version, holding locus, operator family, test-space /
     interface metadata, and realization links (§11.1, §37.1).
   - `DiscreteOperatorSite` records keyed on continuous site, topology
     version, provider, backend, and artifact identity, holding realized
     executable artifact facts and evidence grades (§28.6, §37.1).
   - SCC decomposition results keyed on SCC identifier; carries
     the canonical four-way class assignment (static / dynamic /
     stochastic / training; §20) plus any lowering solver-strategy
     metadata (algebraic, fixed-point, iterative-solve, stepper).
   - Workflow provider bindings keyed on the handle identifying
     which workflow-side component supplied a value, observation,
     or learned parameter (§24).
   - Stochastic sampling traces keyed on draw ID (§13).
   - Runtime event-trigger state keyed on event timestamp for
     edge-triggered `when` clauses (§10).

   Temporal subscripts (`y[t]`, `y[t+1]`) are layer-1 distinct
   ground terms; each per-tick copy is its own e-class.

Layer choice is how a construct participates. Merges write
layer 1; contracts, observations, and backend emulation write
layer 2; event firings and keyed identifiers index layer 3.
Downstream consumers read the layer relevant to their task;
diagnostics surface which layer a fact lives on (§22).

#### 16.2 Monotonicity Invariant

**Summary.** Append-only. Merged e-classes stay merged; attached
envelope facts stay attached. No retraction, tombstoning, or
rollback. Obligation fulfillment selects candidates before e-graph
emission; it does not retract emitted facts. Events add facts; dead
entities continue to exist equationally. Compilation is a left-to-right
accumulation.

The equational core is append-only. Once two e-classes merge,
they stay merged; once an envelope fact attaches, it stays
attached. No retraction, no tombstoning, no rollback. This is
the substrate-level version of referential truth (§0 principle
5): world-claims accumulate; they do not unwrite.

Consequences:

- Obligation fulfillment (§8.11, §10.5) selects explicit or default
  candidates before layer-1 emission. Suppressed defaults remain
  inspectable ledger candidates; they are not already-emitted facts
  being retracted.
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

**Summary.** Envelope facts have four writers (stdlib contracts,
compiler rewrites, workflow `observe`, backend emulation), four
readers (Tier A/B dispatch, extraction pipeline, diagnostics,
plan inspection), and no invalidator. Conflicting facts emit a
coherence error rather than silent preference.

Envelope facts (layer 2) have four classes of writer, four
readers, and no invalidator:

**Writers.**
- **Stdlib contracts.** Capability advertisements (`Invertible`,
  `Differentiable`, `AffineSelfClosed`, etc.) attach on type
  or family declaration.
- **Compiler rewrites.** Tier B approximations, refinement
  inference from relation bodies, conservation-group
  induction from `{ conserved }`.
- **`observe` verb (workflow).** Attaches observation envelope
  facts at workflow composition time (§13.8, §13.9).
- **Backend emulation.** When a backend emulates a missing
  capability under workflow authorization (§31.1), the
  emulation path's error class attaches as a layer-2
  lossy-tolerance envelope fact on the affected e-classes.

**Readers.**
- **Tier A/B dispatch** (§13.2) consumes capability facts to
  select closed-form or approximate routing.
- **Extraction pipeline** (§19) reads refinement and tolerance
  facts to choose projection flavors.
- **Diagnostics / `hypha explain`** (§22) reads every envelope
  fact and surfaces provenance.
- **Plan inspection** reads envelope facts to report the
  derivation chain visible to workflow tooling.

Provenance composes by set union when two envelope facts merge
onto the same e-class; no provenance is dropped.

**Invalidators.** None. The monotonicity invariant (§16.2)
forbids retraction; envelope facts are as permanent as
equational merges. If a fact conflicts with a later fact, the
compiler emits a coherence error rather than silently
preferring one.

#### 16.4 Envelope Flavors

**Summary.** Envelopes for scalar and matrix-valued expressions are
multi-view bundles attached to e-classes. The four standard views are
entry-wise, norm, spectral, and structural. No view is canonical, and
the compiler does not coerce one view into another unless a named rule
proves the implication. Each primitive declares which views it consumes
and emits; contradictions become refuted facts or unmet obligations.

Tolerance envelopes (a subclass of envelope facts) carry one or more
parallel views declaring how error, bounds, or exact structural
certificates are represented:

- **Entry-wise.** Per-element bounds such as
  `A[i,j] in [lo[i,j], hi[i,j]]`. Used for scalar fields,
  component-wise vector results, sign checks, diagonal positivity,
  provider-validation diagnostics, and elementwise arithmetic.
- **Norm.** Matrix / vector norm bounds such as `||A||_2 <= c`,
  `||A - A_approx||_F <= eps`, or named operator-norm bounds.
  Used for perturbation analysis, matmul error propagation,
  solver-error bounds, approximation accounting, and
  `condition_of`.
- **Spectral.** Eigenvalue / singular-value facts such as
  `lambda_min(A) >= a`, `lambda_max(A) <= b`,
  `sigma_min(A) >= a`, `sigma_max(A) <= b`, or
  `spectral_radius(A) <= r`. Used for positive-definiteness,
  stability, condition bounds, Cholesky eligibility, implicit-solve
  safety, and covariance validity.
- **Structural.** Exact certificates such as `symmetric(A)`,
  `diagonal(A)`, `lower_triangular(A)`, `row_sum_zero(A)`,
  `graph_laplacian(A)`, `block_diagonal(A, blocks)`, or
  `zero_pattern(A)`. This is the zero-numerical-tolerance view:
  either the structural property holds or the fact is refuted /
  unavailable.

These views are parallel. A matrix envelope may contain entry-wise
bounds, several norm bounds, spectral intervals, and structural
certificates simultaneously. There is no single canonical envelope
representation into which the others must project.

`ConditionRecord` (§14.1) reuses the same view vocabulary for
conditioning costs. This keeps matrix solves, eigensystems, tensor
operators, and PDE residuals from collapsing entry-wise sensitivity,
normwise amplification, spectral conditioning, and structural
eligibility into one canonical scalar.

Merge and join behavior is per view:

- Entry-wise joins intersect or widen compatible interval records
  according to the evidence source and monotonicity rule.
- Norm joins keep named norm bounds and derive tighter composite
  bounds only when a rule is available.
- Spectral joins intersect compatible eigenvalue / singular-value
  intervals and otherwise retain separate evidence records.
- Structural joins union exact certificates and run contradiction
  checks. `positive_definite(A)` and `negative_semidefinite(A)` on
  the same e-class, for example, produce a coherence diagnostic
  unless another fact restricts the case to a compatible degenerate
  condition.

Propagation is primitive-specific. Examples:

- `A + B` consumes entry-wise and norm views; it emits interval
  addition and norm bounds via triangle inequality.
- `A * B` consumes norm views strongly and entry-wise views only
  where a stdlib rule supplies the needed bound; it emits norm bounds
  via sub-multiplicativity.
- `cholesky(A)` consumes structural and spectral views
  (`symmetric`, `positive_definite`, factorable unit law); it emits
  `lower_triangular(L)`, `positive_diagonal(L)`, and the factorization
  identity.
- Spatial-operator lowering may emit structural views such as
  `graph_laplacian`, `row_sum_zero`, `stencil_pattern`, plus spectral
  sign facts when the discretization proves them.

`approximate` blocks (§15.1) declare a `tolerance_class` naming the
view in which the bound is measured. Tier B rewrites (§13.2) route
via the declared view to the appropriate approximation family. A fact
in one view never silently implies a fact in another: entry-wise
bounds do not prove PSD, PSD does not provide useful entry-wise
bounds, norm bounds do not imply symmetry, and symmetry does not imply
positive definiteness without an additional rule.

### 17. Equality-Introducing Machinery

**Summary.** Eight enumerated merge sources for the equational core:
explicit relation equations, workflow value injection, algebraic
rewrites, `identify`, stdlib-declared inverses, named-type
conversion, closure-policy co-membership, unit-preserving rewrites.
Unified rewrite-predicate language, A-Z rule groupings
(Appendix C), `identify` vs relation `=` distinction.

Eight enumerated merge sources: explicit relation equations,
workflow value injection, algebraic rewrites, `identify`,
stdlib-declared inverses (via capability contracts on stdlib atoms;
see §6), named-type conversion, closure-policy co-membership,
unit-preserving rewrites. The 2x3 faithfulness x orientation matrix
covering `convert`, `identify`, `approximate`, relation `=`.
Unified rewrite-predicate language.

Terminology note. "Workflow value injection" is the merge source by
which a workflow-bound fixed source such as `bind(path, Constant(v))`
collapses the e-class of a model variable with the e-class of a
literal value. This is distinct from workflow `observe` (§13.8),
which attaches distributional metadata as an envelope fact and is not
a merge source. Two mechanisms, one unfortunately-similar colloquial
name; the distinction is by layer (§16.1), not by spelling.

#### 17.1 The Eight Authorization Sources — Prose

**Summary.** Exactly eight authorization sources write — directly
or via authorized rewrite classes — to the equational core:
explicit relation equations, workflow value injection, algebraic
rewrites, `identify`, stdlib-declared inverses, named-type
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
  equations, workflow value injection, algebraic rewrites,
  closure-policy co-membership, unit-preserving rewrites).
- **Rewrite-class authorizers.** The declaration installs a
  rewrite class (or a Layer-3 site record, §16.1) whose merges
  fire later when a structural or site predicate matches.
  Sources 4, 5, 6 (`identify` via Layer-3 site records
  consumed by X2; stdlib-declared inverses via
  capability contracts fed into E-group rewrites; named-type
  conversion via bidirectional rewrite installation).

Both mechanisms are first-class authorization sources; the
distinction is purely operational (when does the layer-1 merge
appear). Downstream tooling reads source tags uniformly.

1. **Explicit relation equations.** A `relation { x = expr }` or
   inline `x = expr` asserts an equation; the compiler merges
   the e-class of `x` with the e-class of `expr`. The canonical
   user-visible source.
2. **Workflow value injection.** `bind(path, Constant(v))` and
   equivalent fixed-source bindings (§24) collapse a model variable
   with a literal value supplied by the workflow. Mechanism: at
   workflow composition the binding becomes an equation
   `variable = <literal>` and fires as source 1. Distinct from
   workflow `observe`, which writes layer 2 (§13.8, §13.9).
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
5. **Stdlib-declared inverses.** Capability contracts
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
(diagnostics, `hypha explain`, provenance reporting) needs to
know which source produced any given merge. Source tags travel
with merges through the e-graph.

No silent inference. Layer-1 merges arise only via these eight
authorization sources. The compiler does not infer equality from
structural shape, type identity, name coincidence, or any signal
outside the enumerated authorizations. Every merge is traceable
to a source tag.

Obligation fulfillment (§8.11, §10.5) selects explicit or default
candidates before a layer-1 merge is emitted. A suppressed default is
not a retracted merge; it is an unselected ledger candidate.

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
`hypha explain` honest when a modeler writes two `identify`
calls intending to state different facts that happen to collapse
to the same layer-1 equation.

#### 17.3 Stdlib Inverses via Capability Contracts

**Summary.** Inverse merges fire from stdlib-declared
capability contracts (`Invertible<inv=log, domain=Real>` on `exp`),
not user annotations. Users extend the catalog by composition, not
declaration. The inverse catalog is inspectable from stdlib alone.

Inverse merges fire from stdlib-declared capability
contracts on atoms, not from user annotations. `exp` declares
`Invertible<inv=log, domain=Real>`; the e-graph then fires
`log(exp(x)) = x` wherever `x: Real` holds structurally (and
symmetrically for `exp(log(x)) = x` on `x: Positive`).

The user has no annotation path to declare a relation invertible.
Stdlib carries capability contracts; user-authored parameterized
relations have no property-declaration surface. If a user relation
needs inverse recognition the compiler cannot derive, the user
refactors it into structurally composable pieces using stdlib atoms
with the requisite contracts.

Consequence: the inverse rewrite catalog is
inspectable from the stdlib alone. Users cannot extend it by
annotation; they extend it by composition.

#### 17.4 Unified Rewrite-Predicate Language

**Summary.** All merge sources share one predicate language for
guards. Guard discharge can consult the type graph, envelope facts,
unit algebra, contract satisfaction, structural shape, site / geometry
facts, shape-phase facts, and backend capabilities. Compile-time only;
runtime values do not drive rewrites.

All merge sources use one predicate language for expressing
guards. A rewrite predicate can reference:

- Refinement predicates on participating types (`x: Scalar<m>
  where { x > 0 }`).
- Type-graph facts (§18): type constructors, conversion legality,
  unit dimensions, generic instantiations, `impl` monomorphs, and
  refinement implication rules.
- Capability satisfaction (`D : Distribution + AffineSelfClosed`).
- Structural shape (generic arity, tensor rank, contract
  satisfaction).
- Envelope facts (§16.3, §16.4), including value bounds,
  distributional facts, matrix facts, tolerance facts, and provenance.
- Site / geometry facts and adjacent keyed state where a rule is
  explicitly site-gated (§11, Appendix C X).
- Backend capability facts for lowering-sensitive guards (§31).
- Unit algebra (dimensional matching).

Predicates are compile-time only; runtime-observed values do
not drive rewrites. Guard discharge may query the evolving monotone
fact store during saturation (§18); a fact discovered by one rewrite
can unlock a later guarded rewrite, provided the fact is monotone and
provenance-tracked. The unified language means a Tier B approximate
rewrite (§13.2) uses the same predicate form as a stdlib algebraic
rewrite, which uses the same form as a `convert` body. One surface,
one discharge procedure.

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
  distributivity, identity elements. Symbolic-math intrinsics
  (`deriv`, `integrate`; §14.3, §14.4) participate via
  A-group rewrites on compositions of `Differentiable` atoms
  and stdlib integration-by-parts rules (Appendix C).
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
`identify`, workflow value injection) and default-off (fire only inside an
authorizing `approximate` block). Gives `.myco` its conservative
default: a model compiles with zero authorized approximations if the
modeler wrote none.

Rewrites partition into **default-on** and **default-off**
buckets:

- **Default-on.** Fire unconditionally whenever their
  predicate (§17.4) holds. Includes: relation-`=` merges,
  algebraic rewrites (A-group), stdlib inverse
  rewrites (E-group), named-type conversion, unit-preserving
  rewrites, `identify`, workflow value injections. All
  lossless or modeler-asserted.
- **Default-off.** Fire only inside an authorizing
  `approximate` block (§15.1). Includes: Tier B statistical
  approximations (Delta method, CLT, Fenton-Wilkinson),
  smoothing substitutions (`max` → `smooth_max`), numerical-
  tolerance rewrites that exceed the default precision.

The partition is what gives `.myco` its conservative default
posture. A model compiles with zero authorized approximations if
the modeler wrote none, and any lossy rewrite is traceable to a
specific block. Default-off rewrites fire one-at-a-time, scoped
to the block's `body`; they do not compose across blocks without
explicit nesting. When nesting creates stacked approximation terms,
their costs compose by §15.6.

Residual sites preserve their original relation names and obligation
keys, so training-emission diagnostics (§25) can expose per-residual
objective terms while extracted realizations still share simplified
compute.

Envelope-narrowing corollary. A default-off rewrite is
`promoted_exact_in_context` at sites where envelope metadata (§16.1
Layer 2) proves its certified error bound is zero. The rewrite may fire
without an explicit `approximate` declaration in that narrowed context,
but it remains tagged with its baseline `default-off` partition and the
promotion proof. Promotion is site-specific; it does not globally move
the rewrite into the default-on bucket.

### 18. The Type Graph

**Summary.** The type graph is a separate static semantic substrate
from the expression e-graph. It carries type constructors, contract
satisfaction, unit dimensions, conversion legality, generic
instantiations, and refinement implication rules. The e-graph owns
value equalities and rewrites. The two interact through a live
guard-discharge bridge: e-graph rewrites ask whether required facts
hold, and the guard may query the type graph plus monotone e-class
facts.

The type graph and expression e-graph are separate substrates with an
explicit bridge. This is the semantic model. A compiler may precompile
or cache type-graph facts into concrete rewrite guards when doing so
is sound, but that optimization does not erase the type graph
semantically. Myco rejects the one-graph model where types, values,
contracts, refinements, and conversion witnesses all become e-graph
terms. Equality and implication are different relations and remain
separate.

The type graph owns static semantic relationships:

- Type constructors and aliases (`Scalar<U>`, `Tensor<U, shape>`,
  `Matrix<U, m, n>`, `Vector<U, n>`), including definitional
  normalization before terms enter the e-graph.
- Contract satisfaction and `impl Contract` monomorph sets.
- Unit dimensions and conservation-group membership.
- Conversion legality: semantic edges with witnesses, faithfulness
  class, and obligations.
- Generic instantiations and invariant parameter relationships (§3.6).
- Refinement implication rules (`positive_definite(A)` implies
  `symmetric(A)` and `square(A)`, etc.).

The e-graph owns value equalities: relation equations, algebraic
rewrites, stdlib inverse rewrites, unit-preserving rewrites,
conversion-result terms, `identify`-authorized merges, and residual
candidate expressions (§17). A rewrite such as:

```text
solve(A, b) -> cholesky_solve(A, b)
```

is an e-graph rewrite. Its guard:

```text
positive_definite(A) and compatible_axes(A, b)
```

is discharged by querying available facts. Those facts may come from
the type graph, envelope metadata (§16.3, §16.4), matrix facts (§3.9),
unit algebra (§5), contract satisfaction (§7), site / geometry facts
(§11), shape-phase facts (§3.8), or backend capability facts (§31).

Guard discharge is live and monotone. Type-graph relations are static
after elaboration, but e-class envelope facts can grow during
saturation. A fact discovered by one rewrite can unlock a later
guarded rewrite as long as the fact is append-only and
provenance-tracked. Cached or precompiled guards are permitted as
performance strategy; they are not a semantic limit on online
derivation.

Refinements are facts with evidence and provenance, not casts and not
user-carried witness objects. If the compiler derives
`positive_definite(A)` from symmetric structure plus a spectral lower
bound, `A` remains the same value; the refinement fact attaches to
`A`'s e-class and can discharge later guards. There is no source-level
`A as PositiveDefiniteMatrix` trust boundary and no
`PositiveDefiniteWitness<A>` argument plumbing.

Conversion edges separate semantic legality from execution
realization. A legal edge records what it means to move from one
type/fact state to another. Extraction and lowering decide whether
that edge realizes as a view, copy, kernel, host route, backend
materialization, or other costed plan (§19.1, §21, §31). A legal edge
may be expensive or unsupported for a selected backend; an illegal
edge never becomes legal because it would be convenient.

Inventory boundary:

- **Type graph:** static semantic relationships between types, units,
  contracts, conversions, generics, and refinement facts.
- **E-graph:** value expressions, relation equalities, rewrite results,
  conversion-result terms, and residual candidates.
- **Envelope:** facts attached to value e-classes: bounds,
  distributional metadata, matrix facts, observations, tolerance facts,
  structural certificates, and provenance.
- **Adjacent keyed state:** runtime / keyed records that reference
  e-classes without asserting equality: events, geometry sites, spatial
  / weak / transfer sites, SCC records, selected-handle sites, provider
  handles, samples, dynamic collection identities, and backend run
  records.

### 19. Residual Graph (Projection)

**Summary.** The residual graph is a user-facing diagnostic view
projected from the e-graph via `cost_of`-guided extraction. It is
not a canonical form: different workflow cost preferences yield
different residuals. Subsections cover the cost model, projection
mechanics, residual classification, and saturation scheduling.

The residual graph as a user-facing diagnostic view projected from
the e-graph. Extraction decisions and what they yield. How
diagnostics reference which view.

#### 19.1 Extraction Cost Model

**Summary.** Residual extraction optimizes against a multi-dimensional
`cost_of` record, not a single scalar. Extraction returns a Pareto
front; workflow configuration selects a point. The same e-graph can
yield different residuals under different policies.

Residual extraction from the e-graph optimizes against a
**multi-dimensional `cost_of` record** (§14.2), not a single scalar.
The extractor consumes both resource economics (`compute`, `memory`)
and faithfulness / numerical-quality economics (`approximation`,
`condition`, `truncation`, `discretization`). The record replaces the
older private vocabulary of precision / latency / memory /
approximation-class axes.

Extraction returns a Pareto front in the cost space by default;
workflow configuration selects a specific point
(compute-first, memory-first, faithfulness-first, or weighted). No
default scalar weighting; the compiler does not assume one dimension
dominates. Extraction policy is selected workflow-side (§24) via
`run.config.extraction_policy`.

Promoted exact rewrites contribute zero to the `approximation` axis but
remain in the extraction trace with `promoted_exact_in_context`
provenance. They are not hidden inside the default-on ledger and they do
not consume an approximation budget. This lets a faithfulness-first
policy treat the extracted expression as exact while `hypha explain`
still shows that a normally default-off rewrite became exact only under
the local envelope.

Stacked approximation terms compose by §15.6. Extraction uses the
composed approximation record for Pareto comparison and retains the
term-level ledger for diagnostics. When the only available composition
rule is conservative summation, the candidate remains valid but carries
a loose-bound marker. When terms cannot be composed into one tolerance
view, the `approximation` field remains structured and the extractor
uses workflow policy to decide whether that candidate is admissible.
Guided closure search is one such structured term: it names the
`ResidualSite`, raw and reduced subsystem counts, searched count,
ordering facts, acceptance criteria, and result status.

Condition costs remain structured by default. A candidate may carry
entrywise, norm, spectral, and structural condition entries with
different evidence and levels (§14.1). Extraction may rank by
`cost_of().condition.summary` when present, but the summary is a derived
view, not the source of truth. If two candidates are incomparable across
condition views, extraction keeps both on the Pareto front unless the
workflow supplies a policy that selects between them.

Consequence: the same e-graph yields different residuals under
different workflow policies. The residual graph is a projection
*parameterized by cost preference*, not a canonical form.

#### 19.2 Residual ↔ E-Graph Projection Mechanics

**Summary.** Residual identity and residual computation are separate.
A `ResidualSite` records the source claim / obligation that must remain
visible to diagnostics and training. A `ResidualRealization` records
the executable expression or block selected by `cost_of`-guided
extraction. Extraction may share realizations aggressively, but it must
not merge residual identities.

The compiler records a `ResidualSite` for each workflow-visible claim
that can produce a residual:

- user relation equations and constraint obligations;
- obligation sites and fulfillment candidates, including package
  defaults such as `balance_zero(flux)`;
- observation / evidence residuals introduced by `observe`;
- training-mode consistency residuals for conditionally inconsistent
  or overconstrained relations;
- provider / topology validation residuals that survive composition;
- suppressed default candidates retained for diagnostics (§10.5).

A `ResidualSite` carries stable identity and provenance:

- **Site id.** A stable semantic key derived from relation name,
  obligation key, lhs / field path, locus, event phase, and axes. Source
  line number is provenance, not identity. If the compiler cannot derive
  a stable key, it emits a diagnostic and may require an explicit
  disambiguation in a future surface.
- **Source provenance.** Relation name, obligation key, source span,
  default-candidate origin, workflow evidence origin, or provider origin.
- **E-class anchors.** The lhs / rhs / argument e-classes that the site
  constrains or scores.
- **Semantic payload.** Units, axes, shape facts, refinements,
  contracts, SCC id, overdetermination classification, and kind
  (`constraint_violation`, `data_fit`, `regularization`,
  `provider_check`, etc.).
- **Activation status.** `active`, `suppressed`, `inactive`, or
  `candidate_default`, with provenance for the decision.
- **Realization links.** Chosen realization id plus alternative
  realizations on the extraction Pareto frontier when available.

The extractor then chooses a `ResidualRealization` for each active site
under `cost_of` (§19.1). A realization may be an explicit expression,
an implicit solve block, an opaque backend/PPL block, a projection
check, or a provider-owned validation. Multiple residual sites may
share one realization after algebraic simplification or CSE. This is
legal and desirable:

```text
ResidualSite(leaf.hydraulic_drop.flow_balance)
ResidualSite(leaf.measured_drop.flow_balance)
    -> shared ResidualRealization(R17)
```

The rule is: **extraction may share residual realizations, but it must
not merge residual identities.** Relation names, generated obligation
keys, observation ids, and workflow-visible residual names live on
`ResidualSite`s, not on extracted expressions. `objective_terms` (§14.2)
and training emission (§25) consume residual sites.

Root selection is site-driven. The residual graph's roots are the
active `ResidualSite`s relevant to the requested mode plus any
workflow-requested output query frontiers needed for diagnostics. A run
that merely simulates need not score observations. A fit, inference, or
score run activates the corresponding observation / objective /
inference sites and must handle them explicitly (§25).

Diagnostics expose both views:

- **By site.** The user-facing claims: source relation, obligation key,
  observation, unit, axes, status, and objective terms.
- **By realization.** The executable computations: expression/block,
  cost, backend lowering, shared users, and alternatives.

Duplicate or equivalent active sites are legal because they may encode
different scientific claims, but they are a common weighting footgun.
The compiler emits a warning when two active sites over the same axes,
units, and e-class anchors appear equivalent. Workflow objective
aggregation emits a second warning if it counts multiple sites sharing
the same realization without an explicit weight policy. Provably
inconsistent duplicates remain hard compile errors under §8.6.

This closes the CC3 / O4.3 name-preservation issue: aggressive algebraic
collapse is allowed for realizations, forbidden for site identity.

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
  error budget. Stacked authorized approximation terms compose
  by §15.6.
- **Promoted exact rewrites** are baseline default-off rewrites whose
  site-local envelope proves zero error (§17.6). They schedule with
  default-on rewrites for that site only, carry
  `promoted_exact_in_context` provenance, and do not consume an
  `approximate` block or error budget.
- **Scheduling priority.** Merges from explicit relation `=`
  and `identify` (sources 1 and 4, §17.1) fire first;
  algebraic and unit-preserving rewrites next; conversion and
  closure-policy last. Order affects extraction choice but
  not correctness; the final e-graph is determined by the
  rewrite set, not the order.
- **Closure-policy timing.** Closure-policy selection (Y1-Y6,
  §8.7) operates at extraction time. During saturation, multiple
  closure-policy candidates coexist as alternative e-class
  representations; selection among them happens when the extractor
  commits to a single residual term, guided by the workflow's
  closure-policy configuration.
- **Site-gated strict rewrites.** Site-gated strict rewrites
  (Appendix C X1 pole L'Hopital operator substitution; X2
  identify-via-Layer-3 site records consumed on field expressions
  over the geometry) fire in the algebraic/unit-preserving tier of
  the scheduling priority. They are strict, so they do not require
  `approximate` authorization.
- **Termination bound.** An absolute rewrite-count cap
  (workflow-configurable) prevents pathological
  non-terminating cases. Hitting the bound is a compile
  warning, not an error; the partial e-graph still extracts
  a residual. Practical models do not approach the bound.
- **Rational-denominator saturation.** Conjugate-multiplication
  rewrites on rational arithmetic (§26.3) can produce
  non-terminating saturation chains in pathological cases; the
  rewrite-count cap catches these. Open work on a non-cap-based
  termination argument for rational denominators is tracked in §35.

Non-confluent rewrite sets (rare; only possible via
`approximate` blocks that introduce oriented lossy rewrites in
both directions) are a compile error detected at block
elaboration, before saturation runs.

### 20. SCC Decomposition and Component Classification

**Summary.** The compiler decomposes the residual graph into
strongly-connected components and assigns each SCC a four-way class:
static, dynamic, stochastic, or training. The class pivots lowering
strategy, training emission, and backend dispatch. SCCs are formed
after workflow binding, so one source bundle can produce different
SCC partitions under different workflows.

After constraint collection, the compiler decomposes the residual
graph into strongly-connected components. Each SCC receives a four-
way classification: **static** (fully resolved pre-run), **dynamic**
(timestepped), **stochastic** (distributional; requires sampling or
closed-form marginalization), **training** (gradient-optimized).
Classification pivots lowering, training emission, and backend
dispatch.

Formation pipeline:

1. Expand generics, collection pools, events, loci, and workflow
   source bindings into concrete obligation keys (§9.2).
2. Saturate the e-graph under the authorized rewrite set (§19.4).
3. Project the requested residual view (§19).
4. Build the dependency graph over residual unknowns.
5. Decompose the graph into SCCs.
6. Classify each SCC by the strongest execution role visible in it.

Acyclic single-node components lower as forward computations. Coupled
components lower through the class-dispatched path below: dynamic
loops, stochastic handoff, training emission, or static prelude solve.

SCC decomposition runs after workflow binding because source objects
can collapse, expose, or promote dependencies. Binding a value as
`Constant` may make a component static; binding it as `Trainable` may
move the same component into training; binding a `Prior` can promote a
component into stochastic inference. A source bundle therefore has a
parameterized SCC shape, and each run record stores the actual
partition used.

Opaque `Controller` source objects join the SCCs that read or write
their paths as non-symbolic atoms. The compiler sees the controller's
contracts and dependency edges, not its body. If the SCC trains through
the controller, §24.3 and §31 own the runtime-gradient boundary.

Hierarchical SCC decomposition is permitted when a differentiable
outer objective depends on an inner implicit solve. The compiler
partitions such components into:

- **P.** Parameters and workflow sources held outside the inner solve.
- **D.** Direct forward computations feeding the solve.
- **X.** Inner unknowns solved by the implicit block.
- **Y.** Quantities consumed by the outer objective or downstream SCCs.

If the compiler cannot produce a valid inner/outer split, it emits a
diagnostic in the E0952 family naming the cycle and the edge that
prevents decomposition. When the split succeeds, gradients compose
over the condensation DAG in topological order; implicit-solve
gradients use the backend's root/linear-solve mechanism under the
hybrid AD boundary (§31).

Per-entity SCCs over homogeneous populations are vectorization
candidates: the backend may map independent entity-local SCCs with
`vmap`/batching on GPU. Cross-entity coupled SCCs remain scalar or
block-structured solves until the compiler can prove separability.
Shape-changing matrix SCCs use the explicit `ShapePhase` /
regime-boundary handlers of §3.8 (`CapacityMask`, `EventReplan`,
`DynamicKeyed`) rather than silently resizing during a solve.

SCC-role predicates may feed rewrite guards and diagnostics. For
example, a rewrite may require "this e-class is inside a training SCC"
or "this residual comes from an implicit dynamic solve." These facts
are derived compiler metadata, not user-authored annotations.

Solver-strategy labels such as algebraic, fixed-point,
iterative-solve, and stepper are lowering sub-dispatch metadata under
the four-way class, not additional SCC classes. Separately,
overdetermination classification (§19.3) describes equation-count and
consistency status. Diagnostics may show all of these labels, but the
canonical SCC class is the four-way execution role above.

### 21. Lowering

**Summary.** Lowering compiles the residual graph into a backend
artifact. Static and dynamic modules take distinct paths; each SCC
lowers per its class; dynamic topology uses explicit shape handlers
(`CapacityMask`, `EventReplan`, `DynamicKeyed`); temporal indexing
produces distinct ground terms rather than a template. Subsections
detail each mechanism.

N-max / alive-mask lowering is the `CapacityMask` path for dynamic
topology. `EventReplan` and `DynamicKeyed` are selected through
regime-boundary crossing policy (§24.6) and backend capability
advertising (§31.1). `y[t]` and `y[t+1]` remain distinct ground terms
(no per-timestep or template e-graph). Handoff to the backend.

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

Within a dynamic module, the compiler may still classify individual
collections as bind-static for lowering. A collection with no
event-time churn can skip mask-update machinery even though the
module as a whole has a runtime loop. This is an optimization and
inspection fact, not a separate module semantic.

#### 21.2 Four-Way SCC Lowering Targets

**Summary.** Each SCC class lowers through a distinct path: static
SCCs fold or prelude-evaluate; dynamic SCCs emit per-tick body code;
stochastic SCCs route to PPL tiers A, B, or C; training SCCs emit
gradient-producing computation with per-residual objective exposure. An
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
  tick t-1 via explicit temporal terms (§21.3). Dynamic SCC
  lowering may sub-dispatch to algebraic, fixed-point,
  iterative-solve, or stepper strategies. Assembled linear
  solves dispatch by matrix structural facts (§3.9, §30):
  triangular solve, Cholesky back-substitution for positive-
  definite systems, and LU-style general solve.
- **Stochastic SCCs.** Lowered to backend PPL primitives
  (§31) or an explicit sampler. Tier A closed-form
  marginals resolve at compile time; Tier B approximate
  rewrites pre-materialize their error-bounded form; Tier
  C hands off opaquely (§13.2).
- **Training SCCs.** Lowered to a gradient-producing
  computation. Loss exposure per residual (§25) enables
  workflow-selected scalar combinations; differentiability
  propagates through contained stdlib atoms via their
  `Differentiable` contracts (§7.2). Myco owns symbolic and
  algorithmic derivatives; runtime AD delegates to the selected
  backend (§31). Extracted residuals preserve original relation
  names through lowering so training emission can expose per-
  residual objective terms (§19.2, §25).

Class dominance: an SCC inherits the most expensive class
among its members. A stochastic variable inside an
otherwise dynamic SCC promotes the whole SCC to stochastic.
The compiler diagnoses the promotion at classification time
so the modeler can decide whether to split the SCC
structurally (by refactoring) or accept the promotion.

Lowering checks backend capability advertisements (§31.1). Missing
required capabilities fail by default; `host` and `emulate` fallback
modes are explicit run-config choices.

A single run targets one backend in the current design scope (§31.6,
§32.1). Opaque callables execute in that same backend context unless
the workflow isolates them into a separate run and passes their outputs
back as sources.

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

Symbolic claims are append-only; materialized runtime state is a
cache. A streaming executor may evict backend storage for old ticks
once the maximum lookback and requested outputs no longer need them,
but the plan's symbolic claims and provenance remain reproducible
from the source bundle and run record.

Long-rollout gradients are workflow-configurable. `full_BPTT`,
`truncated_BPTT(k)`, and `checkpointed` are backend-agnostic regime
names exposed through run-config (§24.5); backends choose the concrete
checkpointing primitive or scan representation.

Event-time topology mutation crosses a regime boundary (§8.10) rather
than mutating an SCC's tensor shape in place. A firing emits a
`TopologyDelta`; applying it to the active `TopologyVersion` produces a
new version whose live entities, edges, loci, axes, sparsity facts,
obligations, SCC family member, selected sites, residual sites, and
backend lowering artifacts are re-derived from source plus event
history. Prior-version ground terms remain valid historical facts.

The committed topology handlers are `runtime_bounded`,
`event_replan`, and `dynamic_keyed` (§3.8, §24.5, §31.1):

- **`runtime_bounded` / `CapacityMask`.** Tensor shapes remain fixed;
  events update alive masks, free lists, capacity records, and sparse /
  zero-pattern facts. Overflow is a runtime error (§21.4), not silent
  growth.
- **`event_replan` / `EventReplan`.** The executor stops at the event
  boundary, applies the topology delta, invalidates every derived plan
  artifact whose provenance mentions the changed topology version, and
  rebuilds or dispatches a cached plan for the new topology fingerprint.
- **`dynamic_keyed` / `DynamicKeyed`.** Runtime maps keyed by entity IDs
  represent changing axes directly. This is semantics-complete on the
  CPU reference backend and valid for backends that advertise direct
  dynamic-keyed support.

Invalidation is provenance-driven. At minimum, a topology delta can
invalidate collection membership, extracted tensor axes, shape and
sparsity facts, obligation-ledger entries, SCC decomposition, selected
handle sites, residual sites, spatial realization artifacts, and backend
plan caches. Incremental e-graph resaturation during a run is an
implementation target, not required for semantic correctness: any
incremental strategy must be equivalent to full replanning from the
source bundle, workflow intent config, and event history.

Mesh discretization lowering is resolved as site / provider artifact
lowering (Appendix C P1; §37.1). Source-level spatial operators, weak
forms, residual forms, and transfer statements remain semantic graph
content. Stdlib identities over those forms may still fire as ordinary
e-graph rewrites, but discretization itself is not an e-graph equality.
Layer-3 `SpatialOperatorSite`, `WeakFormSite`, `ResidualFormSite`, and
`TransferSite` records feed built-in or spore-shipped realization
providers, which return `DiscreteOperatorSite` execution artifacts with
provenance and evidence-graded facts. Pre-e-graph numerical replacement
of semantic operators with stencil / FEM / FV / backend code is
forbidden; pre-e-graph work may only parse, resolve, desugar, and create
semantic site records while preserving source provenance.

#### 21.4 N-max Slots and Alive Masks

**Summary.** Event-time collections lower to a fixed-capacity array
plus an alive-bit mask. N-max is declared at the collection, optionally
overridden by `bind_topology`. Allocation claims free slots in O(1);
retirement flips the bit but leaves equational history intact;
overflow is a runtime error, not silent growth.

Event-time collections (§12.4) lower to a fixed-capacity
array plus an alive mask. This is the concrete lowering for the
`runtime_bounded` / `CapacityMask` dynamic-topology mode (§3.8,
§24.6).

- **N-max selection.** The collection declares an N-max
  capacity at its declaration site. Workflow override via
  `bind_topology` (§24) is permitted up to a
  compile-enforced ceiling.
- **Alive mask.** One Boolean per slot, stored as a packed
  bitmap (or SIMD-lane-aligned on GPU backends).
  Iteration primitives (§12.6) gate kernel lanes via the
  mask; dead slots contribute no work without introducing
  divergent branches.
- **Bind-static collections.** In an otherwise dynamic module, a
  collection with no event-time churn may lower without mask-update
  machinery. It still lives inside the module's runtime loop if other
  SCCs require one; the optimization is per collection, not a module
  reclassification.
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

**Summary.** Plan inspection is a first-class workflow affordance.
The compiled plan exposes textual explanation, one machine-readable
IR, per-path `inspect`, and bounded symbolic `prove` queries. Rendered
graph visualizations are optional downstream tooling, not a v2.1
commitment.

The compiled plan is a canonical output artifact. Reproducibility
rests on the source bundle and workflow (§0.1), but the plan is the
unit users inspect when they want to understand what the compiler
proved, rewrote, approximated, lowered, or left unresolved.

#### 22.1 `hypha explain`

**Summary.** `hypha explain` emits a textual plan report and can emit
the canonical machine-readable plan IR. It reports SCCs, symbolic
resolutions, residuals, selected-handle sites, obligation-ledger status,
fallbacks, execution order, temporal state, workflow bindings, and
provenance.

The textual report includes:

- SCC classification and lowering target (§20, §21).
- Symbolic resolutions, closure-policy choices, and rewrite groups.
- Exact Y6 and guided closure details: raw subsystem count, certified
  reductions, reduced exact count, active workflow budget, searched
  subsystem count when approximate, acceptance criteria, result status,
  and `ResidualSite` attachment (§8.7, §19.2).
- Residual graph nodes, original relation names, and extraction costs
  (§19).
- Numerical fallbacks, backend / device capability requirements,
  selected topology handler, selected fallback policy, and resolved
  run-lock identity (§21.3, §31).
- Realization-provider selections: matched semantic site, selected
  provider and implementation API, emitted artifact ids, evidence
  grades, validation results, rejected providers, and lockfile hashes
  (§37.1).
- Execution order and temporal state requirements.
- Observation, source, and topology bindings that materialize the
  plan.
- Envelope facts: bounds, distributions, approximation tolerances,
  provenance, and rewrite traces (§16).
- Promoted exact rewrites: baseline partition, site, zero-error proof,
  envelope facts consumed, and resulting `cost_of().approximation = 0`
  entry (§15.3, §17.6, §19.1).
- Stacked approximation composition: terms, propagated bounds,
  composition rule (`conservative_sum`, declared law, or
  `uncomposed_approximation_terms`), evidence source, and looseness
  warning (§15.6, §19.1).
- Condition records: entrywise / norm / spectral / structural entries,
  level (I / II / III), provenance, missing scaling or structural
  obligations, and any derived scalar summary used for ranking (§14.1,
  §19.1).
- Regime boundaries and crossing-handler ledger entries: detected
  surfaces, continuity / derivative class, selected crossing policy,
  topology handler, and any workflow-authorized surrogate used by the
  run (§8.10, §24.6).
- Obligation ledger entries: keys, loci / events / guards, explicit
  fulfillments, selected package defaults, suppressed default
  candidates, unfulfilled obligations, and conflicts (§8.11).
- Selected-handle sites: selector primitive, input collection, selector
  expression, result type, empty behavior, tie policy, existence domain,
  hard-selection regime boundaries, and lowering strategy (§12.2).

`hypha explain --format ir` emits the canonical machine-readable IR.
Renderer targets such as Mermaid, D2, Graphviz, or Cytoscape may be
built on top of that IR, but the spec does not commit to renderer
output as part of v2.1.

`hypha explain --vs path_A path_B` is a committed comparison affordance
for explaining why two paths share or do not share an e-class after
rewrite saturation. Exact e-class handle syntax, residual-to-e-class
round trips, and materializing a residual node back into source-like
form remain chunk 04 Phase 2 work (§35).

#### 22.2 `inspect(path)`

**Summary.** `inspect(path)` asks what the plan currently knows about
a node. It returns a symbolic expression or residual frontier,
free-variable set, status, envelope facts, dependencies, and
reduction trace.

Representative result fields:

- `realization` — `explicit(expr)`, `implicit(residual_block)`,
  `selected(handle_site)`, or `opaque(provider)`, naming whether the
  path has a forward expression, an implicit residual realization, a
  selected-reference provenance site, or a provider-owned value such as
  a `Controller` source.
- `expression` — canonical expression if the path reduces to one.
- `free_variables` — workflow sources or unresolved symbols required
  to ground the expression.
- `status` — `ground`, `symbolic`, `overdetermined`,
  `inconsistent`, or `unresolved`.
- `value` — available only when the expression is ground under the
  current workflow binding.
- `depends_on` — source paths, topology paths, observations, and
  relevant run-config fields.
- `envelope` — bounds, distributional facts, tolerances, capability
  facts, and provenance.
- `regime_boundaries` — any boundary records affecting the path,
  including selected crossing policies and relaxation status.
- `reduction_trace` — the merges, rewrites, and relation invocations
  that produced the expression or residual.

`inspect` is the plan-query surface for partial evaluation. It is not
a runtime graph mutation API.

#### 22.3 `prove(claim)`

**Summary.** `prove` is a bounded symbolic truth-claim query over the
compiled plan. It returns `proven`, `refuted`, `undetermined`, or
`contingent`, with a trace, counterexample, or required conditions
when available.

`prove` is not a general theorem prover. It succeeds when the claim
is visible to Myco's existing machinery: type/refinement facts,
conservation groups, e-graph equalities, monotonicity, unit algebra,
distributional envelope facts, or solver/lowering certificates. It
is useful for claims such as "this conservation law holds at every
step", "this path is bounded under the current sources", or "these
two expressions are equivalent after rewrite saturation."

#### 22.4 Hypothetical Rebinding

**Summary.** Hypothetical analysis is a rebind/recompile convenience,
not plan mutation. Tooling may expose `with_binding_override` or an
equivalent workflow helper that constructs a new run record with one
or more source bindings changed, then reuses plan-cache state where
valid.

The original plan remains immutable. Any hypothetical result carries
its own run record and provenance so comparisons are reproducible.

---

## Part III — Workflow Interface

**Summary.** Part III defines the boundary between `.myco` and the
Python workflow that drives it: the compiler declares structure,
Python supplies sources, topology, evidence, run configuration, and
execution orchestration. Covers the workflow source model, training
emission, and how the boundary keeps the compiler projection-free.

The boundary between `.myco` and Python.

### 23. The `.myco` ↔ Python Boundary

**Summary.** `.myco` declares structure; Python supplies values and
drives execution through a catalog-backed workflow surface. The
compiler stays projection-free: solver choice, projection flavor, and
numeric configuration all cross from Python. Subsections cover runtime
`where`, multi-binding compilation, cross-study callable reuse, the
two error tiers, and catalog-backed paths.

`.myco` declares structure; Python supplies values and drives
execution. The compiler does not auto-emit projection or solver
selection; those are workflow choices (§0.1 projection-free
compiler). All materialized values (physical constants, fit
parameters, data series, initial conditions, topology, observations)
cross this boundary.

**Dumb-data Python layer.** Python never sees `.myco` types as
Python classes. The compiled artifact exposes a node catalog (path,
declared type shape, binding role, units, axes, existence domain, and
refinement bounds where declared). Workflow verbs operate over
catalog-backed paths, not over spore-specific Python symbols. Spore
authors ship one artifact (`.myco` sources plus `myco.toml`); there
is no Python mirror package. The Python library grows along one axis
only — generic source, evidence, and run primitives — not along the
shape of any particular model. The catalog/path surface is locked in
§23.5; concrete convenience method names and output container menus
remain workflow-library details.

Python value providers (`Constant`, `Series`, `Prior`, `Trainable`,
`Controller`, `ProcessPrior`, CSV readers, array adapters,
distribution builders) are workflow-side data constructors. They are
not `.myco`
`Distribution<S>` implementations and do not satisfy contracts by
being Python classes. They merely package values or providers for
binding against paths in the node catalog.

Bulk binding UX is a first-class workflow requirement, not a source
language feature. Large models must not require one `bind(...)` call
per scalar. The Python library should accept structured data through
catalog-aware adapters for pandas / Polars dataframes, xarray
objects, nested dict / list data, NumPy-like arrays or matrices, and
file-backed readers such as CSV or Parquet. Adapter spelling is a
workflow-library API detail; adapter semantics are catalog-driven:
external columns, dimensions, keys, and arrays are matched to
`NodePath` / `FacetPath` handles or explicitly resolved canonical
strings, then checked against catalog metadata before the run starts.

Workflow-only capabilities live here rather than in `.myco`: RNG
seeds, checkpoint/restart, wall-clock limits, backend selection,
profile hints, long-rollout gradient regimes, and failure policy.
The source language can make claims about the world; the workflow
decides how a run is executed and supervised.

Mode B heterogeneous contract selection is a `.myco` modeling
problem, not a Python dispatch problem. Python cannot choose a
different contract implementation per instance by inspecting Myco
types at runtime. Per-instance heterogeneity must be represented in
the source model, with sum types / enum variants as the intended
mechanism (§3.10, §35).

Enum-typed workflow bindings use dumb-data tagged records. The
canonical representation is a mapping with a `tag` plus either
`value` for a single positional payload or `fields` for a struct-like
payload:

```python
workflow.bind("growth_rate", {"tag": "Fixed", "value": 0.03})
workflow.bind("stage", {"tag": "Seedling",
                         "fields": {"age": 12.0, "height": 0.08}})
```

The Python library may provide thin helpers such as
`myco.variant("Fixed", value=...)`, but helpers only produce the same
tagged record. They do not import or generate Python mirror classes
for `.myco` enum types. The compiled node catalog owns validation:
unknown tags, missing fields, extra fields, field-type mismatches,
and unit/schema mismatches are workflow-composition errors. A binding
whose discriminant is uniform for a whole field or population can be
specialized statically; per-instance tagged data remains a dynamic
discriminant and lowers according to backend capability (§3.10,
§10.6, §21).

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
  `bind("config.dt", Constant(0.01), where=scenario ==
  "high_res")`. The predicate evaluates at composition,
  not at runtime; the compiled artifact carries only the
  selected bindings.

The three uses share the keyword but live at three different
layers: compile, iteration, composition. Context disambiguates;
diagnostics name the layer when the keyword appears ambiguously.

#### 23.2 Multi-Binding Compilation

**Summary.** One `.myco` compiles once to a parameterized plan;
many workflows bind the same plan under different source and evidence
configurations. Trained controller weights persist across runs that
bind the same controller source, so calibration on one dataset
transfers to prediction on another without recompilation.

One `.myco` compiles once to a plan; many workflows bind the
same plan under different source and evidence configurations.

- **Plan.** Compile emits a plan parameterized by its binding
  surface: which sources, topology, observations, and run-config
  fields the plan accepts.
- **Instantiation.** Each workflow supplies concrete values
  for the parameterized surface via §24. The compiled
  artifact is shared across workflows; binding is cheap.
- **Callable weight reuse.** Trained weights on callables
  attached through `Controller` sources (§24.2) persist across
  workflows that bind the same controller contract.
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

Callables cross study boundaries by conforming to plain contracts
(§7). The "data contract" kind is retired (see anti_spec.md);
controller sources advertise input and output contracts, and
workflows accepting those contracts can bind the callable.

Example. A controller trained in study A outputs values
satisfying `PhotosynthesisRate : Scalar<μmol_CO2_m2_s> +
Positive`. A workflow in study B that consumes
`PhotosynthesisRate` can bind the same trained callable,
provided study B's required input contract matches the
contract attached to the `Controller` source. Contract satisfaction is
checked at workflow composition; mismatches surface as §23.4
composition errors.

The mechanism handles the "train once, reuse" story without a
separate contract kind or a stateful cross-workflow runtime.
The shared artifact is trained weights plus a plain contract;
no extra machinery.

Cross-backend callable portability is not guaranteed by this
contract alone. A controller trained under one backend can be reused
under another only when its serialization format, tensor layout,
and callable runtime are compatible. Same-backend reuse is the
current guarantee; cross-backend migration is tracked as a backend
and workflow portability open (§32, §35).

#### 23.4 Error Tiers: Compile vs Workflow Composition

**Summary.** Errors split into two tiers: `hypha` compile errors
(structural problems visible without bindings) and workflow
composition errors (problems visible only once bindings arrive, like
shape mismatches, contract violations, or N-max ceiling overrun).
Runtime errors form a third tier that lives in backend surfaces, not
this spec.

Errors surface at two distinct layers:

- **`hypha` compile errors.** Structural problems in the
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
distinction: `hypha check` catches tier-1 errors; workflow
composition surfaces tier-2. Runtime errors (numerical
divergence, overflow, solver non-convergence) are a third
tier that this spec does not address normatively; they live
in backend and deployment surfaces.

#### 23.5 Workflow Catalog and Path Surface

**Summary.** The compiled artifact exposes a catalog. The canonical
workflow address is a catalog-backed `NodePath` or `FacetPath`; strings
are accepted as serialization / convenience and immediately resolve
through the catalog. A path names a declaration-level schema slot, not
a runtime Python object. Complex values, generics, events, and dynamic
existence are represented by catalog metadata: type expression, axes,
facets, constraints, and existence domain.

The workflow catalog is the manifest of everything the workflow may
bind, observe, query, configure, or diagnose. A catalog entry carries:

- **Canonical path.** Stable source-derived name for the schema slot.
- **Declared type expression.** The `.myco` type after compilation:
  specialized for concrete model instantiations, or carrying explicit
  type parameters and constraints at reusable spore / interface
  boundaries.
- **Unit and numeric representation.** Dimension, named unit when
  relevant, dtype / representation requirements, and conversion
  affordances at the binding boundary.
- **Axes and shape facts.** Static axes, provider-validated axes,
  runtime-bounded axes, dynamic-keyed axes, matrix facts, and shape
  expression provenance.
- **Binding roles and facets.** Whether the slot accepts source
  values, initial values, observations, topology, process priors,
  controller outputs, or output queries. Facets such as
  `path.initial` are catalog entries or `FacetPath`s, not ad hoc
  string suffixes.
- **Contracts and refinements.** Contracts, refinement facts,
  obligations, and backend capabilities required for a valid binding.
- **Existence domain.** The condition under which the slot exists over
  instance, time, geometry, event phase, or enum variant axes.
- **Provenance and diagnostics.** Source location, declaration origin,
  stable catalog id, and schema text used in workflow-composition
  diagnostics.

`NodePath` is a dumb workflow handle containing the canonical path and
catalog id. It does not expose `.myco` types as Python classes and it
does not implement model semantics. Workflow verbs accept a `NodePath`
or the equivalent canonical string; strings are resolved at
composition time and diagnostics report the resolved catalog entry.
Serialized workflows store canonical strings / ids so plans remain
portable across processes and machines.

```python
model = myco.load("leaf_model")
cat = model.catalog

psi = cat.path("leaf.water_potential")
workflow.bind(psi, Series(df, column="psi", unit="MPa",
                          index=["leaf_id", "time"]))
```

`FacetPath` addresses a bindable or observable facet of a node:

```python
height = cat.path("leaf.height")
workflow.bind(height.facet("initial"), Constant(0.02, unit="m"))
workflow.observe(height, data=height_df, noise=Normal(sigma=obs_sd))
```

A `Selector` is a workflow-only query over catalog metadata, used for
bulk binding, bulk querying, diagnostics, and policy configuration. It
is not `.myco` wildcard syntax and it does not become a source
language construct. Selectors may match by role, unit, contract, axis,
path prefix, event phase, or user-facing tags, but every selected slot
resolves to concrete catalog entries before composition succeeds.

```python
rates = cat.select(role="bindable", contract="PhotosynthesisRate")
workflow.bind_frame(df, mapping={
    "leaf_id": axis("leaf"),
    "time": axis("time"),
    "A_net": rates.one(path="leaf.net_photosynthesis"),
})
```

Paths name schema slots, not runtime instances. Instance, time,
coordinate, variant, and event-phase selection live in binding/query
arguments and catalog metadata rather than in fragile object-like path
strings. A dynamic path may be perfectly valid even where no value
exists at a particular coordinate:

```python
tip = cat.path("root.tip.position")
run.query(tip, at={"root_id": roots, "time": times}, missing="masked")
```

The required missing/existence policies are:

- **`error`.** Strict default for bindings and queries when requested
  coordinates include nonexistent slots.
- **`masked`.** Return values plus an existence mask.
- **`ragged`.** Return dynamic-keyed data for axes whose cardinality
  varies by time, event phase, or parent instance.
- **`nan`.** Convenience output policy only when dtype / unit and
  downstream container support it; never the internal semantic model.

Bindings over dynamic domains must either match the existence domain
or explicitly provide an inactive / mask column. Supplying values for
pre-birth, post-removal, inactive enum-variant, or otherwise
nonexistent slots is a workflow-composition error unless the adapter
declares those rows inactive.

Structured values bind through the whole slot. Variant-specific field
paths require explicit narrowing through the catalog:

```python
stage = cat.path("leaf.stage")
workflow.bind(stage, {"tag": "Seedling",
                      "fields": {"age": 12.0, "height": 0.08}})

seedling_height = stage.variant("Seedling").field("height")
```

An unqualified path such as `leaf.stage.height` is invalid unless
`height` is common to every variant with the same type, unit, and
existence domain. This mirrors the `.myco` rule: field access on
enum-typed values requires narrowing.

`Selected<T>` values produced by selector primitives (§12.2) are not
bindable workflow inputs. Python cannot fabricate a selected handle by
supplying a pool id, tag, or index; selected-handle identity and
provenance are compiler-owned Layer-3 state. Workflows may query
selected handles as outputs or diagnostics through catalog-backed
result records, but feeding such a result into another run must go
through ordinary model fields, observations, or source bindings rather
than raw selected-reference identity.

For generics, compiled concrete models expose specialized catalog
entries wherever specialization is known. Reusable spore/interface
artifacts may expose constrained generic entries, but the catalog
spells the type parameters and required contracts explicitly; workflow
code never constructs or subclasses Myco type parameters in Python.

```python
CatalogEntry(
    path="growth_response.rate",
    type_params={"T": "PhotosyntheticOrgan"},
    requires=["T: HasArea", "T: HasConductance"],
    type="Scalar<Rate>",
)
```

The path surface therefore scales to complex types and dynamic worlds
without making Python a model layer: Python holds catalog-backed
handles and data adapters; the compiler owns type interpretation,
existence reasoning, event phases, generic specialization, and
backend lowering.

### 24. Workflow Source Model

**Summary.** The workflow-composition surface has three binding
verbs: `bind(path, source)`, `bind_topology(path, geometry)`, and
`observe(path, data)`. Fixed values, time series, trainable
parameters, priors, controllers, and process priors are source objects
passed to `bind`, not separate verbs. Run mode
decides how sources participate in execution, training, or PPL.

`.myco` states world claims. Workflow composition materializes those
claims for a particular experiment by attaching sources and evidence
to `NodePath` / `FacetPath` handles resolved through the compiled
catalog (§23.5).

#### 24.1 The Three Binding Verbs

**Summary.** `bind` attaches a source object to a node path,
`bind_topology` materializes geometry, and `observe` attaches
evidence. Orchestration verbs such as `load`, `spawn`, `run`,
`checkpoint`, and output queries are workflow-library operations, not
model bindings.

- **`bind(path, source)`.** Attaches a source object to a `NodePath`,
  `FacetPath`, or equivalent canonical string. The source declares its
  value shape, units, dtype, gradient participation, and any
  contracts.
- **`bind_topology(path, geometry)`.** Supplies concrete topology or
  discretization data for a declared geometry (§11).
- **`observe(path, data)`.** Attaches evidence to a catalog-resolved
  path as layer-2 envelope metadata (§13.8, §13.9). It does not assert
  equality with the data unless the `.myco` model explicitly states a
  hard observation model.

All three verbs resolve their path arguments through the catalog.
Bulk surfaces may accept `Selector`s only when their fanout behavior
is explicit and every selected entry is validated independently.

The verbs fire at workflow composition. Bind-time type checking
validates shape, dtype, units, path existence, contract satisfaction,
N-max ceilings, existence-domain compatibility, and backend
capability requirements before the run starts.

Workflow binding is the only path by which source objects become
Layer-2 envelope facts or Layer-3 provider records (§16, §17.1 source
2). The verbs do not introduce `.myco` syntax; they materialize the
compiled plan's parameterized boundary.

#### 24.2 Source Objects

**Summary.** Source objects carry value, training, prior, and
controller semantics. Compile/run mode decides whether a source is
held fixed, optimized, sampled, or used as an opaque external
callable.

Representative source objects:

- **`Constant(value, unit=None)`.** Fixed scalar, tensor, structured
  value, or small table supplied by the workflow.
- **`Series(data, unit=None, index=None)`.** Time-indexed or
  coordinate-indexed data supplied by the workflow.
- **`Trainable(prior=None, init=None, trajectory=None)`.** Value that
  participates in gradient training. The same source can represent a
  learned constant, initial value, or trajectory depending on the
  bound path and arguments.
- **`Prior(distribution)`.** Epistemic distributional source used by
  inference/PPL modes without implying gradient training by name.
- **`Controller(callable, reads, writes, input_contract, output_contract,
  trainable=True)`.** External callable source. `reads` and `writes`
  are path lists; contracts are plain Myco contracts (§7). The input
  contract is also the visibility boundary: widening what the
  callable may read requires widening the declared input contract.
  No `.myco` keyword introduces a controller.
- **`ProcessPrior(index=..., value=..., law=...)`.** Workflow source
  for epistemic process priors over indexed `.myco` contracts. The
  binding names the index slots and value slots explicitly; the
  process law may be a `GaussianProcess`, a finite-feature process,
  or another curated process law (§28.8). `GPPrior(...)` may exist as a
  Python convenience for `ProcessPrior(law=GaussianProcess(...))`, but
  it is not the canonical source object. Process priors introduce no
  `.myco` syntax.

All source objects are dumb workflow data. They do not expose Myco
types as Python classes and they do not create new source-language
constructs.

#### 24.3 Controller Gradient-Flow Semantics

**Summary.** Controllers register learnable parameters through their
source object. Gradient flow happens via the backend's AD when the
source advertises a differentiable output contract and the selected
backend advertises opaque-callable runtime / AD support. Fixed opaque
controllers may run without AD. Non-differentiable controllers on a
required training-gradient path are workflow-composition errors
unless the workflow explicitly marks the boundary as a gradient stop.
Trained weights persist across runs that bind the same controller
source.

Controllers usually wrap differentiable components (neural nets with
learnable weights). Gradient semantics:

- **Parameter registration.** A `Controller(..., trainable=True)`
  registers its internal learnable parameters with the training objective
  at workflow composition. A workflow may bind the same trained
  controller later with `trainable=False` to freeze it.
- **Backward pass.** Objective gradients from workflow observations
  (§13.8) flow through the model graph to the controller's output,
  into the controller's parameters, via the backend's AD facility
  (§31). This requires the controller's contract to advertise
  `Differentiable` where gradients must pass, and requires the
  selected backend to advertise `opaque_callable_runtime`,
  `opaque_callable_ad`, and the relevant runtime AD profile.
- **Opaque fixed callable.** A controller may run as a fixed opaque
  source without AD, for example `Controller(..., trainable=False)`.
  Useful for fixed heuristics, lookup services, or non-differentiable
  routines whose outputs are model inputs.
- **Required gradient path.** If a training objective requires
  gradients through a controller and the controller or backend cannot
  provide them, workflow composition fails with a capability /
  differentiability diagnostic. The compiler does not silently insert
  a gradient stop.
- **Explicit gradient stop.** A workflow may explicitly mark the
  controller boundary as a gradient stop. In that case the controller
  may influence downstream values, but its internal parameters are
  not learned in the current run and upstream gradient accounting
  records the stop. Exact Python spelling is a workflow API detail;
  the semantic requirement is explicitness.
- **Cross-run weight persistence.** Trained weights persist across
  runs that bind the same controller (§23.3).

The controller is the seam where neural machinery attaches to the
scientific model, but the seam is workflow-side: `.myco` sees only
the world claims involving the bound path.

#### 24.4 `bind_topology` and §11 Geometry

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

#### 24.5 Run-Config and Workflow Configuration Surface

**Summary.** Run-config is the non-binding configuration the
workflow supplies at composition: seed, backend, verbosity, dt
(when used), profile hints, gradient regime, crossing-handler policy,
topology-handler policy, default-fulfillment policy, and fallback policy.
Distinct from source binding since run-config does not bind model
values; it configures how the compiled plan executes. Different runs of
one source bundle can use different run-configs; the compiler may reuse
plan-cache state where the backend / handler capability envelope is
unchanged.

Run-config is the non-binding configuration the workflow
supplies at composition. Distinct from `bind`: run-config
does not bind model values; it configures how the
compiled plan executes.

Representative fields:

- `run.config.seed`. RNG seed for stochastic SCCs.
- `run.config.backend`. Backend selection and its
  capability-fallback mode (error / host / emulate, §31).
- `run.config.backend.intent`. Portable backend intent: backend family,
  compiler mode, device class, and required / rejected capability
  classes. This should avoid machine-specific device IDs unless exact
  reproduction is required.
- `run.config.backend_version`. Optional backend identity / version
  pin when reproducibility requires an exact implementation (§31.4).
- `run.config.topology_handlers`. Allowed dynamic-topology handlers,
  optionally ordered by preference: `CapacityMask`, `EventReplan`, and
  `DynamicKeyed` (§3.8, §21.3). The compiler may select only a handler
  in this allowed set and only when the resolved backend/device
  advertises the required capability facts or the workflow explicitly
  authorizes host / emulate fallback.
- `run.config.verbosity`. Diagnostics level.
- `run.config.dt`. Referenced via `bind("config.dt", Constant(...))`
  or `bind("config.dt", Series(...))` in a discrete-time model
  (§9.1).
- `run.config.gradient_regime`. Long-rollout gradient strategy:
  `full_BPTT`, `truncated_BPTT(k)`, or `checkpointed`.
- `run.config.crossing_handlers`. Handler policy for regime boundaries
  (§8.10, §24.6): strict gradients, one-sided / subgradient rules,
  surrogate relaxations, estimators, and exact topology handlers.
  Default is `strict`.
- `run.config.relaxations`. Optional convenience view for the surrogate
  / smoothing subset of `crossing_handlers`.
- `run.config.default_fulfillments`. Obligation-ledger policy for
  package-provided default candidates (§8.11): `allow`, `warn`, or
  `error`. Default is `allow`; audit workflows can require explicit
  user fulfillments by setting `error`.
- `run.config.extraction_policy`. Preference over
  `cost_of` fields: compute-first, memory-first, faithfulness-first,
  or weighted (§19.1).
- `run.config.closure_policy`. Per-residual-block closure-policy
  selection and budgets. Exact Y6 entries may set enumeration budgets
  and escalation behavior; guided subsystem search entries must set an
  explicit subsystem budget, acceptance criteria, and `on_unmet`
  behavior (§8.7).
- `run.config.objective_policy`. Workflow-side scalarization of
  `objective_terms` across residuals and studies (§25).
- `run.config.approximation_estimation`. Sampling parameters used to
  empirically estimate approximation error bounds (sample count,
  seed, stratification) when a rewrite's certification requires
  numerical estimation (§15.2).
- `run.config.profile`. Execution-profile hints (batch
  size, memory budget).
- `run.config.capability_overrides`. Explicit workflow authorizations
  such as raising exact Y6 enumeration budgets, choosing an inference
  backend, or enabling approximate-inference switches. The compiler
  never assumes these silently.

`run.config.topology_handlers` is the coarse allow-list for dynamic
topology execution in the run. `run.config.crossing_handlers` is the
per-opportunity selection surface. When both are supplied, every
topology handler selected by `crossing_handlers` must belong to the
allow-list, and the resolved backend/device must advertise the
corresponding capability facts unless fallback is explicitly authorized.

Run-config separates **portable intent** from the **resolved run lock**.
The intent config is what users should share: backend family or device
class, allowed topology handlers, fallback policy, and semantic
requirements. At plan binding, the compiler resolves that intent against
the actual backend, compiler mode, device / hardware, version, and
capability probe results. The run record / lock captures the concrete
selection: backend family and version, compiler mode, device kind,
selected topology handler, capability profile set, capability-probe
hash, fallback authorizations, and any host / emulate routes actually
used. This keeps shared workflows portable while preserving exact
reproducibility for completed runs.

Run-config fields may be referenced from workflow bindings as
paths (`bind("run.config.dt", Constant(0.01))`); the
compiler does not bake them into the plan beyond the
binding surface. Different runs of the same plan can use
different run-configs; backend or topology-handler changes may require
rebinding or re-lowering while preserving the same source semantics.

#### 24.6 Relaxation and Crossing-Handler Inventory

**Summary.** Relaxations and crossing handlers are workflow-authorized
choices over regime boundaries. Some handlers are surrogate choices
(`SmoothStep`, estimators); dynamic-topology handlers (`CapacityMask`,
`EventReplan`, `DynamicKeyed`) preserve source semantics while choosing
an execution strategy. The compiler exposes a passive inventory of
opportunities with stable IDs, compatible handlers, defaults, and
diagnostics. Default behavior is strict; no smoothing, surrogate, host
route, or topology-handler fallback is applied unless written in
`.myco` as a model claim or selected by the workflow for a run.

Every compiled model exposes a passive crossing-opportunity inventory:

```python
ops = model.crossing_opportunities()
```

Workflow libraries may also expose `relaxation_opportunities()` as a
filtered convenience for surrogate / smoothing opportunities only.

The call does not change the plan. It returns stable records for
regime boundaries (§8.10) that may require a crossing policy in
gradient-demanding, optimization, topology-changing, or accelerated-
execution contexts.
Representative record shape:

```python
{
    "leaf.pv_curve.turgor_loss": {
        "kind": "piecewise_boundary",
        "surface": "leaf.turgor = 0",
        "continuity": "value_jump | value_continuous_kink | unknown",
        "derivative": "one_sided | none | unknown",
        "affected": ["leaf.water_potential"],
        "compatible_handlers": [
            "strict",
            "within_regime",
            "one_sided",
            "subgradient",
            "smooth_step_blend"
        ],
        "default": "strict",
    }
}
```

Opportunity IDs are stable under source-preserving recompilation
where the boundary path and guard are unchanged. They are diagnostic
handles, not `.myco` symbols.

The workflow selects handlers explicitly:

```python
run.config.crossing_handlers = {
    "leaf.pv_curve.turgor_loss": SmoothStep(width=0.02, budget=0.001),
    "hydraulics.segment_split": EventReplan(),
}
```

or by typed policy rules:

```python
run.config.crossing_handlers = CrossingPolicy(
    default="strict",
    rules=[
        match(kind="piecewise_boundary", path="leaf.*")
            .use(SmoothStep(width="auto", budget=0.001)),
        match(kind="topology_event")
            .use(EventReplan()),
    ],
)
```

Handlers are typed by opportunity kind:

- **`Strict`.** Default. Preserve the source model exactly; reject
  ordinary gradients through nonsmooth / discontinuous crossings.
- **`WithinRegime`.** Differentiate only inside the active regime;
  boundary-crossing sensitivity is not claimed.
- **`OneSided`.** Use left / right directional derivatives at a
  boundary where the side is determined by the active regime.
- **`Subgradient`.** Use a supported subgradient rule for convex or
  explicitly supported nonsmooth primitives.
- **`Saltation` / `ResetSensitivity`.** For hybrid events with a
  declared reset map; unsupported for arbitrary topology mutation
  unless the event provides the required mapping.
- **`Estimator`.** For stochastic discrete choices, use an explicit
  estimator policy rather than ordinary AD.
- **`SmoothStep` / smoothing handlers.** Build a surrogate extraction
  plan with declared width, budget, and error / distortion ledger.
- **`EventReplan` / `CapacityMask` / `DynamicKeyed`.** Dynamic-
  topology crossing handlers (§3.8, §21.3, §31.1) selected according
  to portable workflow intent and resolved backend / device capability
  facts. They are exact execution strategies, not smooth relaxations.

`AutoSmooth` is an opt-in workflow convenience that fills surrogate
handler choices for compatible smoothing opportunities:

```python
run.config.relaxations = AutoSmooth(
    scope="training",
    max_error=0.001,
    require_review=True,
)
```

Even under `AutoSmooth`, the compiler validates handler compatibility
and emits a relaxation ledger. If `require_review=True`, the workflow
must accept the proposed ledger before execution. If `False`, the
run record still captures the ledger so results remain auditable.

A relaxation selected in workflow is a surrogate plan, not a source
model rewrite. If the user writes `smooth_step` directly in `.myco`,
that is a world claim (§8.9). If the workflow selects `SmoothStep`
for a hard boundary, the original source model remains hard and the
run's plan records the smoothed surrogate, handler parameters,
affected residuals, error / distortion budget, and selected crossing
policy.

A topology crossing handler selected in workflow is an execution
strategy for preserving source semantics across a topology-changing
regime boundary, not a model relaxation. The run's plan records the
selected handler, required backend / device capabilities, rejected
alternatives, and any explicitly authorized host or emulation route.

### 25. Training Emission

**Summary.** Training SCCs compile to gradient-trainable code with
warm-start semantics drawn from `Constant` initial values or
`Trainable` priors. Workflow selects projection flavor and objective
aggregation; the compiler exposes named residuals and objective terms
but does not auto-sum them. Constraint enforcement discharges at
compile time where possible, otherwise through an explicit training
penalty or runtime projection selected by the workflow.

How the compiler emits gradient-trainable code for SCCs classified as
training (§20). Training emission has three products:

- A differentiable forward computation for each training SCC.
- A workflow-visible residual catalog of active `ResidualSite`s (§19.2).
- `objective_terms(residual)` hooks that workflow code can aggregate
  into a scalar objective (§14.2).

The compiler does not choose the scalar training objective. It exposes
the ingredients; the workflow composes them.

**Warm-start semantics.** Three distinct sources can initialize a
training run:

- Between-tick warm starts: implicit and iterative solves may start
  from the previous tick's value when the SCC is dynamic.
- Bound initial values: `bind(path.initial, Constant(...))` supplies
  a fixed starting value.
- Trainable priors / guesses: `Trainable(prior=..., init=...)`
  supplies a learned quantity's prior and initial guess. Priors
  contribute objective terms; initial guesses do not by themselves
  assert truth.

**Projection flavor.** Workflow selects admissibility projection
where a runtime projection is desired: `hard_clip`, `sigmoid`, or
`soft_clip`. The compiler never auto-emits a projection flavor (§0.1,
anti_spec.md). Refinement bounds and constraint metadata are exposed
so the workflow can choose deliberately.

**Per-residual exposure.** `model.residuals` is the workflow-visible
catalog of active residual sites produced by projection (§19.2). A
representative `Residual` carries: stable site id, original relation
name or generated obligation key, source / workflow provenance, SCC id,
units, axes, shape, refinement bounds, residual kind, activation
status, chosen realization (`explicit`, `implicit`, `opaque`,
`projection`, `provider_check`), shared-realization id when applicable,
and alternatives on the extraction Pareto frontier when available. The
exact Python object shape is workflow-library API, but these fields are
the semantic payload.

**Stdlib objective helpers.** The workflow library ships helpers that
consume the residual catalog:

- `soft_penalty(weights)` sums weighted squared residual terms.
- `augmented_lagrangian(weights, mu, lambda_init, mu_schedule)`
  exposes dual-state handling while leaving the state representation
  to the backend convention (mutable PyTorch-like state or pure
  JAX-like state threading).

Both helpers are workflow conveniences over the same residual catalog,
not compiler-selected objective functions.

Both helpers consume `objective_terms(residual)` values. They may
choose only `constraint_violation`, combine it with `data_fit`, or
add `regularization`; the compiler does not privilege a combination.

**Unhandled residual policy.** Mismatched data is not an error by
itself; unhandled mismatch is an error. In fit, inference,
conditioning, or score modes, every active residual site that produces
an objective, likelihood, projection, exact-conditioning, or
provider-check obligation must be consumed by an explicit workflow
policy before composition succeeds. Valid handling includes:

- observation noise / likelihood, for example
  `observe(path, data, noise=Normal(...))`;
- exact observation or exact conditioning, where the workflow chooses
  a hard evidence policy;
- objective aggregation through `objective_terms(residual)`,
  `soft_penalty`, or `augmented_lagrangian`;
- exact solve / implicit-block handling for hard model equations whose
  residuals are enforced by the selected realization;
- runtime projection with a workflow-selected projection flavor;
- inference factorization / PPL handoff for stochastic SCCs;
- explicit ignore / diagnostic-only handling when a workflow library
  exposes such a policy with provenance.

If a workflow binds mismatched data or activates overconstrained physics
without selecting one of these paths, workflow composition fails with an
unhandled-residual diagnostic. The compiler does not silently invent
least-squares, measurement noise, penalty weights, projection, or a
gradient stop. In ordinary simulation mode, observations are inert
unless the workflow asks to condition, fit, infer, or score against
them.

**What the compiler does not auto-emit.** The compiler does not pick a
projection flavor, aggregate objective terms, update dual variables,
choose annealing / penalty schedules, or convert observations into a
least-squares objective. Each of those is a workflow policy.

**Constraint discharge regimes.** Constraints discharge in three
ways:

- Compile-time proof: the constraint is proven from the e-graph and
  contributes no runtime term.
- Training penalty: the workflow includes the named residual in the
  objective, commonly through `soft_penalty` or
  `augmented_lagrangian`.
- Runtime projection: the workflow explicitly selects a projection
  strategy for deployment or simulation.

Training-mode consistency-objective substitution is the O-group rule:
an overconstrained relation `lhs = rhs` may expose a residual term
proportional to `(lhs - rhs)^2` in training mode (Appendix C O1).
The `ResidualSite` keeps the relation identity so diagnostics and
training surfaces can refer to the original model claim even when the
chosen realization is shared or algebraically simplified.

**PINN-style pattern.** A workflow can bind observed trajectories or
trainable trajectories with `Series` / `Trainable(trajectory=...)`
while the `.myco` temporal relation supplies the physics residual.
Training emission exposes both observation terms and physics residual
terms; the workflow decides their relative weights.

**Study-level aggregation.** Multi-experiment joint objectives are
workflow-side. One compiled plan can be instantiated for multiple
studies (§23.2); the workflow aggregates their residual catalogs into
one scalar objective when desired.

---

## Part IV — Standard Library

**Summary.** Part IV covers what ships with Myco: numeric types,
distribution families, kernels, units, and matrix/tensor primitives.
Domain-specific units and models stay out of core and ship as
distributable packages on top of the stdlib.

What ships with Myco.

### 26. Numeric Types

**Summary.** `Scalar<U, T = Float64>` takes an explicit numeric
representation parameter with `Float64` as default. Core reps include
`Bool`, fixed-width `Int*` / `UInt*`, `Rational`, `Float32`,
`Float64`, `BigFloat`, and `Complex`. `T` must satisfy a base
`Numeric` contract hierarchy; mixed-T arithmetic is forbidden without
explicit conversion.

`Scalar<U, T = Float64>` with explicit `T` parameter and `Float64`
default. `Rational` for exact constant folding (with termination
caveats). `BigFloat`. Default-compatibility constraints.

#### 26.1 Numeric Representation Hierarchy

**Summary.** The stdlib provides booleans, fixed-width signed and
unsigned integers, exact rationals, floating types, arbitrary
precision extension types, and complex numbers for the `T` parameter.
`Float64` is the per-Scalar default, not module-wide. Forward-mode AD
is not exposed as a user-facing representation.

`Scalar<U, T>` takes an explicit numeric representation parameter
T. The stdlib provides:

| T | Role | Notes |
|---|---|---|
| `Bool` | two-valued logic | consumed by boolean relations, predicates, alive masks |
| `Int8`, `Int16`, `Int32`, `Int64` | fixed-width signed integers | backend-representable |
| `UInt8`, `UInt16`, `UInt32`, `UInt64` | fixed-width unsigned integers | backend-representable |
| `Integer` / `BigInt` | arbitrary-precision integers | extension-style exact integer; GPU-incompatible |
| `Rational` | exact rationals | §26.3 termination caveat; GPU-incompatible |
| `Float32` | IEEE single | backend-dependent availability |
| `Float64` | IEEE double | default; universal backend support |
| `BigFloat` | arbitrary-precision floats | exact rounding semantics; GPU-incompatible |
| `Complex` | complex numbers | ordinary numeric representation; algebraic contracts locked in §26.4 |

Forward-mode AD is not a user-facing representation.
Backends own AD (§31); dual numbers would duplicate what the
backend already provides. Retired to anti_spec.md.

Default `T = Float64` is per-Scalar, not module-wide. Mixing
T within one expression is forbidden without explicit
`convert T1 -> T2` (§26.2).

#### 26.2 Default-Compatibility Constraints on T

**Summary.** `T` must satisfy a base `Numeric` hierarchy: ring
closure, total ordering where applicable, zero and one identity
elements, and backend representability. `Complex` satisfies the
algebraic numeric contracts but not total ordering. Mixed-T arithmetic
is a compile error and requires explicit `convert`. `Float32 ->
Float64` is lossless; `Float64 -> Float32` requires an `approximate`
block with a precision-downcast tolerance class.

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
lossless. Precision downcast (`Float64 -> Float32`, or the analogous
tensor element-type downcast) is lossy-tolerance and must appear
inside an authorizing `approximate` block (§15) rather than bare
`convert`.

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

#### 26.4 Complex Numeric Semantics

**Summary.** `Complex` is an ordinary numeric representation for
`Scalar<U,T>`, alongside the real and exact numeric representations.
It is not a separate scalar kind or a separate `Numeric` sub-hierarchy.
`Scalar<U, Complex>` is one unit-bearing scalar quantity whose real and
imaginary components share unit `U`. Complex values satisfy algebraic
contracts and conjugation / magnitude contracts, but not ordering
contracts. Backend execution is capability-gated.

The canonical spelling is:

```myco
z: Scalar<ohm, Complex>
```

`z` is one scalar quantity with unit `ohm` and complex numeric
representation. It is not a record with independently unitable
components. Stdlib atoms preserve units as follows:

```text
real(z: Scalar<U, Complex>) -> Scalar<U>
imag(z: Scalar<U, Complex>) -> Scalar<U>
conj(z: Scalar<U, Complex>) -> Scalar<U, Complex>
abs(z: Scalar<U, Complex>) -> Scalar<U>
phase(z: Scalar<U, Complex>) -> Phase
complex(re: Scalar<U>, im: Scalar<U>) -> Scalar<U, Complex>
```

Both arguments to `complex(re, im)` must share the same unit `U` or
have an explicit unit conversion path. `phase(z)` requires `z != 0`
or produces a domain obligation; it returns the stdlib `Phase` semantic
quantity type (§5.0), not an arbitrary `Scalar<dimensionless>`.

`Complex` satisfies:

```text
Plus, Minus, Times, Divide, Zero, One, Conjugate, Abs/Magnitude
```

It does not satisfy total ordering:

```myco
max(z1, z2)            # rejected for Complex
max(abs(z1), abs(z2))  # valid
```

Stdlib trig atoms consume angle-compatible quantities, not arbitrary
dimensionless ratios. Angle-compatible means `Angle`, `Phase`, or
`Scalar<rad>` while the named dimensionless unit is still present:

```text
sin(theta: Angle) -> Scalar<dimensionless>
cos(theta: Scalar<rad>) -> Scalar<dimensionless>
atan2(y: Scalar<U>, x: Scalar<U>) -> Phase
```

Complex-valued stdlib atoms with branch choices (`phase`, complex
`log`, complex `sqrt`, fractional powers, and inverse trig functions)
carry domain and branch-cut facts. Their branch cuts are ordinary
regime boundaries (§8.10): gradients flow within a branch, but no
ordinary smooth gradient crosses a branch cut or the undefined point
without an explicit crossing policy.

Complex differentiability is fact-specific. Holomorphic stdlib atoms
may emit complex-analytic derivative facts on their proven domains.
Atoms such as `real`, `imag`, `conj`, `abs`, and `phase` may be
real-differentiable where defined, but they do not emit holomorphic
derivative facts. Runtime complex AD is a backend capability with an
advertised convention (for example real/imag splitting or Wirtinger
calculus); backend AD values remain execution results and provenance,
not new symbolic derivative facts (§31).

Execution capabilities are advertised separately:

```text
supports_complex
supports_complex_linalg
supports_complex_ad
```

If a plan requires complex arithmetic, complex linear algebra, or
complex runtime AD and the selected backend does not advertise the
needed capability, workflow composition fails by default (§31.1).
`host` or `emulate` policy may be selected explicitly; such lowering
does not change source semantics.

### 27. Distribution Families (Z-group)

**Summary.** Tier 1 ships 19 univariate continuous, 6 discrete, and 3
multivariate families, plus the `Truncated<D>` and `Mixture` meta-
families. Conjugate-posterior rewrites are enumerated as a closed
catalog. Tier B approximate rewrites (Delta, Fenton-Wilkinson, CLT,
block-maxima to GEV) fire under `approximate` blocks. Tier 1, 2, 3
scope the family catalog; Tier A, B, C are the orthogonal dispatch axis.

Tier 1 univariate continuous families (19): Normal, LogNormal, Uniform,
Beta, Gamma, Exponential, ChiSquared, Cauchy, Student-t, Laplace,
HalfNormal, HalfCauchy, InverseGamma, Lévy, Weibull, Pareto, Fréchet,
Gumbel, GEV. Tier 1 discrete: Bernoulli, Binomial, Categorical,
Poisson, NegBinomial, Hypergeometric. Tier 1 multivariate:
MultivariateNormal, Dirichlet, Multinomial. Meta-families: `Truncated<D>`,
`Mixture<D₁,…,D_N | weights>`. Conjugate-posterior rewrites.
Tier B approximate rewrites: Delta method, Fenton-Wilkinson, CLT,
block-maxima → GEV.

**The `Distribution<S>` contract.** Every Tier 1 and Tier 2
distribution family implements the `Distribution<S>` capability
contract, where S is the sample type. Scalar distributions use
`Scalar<U>` as S; multivariate and structured joint families use
tensor, vector, simplex, or record-shaped sample types. The contract
has one visible density obligation, default-derived density
conveniences, backend/runtime sampling capabilities, and optional
capability sub-contracts that advertise algebraic closures used by
Tier A dispatch (§13.2).

Visible density obligation:

- `relation log_density(self: Self, sample: S, out: Scalar<dimensionless>)`
  — log density / log mass at `sample`. Required for likelihood
  contributions (§13.8 `observe`), training emission (§25), and
  Tier A closed-form posterior construction. Stdlib atoms for Tier 1
  families supply closed forms; user-defined distributions compose
  `log_density` from visible `.myco` relations and stdlib atoms.

Convenience density surface:

- `density` / `pdf` is default-derived from `log_density` when
  available. A stdlib family may provide a direct closed-form density
  when that avoids a numerical round-trip, but `pdf` is not an
  additional core obligation.

Sampling and reparameterization:

- Sampling is a backend/runtime capability, not an ordinary `.myco`
  relation method. Families advertise sampleability for workflows
  and Tier C handoff through backend capability metadata.
- Visible reparameterization, when available, is relation-shaped:
  `relation reparameterize(self: Self, base_noise: B, out: S)`.
  It grants differentiable sample construction facts only when the
  relation body is visible or the backend capability is explicitly
  audited.

Opaque stochastic families:

- Curated stdlib/backend families may be `OpaqueStochasticFamily<S>`
  when their density evaluator is structurally opaque (for example,
  general alpha-stable outside the Normal / Cauchy / Levy-visible
  subcases). They are Tier-C-first and fact-poor by default: no
  symbolic `log_density`, no automatic closure facts, no symbolic
  derivative facts, and no condition facts through the opaque density.
  HMC / NUTS / VI require backend-certified differentiable
  opaque-log-density support. Finite-difference or emulation routes
  require explicit workflow `emulate` policy. User-authored `.myco`
  distributions do not have an opaque-density escape hatch; they
  expose visible `log_density` relations.

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

User-defined distributions implement `Distribution<S>` by supplying
visible `log_density` relations composed over stdlib atoms and
ordinary `.myco` relations. The compiler derives which optional
sub-contracts hold when possible; when it cannot, the user-defined
family routes to Tier C for inference rather than receiving symbolic
facts. This is the only extensibility path — no annotation surface
for advertising closures or opaque densities.

#### 27.1 Tier 1 Distribution Families, Table

**Summary.** Tier 1 families ship as capability-tagged stdlib
declarations with Distribution, Affine/Sum/Product/ScaleSelfClosed,
SmoothTransformable, ReparameterizedSampleable, and Conj(X) tags.
MVN consumes matrix facts for how `Σ` carries axes, entry units,
symmetry, and positive-definiteness obligations; Dirichlet and
Multinomial use vector/simplex facts rather than matrix covariance
machinery.

Tier 1 families ship as capability-tagged stdlib declarations
(§7.2). Capability columns use shorthand: **D** =
`Distribution<S>`, **A** = `AffineSelfClosed`, **S** =
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

**Discrete (6).**

| Family | Support | Parameters | Capabilities |
|---|---|---|---|
| `Bernoulli` | `{0, 1}` | `p` | D, Conj(Beta) |
| `Binomial` | `{0 … n}` | `n`, `p` | D, S (shared p), Conj(Beta) |
| `Categorical` | `{0 … K-1}` | `p[K]` | D |
| `Poisson` | ℕ | `λ` | D, Conj(Gamma) |
| `NegBinomial` | ℕ | `r`, `p` | D |
| `Hypergeometric` | `[max(0, n-(N-K)), min(n, K)]` | `N`, `K`, `n` | D |

**Multivariate (3).**

| Family | Support | Parameters | Capabilities |
|---|---|---|---|
| `MultivariateNormal` | ℝᵈ | `μ`, `Σ` | D, A, R (Cholesky, §13.6) |
| `Dirichlet` | simplex Δᵈ⁻¹ | `α[d]` | D, Conj(Multinomial) |
| `Multinomial` | `Σⱼ xⱼ = n` | `n`, `p[K]` | D, Conj(Dirichlet) |

MVN requires matrix facts on `Σ`: axis compatibility with `μ`,
`entry_unit_law(Σ[i,j]) = unit(μ[i]) * unit(μ[j])`, symmetry, and
positive semidefiniteness / positive definiteness depending on
sampling vs density use. Unknown required facts become obligations.
Dirichlet and Multinomial are vector/simplex-valued and do not depend
on covariance-matrix machinery.

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
satisfies `Distribution<S>` with scalar sample type. Capabilities:
inherits D's capabilities minus closures broken by truncation
(`AffineSelfClosed` is generally lost; `ReparameterizedSampleable`
survives via inverse-CDF sampling). Refinement types
(§3.2) interact cleanly: `x ~ Truncated<Normal>(0, 1, 0, 1)`
auto-satisfies `UnitInterval`.

**`Mixture<D₁, …, Dₙ | weights>`, weighted combination.** A
mixture of n component distributions with non-negative weights
summing to 1. Components can be distinct families; shared-
support requirement is enforced structurally. Weights are
themselves values, workflow-supplied as `Constant` or `Trainable`
sources. Capabilities: `Mixture` is a `Distribution`
but closes under fewer algebraic operations than its
components; specifically, `AffineSelfClosed` survives only
when every component satisfies it.

Both meta-families compose: `Mixture<Truncated<Normal>(…),
Truncated<Normal>(…)>` is a legitimate Tier 1 construction.
Nesting depth is bounded only by backend handoff costs.

#### 27.3 Conjugate-Posterior Rewrite Catalog

**Summary.** The stdlib enumerates a closed catalog of conjugate-
posterior rewrites covering Beta-Bernoulli/Binomial, Gamma-Poisson,
Gamma-Gamma,
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
| `Gamma(α, β)` prior on rate λ | `Gamma(k, λ)` observations with known shape k and n draws summing s | `Gamma(α + n*k, β + s)` |
| `Normal(μ₀, σ₀²)` | `Normal(μ, σ²)` known σ, n draws mean x̄ | `Normal((σ² μ₀ + n σ₀² x̄)/(σ² + n σ₀²), (σ₀² σ²)/(σ² + n σ₀²))` |
| `InverseGamma(α, β)` | `Normal(μ, σ²)` known μ, n draws, sum-sq s | `InverseGamma(α + n/2, β + s/2)` |
| `Dirichlet(α)` | `Multinomial(n, p)` counts c | `Dirichlet(α + c)` |

The catalog above is closed for this release. Normal-InverseGamma
joint-prior machinery is explicitly gated on the rewrite-pattern
language for joint priors (§35). Additional conjugate pairs that
modelers need route to Tier C or later catalog expansion. The
rewrites fire automatically when the compiler detects a matching `~`
structure; no user directive is required.

#### 27.4 Extended Capability Table

**Summary.** Tier A dispatch needs extra capability columns beyond the
core tags: support, log_density, moments, reparam, sampling, entropy,
kl_div. The full table lives in the stdlib reference; this spec is
normative only about which columns exist. Missing entries are "not
closed-form" and fall through to Tier B or Tier C.

For Tier A dispatch (§13.2), the compiler needs more than
the core capability tags (§27.1). The extended per-family
table records:

| Column | Meaning |
|---|---|
| `support` | the domain on which density is non-zero |
| `log_density` | closed-form log density availability |
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

**Summary.** Tier 1 ships: 28 families plus two meta-families with
capability contracts and conjugate-rewrite wiring. Tier 2 mechanics
for structured joint roots, named field projections, and coupling
metadata are locked; remaining Tier 2 work is family-catalog and
capability-table polish for copulas, Wishart variants, and related
joint families. Tier 3 process-prior routing is scoped for GP-style
families; other non-parametric catalogs remain open. Tier 1/2/3 orders
the family catalog; Tier A/B/C orders dispatch and is orthogonal.

Tiers are the PPL scoping axis distinct from the distribution-
family catalog:

- **Tier 1.** Ships in this release. The 28 families in §27.1
  plus the two meta-families in §27.2, with capability contracts
  and closed-form rewrites (§27.3) wired in. Includes three
  multivariate members (MVN, Dirichlet, Multinomial), with
  MVN using the Cholesky reparameterization locked in §13.6.
- **Tier 2.** Partial but no longer blocked on core mechanics. The
  multivariate subset that admits a factorized representation or a
  closed-form reparameterization ships in Tier 1 (MVN via Cholesky,
  Dirichlet/Multinomial via conjugacy). The structured-joint surface
  is locked by §13.7 / §13.10: one joint root, named `.at()`
  projections, record-`~` sugar, and joint-envelope coupling facts.
  Remaining work is the family catalog and per-family capability
  tables for copulas, Wishart / InverseWishart / LKJ (gated on matrix
  fact propagation for SPD matrices, determinant, trace, and
  factorable unit laws), and related joint families. Higher-order
  process-valued priors route through process-prior and kernel
  machinery (§28). Tracked in §35 Other Opens as catalog polish, not
  unresolved core semantics.
- **Tier 3.** Partially scoped. Process-valued uncertainty is not a
  third uncertainty kind: an epistemic or aleatoric source may have
  process / field sample shape (§28.8). Gaussian-process-style priors
  route through `ProcessPrior<I,V>` and kernel finite-projection
  machinery, then dispatch through Tier A/B/C like any other stochastic
  SCC. Remaining Tier 3 work is the catalog and capability boundary for
  other non-parametric families (Dirichlet Process, Chinese Restaurant
  Process, Pitman-Yor, Indian Buffet Process, Beta Process) and their
  backend/PPL handlers, not the core GP routing.
- **Tier A / B / C.** Dispatch tiers (§13.2), orthogonal to
  Tier 1/2/3. A = exact closed-form, B = approximate rewrites
  (Delta, Fenton-Wilkinson, CLT, block-maxima → GEV), C =
  opaque PPL handoff.

"Tier 1 ships" is the positive commitment. "Tier 2 partial /
Tier 3 partially scoped" are explicit design boundaries, not deferrals
to a future Myco version: they belong inside the current design
envelope and block shipping only of the specific families that
need their machinery. Tier A/B/C are about dispatch, not about
what exists: a Tier 1 family can dispatch to any of A/B/C
depending on the transformation applied to it.

### 28. Kernels

**Summary.** Kernels are parameterized relations over two input
domains and one scalar output slot. Point-point same-locus kernels are
the common spatial covariance / interaction specialization, not the
definition. Kernel-ness is expressed via compiler-facing facts and
capability contracts (`PositiveDefinite<A>`, `Stationary<L>`,
`Isotropic<L>`, `CompactSupport<A, B>(radius)`) rather than a separate
keyword or type kind. Stdlib ships Matérn, RBF, rational-quadratic,
and Wendland; composition rules preserve contracts. Finite and
integral kernel-operator semantics are ordinary Myco math. Exact
support / locality semantics are graph facts. Sparse / index lowering
is planner-owned and consumes exact facts without becoming source
semantics. Low-rank / feature approximation semantics are graph and
plan facts. Process-prior / GP-HSGP consumer semantics are locked:
processes are indexed stochastic roots with demand-driven finite
projections; concrete backend and PPL implementations remain tracked
work.

Chunk 03 can now resume on the settled e-graph substrate; the kernel
surface below is committed through finite assembly and ordinary
integral / sum kernel operators, plus exact support / locality facts.
Sparse / index lowering semantics are committed; concrete backend
implementations, concrete low-rank algorithm kernels, and concrete
process-inference backends remain tracked work.

#### 28.1 Kernels as Parameterized Relations with Capability Contracts

**Summary.** A kernel is a parameterized relation with two explicit
input domains and one explicit scalar output slot. The inputs may be
continuous points, discrete entities, mesh cells, intervals,
subdomains, or cross-domain pairs. No separate `kernel` keyword
exists. Kernel-ness comes from compiler-facing facts and capability
contracts on stdlib atoms and derived relation bodies.

A kernel relation has the general shape:

```myco
relation k<A, B, U>(
    x: A,
    y: B,
    out: Scalar<U>,
) {
    out = ...
}
```

The familiar spatial covariance / interaction specialization is:

```myco
relation k<L: Locus, U: Unit>(
    x: Point<L>,
    y: Point<L>,
    out: Scalar<U>,
) {
    out = ...
}
```

No separate `kernel` keyword, no separate type kind. Kernel-ness is a
fact role the compiler derives from body composition plus capability
contracts on atoms, mirroring how relation differentiability is
derived from stdlib expression atoms (§6). The relation shape supports
continuous/continuous, discrete/discrete, and cross-domain kernels:

```myco
relation root_uptake_weight(
    root: RootSegment,
    z: Point<SoilDepth>,
    out: Scalar<dimensionless>,
) { ... }

relation neighbor_competition(
    a: Tree,
    b: Tree,
    out: Scalar<dimensionless>,
) { ... }
```

Kernel facts are typed by the domain shape that makes them meaningful:

- `PositiveDefinite<A>`. Applies to same-domain kernels
  `A, A -> Scalar<U>` and guarantees the Gram matrix
  `K_{ij} = k(x_i, x_j)` is PSD for any finite set of A-values.
  Required for use as a Gaussian Process covariance kernel.
- `Stationary<L>`. Applies to point kernels over a locus where
  translation has meaning; guarantees `k(x, y) = ktilde(x - y)`.
- `Isotropic<L>`. Applies to point kernels over a normed/metric locus;
  guarantees `k(x, y) = khat(distance(x, y))`.
- Support facts: `support(k)`, `nonzero_region(k)`,
  `zero_when(k, predicate)`, `boundary(k)`, and
  `boundary_smoothness(k, boundary)`. Support is predicate-shaped;
  radius, hop count, bounding boxes, and directionality are structured
  summaries when derivable.
- `CompactSupport<A, B>(radius)`. A structured summary for the common
  metric-radius case. It is derived from, or shipped alongside, exact
  support predicates; it is not the whole support model.

For brevity, examples may omit the domain parameter when it is
obvious from the relation signature. Stdlib covariance kernels such
as Matérn (nu = 1/2, 3/2, 5/2, infinity), squared-exponential (RBF),
rational-quadratic, and Wendland carry audited facts. Non-stationary
kernels (e.g. polynomial `k(x, y) = (x * y + c)^d`, Brownian
`k(x, y) = min(x, y)`) can satisfy `PositiveDefinite` without
satisfying `Stationary`. Cross-domain kernels can carry facts such
as exact support predicates or `CompactSupport<A, B>` summaries
without pretending to be same-domain GP covariance kernels.

The usual operations on same-domain covariance kernels preserve the
contracts: sum preserves `PositiveDefinite` and `Stationary`, product
preserves `PositiveDefinite`, scaling by a positive scalar preserves
both, and radial wrapping (`khat(distance(...))`) elevates a
stationary point kernel to `Isotropic` when the locus supplies the
needed metric facts. These closure rules are how the compiler derives
kernel contracts from composition without user property-declaration
surface.

#### 28.2 Ambient-Locus via Composition

**Summary.** Kernel input domains are fixed by the call site, not by a
special kernel kind. Point kernels take `Point<L>` arguments where `L`
is ambient at the call site; cross-domain kernels name their two input
types directly. Kernel families that require specific locus or domain
structure express it via ordinary contracts, not specialized kernel
types.

Point kernels take `Point<L>` arguments, where the locus `L` is
ambient and fixed by where the kernel relation is invoked, not by a
per-kernel declaration. This avoids kernel families that only work on
one space; e.g. squared-exponential is usable on any `L` that admits a
norm, and the compiler picks up the norm from the locus definition
(§11.1). A kernel that requires a specific structure (e.g. spherical
kernels requiring `L = Sphere`) expresses that via a contract on the
locus, not via a specialized kernel type. Cross-domain kernels name
the participating source domains directly, such as `RootSegment` to
`Point<SoilDepth>` or `Tree` to `Tree`.

Composite kernels compose ambient-locus the same way any other
parameterized relation composes: the composed relation invokes the
component relations into explicit temporary output slots, then relates
the final output to their sum, product, or scaling. The compiler
checks that the component kernels' input domains and facts line up.
Product kernels on product loci (`L = L_x x L_y`) use paired point
arguments; the PositiveDefiniteness closure rule covers the
same-domain covariance case.

#### 28.3 Kernel Matrix and Gram Assembly

**Summary.** `kernel_matrix(k, xs, ys)` is the general finite assembly
surface for two-domain kernel relations. `gram(k, points)` is the
same-domain covariance specialization, sugar for
`kernel_matrix(k, points, points)` plus same-domain fact rules. Gram
emits symmetry / PSD / PD facts only from established kernel and
point-set evidence; it never silently adds jitter or routes to an
opaque backend.

For a general two-domain kernel relation:

```myco
relation k<A, B, U>(x: A, y: B, out: Scalar<U>)
```

finite assembly is:

```myco
W = kernel_matrix(k, xs, ys)
```

with semantics:

```text
W[i, j] = k(xs[i], ys[j])
row_axes(W) = xs
col_axes(W) = ys
entry_unit_law(W[i,j]) = output_unit(k)
construction_provenance(W) = evaluated_pairwise(k, xs, ys)
kernel_matrix_of(W, k, xs, ys)
```

This covers continuous/discrete and discrete/continuous operator
matrices such as root uptake over soil depth or shade from leaves
onto canopy points. It does not emit symmetry, PSD, or covariance
facts merely because it is kernel-shaped.

For a same-domain kernel:

```myco
relation k<A, U>(x: A, y: A, out: Scalar<U>)
```

`gram(k, points)` is sugar for `kernel_matrix(k, points, points)` with
additional same-domain facts when evidence supports them:

```text
gram_of(K, k, points)
row_axes(K) = points
col_axes(K) = points
construction_provenance(K) = evaluated_pairwise(k, points, points)
```

Fact emission:

- `SymmetricKernel<A>` emits `symmetric(K)`.
- `PositiveDefinite<A>` emits `positive_semidefinite(K)`.
- `StrictPositiveDefinite<A>` plus `distinct(points)` emits
  `positive_definite(K)`.
- Exact support / `zero_when` facts emit `zero_pattern` when the
  A-domain distance / adjacency evidence proves finite pairs are zero;
  `CompactSupport<A, A>(radius)` is one common summary that can help
  establish those facts.

The PSD/PD split is intentional. Many covariance kernels prove only
`positive_semidefinite(K)`. Ordinary `cholesky(K)` requires
`positive_definite(K)` (§30); PSD alone is an unmet obligation. The
compiler must not silently add jitter, select a pivoted factorization,
or hand off to an opaque backend. Valid routes include proving
distinctness plus strict positive definiteness, explicitly modeling
jitter, or choosing a primitive / workflow policy that accepts PSD.

#### 28.4 Kernel Operators, Measures, and Approximate Lowering

**Summary.** Kernel operators are ordinary `integrate` / `sum`
expressions, not a separate source construct. Domains contribute only
their canonical geometric or counting measure; model-specific
densities and weights must appear as graph values. The compiler may
recognize linear kernel-action patterns and emit operator facts, but
continuous `integrate` expressions remain continuous semantics until
closed exactly or explicitly lowered by workflow approximation policy.

The canonical continuous form is ordinary integration:

```myco
effect(x) =
    integrate(k(x, y) * source(y), y, Domain)
```

The canonical finite form is ordinary aggregation:

```myco
effect(x) =
    sum(k(x, sample.location) * sample.value * sample.weight
        for sample in observations)
```

No `kernel_apply`, `kernel_operator`, or `convolve` source form is
required for v2.1. A future stdlib convenience may desugar to these
ordinary expressions, but the semantic surface is the expression
itself.

**Measures and weights.** `integrate(expr, y, Domain)` uses the
domain's canonical measure from §14.3: length on intervals, area on
surfaces, volume on volume loci, and counting measure for finite
collections where integration is specialized to summation. If the
domain has no canonical measure, measure choice is an unmet
obligation.

Biological, empirical, quadrature, mesh, or normalization weights are
not hidden in the kernel operator. They must be ordinary factors with
ordinary units and provenance:

```myco
light(z) =
    integrate(canopy_k(z, h) * leaf_area_density(h) * transmission(h),
              h,
              CanopyHeight)

effect(x) =
    sum(k(x, node.pos) * field(node.pos) * node.volume
        for node in mesh.nodes)
```

This keeps model claims in the graph: canonical geometry is implicit;
noncanonical weighting is explicit.

**Recognition.** The compiler recognizes kernel operators by
normalizing ordinary expressions, not by user annotation. Recognition
is permitted when an `integrate` or `sum` body can be expressed as a
linear kernel action over the bound variable:

```myco
integrate(k(x, y) * source(y), y, Domain)
integrate(source(y) * k(x, y) * density(y), y, Domain)
sum(k(x, item.pos) * item.value * item.weight for item in items)
```

The recognized operator may emit compiler-facing facts such as:

```text
kernel_integral_of(effect, k, source_factor, x, y, Domain)
kernel_sum_of(effect, k, source_factor, x, item, items)
linear_in(effect, source_factor)
operator_domain(effect) = Domain
operator_target(effect) = x
operator_measure(effect) = canonical_measure(Domain)
```

Additive combinations of recognized kernel actions remain
recognizable as sums of actions. Nonlinear wrapping does not:

```myco
integrate(exp(k(x, y) * source(y)), y, Domain)
```

is valid Myco if otherwise well typed, but it is not a linear kernel
operator and does not receive sparse / convolution / low-rank lowering
facts.

**Operator facts.** Kernel facts live on kernel relations; operator
facts live on recognized operator expressions. The compiler derives
operator facts from the combination of kernel facts, domain facts,
measure facts, weights, boundaries, and the source expression. No
operator property is inherited from a kernel relation alone unless
the use-site context preserves it.

For example, a compact-support kernel action may emit:

```text
local_coupling(effect)
dependency_radius(effect) = radius(k)
zero_pattern(effect) when finite axes / adjacency facts prove separation
```

A normalized nonnegative kernel over a compatible domain may emit
`constant_preserving(effect)` or `positivity_preserving(effect)`.
Multiplying by a mask, empirical weight, boundary cutoff, or
non-normalized density removes those facts unless a separate rule
proves them for the full operator expression.

**Finite exactness vs. continuous approximation.** A `sum` over a
finite collection is exact finite semantics. The compiler may lower it
through `kernel_matrix(k, targets, sources)` and matrix-vector
contraction when axes and unit laws match; no approximation ledger is
needed because the source expression is already finite.

An `integrate` over a continuous domain remains a continuous semantic
object. Closed-form antiderivatives, exact symbolic rewrites, or exact
finite-measure reductions are exact. Numerical quadrature, mesh
sampling, inducing points, truncation, and low-rank forms are
approximations unless their exactness is proven. Such lowerings
require workflow-selected approximation policy or an explicit
`.myco` `approximate` model claim, and must emit provenance such as:

```text
quadrature_lowering_of(discrete_effect, continuous_effect)
approx_error(discrete_effect, continuous_effect, envelope)
relaxation_ledger_entry(discrete_effect)
```

The compiler does not silently replace a continuous kernel integral
with a finite computation to make a backend run.

#### 28.5 Exact Support, Locality, and Truncation

**Summary.** Exact support is a predicate-shaped model fact, not a
radius-only annotation. Myco distinguishes closed support,
nonzero-region, and exact-zero predicates; sparse zero patterns come
from `zero_when`, not support alone. Tail bounds create approximation
opportunities, while truncation of infinite-tail kernels is explicit
modeling or workflow approximation, never compiler housekeeping.

Support vocabulary:

- `support(k) = P(x, y)`. The closed support region for a kernel
  relation.
- `nonzero_region(k) = Q(x, y)`. The region where the kernel may be
  nonzero.
- `zero_when(k, R(x, y))`. An exact predicate under which the kernel
  value is proven zero.
- `boundary(k) = B(x, y)`. The boundary of the support / regime.
- `boundary_smoothness(k, B) = C0 | C1 | C2 | ... | C∞ |
  discontinuous | unknown`. The differentiability order proven across
  the boundary.
- `tail_bound(k, outside_region, envelope)`. A quantitative decay
  bound, not an exact zero fact.
- `truncation_of(truncated, original, region)`. Provenance for an
  explicit truncated model or workflow approximation.

For a smooth compact kernel such as a Wendland family:

```text
support(k) = distance(x, y) <= r
nonzero_region(k) = distance(x, y) < r
zero_when(k, distance(x, y) >= r)
boundary(k) = distance(x, y) = r
boundary_smoothness(k, boundary) = C2
CompactSupport<A, B>(r)
metric_radius(k) = r
```

For a hard cutoff whose inner value is nonzero at the boundary:

```myco
where distance(x, y) <= r {
    out = base_k(x, y)
} else {
    out = 0
}
```

the compiler may derive:

```text
support(k) = distance(x, y) <= r
zero_when(k, distance(x, y) > r)
boundary(k) = distance(x, y) = r
boundary_smoothness(k, boundary) = discontinuous | unknown
```

Closed support, nonzero region, and exact-zero predicates are distinct
because they serve different downstream consumers: support/locality
describes dependency geometry, `zero_when` drives exact sparse
patterns, and boundary smoothness controls gradient / event behavior.

**Evidence sources.** Exact support facts may come from:

- visible relation bodies that branch to exact zero,
- audited stdlib facts for curated kernels,
- provider-validated finite artifacts such as a validated sparse
  kernel matrix pattern or neighbor graph.

User-authored `.myco` cannot assert unchecked support facts such as
`property k is CompactSupport(r)`. Provider validation can establish
facts about the concrete artifact it provides, e.g.
`zero_pattern(K)`, but it does not turn the source relation into a
globally compact kernel unless the source / stdlib evidence supports
that relation-level fact.

**Predicate-shaped locality.** The core support fact is a predicate,
not a metric radius:

```text
support(k) = edge_exists(a, b)
support(k) = upstream_of(y, x) && path_length(y, x) <= r
support(k) = abs(dx) <= rx && abs(dy) <= ry && windward(x, y)
```

Structured summaries are derived from, or shipped alongside, the
predicate:

```text
metric_radius(k) = r
graph_hop_radius(k) = 1
anisotropic_box(k) = (rx, ry)
directed_support(k)
local_coupling(k)
```

Spatial indexes and sparse lowering consume summaries such as radius,
hop radius, bounding boxes, and directionality; exact semantic
dependency and zero facts consume the predicate.

**Support boundaries and gradients.** Support boundaries are regime
boundaries only to the extent their smoothness affects the operation.
Smooth compact support emits differentiability facts up to the
proven order. Discontinuous or unknown boundaries do not silently
authorize gradients across the boundary; gradient-sensitive use must
route through the ordinary crossing machinery from §8.10:
strict rejection, one-sided derivative, subgradient, relaxation, or
estimator policy as appropriate.

**Exact support vs. truncation.** If a truncated kernel is written in
`.myco`, the truncation is a model claim:

```myco
relation truncated_rbf(x: Point<L>, y: Point<L>, out: Scalar<U>) {
    d = distance(x, y)

    where d <= cutoff {
        out = rbf(d, ell)
    } else {
        out = 0
    }
}
```

This emits exact support / zero facts if the body proves them. If a
workflow truncates an infinite-tail kernel for speed, the result is an
approximation:

```text
tail_bound(rbf, distance > cutoff, eps)
truncation_of(truncated_op, original_op, distance <= cutoff)
approx_error(truncated_op, original_op, envelope)
relaxation_ledger_entry(truncated_op)
```

Tail bounds expose opportunities and envelopes; they never become
exact `zero_pattern` facts by themselves.

**Downstream consequences.** Exact support may emit:

```text
local_coupling(effect)
dependency_region(effect(x)) = { y | support(k)(x, y) }
zero_pattern(W[i, j]) when zero_when(k(targets[i], sources[j]))
sparse_candidate(W)
neighbor_index_candidate(k, metric_radius = r)
truncation_candidate(k, cutoff, envelope)  # from tail bounds only
```

Dependency / locality facts are semantic. Exact zero patterns require
`zero_when` proof for concrete finite axes or provider validation.
Indexing candidates are lowering opportunities, not source semantics.
Tail-bound candidates are approximation opportunities, not sparse
facts.

**Composition.** Support facts compose through ordinary expression
structure:

- Additive combinations use the union of supports:
  `support(k1 + k2) <= support(k1) union support(k2)`.
  `zero_when(k1 + k2)` requires both summands to be zero unless
  cancellation is separately proven.
- Multiplicative combinations use the intersection:
  `support(k1 * k2) <= support(k1) intersect support(k2)`.
  `zero_when(k1 * k2)` holds when either factor is proven zero.
- Scaling preserves support when the scale is proven nonzero, collapses
  support when the scale is proven zero, and otherwise emits only the
  conservative upper bound.
- Operator expressions may refine dependency regions with source-side
  masks or zero facts only when those facts are structural or
  provider-validated. A runtime value that happens to be zero does not
  create a static zero pattern.

#### 28.6 Sparse / Index Lowering and Provider Patterns

**Summary.** Sparsity is semantic evidence; sparse storage is an
execution choice. Exact `zero_when` and support facts may produce
finite patterns, neighbor-index opportunities, and matrix-free
lowerings. The planner may choose dense, sparse, block-sparse,
neighbor-list, spatial-index, or matrix-free execution from legal
exact candidates. Approximate indexes or sparsification require
workflow approximation policy and ledger facts.

The semantic layer emits facts such as:

```text
zero_pattern(W[i, j])
sparse_candidate(W)
dependency_region(effect(x))
neighbor_index_candidate(effect, predicate)
```

These facts describe the mathematical object or dependency graph.
They do not say that source-level `W` is `CSR`, `CSC`, block-sparse,
or stored at all.

**Pattern materialization.** The planner may materialize exact finite
patterns when the zero / support predicate is decidable for finite
axes:

```text
row_neighbors[i] = { j | not zero_when(k(xs[i], ys[j])) }
csr_pattern_of(P, W)
neighbor_list_for(effect, P)
pattern_from_support(P, support(k), xs, ys)
```

Pattern materialization is exact when it is derived from exact
`zero_when` / support facts or provider validation. It is still a
plan artifact, not source semantics.

**Coverage.** Exact sparse / index lowering requires complete
coverage of every possibly nonzero pair:

```text
complete_for(index, support_predicate, axes)
```

meaning if `support_predicate(x, y)` may be true, `y` is returned by
`index.query(x)`. Soundness is optional:

```text
sound_for(index, support_predicate, axes)
```

meaning every returned pair satisfies the predicate. A conservative
index may be complete but not sound; false positives are legal if the
lowering evaluates the exact predicate before accumulation:

```text
candidates = bbox_index.query(x)
for y in candidates:
    if support(k)(x, y):
        acc += k(x, y) * source(y)
```

An approximate nearest-neighbor index that may omit possibly nonzero
pairs is not an exact lowering unless a proof establishes
`complete_for`. Otherwise it requires approximation policy and emits
`approximate_index_lowering` / error-ledger facts.

**Pattern phase and invalidation.** Sparse patterns and indexes carry
phase and dependency facts:

```text
pattern_phase(P) =
    compile_static | bind_static | step_static | dynamic_query
depends_on(P, facts...)
invalidates_on(P, event)
```

`compile_static` patterns are fixed by source / topology facts.
`bind_static` patterns are fixed after workflow binding.
`step_static` patterns may be reused inside one solve step but must
be rebuilt when their dependencies change. `dynamic_query` patterns
cannot be reliably prebuilt and must query at runtime or use a
dynamic sparse runtime. Reusing a pattern outside its valid phase is
illegal unless the workflow explicitly selects an approximation or a
replanning / capacity-mask policy that preserves semantics.

**Operator-general lowering.** Sparse / index lowering applies to
kernel actions, not only materialized matrices. These are all legal
lowering targets for recognized exact candidates:

```text
sparse_matrix_lowering(W)
sparse_matvec_lowering(effect)
neighbor_sum_lowering(effect)
matrix_free_kernel_action(effect)
runtime_spatial_query(effect)
fixed_pattern_dynamic_values(effect)
```

For example:

```myco
W = kernel_matrix(k, targets, sources)
effect = W * values
```

may materialize a sparse matrix, while:

```myco
effect(x) =
    sum(k(x, y) * value(y) for y in sources)
```

may lower to matrix-free neighbor iteration. If the pattern is fixed
but kernel values depend on changing parameters, the planner may
reuse row / column indices and recompute numeric values:

```text
fixed_pattern_dynamic_values(effect)
```

All choices preserve the same exactness rules.

**Workflow policy.** Workflows may rank legal exact lowerings without
changing source semantics:

```text
lowering_candidate(effect, dense)
lowering_candidate(effect, csr)
lowering_candidate(effect, neighbor_list)
lowering_candidate(effect, matrix_free)
requires_capability(effect, sparse_matvec)
requires_capability(effect, dynamic_query)
cost_of(candidate) = ...
```

A workflow storage / lowering policy may prefer `matrix_free` over
`CSR`, or `CSR` over `dense`, among legal exact candidates. It cannot
authorize dropped pairs, ANN misses, threshold sparsification, or
tail truncation; those are approximation policies and must route
through §15.1 / §28.4 / §28.5 approximation provenance.

`hypha explain` should expose the distinction:

```text
legal exact:
  dense
  csr: pattern_phase = bind_static
  matrix_free: complete_for(index, support)
approximate:
  ann_query: requires approximation policy
  tail_truncation: requires tail_bound + approximation policy
```

**Provider artifacts.** Workflow providers may supply sparse patterns,
neighbor graphs, spatial indexes, matrix-free query structures, and
realized discrete operator artifacts as validated artifacts:

```text
csr_pattern_of(P, W)
complete_for(index, support_predicate, axes)
sound_for(index, support_predicate, axes)
pattern_phase(P) = bind_static
depends_on(P, axes, radius)
validated_by(P, exact_distance_check)
validated_by(P, topology_adjacency_certificate)
discrete_operator_of(D_h, spatial_operator_site)
discretization_lowering_of(D_h, continuous_site, topology_version, scheme_id)
```

Provider validation produces artifact-level facts for the current run.
It may satisfy obligations on a concrete `kernel_matrix` or operator
lowering. It does not grant unchecked relation-level facts such as
`support(k)` or `CompactSupport<A, B>(r)` unless the source relation
or audited stdlib implementation already establishes them.

A realized discrete operator artifact may emit structural and numerical
facts such as `row_sum_zero`, `stencil_pattern`, `local_coupling`,
`conservative_transfer`, `adjoint_pair`, `solver_residual_bound`,
`quadrature_order`, or `discretization_order`. Each emitted fact carries
an evidence grade (§37.1). The artifact can be an assembled sparse
matrix, stencil bundle, finite-volume flux action, FEM/DG weak-form
action, matrix-free callable, remap operator, or provider-owned handle.
It is an execution artifact, not a second source of `.myco` truth.

#### 28.7 Separability, Feature Expansions, and Low-Rank Approximation

**Summary.** Low-rank is not one semantic category. Myco
distinguishes exact separability, exact finite feature expansions, and
approximate feature / low-rank expansions. Exact forms may rewrite
freely when proven. Approximate forms require source `approximate`
claims or workflow approximation policy with scoped error provenance.
Feature maps, modes, inducing points, and random features are ordinary
relations or workflow artifacts, not a source `basis` construct.

**Exact separability.** A kernel over a product structure may factor
exactly:

```text
k((x1, z1), (x2, z2)) = kx(x1, x2) * kz(z1, z2)
```

Compiler facts:

```text
separable_kernel(k, product_axes=[X, Z])
kernel_factors(k) = [kx, kz]
product_domain(D, [X, Z])
product_axes(points, [X_points, Z_points])
```

For product finite axes:

```text
K = gram(k, X_points x Z_points)
Kx = gram(kx, X_points)
Kz = gram(kz, Z_points)
kronecker_factorization(K, [Kx, Kz])
```

For operators, kernel separability and product-domain facts authorize
tensor-product quadrature / operator lowerings. A fully separated
action additionally requires source separability:

```text
separable_source(f, product_axes=[X, Z])
source_factors(f) = [fx, fz]
separable_operator_action(T)
```

Approximate separability is not a kernel fact; it is an approximation
with the same provenance and envelope requirements as other low-rank
approximations.

**Exact finite feature expansions.** A kernel may have an exact finite
feature representation:

```text
k(x, y) = sum_m phi_m(x) * lambda_m * phi_m(y)
```

Facts:

```text
exact_feature_expansion(k, Phi, Lambda)
feature_map(Phi, domain=A, feature_axis=M)
rank_bound(gram(k, points)) <= M
```

These facts may be recognized from visible `.myco` relations:

```myco
relation seasonal_kernel(day1, day2, out: Scalar<U>) {
    out = a0
        + a1 * cos(day1) * cos(day2)
        + b1 * sin(day1) * sin(day2)
}
```

or shipped as audited stdlib facts for families such as fixed-degree
polynomial kernels or finite spectral families. No new `basis`
declaration is introduced; features are ordinary relations and axes.

**Approximate feature expansions.** Nyström, inducing points, random
features, truncated spectral / HSGP, truncated SVD, and related
methods produce approximations unless exactness is separately proven:

```text
approx_feature_expansion(k_M, k, Phi_M, Lambda_M)
approximation_scope(k_M, scope)
approx_error(k_M, k, envelope)
relaxation_ledger_entry(k_M)
rank_bound(gram(k_M, points)) <= M
```

The approximation scope is explicit:

- `approx_relation(k_M, k, DomainA x DomainB, envelope)`.
- `approx_matrix(K_M, K, axes, norm, envelope)`.
- `approx_operator(T_M, T, source_space -> target_space, envelope)`.

A lower-scope approximation does not imply a higher-scope one. A
truncated SVD of one Gram matrix is not a relation-level kernel fact.
A relation-level kernel approximation does not imply an operator norm
bound unless the required measure and source-space facts are present.

**Error propagation.** Approximation envelopes propagate only through
named implication rules that state their norm / tolerance class and
required axis, measure, or source-space facts. For example, a uniform
relation error over a finite point set may imply conservative matrix
error bounds:

```text
entrywise_error(K_M, K) <= eps
frobenius_error(K_M, K) <= n * eps
spectral_error(K_M, K) <= n * eps
```

Stronger facts may be emitted when established directly, e.g.
`spectral_error(K_M, K) <= eps_K` or
`operator_error(T_M, T) <= eps_T`. Preserved structural facts such as
symmetry, PSD, support, conservation, or rank are emitted only when
the approximation construction proves them. Error bounds do not
silently authorize substituting approximate objects into exact
obligations.

**PSD, PD, and rank.** Feature covariance constructions may emit PSD
and rank facts when their algebra proves them:

```text
K_M = Phi * Lambda * transpose(Phi)
nonnegative_diagonal(Lambda)
positive_semidefinite(K_M)
rank_bound(K_M) <= M
```

They emit `positive_definite(K_M)` only with explicit full-rank
evidence or an explicit positive diagonal / noise component:

```text
K_obs = Phi * Lambda * transpose(Phi) + sigma2 * identity(n)
sigma2 > 0
positive_definite(K_obs)
```

No low-rank approximation silently adds jitter or upgrades PSD to PD.
Consumers must either accept PSD / low-rank covariance, use a
PSD-compatible primitive, or satisfy explicit PD obligations.

**Three authorization routes.** Exact feature / separability rewrites
may be compiler-selected when proven from visible relations, audited
stdlib facts, or validated exact artifacts. Approximate low-rank /
feature forms require either:

- an explicit source `approximate` model claim, or
- workflow approximation policy.

A finite-feature relation written directly in `.myco` is simply the
model unless it is explicitly related to a richer kernel by
approximation provenance.

**Nyström and inducing axes.** Nyström / inducing-point methods are
finite intermediary-axis approximations, not GP-only constructs:

```text
Z = inducing_axis
Kxz = kernel_matrix(k, X, Z)
Kzz = gram(k, Z)
K_approx = Kxz * solve(Kzz, transpose(Kxz))
nystrom_approximation(K_approx, K, inducing_axis=Z)
approximation_scope(K_approx) = finite_matrix(X)
```

All solver obligations on `Kzz` remain ordinary matrix obligations.
If `positive_definite(Kzz)` is unknown, ordinary inverse / Cholesky
routes report it. PSD-compatible primitives may be selected
explicitly. Stabilization or diagonal noise is explicit model /
workflow policy with provenance, never automatic.

**Random features.** Random-feature approximations are workflow
artifacts with random-draw provenance, not model stochastic roots:

```text
random_feature_approximation(k_M, k, feature_axis=M)
feature_draw(Phi_M, distribution, seed)
probabilistic_error_bound(k_M, k, confidence=0.99, envelope)
reproducible_artifact(k_M) | stochastic_plan_artifact(k_M)
```

Fixed seeds produce reproducible artifacts. Unfixed seeds produce
stochastic plan artifacts. Neither introduces `~` into the source
model unless the user explicitly models that randomness.

**Spectral / HSGP-style truncations.** HSGP is one spectral
truncation pattern over an explicit bounded domain and boundary
condition:

```text
k(x, y) = sum_m lambda_m * phi_m(x) * phi_m(y)
k_M(x, y) = sum_{m in M} lambda_m * phi_m(x) * phi_m(y)
```

Facts:

```text
spectral_family(k, domain, boundary_condition)
mode_set(Phi_M, domain, boundary_condition, count=M)
orthonormal_modes(Phi_M, measure)
lambda_nonnegative(Lambda_M)
spectral_truncation_of(k_M, k, modes=Phi_M)
approx_feature_expansion(k_M, k, Phi_M, Lambda_M)
tail_bound(k, modes_not_in(Phi_M), spectral_envelope)
```

Domain, boundary, mode normalization, and spectral-density facts are
ordinary obligations or workflow artifact facts. PSD / rank facts
follow when the mode construction proves them; PD still requires
full-rank or explicit positive diagonal evidence.

**Consumer note.** The process-prior machinery in §28.8 consumes the
facts above. Low-rank covariance objects are ordinary PSD /
rank-bounded matrix or operator objects until a process consumer uses
them; HSGP is one workflow approximation pattern over these facts, not
a source-language mechanism.

#### 28.8 Process Priors and GP / HSGP Consumers

**Summary.** Process priors are workflow-side sources over indexed
`.myco` contracts. The source model declares indexed relations,
fields, and contracts; the workflow binds a `ProcessPrior<I,V>` by
naming the index slots and value slots explicitly. Gaussian processes
are one `ProcessLaw<I,V>` that consumes kernel / covariance facts and
emits demand-driven finite projection problems. HSGP, inducing-point,
random-feature, and other low-rank forms are explicit exact or
approximate process-law / workflow artifacts with provenance, not
special syntax. Structured process values are first-class.

Process-valued uncertainty is not a third branch beside epistemic and
aleatoric uncertainty. The uncertainty kind still classifies the
source of uncertainty; process-valuedness describes the sample shape
and identity of the stochastic root:

```text
uncertainty kind: epistemic | aleatoric
sample shape: scalar | vector | record | field/process |
              graph-indexed family | temporal path
```

Core facts:

```text
process_root(P)
process_index_contract(P) = I
process_value_contract(P) = V
projection_of(x_i, P, index_i)
same_process_root(x_i, x_j)
process_coupling(P, kernel_or_law)
epistemic_process(P) | aleatoric_process(P)
```

The words `index` and `value` do not create input/output direction in
the Myco graph. They identify the finite projection axis and the
sample component being coupled by the process law. Information still
flows through the ordinary graph constraints as freely as the model and
solver permit.

**Source / workflow split.** `.myco` declares the indexed world shape:

```myco
relation vulnerability_curve(
    psi: Scalar<MPa>,
    loss: Scalar<dimensionless>,
) { ... }
```

The workflow binds an epistemic process prior to that shape:

```python
workflow.bind(
    "Plant.vulnerability_curve",
    ProcessPrior(
        index=["psi"],
        value="loss",
        law=GaussianProcess(mean=Zero(), kernel=matern52),
    ),
)
```

The binding must name slots. The compiler does not guess which side of
a relation is "input" or "output" from position, ordering, or naming
convention. Canonical emitted facts include:

```text
process_target = Plant.vulnerability_curve
process_index_slots = [psi]
process_value_slot = loss
process_index_contract = Scalar<MPa>
process_value_contract = Scalar<dimensionless>
process_law = GaussianProcess
```

`GPPrior(...)` may exist as Python convenience sugar for
`ProcessPrior(law=GaussianProcess(...))`, but `ProcessPrior` is the
canonical workflow object. There is no GP-specific `.myco` syntax in
v2.1.

**Process-law contract.** The general source object is:

```text
ProcessPrior<I, V>
```

where `I` is the index contract and `V` is the value contract. A
process law advertises finite-projection capabilities:

```text
ProcessLaw<I,V>
FiniteProjectionLaw<I,V>
ClosedFormConditionable<I,V>
ProjectionLogDensity<I,V>
ProjectionSampleable<I,V>
ApproximationFamily<I,V>
```

A Gaussian process law has the shape:

```text
GaussianProcessLaw<I,V>
requires:
  mean: I -> V
  covariance/kernel: I x I -> Covariance<V>
emits for finite indices:
  finite joint distribution, usually MVN when facts permit
```

Non-Gaussian process laws may emit an opaque finite joint and route to
`ProcessInferenceTask` / Tier C unless they advertise stronger
finite-projection or conditioning capabilities.

**Demand-driven finite projections.** A process envelope remains
abstract until a finite projection is demanded by a model read, an
observation, or a workflow prediction/output query. For one process
root `P`, all such points are gathered into projection axes:

```text
process_projection(y_i, process=P, index=psi_i)
projection_axis(P, observed_points)
projection_axis(P, required_model_points)
projection_axis(P, prediction_points)
same_process_root(y_i, y_j)
coupled_by(P, K)
```

For a GP law:

```text
points = observed_points union required_model_points union prediction_points
K = gram(kernel(P), points)
mu = mean(P, points)
finite_process_joint(y_points, mu, K)
```

Finite projection construction preserves process identity. Projections
from the same process are coupled by one stochastic root; they are not
independent roots merely because they appear at different points in the
graph.

**Observations condition projections.** Observations condition finite
projections; they do not equationally merge the process value with the
data. Exact observation is an explicit mode. Noisy observation requires
an explicit noise law:

```python
workflow.observe(
    "Plant.vulnerability_curve.loss",
    data=points,
    noise=Normal(sigma_obs),
)
```

Semantics:

```text
projection f_i = P.at(x_i)
observation y_i = f_i + eps_i
eps_i ~ NoiseLaw(...)
likelihood_term(log_density(noise, y_i - f_i))
```

There is no silent nugget, jitter, stabilization, or observation noise.
Correlated observation noise is another explicit joint/process family:

```python
workflow.observe(
    "Plant.vulnerability_curve.loss",
    data=points,
    noise=ProcessNoise(kernel=obs_noise_kernel, over=["psi"],
                       value="loss"),
)
```

For observations plus prediction queries, the finite joint includes
both observed and queried projection points:

```text
[y_obs, y_pred] ~ MVN([mu_obs, mu_pred],
  [[Koo, Kop],
   [Kpo, Kpp]])
```

Closed-form conditioning is available only when the process law, kernel
facts, matrix facts, and observation-noise facts satisfy the required
obligations:

```text
closed_form_process_predictive(P, query_axis)
posterior_predictive_of(pred_values, P, observations, query_axis)
predictive_mean(pred_values)
predictive_covariance(pred_values)
```

Otherwise the compiler constructs:

```text
process_inference_task(P, observed_axis, query_axis)
```

and routes through the ordinary Tier A/B/C dispatch ladder. Posterior
samples, predictive means, covariances, and draws are workflow results
with provenance. They do not mutate the source process and do not
become new global process facts unless a closure rule proves that
parametric posterior process.

**Structured process values.** `V` may be scalar, vector, tensor,
enum-narrowed record, or named record. A process prior over multiple
value slots creates one structured process root. Separate
`ProcessPrior` bindings create separate roots unless a visible shared
latent construction or joint process law couples them.

Example:

```myco
relation growth_response(
    temp: Scalar<K>,
    water: Scalar<dimensionless>,
    height_gain: Scalar<m>,
    leaf_area_gain: Scalar<m^2>,
) { ... }
```

Workflow:

```python
workflow.bind(
    "Plant.growth_response",
    ProcessPrior(
        index=["temp", "water"],
        value=["height_gain", "leaf_area_gain"],
        law=GaussianProcess(mean=structured_mean,
                            kernel=growth_kernel),
    ),
)
```

Finite projections flatten into a product axis:

```text
projection axis: points i = 1..N
component axis: fields/components a in components(V)
joint covariance axis: (i, a) x (j, b)

process_component_axis(P) = [height_gain, leaf_area_gain]
joint_process_axis(P) = projection_axis x component_axis
entry_unit_law(K[(i,a),(j,b)]) = unit(value[a]) * unit(value[b])
```

For example:

```text
cov((temp_i, water_i, height_gain),
    (temp_j, water_j, leaf_area_gain)) : Scalar<m * m^2>
```

For a structured process value, `P.at(index)` returns one value of
contract `V`; field access is a deterministic component projection:

```text
process_projection(g, P, index)
component_projection_of(h, g, component=height_gain)
component_projection_of(a, g, component=leaf_area_gain)
same_process_root(h, a)
```

Mean functions normalize to:

```text
process_mean(P): I -> V
```

Component-wise workflow mean helpers are convenience only:

```python
mean={
    "height_gain": Zero(),
    "leaf_area_gain": LinearMean(...),
}
```

They normalize to a structured mean value. Mean values obey ordinary
units and contracts; unknown mean parameters are ordinary trainable
sources.

**Structured covariance validity.** Multi-output covariance validity is
proved over the flattened `(index, component)` domain. Legal routes:

```text
k_flat: (I,Component<V>) x (I,Component<V>)
    -> Scalar<unit(a) * unit(b)>
PositiveDefinite<I x Component<V>>(k_flat)
gram(k_flat, joint_axis) => positive_semidefinite(K_joint)
```

or an audited / derived operator-valued kernel fact:

```text
PositiveOperatorValuedKernel<I,V>
```

Construction rules can also prove validity. A separable output kernel
is PSD when `k_input` is PD/PSD and `B` is PSD:

```text
k((x,a),(y,b)) = k_input(x,y) * B[a,b]
```

An LMC / coregionalization construction is PSD when each input kernel
and component covariance is PSD:

```text
k((x,a),(y,b)) = sum_q B_q[a,b] * k_q(x,y)
```

Visible shared-latent factor constructions are another valid route.
User source cannot assert structured covariance validity without
evidence.

**Dispatch and backend handoff.** Process priors use the same dispatch
ladder after finite projection construction:

```text
ProcessPrior bound
-> finite projections demanded
-> finite joint / operator problem constructed
-> A: closed-form process conditioning
-> B: authorized approximation
-> C: whole stochastic SCC / process inference task
```

Tier C receives the whole unresolved stochastic SCC: latent sources,
process projections, observations, deterministic downstream relations,
constraints, approximation/provenance facts, and kernel/process-law
facts. Myco does not hand off one observation, one kernel, or one
process factor at a time.

Remaining kernel-adjacent work:

- **Concrete sparse backend implementations.** The exactness and
  planner vocabulary is committed; backend storage kernels, cost
  calibration, and capability profiles for `CSR`, block-sparse,
  neighbor-list, and dynamic query runtimes remain implementation work.
- **Concrete low-rank implementation kernels.** The semantic split is
  committed; concrete backend kernels, cost calibration, and stdlib
  family catalogs for Nyström, random features, spectral truncation,
  and HSGP-style plans remain implementation work.
- **Concrete process-inference implementations.** The source/workflow
  split, finite-projection semantics, structured values, and dispatch
  contract are committed. Backend kernels, PPL serializers, closed-form
  conditioning implementations, approximation-family catalogs, and
  non-GP process-law catalogs remain implementation / catalog work.

### 29. Units Library

**Summary.** The core units library ships SI base units, common
SI-derived units via derived-unit algebra, the named dimensionless
angle unit `rad`, stdlib semantic quantity types `Angle` and `Phase`,
standard affine conversions between equivalent spellings, and
dimensionless-ratio handling. Domain-specific unit libraries
(ecophysiology, chemistry, finance) stay out of core and ship as
distributable packages on top.

SI base units (m, kg, s, A, K, mol, cd). Common SI-derived units
(N, Pa, J, W, C, V, Ω, Hz, rad, etc.) via derived-unit algebra (§5).
`rad` is a named dimensionless unit; `Angle` and `Phase` are stdlib
semantic quantity types over `Scalar<rad>` (§5.0). Standard affine
conversions between equivalent SI-derived spellings. Dimensionless-
ratio handling.

Domain-specific unit libraries (ecophysiology, chemistry, astronomy,
finance, etc.) are **out of scope** for Myco core: they ship as
distributable Myco packages that import the core units library and
add domain-specific units, refinements, and conversion declarations
on top. This keeps the core stdlib narrow and keeps domain expertise
under the domain's own project maintenance.

### 30. Matrix and Tensor Primitives

**Summary.** Matrix / tensor primitives are fact consumers and fact
emitters. They do not ask for a user-marked matrix role. They require
established graph facts (§3.9), emit new facts with provenance, and
report unmet obligations when a required fact is unknown. Backend
kernels are implementation choices that preserve the same semantics,
not semantic fallbacks.

Chunk 05 is closed for the source-level matrix / tensor layer. This
section commits the stdlib function surface, finite matrix assembly
syntax, provider-slot distinction, and primitive fact contracts; type
content lives in §3.9 per the chunk 05 scope decision.

The matrix / tensor stdlib ships the linear-algebra primitives that
the rest of the spec depends on by name, in particular the Cholesky
factorization used in MVN reparameterization (§13.6, Z10) and the
kernel Gram-matrix machinery (§28). Committed primitives and their
fact contracts:

Naming policy: standard math names are lowercase (`det`, `trace`,
`transpose`, `adjoint`, `solve`, `norm`); `inverse(A)` is the
canonical spelling rather than `inv(A)`; matrix product uses ordinary
`*` with shape / axis facts governing contraction; numeric matrix
rank is `matrix_rank(A)` to avoid collision with shape rank
(`rank(shape)`).

| primitive | required facts | emitted facts | unmet-obligation behavior |
|---|---|---|---|
| `cholesky(A)` | `rank(A)=2`, `square(A)`, `symmetric(A)` or `hermitian(A)`, `positive_definite(A)`, `factorable_unit_law(A)`, backend kernel availability | `lower_triangular(L)`, `positive_diagonal(L)`, `A = L * L^T` (or Hermitian transpose), output `entry_unit_law(L)` | Missing symmetry, definiteness, or factorability is reported as an unmet obligation. PSD alone does not authorize ordinary Cholesky; pivoted or low-rank factorizations are distinct primitives / policies. |
| `lu(A)` | `rank(A)=2`, `square(A)`, `invertible(A)` or a route to pivoting facts | `(L, U, P)` with `P*A = L*U`, `lower_triangular(L)`, `upper_triangular(U)`, `permutation(P)` | If invertibility or pivot route is unknown, report the missing fact. |
| `qr(A)` | `rank(A)=2`, numeric entries, and a scaling policy when heterogeneous units make orthogonality unit-dependent | `orthogonal(Q)`, `upper_triangular(R)`, `A = Q*R`, rank report where rank-revealing QR is selected | Missing scaling / rank facts are obligations; no automatic nondimensionalization. |
| `svd(A)` | `rank(A)=2`, numeric entries, and a scaling policy when heterogeneous units make singular values unit-dependent | `orthogonal(U)`, `diagonal(S)`, `nonnegative_entries(diag(S))`, `orthogonal(V)`, singular-value / rank facts when classifiable | Missing scaling policy blocks interpretation rather than silently producing meaningless units. |
| `eigen(A)` | `square(A)`; `symmetric(A)` / `hermitian(A)` for the real-symmetric route; `supports_complex_linalg` for the general complex route | eigenvalue / eigenvector facts, `spectral_radius_bound(A)` or `eigenvalue_bounds(A)` when classifiable | General non-symmetric eigen requires Complex semantics (§26.4) and backend capability facts. |
| `solve(A, b)` | `rank(A)=2`, `compatible_axes(A, b)`, and `solvable(A, b)`; specialized routes consume `lower_triangular`, `upper_triangular`, `positive_definite`, `full_rank`, or rank facts | solution axes / units, residual report, `condition_of(solve_block)` facts | Under/overdetermined or ill-conditioned blocks become explicit obligations / diagnostics. Solver orientation is a lowering choice, not source semantics. |
| `solve_triangular(A, b)` | `lower_triangular(A)` or `upper_triangular(A)`, compatible axes, nonzero diagonal / solvability facts | solution axes / units and direct triangular-solve provenance | Unknown triangularity or diagonal solvability reports an unmet obligation. `solve` may dispatch here only when facts prove eligibility. |
| `least_squares(A, b)` | rectangular or rank-deficient system facts, compatible axes, scaling policy | solution / residual facts, rank / conditioning diagnostics | Missing scaling, rank, or compatibility facts are obligations. |
| `inverse(A)` | `square(A)`, `invertible(A)`, and materialization authorization when the inverse is needed as a value | inverse identities, inverse `entry_unit_law`, condition facts | `inverse(A) * b` may rewrite to `solve(A,b)` because semantics are preserved. Materializing an inverse without required facts is an unmet obligation. |
| `det(A)` | `square(A)` and determinant-capable scalar / unit facts | determinant unit law, triangular product simplification when `triangular(A)` is established | Missing square or unit facts are reported. |
| `trace(A)` | `square(A)` and diagonal-entry unit comparability | trace unit law plus diagonal / block-diagonal simplifications | Missing square or unit-comparability facts are reported. |
| `transpose(A)` | `rank(A)=2` | swapped axes, transposed `entry_unit_law`, flipped upper/lower triangular facts, preserved applicable facts (§3.9) | Rank mismatch is a compile error or obligation depending on shape phase. |
| `adjoint(A)` | `rank(A)=2`; real route where adjoint reduces to transpose, or Complex semantics / backend support where entries are complex | conjugate-transpose facts; Hermitian route facts when applicable | Missing Complex/backend support is a capability obligation. |
| `norm(expr, kind)` | supported kind (`"1"`, `"2"`, `"fro"`, `"inf"`), unit / scaling policy where needed | norm envelope facts used by `condition_of` and approximation accounting | Heterogeneous units without scaling policy block interpretation. |
| `condition_of(expr)` | expression shape, unit / axis comparability, and norm / scaling policy for matrix-valued expressions | `ConditionRecord` entries, `condition_mode`, `condition_bound` / `condition_summary` when available | Heterogeneous units without a scaling policy make condition interpretation unknown; the diagnostic asks for scaling evidence. |
| `matrix_rank(A)` | `rank(A)=2`, numeric entries, tolerance / scaling policy | `rank_value(A)`, full-rank / nullspace facts when classifiable | Missing tolerance / scaling policy reports an obligation. |
| `kernel_matrix(k, xs, ys)` | kernel-domain compatibility for `k: A,B -> Scalar<U>`, finite axes for `xs: A` and `ys: B`, output-unit law | `kernel_matrix_of(W,k,xs,ys)`, row/col axes, entry-unit law, pairwise-evaluation provenance, zero-pattern facts when `zero_when` proves finite pairs are exact zeros | Does not emit symmetry, PSD, or covariance facts merely because it is kernel-shaped. Missing domain/axis/unit facts are obligations. |
| `gram(k, points)` | same-domain kernel compatibility for `k: A,A -> Scalar<U>` and finite point axis; downstream covariance use requires the relevant kernel facts (`SymmetricKernel<A>`, `PositiveDefinite<A>`, `StrictPositiveDefinite<A>` plus `distinct(points)` for PD) | `gram_of(K,k,points)`, row/col axes, entry-unit law, pairwise-evaluation provenance, `symmetric(K)`, `positive_semidefinite(K)`, `positive_definite(K)`, and zero-pattern facts only when proven | PSD alone does not authorize ordinary Cholesky. If PD is required and unknown, report `positive_definite(K)` as unmet; the compiler does not silently add jitter, pivot, or opaque-handoff. |
| `zeros<U>(shape)` | structural shape expression and unit parameter | zero tensor, zero-pattern facts | Shape expressions outside the solved subset become obligations. |
| `ones(shape)` | structural shape expression | dimensionless all-ones tensor | Shape expressions outside the solved subset become obligations. |
| `identity(n)` | structural square dimension | dimensionless identity matrix; diagonal, orthogonal, positive-definite facts | Unknown dimension phase follows §3.8 shape-phase rules. |
| `diag(v)` | vector input | diagonal matrix with diagonal entries from `v` | Non-vector input is rejected. |
| `diag_of(A)` | matrix input | vector of diagonal entries | Non-matrix input is rejected. |
| `stack` / `hstack` / `vstack` | shape constraints from §3.8 | derived shape, axis, and unit facts | Shape incompatibility is an unmet obligation. |
| spatial operator lowering | geometry/domain facts, discretization facts, boundary/locus facts | `stencil_pattern`, `local_coupling`, `discretization_order`, `mesh_dependent`, and conservation facts such as `row_sum_zero` or `graph_laplacian` when proven | Missing conservation / stability facts are visible in inspection; the compiler does not claim preserved physics it cannot establish. |

For real-valued matrices, `adjoint(A)` rewrites to `transpose(A)`. For
complex-valued matrices, `adjoint(A)` is the conjugate transpose:

```text
adjoint(A) = conj(transpose(A))
hermitian(A) means A = adjoint(A)
unitary(A) means adjoint(A) * A = identity(n)
```

`transpose(A)` never conjugates. Complex linear algebra consumes the
same matrix fact lattice as real linear algebra, plus the Complex
numeric semantics in §26.4 and backend capability facts from §31.

`Matrix<U, m, n>` is the canonical base constructor. Full structural
property names such as `PositiveDefinite`, `Symmetric`,
`LowerTriangular`, and `Orthogonal` are the normative names in prose;
short aliases, if provided by the stdlib, desugar to those full
refinement names. Forms such as `Matrix<_, PositiveDefinite>` are not
canonical.

Finite matrix assembly is source-level construction from existing
graph values:

```myco
a: Scalar<dimensionless>
b: Scalar<dimensionless>
c: Scalar<dimensionless>
d: Scalar<dimensionless>

A = matrix[[a, b]; [c, d]]
```

This form emits `shape(A) = (2, 2)`, row / column axis facts, entry
provenance facts (`A[0,0] = a`, etc.), and a homogeneous
`Matrix<U, 2, 2>` type when entries establish a shared unit `U`.
Heterogeneous-unit assemblies are ordinary matrices with explicit
`entry_unit_law` facts; downstream primitives consume those facts as
usual (§3.9). Rows must have equal length. The assembly form does
not introduce numeric values by itself, and CC1 applies recursively
to every entry expression (§4).

This is distinct from a provider slot:

```myco
A: Matrix<dimensionless, 2, 2>
```

The declaration above says that the graph contains a matrix-valued
node with fixed unit and shape; the workflow must bind, infer,
observe, train, or otherwise provide it according to context.
Concrete numeric matrix data enters through workflow providers, not
`.myco` source.

Each primitive carries a capability contract that records what facts
it requires and what facts its result satisfies (see §3.9). The
primitive body may be opaque at the e-graph layer because it wraps
backend linear-algebra kernels (BLAS / LAPACK / cuBLAS equivalents
via the Part V backend trait), but its contract is not opaque: inputs,
outputs, emitted facts, and unmet obligations remain inspectable.

---

## Part V — Backend Abstraction

**Summary.** Part V specifies the abstraction by which Myco compiles
plans against a trait surface rather than a specific runtime. The
locked design is a small `CoreBackend` plus advertised capability
profiles; hybrid Myco-owned / backend-owned AD; explicit capability
mismatch policy; whole-SCC Tier C PPL handoff; opaque-callable runtime
semantics; realization-provider execution hooks; backend trait
versioning; no primary backend; and a semantics-complete CPU reference
backend as the first conformance target.

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

The backend surface is a small mandatory core plus advertised
capabilities. A backend must satisfy `CoreBackend`: run identity /
version reporting, capability inspection, diagnostic emission,
deterministic seed handling, dense tensor allocation / handles,
elementwise arithmetic, broadcast, reductions, reshape / view /
transpose operations, dense matrix multiplication, and ordinary
scalar math. This is enough to run a basic deterministic numerical
plan and to report why a richer plan cannot bind.

Scientific machinery beyond that core is capability-advertised, not
mandatory. Cholesky, SVD, eigendecomposition, sparse kernels,
iterative solvers, runtime AD modes, PPL inference modes,
dynamic-keyed axes, complex arithmetic, complex linear algebra,
complex runtime AD, custom realization-provider execution, host
interop, and opaque-callable gradients are all backend capabilities. A
backend may
also advertise a named **capability profile**, a composable bundle of
capabilities with `requires`, `provides`, and `implies` rules. This
is an implementation-surface vocabulary, not a `.myco` source
contract or supercontract.

Example profiles:

```text
CapabilityProfile LinearAlgebraBasic
  requires CoreBackend
  provides solve, solve_triangular

CapabilityProfile LinearAlgebraDecompositions
  requires LinearAlgebraBasic
  provides cholesky, qr, svd, eigen

CapabilityProfile PPLHMC
  requires CoreBackend, RuntimeADReverse
  provides hamiltonian_monte_carlo, mcmc_diagnostics

CapabilityProfile OpaqueCallableAD
  requires CoreBackend, opaque_callable_runtime, RuntimeADReverse
  provides opaque_callable_ad, controller_gradients
```

The compiler lowers each plan or SCC to a set of required backend
capabilities. The selected backend either satisfies those
requirements directly, the workflow explicitly authorizes a
capability-mismatch policy (§31.1), or plan binding fails with a
capability diagnostic. Optionality is represented as advertised
evidence, not as `Option` / `Result`-returning backend methods.

**AD ownership boundary.** Myco owns symbolic and algorithmic
differentiation: rewrite-based derivatives, structural chain-rule
expansion, Jacobian construction, and derivative-related provenance.
Backends own runtime AD over emitted kernels and opaque callables,
advertised through capability flags. This hybrid boundary is
normative: runtime AD is delegated, but the compiler remains
responsible for the mathematical derivative structure it can see.
Runtime AD results are execution values and provenance, not new
symbolic facts. They may satisfy training or inference execution
needs, but they do not certify derivative identities, envelope
propagation, conditioning facts, or rewrite eligibility unless the
compiler can derive the same structure from visible terms or the
backend advertises an audited capability that explicitly certifies
the relevant derivative fact.

#### 31.1 Capability Advertising and Fallback Modes

**Summary.** Backends advertise capability facts and capability
profiles (complex support, forward AD, HMC, sparse matmul, dynamic
topology modes, device / compiler-mode support, hidden-fallback
detection, etc.). The compiler verifies required capabilities at
plan-binding time. Three capability-mismatch modes handle missing
support: `error` (fail), `host` (route to host-side reference),
`emulate` (substitute approximate algorithm and enter
approximation-error layer). Fallback is per-run via
`run.config.backend`, and the resolved backend/device plan is captured
in the run lock.

Backends advertise capabilities (e.g. `supports_complex`,
`supports_complex_linalg`, `supports_complex_ad`,
`supports_forward_ad`, `supports_reverse_ad`,
`supports_hamiltonian_monte_carlo`, `supports_sparse_matmul`,
`supports_cholesky`, `supports_svd`,
`supports_runtime_bounded_topology`,
`supports_autograd_through_masked_topology`,
`supports_event_replan`, `supports_event_replan_cache`,
`supports_dynamic_keyed_axes`, `supports_dynamic_sparse_adjacency`,
`supports_ragged_axes`, `supports_dynamic_output_shapes`,
`supports_matrix_free_action`, `supports_weak_form_assembly`,
`supports_unstructured_mesh_ops`, `supports_staggered_field_placement`,
`supports_conservative_remap`, `supports_custom_realization_provider`,
`supports_rust_realization_provider`,
`supports_python_realization_provider`,
`supports_host_execution`, `supports_hidden_fallback_detection`,
`opaque_callable_runtime`, `opaque_callable_ad`) and capability
profiles such as
`LinearAlgebraBasic`, `LinearAlgebraDecompositions`,
`RuntimeADReverse`, `DynamicTopology`, `OpaqueCallableAD`, or
`PPLHMC`. Profiles expand through their `requires` / `provides` /
`implies` rules into concrete capability requirements. The compiler
verifies the resulting requirement set at plan-binding time. When a
required capability is missing, the compiler consults the workflow's
fallback policy:

- **`error`.** Fail at plan-binding time with a capability-mismatch
  diagnostic (workflow-composition error tier, §19.4). Conservative
  default. `host` and `emulate` never happen silently.
- **`host`.** Route the offending subgraph to a host-side reference
  implementation. Correctness preserved at the cost of device-host
  traffic. Useful for CPU-only families (e.g. `Rational` arithmetic,
  §26).
- **`emulate`.** Substitute an approximate or slower algorithm that
  the backend does support (e.g. dense solve in place of a missing
  sparse solve, finite-difference AD in place of missing forward AD).
  The substitution enters the approximation-error layer (§16 adjacent
  keyed state) so its effect on guarantees is tracked.

Backend family alone is not a capability set. The advertised facts are
specific to the resolved backend family, backend version, compiler mode
(eager, compiled, AOT/exported, etc.), device kind / hardware class
(CPU, CUDA GPU, MPS GPU, TPU, etc.), and any backend feature flags.
For example, a tensor runtime may advertise `DynamicKeyed` on CPU
eager execution while rejecting it for a compiled accelerator target;
that is one backend family with two different capability profiles.

Topology-handler selection follows the same rule as every other
capability decision. The `.myco` model declares the world can create,
retire, re-key, or reconnect entities; the workflow declares which
handler classes are acceptable (`CapacityMask`, `EventReplan`,
`DynamicKeyed`, or later handlers); the resolved backend/device
advertises which handlers it can lower directly. The compiler chooses a
valid handler from the allowed set, records the choice in the run lock,
or fails with a capability diagnostic. It never silently changes
topology semantics, grows a capacity, routes to host execution, or
enables a backend's hidden CPU fallback to make a run appear to work.

Fallback mode is set per-run via `run.config.backend` (§24.5);
workflows can also scope fallback to specific capabilities. If no
fallback policy is specified, the mode is `error`. Explicit `host` or
`emulate` routes are execution-plan facts and must appear in
`hypha explain` and the run lock.

#### 31.2 PPL Handoff Protocol

**Summary.** Tier C handoff is whole-stochastic-SCC handoff after
Tier A exact rewrites and authorized Tier B approximations have run
to exhaustion. The compiler serializes each unresolved stochastic
SCC as an inference task: latent nodes, observations, deterministic
terms, supports / bounds, capability requirements, log-density
recipe, and requested inference kind. The backend returns samples,
traces, and diagnostics; returned samples are opaque draws with
provenance, not new parametric envelope facts.

Tier C stochastic SCCs (§13.2) ship to the backend's PPL handler as
opaque log-density problems. The handoff is a protocol, not a
library call. The compiler owns task construction and serialization;
the backend owns inference execution.

The backend receives one task per unresolved stochastic SCC, not
one task per factor. The task contains the latent nodes, observed
nodes / data, visible deterministic terms, support and refinement
bounds, capability requirements, a log-density assembly recipe, and
the requested inference kind (`hmc`, `nuts`, `vi`, `importance`,
etc.). Whole-SCC handoff lets the backend see posterior geometry,
shared latents, observation structure, and constraints that would be
lost under per-factor handoff.

The backend returns an `InferenceResult`: posterior draws or samples,
optional log-density evaluations, traces / chains, diagnostics
(acceptance statistics, effective sample size, R-hat, divergences,
backend warnings), and task provenance. Samples come back without
envelope facts about the parametric form (§13 recommits this);
downstream code treats them as opaque draws or empirical summaries,
not as newly proven distribution families.

#### 31.3 Opaque-Callable Runtime

**Summary.** The backend supplies the runtime that calls back into
Python during simulation for `Controller` sources, threads
gradients through Python for training emission, and manages memory
and device-residency for interop. The compiler sees only the
callable's advertised input and output contract, not its interior.
Trainable callables require compatible backend opaque-callable AD;
fixed opaque callables do not.

`bind(path, Controller(...))` (§24.2) hands the compiler a Python
callable (a learned function, typically a neural network). The backend provides
the runtime that calls back into Python-land during simulation,
threads gradients back through Python for training emission (§25),
and manages any memory / device-residency needed for the interop.
The opaque-callable runtime sits at the backend ↔ workflow boundary;
the compiler does not see the callable's interior, only its advertised
input / output contract.

Opaque callables execute in the selected run backend context by
default (§32.1). A callable may execute as a fixed value-producing
source when it satisfies its input / output contracts and the backend
advertises `opaque_callable_runtime`. A callable may participate in
training gradients only when the workflow marks it trainable, the
callable contract advertises the necessary differentiability, and
the selected backend advertises `opaque_callable_ad` plus a compatible
runtime AD profile. A callable trained under one backend is portable
to another only when serialization, tensor layout, callable runtime,
and AD capabilities are compatible (§23.3).

If a non-differentiable or backend-incompatible callable lies on a
required training-gradient path, workflow composition errors by
default. Gradient-stop behavior is legal only when explicitly marked
by the workflow; it is never inferred silently.

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

**Summary.** Tier C handoff serializes stochastic SCC tasks across
the trait boundary. The wire format includes SCC identity, stochastic
e-class identities, envelope parametric metadata, layer-1 terms,
deterministic dependency terms, capability requirements, support /
refinement constraints, observation constraints, and requested
inference kind. The compiler owns serialization; backends own
deserialization and backend-specific canonicalization post-receipt.

Stochastic e-classes (§13 distributional metadata in the envelope)
need to cross the trait boundary when Tier C SCCs hand off to the
backend's PPL. The serialization contains the SCC identity; e-class
identities; parametric forms recorded in envelope metadata (family,
parameters, shape); current layer-1 equational-core terms; visible
deterministic dependency terms; capability requirements; support /
refinement constraints; observation constraints (§13.9); and the
requested inference kind. This is the wire format the PPL handoff
protocol (§31.2) uses. The compiler owns the serialization; backends
own deserialization and any backend-specific canonicalization after
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

### 32. Backend Follow-On Items

**Summary.** Backend semantics are locked. Follow-on implementation
items remain: exact PPL message schema, inference-kind enumeration,
workflow spelling for explicit gradient stops and capability-scoped
fallback, future mixed-backend execution, and implementation-facing
trait method signatures. AD ownership, capability-profile shape,
PPL handoff, opaque-callable gradient semantics, versioning, no
primary-backend policy, and the first conformance backend target are
no longer open.

The backend trait shape is intentionally lean: `CoreBackend` is the
mandatory substrate, and richer execution surfaces are advertised via
capabilities and capability profiles (§31). Exact method signatures
remain to be spelled out in the implementation-facing trait
definition.

#### 32.1 Mixed-Backend Policy

**Summary.** A single run targets one backend in the current scope.
Future SCC-level mixed-backend execution remains open. If a workflow
needs specialized handling for one SCC today, the intended escape
hatch is workflow-layer glue rather than compiler-managed
cross-backend marshalling.

Single-backend-per-run keeps capability negotiation, device
residency, and reproducibility understandable. A workflow that needs
a specialized PPL backend for one SCC can run that SCC in isolation
and pass its samples / outputs into the main run through workflow
sources. Compiler-managed SCC-level backend handoff is a future
backend item, not a current guarantee.

#### 32.2 First Concrete Backend

**Summary.** The first implementation target is a semantics-complete
CPU reference backend: Python-hosted in the workflow layer,
CPU-executed, vectorized through NumPy / SciPy where that preserves
semantics, and explicit about slower reference paths where it cannot.
This is a debugging and conformance target, not a primary language
backend. Capability-rich JAX-, PyTorch-, Burn-, GPU-, Rust CPU-, and
PPL-oriented backends remain peer implementations of the same trait.

The first concrete backend target is a semantics-complete CPU
reference implementation. It should prioritize correctness,
inspectability, deterministic diagnostics, dynamic-keyed execution,
provider validation, and broad coverage of source semantics over
accelerator performance. In the Python workflow layer it should use
NumPy / SciPy for ordinary dense numerical execution when those
kernels preserve Myco semantics, with explicit host-side reference
routines for features outside NumPy / SciPy's shape. "Reference"
means conformance-first, not intentionally slow scalar Python.

This decision does not privilege CPU / NumPy as the language's
primary backend (§31.6). It is the conformance target: the backend
that helps implementers and users see whether Myco semantics are
right before optimizing execution. A Rust CPU backend may be added
later as a performance-oriented implementation of the same trait, not
as a replacement for the reference target. JAX-, PyTorch-, Burn-, GPU-,
and specialized PPL-oriented backends remain first-class trait
implementations selected by workflow configuration and advertised
capabilities.

---

## Part VI — Known Open Items

**Summary.** Part VI enumerates remaining open design / catalog /
tooling items and the resolved blocker ledger carried forward
explicitly so they are not silently recommitted during consolidation.
Open items now cluster around stdlib inventory, package / workflow API
details, controller affordances, exact-numeric portability, selected
geometry polish, Tier 2 / Tier 3 family catalogs, and implementation /
cost-calibration work. The B-tagged blockers, matrix heterogeneous-unit
resolution, backend abstraction, the type-graph / e-graph bridge,
Complex numeric semantics, O4 carryovers, and cost-field cluster are
closed.

Carried forward explicitly so they are not silently committed during
consolidation.

### 33. Design Blockers

**Summary.** The named B-blockers are now resolved. B1 distribution
contract and opaque stochastic family policy, B2 joint declaration
syntax, B4 coupling machinery, B5 matrix heterogeneous-unit
resolution, and B6 backend abstraction all have locked homes in the
canonical spec.

- **B1.** RESOLVED: `Distribution<S>` is sample-type based; visible
  distributions expose relation-shaped `log_density`; `density` /
  `pdf` is default-derived; sampling is backend/runtime capability;
  curated opaque stdlib/backend stochastic families are Tier-C-first
  and fact-poor unless a visible rewrite or audited backend
  capability supplies a specific fact.
- **B2.** RESOLVED: joint declarations use one structured stochastic
  root with named `.at()` projections; record-`~` sugar is allowed
  and desugars to that root. Tuple and positional joint
  destructuring are banned.
- **B4.** RESOLVED: coupling lives as joint-envelope metadata on the
  structured stochastic root. Same-root fields are dependent by
  default unless the joint envelope proves independent partitions or
  a dependency graph; distinct field names alone do not prove
  independence.
- **B5.** RESOLVED: heterogeneous-unit matrix accounting uses
  compiler-facing matrix facts (`row_axes`, `col_axes`,
  `entry_unit_law`, construction provenance, provider validation)
  over ordinary tensors; no `basis` syntax or user-marked matrix
  role types.
- **B6.** RESOLVED: backend abstraction uses a small `CoreBackend`
  plus advertised capabilities and profiles; hybrid AD; explicit
  capability mismatch policies; whole-SCC Tier C PPL handoff;
  opaque-callable runtime semantics; trait versioning; no primary
  backend; and a semantics-complete CPU reference conformance target
  (see Part V).

### 34. Chunk-Slotted Work

**Summary.** Remaining chunk-tracked work is now narrow: chunk 03 has
implementation / backend / cost-calibration follow-through after the
kernel semantics lock, and chunk 11 has enum polish / diagnostics /
lowering details after the core sum-type lock. Chunks 05, 06, 07, 08,
09, 12, and 13 are resolved and kept here as completed references.

- **Chunk 05.** RESOLVED: matrix details. Heterogeneous-unit type
  mechanics are resolved by matrix facts (§3.9); shape expressions,
  envelope views, the structural fact lattice, tensor `convert`
  scope, dynamic topology shape handling, scalar reconciliation, and
  the primitive catalog are locked. Finite matrix assembly is the
  source syntax for assembling matrices from graph values; concrete
  numeric matrix data remains workflow-bound. Execution concerns are
  handled by the resolved Part V backend abstraction.
- **Chunk 06.** RESOLVED: backend abstraction. Myco targets a
  versioned trait surface with `CoreBackend` plus capability profiles;
  workflows select one backend per run; capability mismatches error by
  default unless explicit `host` / `emulate` policy is configured;
  Tier C hands whole unresolved stochastic SCCs to backend PPL
  handlers; opaque callables require explicit runtime and AD
  capabilities; no backend is primary; the first conformance target is
  a semantics-complete CPU reference backend.
- **Chunk 07.** RESOLVED: type-graph ↔ e-graph bridge. The semantic
  model is two substrates with an explicit live guard-discharge
  bridge: the type graph carries static semantic relationships and
  the e-graph carries value equalities. Precompiled / cached guards
  are an optimization only. Refinements are evidence-backed facts,
  generic parameters are invariant by default, conversion legality is
  separate from realization cost, and monotone facts discovered during
  saturation may unlock later guarded rewrites.
- **Chunk 08.** RESOLVED: user-`fn` ban and parameterized-relation
  lock (applied in §6 / §7 / §8 / §28). Kernels are parameterized
  relations, not a separate keyword; reusable user-authored model
  structure adds graph obligations via relations, not expression-
  position functions.
- **Chunk 09.** RESOLVED: workflow data layer. Python is a dumb data
  / orchestration layer, not a model layer; spore authors ship `.myco`
  plus `myco.toml`, not Python mirror packages. The canonical workflow
  address model is catalog-backed `NodePath` / `FacetPath` with
  canonical string serialization; `Selector`s are workflow-only catalog
  queries for bulk binding / querying / diagnostics. Catalog entries
  carry type, unit, axes, contracts, facets, and existence domains so
  complex types, generics, enums, and event-driven dynamic worlds stay
  expressible without Python owning Myco semantics.
- **Chunk 03.** Kernels, resumed after substrate lock. Kernel identity
  is locked: kernels are parameterized relations over two input
  domains and one scalar output, with point-point same-locus kernels
  as a specialization rather than the definition. Kernel facts /
  contracts, finite assembly, Gram obligations, and ordinary
  `integrate` / `sum` kernel operators are locked (§28). Exact
  support, sparse / index lowering semantics, and provider-pattern
  provenance are locked. Low-rank / feature approximation semantics
  and process-prior / GP-HSGP consumer semantics are locked. Concrete
  sparse / low-rank / process-inference backend implementations and
  cost calibration remain implementation / catalog work.
- **Chunk 11.** Sum types / enums. Core surface locked (§3.10):
  `enum`, flat exhaustive `match`, unit / positional / struct-like
  variants, no wildcard/default arm, explicit narrowing before field
  access, static-vs-dynamic discriminant lowering, event-only `becomes`
  transitions, workflow tagged-record binding, and explicit-match-only
  `Prior<S>` in v2.1. Extended pattern sugar, diagnostics, and some
  implementation-level lowering details remain open. Resolves the
  Mode B open in §35 and the number-or-distribution materialization
  question.
- **Chunk 12.** Resolved cost/objective vocabulary. `cost_of(expr)`
  owns planner/extraction economics (§14.2, §19.1);
  `objective_terms(residual)` owns training-objective decomposition
  (§14.2, §25). The former open is recorded in
  `planning/v2/v2.1_chunk_reports/12_cost_field_unification.md`.
- **Chunk 13.** RESOLVED: PPL blocker cluster B1/B2/B4. Distribution
  contract shape, opaque stochastic family policy, record-`~` joint
  sugar, and joint-envelope coupling metadata are locked in §13 and
  §27; detailed rationale lives in
  `planning/v2/v2.1_chunk_reports/13_ppl_blockers.md`.
- **Complex numeric representation.** RESOLVED: `Complex` is an
  ordinary `Scalar<U,T>` numeric representation (§26.4), not a separate
  scalar kind or sub-hierarchy. It satisfies algebraic / conjugation /
  magnitude contracts but not ordering; `Scalar<U, Complex>` has one
  unit `U` shared by real and imaginary components; `phase` returns
  stdlib `Phase` over `Scalar<rad>`.

### 35. Other Opens

**Summary.** Catalog of smaller remaining items: source-level retraction
if ever admitted, exact-numeric GPU portability, vector / tensor seam
transforms, rational-denominator termination beyond the rewrite cap,
controller-interface affordances, stdlib inventory, Tier 2
family-catalog polish, remaining Tier 3 non-parametric machinery,
package dependencies, and event-scheduling policy API. Obligation
retraction is resolved by the `ObligationSite` / `fulfills` ledger
(§8.11, §10.5).
Heterogeneous selection is resolved by `Selected<T>` /
`Option<Selected<T>>` selector semantics (§12.2). Event-driven topology
mutation is resolved by versioned topology, explicit topology handlers,
and backend capability negotiation (§3.8, §21.3, §24.5, §31.1).
Residual-to-e-graph projection and per-residual objective identity are
resolved by `ResidualSite` / `ResidualRealization` semantics (§19.2,
§25). Envelope ownership is resolved by the four-writer / four-reader /
no-invalidator model
(§16.3). CC1 diagnostic shape is resolved in §4.1.

General source-level retraction remains out of the core language; if a
future feature admits it, it must preserve the monotonicity invariant
or live entirely in adjacent keyed state. Exact numeric GPU portability
remains a backend-capability issue: current GPU-targeted SCCs reject
`Rational`, arbitrary-precision `Integer`, and `BigFloat` (§26.1,
§26.3, §31.1); future support is advertised capability, not source
semantics. Rational-denominator saturation currently uses the rewrite
cap (§19.4, §26.3); a non-cap-based termination argument remains open.
**Chunk 04 carryovers:**
O4.1 obligation retraction is resolved by the ledger design; W1 is no
longer a rewrite group. O4.3 per-residual training emission is
resolved: residual identities live on `ResidualSite`s while extraction
may share `ResidualRealization`s (§19.2, §25). O4.6 heterogeneous
selection is resolved: selector provenance lives in `SelectedSite`
Layer-3 records; projected fields are ordinary expressions (§12.2).
O4.7 event-driven topology mutation is resolved: events emit
`TopologyDelta`s that create new `TopologyVersion`s at regime
boundaries; execution uses explicit `CapacityMask`, `EventReplan`, or
`DynamicKeyed` handlers selected through workflow intent and backend /
device capability facts (§3.8, §21.3, §24.5, §31.1).

O4.8 spatial operator lowering is resolved: source weak / residual
forms and spatial operators stay in source-level semantic graph content
(§11.1, §14.3), while discretization lowers through Layer-3 semantic
site records and realization-provider execution artifacts (§21.3,
§37.1; Appendix C P1). E-graph rewrites may express exact stdlib
identities between semantic forms, but a stencil, FEM / DG action,
finite-volume flux action, remap operator, or matrix-free backend handle
is not an equality with the continuous source expression. Pre-e-graph
numerical codegen is out; pre-e-graph canonicalization may only create
semantic site records and preserve provenance.

Macros (dropped from the current surface; revisit if concrete
boilerplate pain emerges). Softmax / weighted aggregation spelling is
resolved as stdlib value aggregations: `softmax`, `weighted_sum`, and
`weighted_average` (§12.1). They use explicit semantic alignment and
never fabricate `Selected<T>` handles.

Vector / tensor seam transforms remain outside the v2.1 scalar-field
`identify` guarantee (§11.1, Appendix C F): component remapping,
orientation flips, and non-orientable-surface cases need a future
geometry pass.

Y6 blowup-threshold diagnostics are resolved by workflow-budgeted exact
enumeration (§8.7, Appendix C Y6). There is no magic language-wide
threshold. Exact Y6 computes raw and reduced subsystem counts, applies
only certified graph reductions, then obeys the active workflow budget.
Guided subsystem search is workflow-authorized extraction approximation,
not exact Y6, and records a Layer-4 approximation term on the relevant
`ResidualSite`.

**Controller-interface affordances in the Python layer.** General-
system question: what hooks does Myco need to expose so workflows
can cleanly implement patterns like taxonomic embeddings, context
injection, per-category modulation, FiLM-style conditioning? Not
FiLM specifically; the meta-question of which controller-binding
surfaces belong in the language / stdlib vs which are workflow
idioms the user builds on their own. The goal is to avoid baking
project-specific patterns into the language while still exposing
enough machinery that workflow authors can implement them cleanly
against `Controller` sources (§24.2).

**Tier 2 family-catalog polish.** The core mechanics are locked:
`Distribution<S>` over sample types, visible `log_density`, curated
opaque family policy, structured joint roots, record-`~` sugar,
named field projections, and joint-envelope coupling facts. Remaining
Tier 2 work is catalog-level: which copula, Wishart / InverseWishart
/ LKJ, and related joint families ship immediately; their capability
tables; and their exact matrix-fact obligations for SPD matrices,
determinants, traces, and factorable unit laws. The multivariate
subset that admits factorization or closed-form reparameterization
(MVN, Dirichlet, Multinomial) already ships in Tier 1 so this open
does not block the common cases.

**Tier 3 distribution machinery.** GP-style process priors are scoped
in §28.8: they bind through workflow `ProcessPrior<I,V>`, produce
demand-driven finite projections, consume kernel / covariance facts,
and dispatch through Tier A/B/C. Remaining Tier 3 work is catalog and
backend/PPL machinery for other non-parametric families (Dirichlet
Process, Chinese Restaurant Process, Pitman-Yor, Indian Buffet
Process, Beta Process), plus concrete closed-form / approximate /
opaque handlers for process laws that do not fit the GP finite-joint
path.

**Cost/objective vocabulary resolved.** Chunk 12 is no longer an open
design item. `cost_of(expr)` owns extraction economics with `compute`,
`memory`, `approximation`, structured `condition`, `truncation`, and
`discretization` fields (§14.2, §19.1). `objective_terms(residual)`
owns training-objective decomposition (§14.2, §25). Peak allocation is
therefore a first-class `memory` field of `cost_of`, not a separate
surface.

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

**Envelope-narrowing promotion resolved.** A baseline default-off
rewrite whose declared `error_bound` is proven zero by the local
envelope is marked `promoted_exact_in_context` (§15.3, §17.6). It may
fire without an explicit `approximate` block for that site, contributes
zero to `cost_of().approximation`, and stays visible in the rewrite /
promotion trace instead of moving into the global default-on bucket.
This preserves the bookkeeping trail for diagnostics while keeping
faithfulness accounting exact (§19.1, §22).

**Approximation cost composition resolved.** Multiple authorized
approximation terms compose conservatively by default (§15.6). The
compiler never assumes cancellation or independence. Stdlib / compiler
rules or evidence-graded provider facts may sharpen the composition
(e.g., Lipschitz propagation, dominance, proven independent RMS, or
family-specific closed form). Otherwise comparable terms use
`conservative_sum`; incomparable tolerance views remain structured as
`uncomposed_approximation_terms` and must be surfaced by `hypha explain`
(§19.1, §22).

**Condition cost representation resolved.** `cost_of().condition` is a
structured `ConditionRecord`, not a scalar (§14.1, §14.2). It carries
entrywise, norm, spectral, and structural condition entries plus an
optional provenance-backed `summary` for extraction ranking. Scalar
operations may populate a one-component entrywise record and summary,
but matrix solves, eigensystems, tensor operators, PDE residuals, and
other multi-output operations keep their view-specific conditioning
visible. `hypha explain` reports the entries, levels, provenance,
missing scaling / structural obligations, and summary rule (§19.1,
§22).

**Stdlib canonical inventory.** The set of stdlib expression atoms and
stdlib-shipped parameterized relations is referenced throughout the
spec but not enumerated in one place. Deferred to a dedicated chunk
that locks: the full list of axiomatic primitives (`exp`, `log`,
`sin`, `cos`, `sqrt`, arithmetic, `smooth_max`, etc.), the
classification of each surface (expression atom vs parameterized relation), the
capability contracts and abstract cost tags for each, and the
classification of distributions (`log_density` / backend sampling
capability) and kernels.
Cross-refs §6, §7, §13.8, §14, §28, §30.

**Mode B resolved: per-instance heterogeneous contract binding.** Chunk 08
pins three modes for pluggable behavior: Mode A (concrete type),
Mode B (contract-typed field, heterogeneous across instances of a
population), Mode C (generic type parameter, homogeneous within a
type instantiation). Mode B is only usable if `.myco` has a
mechanism for declaring that different instances of the same
population can carry different contract implementations, since the
Python dumb-data layer cannot drive per-instance type dispatch
(chunk 09 principle). Resolution path: chunk 11 (sum types / enums,
§3.10) introduces tagged unions as the core mechanism; a contract-
typed variant field inside an enum lets a population carry mixed VC
families or any other contract-bound heterogeneity, with the compiler
picking compile-time specialization when the discriminant is static
and a runtime discriminant-tagged kernel when per-instance. Core enum
syntax, exhaustive flat matching, event-only `becomes` transitions,
and workflow tagged-record binding are locked. Lifted `Prior<S>` sugar
does not ship in v2.1. Remaining chunk 11 items are extended pattern
sugar, diagnostics, and implementation-level lowering details.
Cross-refs chunk 08 (three modes), chunk 09 (dumb-data Python),
chunk 11 (sum types), §3.10 (enums), §7 (contracts), §12
(collections / populations).

**Package dependency story.** Vocabulary is locked (`spore` for
packages, `hypha` for the CLI, `myco.toml` manifest, `myco.lock`
lockfile) and the overall shape follows Cargo + uv conventions
(chunk 10). Resolver algorithm, version semantics (what counts as
a breaking change for a parameterized relation, a contract, or a
capability shift), feature model, build-script / codegen surface,
workspace ↔ Python interaction, cross-spore export policy, registry
story, and platform / backend metadata in the manifest are all open.
None of this blocks the core language lock; full spec-level prose is
deferred post-v2.1 per chunk 10. Cross-refs §2, §36, §37.

**Event scheduling-policy Python API signature.** §10.1 commits to
the contract (a Python-side policy orders competing firings; three
stdlib policies ship: priority, random-with-seed, FIFO) but defers
the exact Python API signature to §24 (workflow source model) since it is
a workflow-layer concern. Open: the canonical signature for custom
policies (e.g., `policy(pending_firings, state) -> List[Firing]`
vs. a class-based interface with explicit hook methods), how custom
policies interact with determinism and reproducibility guarantees,
and the exact menu of state the policy sees. Should be resolved
with the workflow API details around §24.

---

## Part VII — Developer Experience

**Summary.** Part VII names developer-experience surfaces outside the
language and compiler proper: CLI, dependency management, editor
tooling, doc generation, agent/LLM integration. Some surfaces are
committed at the vocabulary/API level (`hypha`, `hypha check`,
`hypha explain`, `hypha doc`); Cargo-style workspaces, rustdoc-style
generated documentation, and agent-friendly spore documentation
retrieval are explicit DX goals. Detailed flags, schemas, and editor
behavior remain open.

Outside the language and compiler proper, but on the roadmap. Listed
here so the surfaces remain tied to the language design without
pretending every tool detail is locked.

### 36. Command-Line Interface

**Summary.** `hypha` is the single user-facing CLI. It spans compile,
run, check, fmt, explain, doc generation, and package-management
subcommands. Flag conventions, exit codes, and most output formats
remain open, but `hypha check` and `hypha explain` are committed
surfaces.

`hypha` is the user-facing CLI, analogous to `cargo` or `uv`.
Whether an internal compiler binary exists behind it is an
implementation detail. Committed subcommands:

- `hypha check` catches tier-1 `.myco` compile errors without
  workflow binding or code generation (§23.4).
- `hypha explain` exposes textual plan reports and the
  machine-readable IR (§22).
- `hypha fmt` formats source once the grammar is locked.
- `hypha doc` generates documentation; `hypha doc <spore>` resolves an
  installed, workspace-local, or registry spore and emits / retrieves
  human-readable and agent-friendly documentation (§39, §40).
- Package-management subcommands operate on spores (§37).

### 37. Dependency Management and Package Registry

**Summary.** A distributable Myco package is a spore. Spores use
`myco.toml` manifests and `myco.lock` lockfiles. `hypha` manages
compile/run/check/fmt/explain/doc and package-management subcommands.
The package approach follows the Cargo + uv convention: explicit
manifests, Cargo-style workspaces, reproducible locks, and a registry
story that remains open.

Locked vocabulary:

- **Spore.** A distributable Myco package: source files, manifest,
  docs, tests, and optional generated artifacts.
- **`myco.toml`.** Spore manifest.
- **`myco.lock`.** Reproducibility lockfile.
- **`hypha`.** User-facing CLI for language and package operations.
- **Workspace.** A Cargo-style root that groups multiple local spores
  under shared resolution, lockfile, docs, and tooling commands.

Open package items: resolver semantics, version constraints, feature
model, build scripts, workspace-Python interaction, registry layout,
workspace membership semantics, platform/backend metadata, and
cross-spore export policy. The minimum scope is local path
dependencies, workspace-local spores, manifest parsing, lockfile
writing, and deterministic source resolution.

#### 37.1 Realization Providers

**Summary.** A realization provider is a spore-shipped implementation
that realizes semantic sites such as `SpatialOperatorSite`,
`WeakFormSite`, `ResidualFormSite`, or `TransferSite` as executable
artifacts. Providers are declared in TOML and implemented through
versioned Python or Rust APIs. They may emit validated artifact-level
facts, but they never override `.myco` source semantics or assert
unchecked truth.

Realization providers live in spores, outside ordinary `.myco` source:

```text
ocean99/
  myco.toml
  src/ocean99.myco
  realizations/ocean99.toml
  python/ocean99_realize.py
  rust/ocean99_realize/
  validation/
```

Users install them through the ordinary spore mechanism:

```text
hypha add ocean99
```

Feature-gated provider variants belong to the open package feature
model (§37); this example commits only to ordinary spore installation.

The TOML declaration is the public contract; Python / Rust code is the
implementation:

```toml
[realization_provider.ocean99_cgrid]
api = "myco.realization.v1"
kind = "spatial_operator_provider"

matches.sites = ["SpatialOperatorSite", "WeakFormSite", "ResidualFormSite", "TransferSite"]
matches.operators = ["grad", "diverg", "pressure_solve", "weak_residual"]
matches.geometry = ["SphericalShell", "CurvilinearGrid"]
matches.discretization = ["finite_volume_c_grid"]

requires = [
  "field_placement:cell_face_edge",
  "metric_factors",
  "boundary_conditions",
  "topology_handler:CapacityMask",
  "backend:sparse_matvec"
]

claims = [
  "discrete_operator_of:provider_validated",
  "row_sum_zero:provider_validated",
  "conservative_flux_pair:provider_validated",
  "adjoint_pair:validated_if_enabled"
]

[realization_provider.ocean99_cgrid.impl.python]
entrypoint = "ocean99_realize:CGridProvider"

[realization_provider.ocean99_cgrid.impl.rust]
crate = "ocean99_realize"
symbol = "ocean99_cgrid_provider"
```

The implementation receives semantic site records and workflow /
backend context, not raw source text. Python and Rust APIs share the
same conceptual shape:

```python
class RealizationProvider:
    def candidates(self, site, context): ...
    def realize(self, candidate, context): ...
```

```rust
trait RealizationProvider {
    fn candidates(&self, site: &Site, ctx: &Context) -> Result<Vec<Candidate>>;
    fn realize(&self, candidate: &Candidate, ctx: &Context) -> Result<Artifact>;
}
```

Returned artifacts are recorded as `DiscreteOperatorSite` records keyed
on the continuous semantic site, topology version, provider, backend,
and artifact identity. They may be assembled matrices, stencil bundles,
finite-volume flux actions, FEM / DG weak-form actions, remap
operators, matrix-free callables, or provider-owned handles. Arbitrary
provider code may produce executable artifacts; it may not produce
trusted facts without evidence. Every emitted fact carries an evidence
grade:

- `compiler_proven`
- `stdlib_derived`
- `provider_validated`
- `audited_package_certified`
- `empirical_tested`
- `unknown`

Obligations specify which evidence grades can satisfy them. For
example, a runtime conservation diagnostic may accept
`row_sum_zero:provider_validated`, while an exact symbolic conservation
proof may require `compiler_proven` or `stdlib_derived`. Facts with
`unknown` evidence are visible diagnostics but do not discharge
obligations.

The lockfile / run record captures the concrete realization used:

```toml
[resolved_realization.ocean99_cgrid]
provider = "ocean99"
version = "1.4.2"
api = "myco.realization.v1"
impl = "rust"
source_hash = "..."
build_hash = "..."
selected_candidate = "cgrid_fv_pressure_split"
backend = "torch"
device = "cuda"
facts_emitted = ["row_sum_zero:provider_validated", "stencil_pattern:provider_validated"]
fallbacks = []
validation = ["constant_preserving:pass", "mass_conservation:pass"]
```

The invariant is:

```text
.myco source defines meaning.
realization providers realize semantic sites.
providers may add executable artifacts and evidence-graded facts.
providers may not override source semantics.
```

If a provider drifts from the source model, it should stop matching,
fail validation, or emit fewer / weaker facts. It may still run as an
opaque provider only when the workflow explicitly allows that loss of
compiler-visible guarantees.

### 38. Editor Tooling

**Summary.** Editor-side surfaces: a language server (LSP), VS Code
extension, tree-sitter grammar, formatter, linter, and the full
syntax-highlighting, diagnostics, hover, goto-definition, and
refactoring affordances.

Language server (LSP). VS Code extension. Tree-sitter grammar. Syntax
highlighting, diagnostics, hover, goto-definition, refactoring
affordances. Formatter and linter surfaces are tracked here; their
CLI spellings route through §36.

### 39. Documentation Generation and Website

**Summary.** Docstring conventions, a doc generator for user-defined
types, contracts, events, and universals, and a website layout
covering language reference, tutorials, API docs, and examples.
Generated documentation should feel rustdoc-like: source-linked,
cross-referenced, navigable by item, and stable enough for humans and
agents to consume.

Docstring conventions. `hypha doc` generates documentation for
user-defined types, contracts, events, universals, parameterized
relations, and spores. `hypha doc <spore>` retrieves or builds the
documentation bundle for that spore, including an agent-friendly
structured view intended for LLM/code-agent consumption. Website layout:
language reference, tutorials, API docs, examples. Generated docs may
embed diagrams produced from the §22 machine-readable IR, but renderer
targets remain optional tooling rather than a core spec commitment.

### 40. Agent / LLM Integration

**Summary.** Agent skills for writing, reviewing, and validating
`.myco` models, harness support for running Myco-aware agents, and
conventions (canonical examples, anti-patterns, diagnostic
interpretation) so LLMs can reason about the language correctly.

Agent skills for writing, reviewing, and validating `.myco` models.
Harness support for running Myco-aware agents. Conventions so LLMs can
reason about the language correctly (canonical examples, known
anti-patterns, diagnostic interpretation). `hypha doc <spore>` is the
preferred retrieval path for agent-facing package documentation so
agents consume the same source-linked docs humans inspect.

---

## Appendices

### Appendix A — Reserved Keywords and Syntactic Surface

**Summary.** Appendix A enumerates the reserved keyword surface of
`.myco`: declaration keywords, type-formers, body forms, the
stochastic operator, not-yet-assigned reservations, structural
punctuation, and stdlib-reserved identifiers. Additions to this list
are a breaking change to the parse surface.

The `.myco` surface reserves the following keywords. Reserved keywords
cannot be used as user identifiers and will emit a `hypha` parse error
if encountered in identifier position.

**Declaration keywords.** `type`, `node`, `universal`, `base_unit`,
`unit`, `contract`, `relation`, `constraint`, `event`, `geometry`,
`locus`, `chart`, `topology`, `metric`, `domain`, `convert`,
`identify`, `enum`, `use`.

**Type-former keywords.** `Scalar`, `Tensor`, `Vector`, `Matrix`,
`Collection`, `impl`, `some`, `val`, `where`.

**Body-form keywords.** `let`, `if`, `else`, `for`, `in`, `is`,
`match`, `trace`, `requires`, `fulfills`, `default`, `conserved`,
`approximate`, `initial`, `temporal`, `when`, `becomes`, `as`, `on`,
`field`, `test`.

**Stochastic operator.** `~` (distribution-binding operator;
stochastic relation). SDE families carry integration-convention type
parameters such as `BrownianMotion<Ito>` and
`BrownianMotion<Stratonovich>`; the convention is not a parameter on
`~` itself.

**Reserved but not yet assigned semantics.** `self` (reserved for
refinement-predicate body use and future module-instance use).

**Structural punctuation.** `::` (path separator), `->` / `<->`
(convert-direction arrows), `<=`, `>=`, `<`, `>`, `==`, `!=`,
`=` (relation-equality and binding use site-determined by
context), `|` (currently unassigned, reserved for future
pattern or pipe use).

**Stdlib-reserved identifiers.** The stdlib atom namespace reserves
`exp`, `log`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`,
`sqrt`, `abs`, `real`, `imag`, `conj`, `phase`, `complex`, `sign`,
`floor`, `ceil`, `round`, `min`, `max`, `sum`, `softmax`,
`prod`, `mean`, `std`, `var`, `argmin`, `argmax`,
`option_argmin`, `option_argmax`, `argmin_all`, `argmax_all`,
`same_entity`, `solve`,
`solve_triangular`, `least_squares`, `cholesky`, `lu`, `qr`, `svd`,
`eigen`, `inverse`, `det`, `trace`, `transpose`, `adjoint`, `norm`,
`dot`,
`matrix_rank`, `kernel_matrix`, `gram`, `zeros`, `ones`, `identity`, `diag`,
`diag_of`, `stack`, `hstack`, `vstack`, `deriv`, `integrate`,
`condition_of`, `objective_terms`, `cost_of`, `value_in`, `grad`, `diverg`,
`laplacian`, `curl`, `normal_grad`, `trace_from`, `limit_from`,
`jump`, `average`, `normal`, `normal_traction`, `test_space`, `smooth_max`,
`smooth_abs`, `smooth_step`, `soft_select`, `hard_select`,
`weighted_sum`, `weighted_average`, `condition_weighted`,
`soft_clip`, `hard_clip`,
`sigmoid`, plus the distribution-
family names enumerated in §27. The stdlib universal namespace
reserves `pi`, `e`, and the parametric family `integer<N: val>`
(target of the integer-literal desugar in §4). User-declared
parameterized relations occupy the relation namespace; they do not
shadow stdlib expression atoms.

The full list is normative as of the current lock. Additions are a
breaking change to the parse surface and follow the source-
language stability process (to be designed post-build).

### Appendix B — Grammar / EBNF Summary

**Summary.** Placeholder for the normative EBNF summary of the
`.myco` surface. Lands once the surface is stable enough to commit
to a grammar (production per construct across §2 through §14).

Open. A normative EBNF summary of the `.myco` surface will appear
here once the surface is stable enough to commit to a grammar.
The concrete form is a production per source-language construct
(types, values, units, parameterized relations, contracts,
constraints, events, geometry, stochastic forms, and stdlib expression
atoms), plus the workflow path/surface grammar from §23-§24 where it
touches source-visible names. Placeholder for a later pass.

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
  8.314` once `bind("R", Constant(...))` fires). Per the CC1 literal-numerics
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
- D6. Stdlib-declared name-preserving semantic quantity operations
  preserve wrappers such as `Angle` / `Phase`; arbitrary arithmetic
  does not infer semantic names from units alone.

**E. Stdlib-inverse round-trip elimination.** Requires declared or
registered inverse. LOCKED.

- E1. For declared-bijective `f` with explicit inverse: `f⁻¹(f(x)) → x`,
  `f(f⁻¹(y)) → y` (gated on envelope bounds proving input in `f`'s
  declared domain)
- E2. Built-in inverse pairs: `exp(log(x)) → x` (gated `x > 0`),
  `log(exp(x)) → x` (always)

**F. Geometry-specific strict merge.** Scalar-field seam identification.
LOCKED; vector / tensor seam transforms remain OPEN (§11.1, §35).

- F1. `identify phi=0 <-> phi=2*pi` merges scalar-field e-classes at
  the seam

**G. Transcendental simplifications (gated).** LOCKED.

- G1. `exp(a)*exp(b) → exp(a+b)`, `log(a*b) → log(a)+log(b)` (gated
  `a,b > 0`), `exp(a)^b → exp(a*b)` (Arrhenius canonicalization)
- G2. Trig fundamentals over angle-compatible arguments (`Angle`,
  `Phase`, or `Scalar<rad>`): `sin(0) → 0`, `cos(0) → 1`,
  `tan(0) → 0`; Pythagorean `sin(x)^2 + cos(x)^2 → 1`.
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

**K. Kernel support and truncation.** Exact support consequences and
approximation-gated truncations from §28.5.

- K1. `K(a,b) → 0` when `zero_when(k, predicate(a,b))` is proven,
  including metric-radius cases summarized by `CompactSupport(radius)`.
  This is exact and may emit sparse zero patterns. LOCKED.
- K2. Separable decomposition: `K((x₁,y₁),(x₂,y₂)) → K_x(x₁,x₂) *
  K_y(y₁,y₂)` when exact `separable_kernel` and product-axis /
  product-domain facts are proven. LOCKED (§28.7; exact only).
- K3. Low-rank `K → U·Vᵀ` (truncated SVD, Nyström, random Fourier
  features, spectral / HSGP truncations) requires exact feature
  expansion facts or explicit approximation provenance with scoped
  error / relaxation ledger facts. LOCKED (§28.7; concrete algorithm
  catalogs remain implementation work).
- K4. Infinite-tail truncation `K(a,b) → 0` outside a chosen region
  requires workflow approximation policy or an explicit `.myco`
  `approximate` claim plus tail-bound / error-ledger facts. LOCKED.

**L. Smoothing rewrites.** User-written smooth forms only; `where` is
never silently smoothed (§8.3 runtime `where` lock).

- L1. `smooth_min(a, b, large_sharpness) → min(a, b)` when sharpness
  exceeds tolerance. LOCKED. Reverse direction (`min → smooth_min`)
  forbidden per "no silent smoothing."
- L2. `where p then a else b → a*sigmoid(k*p) + b*(1-sigmoid(k*p))`
  only in user-written smooth form, never auto-fired. OPEN (depends on
  smoothing-surface finalization; §8.3, §8.9).

**M. Series / linearization.** First-order expansions and asymptotic
truncation. LOCKED when authorized by an `approximate` block.

- M1. First-order Taylor `f(x) → f(x₀) + f'(x₀)*(x-x₀)` around declared
  operating point
- M2. Drop higher-order terms when envelope bounds their contribution
  below tolerance

**N. Numerical quadrature substitution.** Every PDE passes through
this. Source semantics and approximation policy are locked: continuous
`integrate` is not silently replaced by finite compute (§14.3, §28.4).
Concrete quadrature / lowering catalogs remain implementation work
(§35 stdlib inventory, chunk 03).

- N1. `integrate(f, var, lo, hi) → quadrature_n(...)` only under
  workflow-selected approximation policy or an explicit `.myco`
  `approximate` claim, with `quadrature_lowering_of` provenance and
  error / relaxation ledger facts unless exactness is proven.

**O. Training-time consistency-objective substitution.** Mode-conditional.
Residual identity is preserved by `ResidualSite` semantics (§19.2);
workflow objective policy decides whether and how the exposed term is
consumed (§25).

- O1. In train mode, overconstrained `lhs = rhs` may expose
  `objective_terms(residual).constraint_violation` proportional to
  `(lhs - rhs)²`. The compiler exposes the term; workflow policy
  supplies weights and aggregation.

**P. Mesh discretization (continuous → discrete).** LOCKED as site /
provider artifact lowering, not as an e-graph rewrite and not as
pre-e-graph numerical codegen (§21.3, §37.1). Mesh resolution `h`,
basis, quadrature, stencil shape, flux form, sparse layout, solver, and
backend kernel are realization choices with provenance and, when
approximate, `cost_of().discretization` / error-ledger accounting.

- P1. A semantic `SpatialOperatorSite`, `WeakFormSite`,
  `ResidualFormSite`, or `TransferSite` may realize as an assembled
  sparse matrix, stencil bundle, FEM / DG weak-form action,
  finite-volume flux action, remap operator, matrix-free callable, or
  provider-owned handle. The returned `DiscreteOperatorSite` is an
  execution artifact with evidence-graded facts such as
  `stencil_pattern`, `row_sum_zero`, `conservative_transfer`, or
  `discretization_order`; it is not a source-level theorem that replaces
  the continuous expression.

**Q. Probabilistic truncation / marginalization.** Interacts with `~`
(§13). LOCKED via CC4 / chunk 04.

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
- S2. `Controller` source callable: `g(inputs) → output` forward
  only (black box, §24.2)

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
- U4. Cross-wrapper arithmetic such as `Angle + Phase` requires an
  explicit `convert` or stdlib-declared operation; same underlying unit
  `rad` is not enough to infer semantic equivalence.

**V. Observation injection.** Ground-truth data pinning (§13.9).
LOCKED.

- V1. `observe(path, data)` attaches observed data as a layer-2
  envelope fact on `path`'s e-class (§13.8, §13.9);
  `log_density(data, logp)` contributes to the training objective
  (§25). Not an equational merge: `path` is not rewritten to `data`
  in layer 1, and the same `path` elsewhere remains stochastic. Data
  is never rewritten by inferred constraints.

**W. Obligation fulfillment.** Ledger selection, not rewrite.
RESOLVED (O4.1 resolved by §8.11, §10.5, §16).

- W1. `relation X on locus fulfills flux_condition(axial_flux): ...`
  satisfies the named obligation. Generated defaults are candidate
  fulfillments; unselected defaults remain inspectable ledger entries
  and are not emitted as facts.

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

- Y1. `weighted_average(candidates) → mean` (closure-policy shorthand
  for arithmetic mean of candidate outputs, semantically equivalent to
  the uniform-weight case of stdlib `weighted_average(values, weights)`).
  LOCKED.
- Y2. `soft_select(preference_list, sharpness) → weighted_sum(candidates,
  softmax(ranks, temperature = sharpness))`. LOCKED. This is a value
  aggregation, not a selected-handle producer.
- Y3. `hard_select(preference_list)` picks highest-ranked by name;
  non-differentiable (rejected in train mode unless discarded paths
  have no learned parameters upstream). LOCKED.
- Y4. `condition_weighted`: uses `condition_of(·)` intrinsic to weight
  candidates by well-conditionedness. LOCKED (un-deferred 2026-04-20,
  closes O4.5).
- Y5. User-defined custom policy: any parameterized relation taking
  candidates plus hyperparameters and writing a forward output slot.
  Extensibility surface. LOCKED.
- Y6. General exact `C(N,M)` enumeration for overconstrained blocks
  (`N > M+1`): planner enumerates all relevant maximal square
  subsystems after certified graph reductions; policy receives the
  exact solution set. Raw count, reduced count, reduction proofs, and
  active workflow budget are diagnostic facts. Guided subsystem search
  is workflow-authorized extraction approximation, not exact Y6.
  LOCKED.

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
  `log_density_Y(y) = log_density_X(f⁻¹(y)) - log |det J_f(f⁻¹(y))|`.
  Produces a `Distribution<Scalar<U_Y>>` envelope fact on `Y`'s e-class
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
| Strict | ~25 (A1-10, B1-2, C1-4, D1-3, E1-2, F1, G1-3, H1-2, I1, K2 exact) | ~5 (D4-5, K1 exact support, X1, X2) | ~30 |
| Distribution-family | ~3 (Z1, Z5, Z10) | ~1 (Z11) | ~4 |
| Fuzzy-model | — | ~2 (L1-2) | 2 |
| Fuzzy-tolerance | ~4 (K3 approximate forms, M1, Q1-2) | ~5 (K3 approximate forms, K4, M2, N1, O1) | ~9 |
| One-way (lossless uni) | — | ~11 (R1-3, S1-2, T1, U1-3, V1, W1) | ~11 |
| N-way extraction | — | ~6 (Y1-6) | 6 |
| Forbidden | 1 (J1 temporal) | — | 1 |

Grand total approximately 61 rules, depending on sub-rule counting
and on how many Z-slots (Z2-Z4, Z6-Z9) the v2.1 conjugate-posterior
enumeration ultimately occupies.

**Cross-cutting items (flags, not rewrites).** CC1, CC2, CC3, CC4, and
CC5 are absorbed into normative spec text: CC1 literal-numerics (§4,
§4.1), CC2 sanity inverses (§5.2 round-trip), CC3 residual-site
identity (§19.2, §25), CC4 stochastic `~` rewrite blank (§13.8), and
CC5 site-gated strict rewrites (§17, Appendix C X). CC5 category and
data path are X1 pole L'Hopital
(removable-singularity operator substitution) and X2 identify
(quotient-induced value equality), site-indexed via Layer-3 adjacent
keyed state with provenance tagging; cross-geometry pollution
impossible by construction.

---
