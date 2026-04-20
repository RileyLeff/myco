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
interface, the standard library, backend abstraction, and the open
items still under design.

---

## 1. Canonical Glossary

The vocabulary used throughout this document. Each term one line.
Terms: `variable`, `relation`, `event`, `controller` (workflow-only),
`initial`, `temporal`, `data contract`, `locus`, `workflow`,
`e-class`, `envelope`, `universal`, `approximate`, `observe`.

---

## Part I — The Language

The surface a modeler writes in `.myco`.

### 2. Types

Primitives (`Scalar<U, T = Float64>`, `Tensor<U, shape>` with `Vector`
and `Matrix` as shape-refined aliases). Named types. Generics: val
generics, type generics, named-argument rule for multi-parameter
generics. Structural refinements on matrices (Symmetric, PosDef,
Diagonal, Triangular, Orthogonal). Heterogeneous-collection keywords
(`impl` for static monomorphization over element types, `some` for
runtime sizing).

### 3. Values and Literal Policy

Zero literal numerics in value position. Three exception positions:
unit definitions, affine conversion bodies, structural positions
(shape tuples, indices, generic-parameter definitions). All numeric
values enter from the workflow. See `spec_dev_notes.md` for the
derivation.

### 4. Units

Base units, derived units, affine conversions, dimensional algebra,
unit-generic types.

### 5. Contracts

Contract declaration. Multi-contract satisfaction (`: A + B + C`).
Supertraits (`contract B : A`). Data contracts (output-only).
Named-type comparison rules.

### 6. Relations and Equality

Relations as world-claims. Overdetermination is not an error; closure
policies combine competing claims. Policies Y1-Y6 including
un-deferred `condition_weighted` (backed by `condition_of`
Levels I-III). Merge semantics.

### 7. State and Time

`initial:` and `temporal:` blocks live in type bodies. Module-scope
only for truly cross-entity relations. `d(x) = expr` for ODE form,
`step(x) = expr` for discrete-update form. `dt` is workflow-provided.
No `[t+1]` subscript surface.

### 8. Dynamic Topology and Events

`event` declarations for topology change. Referential-truth semantics:
things do not know they are dead. Events add facts; no tombstoning, no
retraction. Cross-container events (nearest-common-ancestor rule).
Generic events (cartesian-product expansion).

### 9. Geometry and Locus

Horse/fly composition pattern for spatial frames. `bind_topology` at
workflow time for concrete meshes. `on locus:` clause applies
symmetrically to `relation` and `temporal`.

### 10. Collections and Iteration

`impl Contract` (heterogeneous element type, static monomorphization)
vs `some` (runtime sizing). Iteration patterns. Aggregation lowering.
Narrowing with `where x is T`.

### 11. Probabilistic Programming

`~` as layer-2 distributional metadata, not an equality merge.
Aleatoric/epistemic split. Tier A/B/C routing (exact closed-form /
approximate rewrite / opaque PPL handoff). Independence via structural
identity; no naked correlation. Cholesky reparameterization.

### 12. Compiler Intrinsics

`deriv`, `integrate`, `condition_of` (Levels I symbolic / II algorithmic
/ III runtime), `loss_of`. What each intrinsic means, what the compiler
guarantees about it, how it interacts with the e-graph.

### 13. Approximate Blocks

The 2×2 matrix of approximation flavors: (lossy-model vs
lossy-tolerance) × (univariate vs bivariate). Syntax, semantics,
envelope consequences.

---

## Part II — Compiler Substrate

What the compiler sees and manipulates.

### 14. The E-Graph

The e-graph as the internal equality substrate. Three-layer split:
(1) equational core, (2) envelope metadata attached to e-classes,
(3) adjacent keyed state (timesteps, events, identity-tagged copies).

### 15. Equality-Introducing Machinery

Eight enumerated merge sources: explicit relation equations,
observation injection, algebraic rewrites, `identify`, function
inverses, named-type conversion, closure-policy co-membership,
unit-preserving rewrites. The 2×3 faithfulness × orientation matrix
covering `convert`, `identify`, `approximate`, relation `=`. Unified
rewrite-predicate language.

### 16. Residual Graph (Projection)

The residual graph as a user-facing diagnostic view projected from the
e-graph. Extraction decisions and what they yield. How diagnostics
reference which view.

### 17. Lowering

N-max / alive-mask lowering for dynamic topology. `y[t]` and `y[t+1]`
as distinct ground terms (no per-timestep or template e-graph).
Handoff to the backend.

---

## Part III — Workflow Interface

The boundary between `.myco` and Python.

### 18. The `.myco` ↔ Python Boundary

`.myco` declares structure; Python supplies values and drives
execution. The compiler does not auto-emit projection or solver
selection — those are workflow choices. All numeric values (physical
constants, fit parameters, data series, initial conditions, topology,
observations) cross this boundary.

### 19. Eight Workflow Verbs

`assume_constant`, `assume_series`, `learn_constant`, `learn_initial`,
`learn_trajectory`, `bind_controller`, `bind_topology`, `observe`. For
each verb: what it binds, when it fires, gradient-flow implications.

---

## Part IV — Standard Library

What ships with Myco.

### 20. Numeric Types

`Scalar<U, T = Float64>` with explicit `T` parameter and `Float64`
default. `Rational` for exact constant folding (with termination
caveats). `BigFloat`. Default-compatibility constraints.

### 21. Distribution Families (Z-group)

Tier 1 univariate continuous families (19): Normal, LogNormal, Uniform,
Beta, Gamma, Exponential, ChiSquared, Cauchy, Student-t, Laplace,
HalfNormal, HalfCauchy, InverseGamma, Lévy, Weibull, Pareto, Fréchet,
Gumbel, GEV. Tier 1 discrete: Bernoulli, Categorical, Poisson,
NegBinomial, Hypergeometric. Tier 1 multivariate (gated on B5):
MultivariateNormal, Dirichlet, Multinomial. Meta-families: `Truncated<D>`,
`Mixture<D₁,…,D_N | weights>`. Conjugate-posterior rewrites.
Tier B approximate rewrites: Delta method, Fenton-Wilkinson, CLT,
block-maxima → GEV.

### 22. Kernels

STUB — chunk 03 unified-machinery thread pending e-graph substrate lock.
Kernels are ordinary functions with properties. Standard library covers
common families: Matérn, Gaussian, compact-support splines.

### 23. Units Library

SI base and derived. Biology / ecology extensions. Standard affine
conversions. Derived-unit algebra.

---

## Part V — Backend Abstraction (STUB)

Pending chunk 06 design completion. Specific trait shape and open forks
tracked separately; this part is normative in scope only.

### 24. Backend Trait Surface

Minimum trait API: numerical execution, automatic differentiation, PPL
handoff, opaque-callable runtime. Capability advertising with fallback
policy.

### 25. Open Backend Items

AD ownership fork (Myco-owned / backend-delegate / hybrid — leans
hybrid). Mixed-backend policy (leans single-backend-per-run). PPL
protocol specifics. Versioning. Gradient-flow semantics for
`bind_controller` callables.

---

## Part VI — Known Open Items

Carried forward explicitly so they are not silently committed during
consolidation.

### 26. Design Blockers

- **B1.** Opaque log_pdf stdlib policy.
- **B2.** Joint declaration syntax.
- **B4.** Coupling machinery.
- **B5.** Matrix heterogeneous-unit resolution.
- **B6.** Backend abstraction (see Part V).

### 27. Chunk-Slotted Work

- **Chunk 05** — matrix details (heterogeneous units, envelope flavors,
  subtype lattice, shape refinements, scalar reconciliation).
- **Chunk 06** — backend abstraction.
- **Chunk 07** — type-graph ↔ e-graph bridge.
- **Chunk 08** — B2 + B4 joint syntax / coupling.
- **Chunk 03** — kernels, resume after substrate lock.

### 28. Other Opens

`replaces` obligation retraction (monotonicity tension with the e-graph).
Tier 0 Phase 2 Q3 (residual ↔ e-graph relationship) and Q4 (envelope
ownership). Literal-constants diagnostic surface (CC1 enforcement
messages). GPU-incompatibility of BigFloat and Rational. Conversion-graph
cost model. Backend AD ownership (Part V §25, listed separately for
visibility).

---
