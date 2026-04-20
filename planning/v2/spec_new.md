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

`impl Contract` for static monomorphization: heterogeneous element
types known at compile time, one pool per concrete type (chunk 11
subsection). `some` for runtime sizing: homogeneous element type, size
not statically known. Positively replaces retired `dyn`.

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

### 4. Values and Literal Policy

Zero literal numerics in value position. Three exception positions:
unit definitions, affine conversion bodies, structural positions
(shape tuples, indices, generic-parameter definitions). All numeric
values enter from the workflow. See `spec_dev_notes.md` for the
derivation.

### 5. Units

Base units, derived units, affine conversions, dimensional algebra,
unit-generic types.

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
functions, and distribution families. Data contracts (output-only).
Named-type comparison rules.

### 8. Relations and Equality

Relations as world-claims. Overdetermination is not an error; closure
policies combine competing claims. Policies Y1-Y6 including
un-deferred `condition_weighted` (backed by `condition_of`
Levels I-III). Merge semantics.

### 9. State and Time

`initial:` and `temporal:` blocks live in type bodies. Module-scope
only for truly cross-entity relations. `d(x) = expr` for ODE form,
`step(x) = expr` for discrete-update form. `dt` is workflow-provided.
No `[t+1]` subscript surface.

### 10. Dynamic Topology and Events

`event` declarations for topology change. Referential-truth semantics:
things do not know they are dead. Events add facts; no tombstoning, no
retraction. Cross-container events (nearest-common-ancestor rule).
Generic events (cartesian-product expansion).

### 11. Geometry and Locus

Horse/fly composition pattern for spatial frames. `bind_topology` at
workflow time for concrete meshes. `on locus:` clause applies
symmetrically to `relation` and `temporal`.

### 12. Collections and Iteration

`impl Contract` (heterogeneous element type, static monomorphization)
vs `some` (runtime sizing). Iteration patterns. Aggregation lowering.
Narrowing with `where x is T`.

### 13. Probabilistic Programming

`~` as layer-2 distributional metadata, not an equality merge.
Aleatoric/epistemic split. Tier A/B/C routing (exact closed-form /
approximate rewrite / opaque PPL handoff). Independence via structural
identity; no naked correlation. Cholesky reparameterization.

### 14. Compiler Intrinsics

`deriv`, `integrate`, `condition_of` (Levels I symbolic / II algorithmic
/ III runtime), `loss_of`. What each intrinsic means, what the compiler
guarantees about it, how it interacts with the e-graph.

### 15. Approximate Blocks

The 2×2 matrix of approximation flavors: (lossy-model vs
lossy-tolerance) × (univariate vs bivariate). Syntax, semantics,
envelope consequences.

---

## Part II — Compiler Substrate

What the compiler sees and manipulates.

### 16. The E-Graph

The e-graph as the internal equality substrate. Three-layer split:
(1) equational core, (2) envelope metadata attached to e-classes,
(3) adjacent keyed state (timesteps, events, identity-tagged copies).

### 17. Equality-Introducing Machinery

Eight enumerated merge sources: explicit relation equations,
observation injection, algebraic rewrites, `identify`, stdlib-declared
function inverses (via capability contracts on fns; see §6),
named-type conversion, closure-policy co-membership, unit-preserving
rewrites. The 2×3 faithfulness × orientation matrix covering `convert`,
`identify`, `approximate`, relation `=`. Unified rewrite-predicate
language.

### 18. Residual Graph (Projection)

The residual graph as a user-facing diagnostic view projected from the
e-graph. Extraction decisions and what they yield. How diagnostics
reference which view.

### 19. Lowering

N-max / alive-mask lowering for dynamic topology. `y[t]` and `y[t+1]`
as distinct ground terms (no per-timestep or template e-graph).
Handoff to the backend.

---

## Part III — Workflow Interface

The boundary between `.myco` and Python.

### 20. The `.myco` ↔ Python Boundary

`.myco` declares structure; Python supplies values and drives
execution. The compiler does not auto-emit projection or solver
selection — those are workflow choices. All numeric values (physical
constants, fit parameters, data series, initial conditions, topology,
observations) cross this boundary.

### 21. Eight Workflow Verbs

`assume_constant`, `assume_series`, `learn_constant`, `learn_initial`,
`learn_trajectory`, `bind_controller`, `bind_topology`, `observe`. For
each verb: what it binds, when it fires, gradient-flow implications.

---

## Part IV — Standard Library

What ships with Myco.

### 22. Numeric Types

`Scalar<U, T = Float64>` with explicit `T` parameter and `Float64`
default. `Rational` for exact constant folding (with termination
caveats). `BigFloat`. Default-compatibility constraints.

### 23. Distribution Families (Z-group)

Tier 1 univariate continuous families (19): Normal, LogNormal, Uniform,
Beta, Gamma, Exponential, ChiSquared, Cauchy, Student-t, Laplace,
HalfNormal, HalfCauchy, InverseGamma, Lévy, Weibull, Pareto, Fréchet,
Gumbel, GEV. Tier 1 discrete: Bernoulli, Categorical, Poisson,
NegBinomial, Hypergeometric. Tier 1 multivariate (gated on B5):
MultivariateNormal, Dirichlet, Multinomial. Meta-families: `Truncated<D>`,
`Mixture<D₁,…,D_N | weights>`. Conjugate-posterior rewrites.
Tier B approximate rewrites: Delta method, Fenton-Wilkinson, CLT,
block-maxima → GEV.

### 24. Kernels

STUB — chunk 03 unified-machinery thread pending e-graph substrate lock.
Kernels are ordinary functions with properties. Standard library covers
common families: Matérn, Gaussian, compact-support splines.

### 25. Units Library

SI base and derived. Biology / ecology extensions. Standard affine
conversions. Derived-unit algebra.

---

## Part V — Backend Abstraction (STUB)

Pending chunk 06 design completion. Specific trait shape and open forks
tracked separately; this part is normative in scope only.

### 26. Backend Trait Surface

Minimum trait API: numerical execution, automatic differentiation, PPL
handoff, opaque-callable runtime. Capability advertising with fallback
policy.

### 27. Open Backend Items

AD ownership fork (Myco-owned / backend-delegate / hybrid — leans
hybrid). Mixed-backend policy (leans single-backend-per-run). PPL
protocol specifics. Versioning. Gradient-flow semantics for
`bind_controller` callables.

---

## Part VI — Known Open Items

Carried forward explicitly so they are not silently committed during
consolidation.

### 28. Design Blockers

- **B1.** Opaque log_pdf stdlib policy.
- **B2.** Joint declaration syntax.
- **B4.** Coupling machinery.
- **B5.** Matrix heterogeneous-unit resolution.
- **B6.** Backend abstraction (see Part V).

### 29. Chunk-Slotted Work

- **Chunk 05** — matrix details (heterogeneous units, envelope flavors,
  subtype lattice, shape refinements, scalar reconciliation).
- **Chunk 06** — backend abstraction.
- **Chunk 07** — type-graph ↔ e-graph bridge.
- **Chunk 08** — B2 + B4 joint syntax / coupling.
- **Chunk 03** — kernels, resume after substrate lock.

### 30. Other Opens

`replaces` obligation retraction (monotonicity tension with the e-graph).
Tier 0 Phase 2 Q3 (residual ↔ e-graph relationship) and Q4 (envelope
ownership). Literal-constants diagnostic surface (CC1 enforcement
messages). GPU-incompatibility of BigFloat and Rational. Conversion-graph
cost model. Backend AD ownership (Part V §27, listed separately for
visibility). Macros (dropped from v2.1 surface; revisit if concrete
boilerplate pain emerges).

---

## Part VII — Developer Experience (Deferred)

Outside the language and compiler proper, but on the roadmap. Deferred
until Parts I–IV are locked. Listed here so the surfaces aren't
forgotten during consolidation.

### 31. Command-Line Interface

The `myco` CLI: compile, run, check, fmt, explain, and related
subcommands. Flags, exit codes, output conventions.

### 32. Dependency Management and Package Registry

How `.myco` packages declare dependencies on each other. Version
resolution. Package registry layout and publishing workflow. Lockfile
format. Interaction with the Python workflow layer's package system
(distinct but adjacent).

### 33. Editor Tooling

Language server (LSP). VS Code extension. Tree-sitter grammar. Syntax
highlighting, diagnostics, hover, goto-definition, refactoring
affordances.

### 34. Documentation Generation and Website

Docstring conventions. Doc generator for user-defined types, contracts,
events, universals. Website layout: language reference, tutorials, API
docs, examples.

### 35. Agent / LLM Integration

Agent skills for writing, reviewing, and validating `.myco` models.
Harness support for running Myco-aware agents. Conventions so LLMs can
reason about the language correctly (canonical examples, known
anti-patterns, diagnostic interpretation).

---
