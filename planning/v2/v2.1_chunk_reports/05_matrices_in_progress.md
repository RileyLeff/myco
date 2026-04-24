# Myco v2.1 — Matrix / Tensor Types Design Report (IN PROGRESS)

**Date:** 2026-04-20
**Authors:** Riley Leff, Claude (Opus 4.7)
**Reviewers:** None yet
**Status:** IN PROGRESS. Shape polymorphism direction locked (Option
C — `Tensor<U, shape>` primitive with `Vector<U, n>` / `Matrix<U, m, n>`
as shape-refined aliases). Heterogeneous-unit facts, matrix-envelope
views, structural fact lattice, tensor `convert`, collections
boundary, and dynamic-topology shape handling are locked. Backend /
AD / GPU lowering concerns factored out into chunk 06.

---

## 1. Why this chunk exists

During Z-group stdlib scope for v2.1 (chunk 04 §11 sub-question 4),
it became clear that several foundational pieces of scientific
modeling machinery depend on first-class matrix / tensor types with
linear-algebra primitives that **do not currently exist** in v2.1:

- **Wishart / InverseWishart distributions** — matrix-valued random
  variables, need `cholesky` sampling, `det` / `trace` in log_pdf.
- **Level III `condition_of`** (locked in chunk 04 §11 O2.4) — problem
  conditioning of linear-solve SCC blocks requires matrix operator
  norm / singular-value estimates (e.g. condest-style backend
  estimators, σ_max / σ_min).
- **MVN internal reparameterization (Z10 rewrite, locked in CC4)** —
  `cholesky(Σ)` to reparameterize to independent bases.
- **Gram matrices in kernel methods** (chunk 03) — low-rank kernel
  approximations (K3: SVD, Nyström, random Fourier features) require
  matrix decompositions.
- **Linear-system solves as first-class modeling primitive** —
  hydraulic networks, discretized PDEs, any implicit-function SCC.
- **Jacobian-based inference methods** — assembled Jacobian of a
  residual system is inherently matrix-shaped.

The Z-group resolution in chunk 04 deferred Wishart/InverseWishart
pending this chunk. Level III of `condition_of` shipped on the
assumption that matrix operator-norm machinery lands in v2.1 — this
chunk makes that commitment concrete.

**Riley's framing:** *"strongly feel like we should be able to do
matrices in myco. this is a real missing piece for real actual
modeling and math. ... i would rather go for correctness and
completeness in the spec so we don't end up with loose ends
everywhere."* (2026-04-20)

**Scope discipline.** This chunk is strictly about the type system
and language surface for tensors. Backend routing, AD ownership, GPU
lowering, solver dispatch to cuBLAS / MKL / LAPACK — all factored
into chunk 06 (`06_backend_abstraction_in_progress.md`). Chunk 05
specifies *what a tensor is* as a Myco type and what mathematical
claims can be made about it; chunk 06 specifies *how it executes*.

---

## 2. Current state of matrix / tensor machinery in v2.1

Audit (2026-04-20) confirms: **no first-class matrix or tensor type
exists in v2.1.**

### Adjacent surfaces present

- **Collections** — fixed-size `[Type; N]` and dynamic-size
  `[Type; some]` with iteration / aggregation
  (`02_collections_iteration_report.md:12-76, 132-172, 246-369`).
- **Scalar fields on manifolds** — continuous/discrete domains with
  `grad`, `diverg`, `laplacian`, `curl`, `normal_grad`
  (`01_geometry_design_report.md` §2-3; `open_questions.md:660`).
  These operators return fields, not matrices.
- **Unit propagation for scalars** — `Scalar<U>` machinery in
  `v2.1_in_progress.md:213-277, 624-649`. Unit propagation through
  outer products / tensor contraction is **undesigned**.
- **Autodiff** — `deriv(expr, var)` as first-class operator
  (`open_questions.md:766`); runtime AD via implicit-function theorem
  through SCCs. Matrix derivatives via `deriv()` are undesigned.
- **Scalar distributions stdlib** — `Normal<U>`, `LogNormal<U>`,
  `Uniform<U>`, `StudentT<U>`, `HalfNormal<U>`, `Exponential<U>`,
  `Cauchy<U>`, `Beta`, `Dirichlet`, `Gamma<U>`, discrete families
  (`open_questions.md:741-742`).
- **Refinement-type envelopes** — `Positive<U>`, `Bounded<U>`,
  `constraint` / `where { self >= 0 }` — scalar-focused
  (`v2.1_in_progress.md:245-249`).
- **Generic type parameters** — type and value generics
  (`v2.1_in_progress.md:229-285`) including named generic arguments.
  No type-level predicates on shape currently.
- **`convert` surface** — bidirectional (`convert <->`), one-way
  (`convert ->`), bare (relabel) (`v2.1_in_progress.md:174-225`);
  invertibility verification via O2.1 bounded counterexample search.
- **Named types and conservation** — `v2.1_in_progress.md:100-127`;
  named-type stripping rules U1-U3 in chunk 04 rewrite catalog.

### What is explicitly absent

- **No `Matrix<U, R, C>` / `Tensor<U, Shape>` type constructors.**
  Not in spec, v2.1_in_progress, or any chunk report.
- **No linear-algebra primitives** — `det`, `trace`, `inv`, `cholesky`,
  `solve`, `svd`, `eigen`, or matmul `@` operator.
- **No matrix structural fact / refinement model** for symmetry,
  definiteness, diagonality, triangularity, banding, or orthogonality.
- **No matrix envelopes** — spectral bounds, norm bounds, PSD-as-fact
  all undesigned.
- **No MVN, Wishart, or any multivariate distribution** — explicitly
  deferred (`open_questions.md:853-859`) with the exact blocking
  note: *"Deferred pending vector/matrix/container story lock."*
- **No `condition_of(expr)` intrinsic in current v2.1 source** —
  flagged as deferred (`open_questions.md:180-181`); chunk 04 locked
  it to ship in v2.1 assuming matrix operator-norm machinery lands
  here.
- **No Gram-matrix machinery** — kernels are ordinary functions
  (`03_kernels_in_progress.md` §2); Gram matrices, low-rank
  approximations absent.
- **No structured solver selection** — SCC solver dispatch exists
  conceptually but Cholesky-for-PSD vs LU-for-general is not
  formalized in the language.

### Open questions already in the docs touching this area

- `open_questions.md:180-181` — `condition_of(expr)` intrinsic
  deferred. **(Now locked to ship per chunk 04 O2.4; matrix
  machinery is the precondition.)**
- `open_questions.md:622` — vector/tensor seam transforms deferred
  (geometry report).
- `open_questions.md:766` — `deriv` primitive needs to handle
  matrix/tensor expressions for non-Euclidean spatial operators.
- `open_questions.md:853-859` — MVN deferred pending vector/matrix
  story; typing question is how mean vectors and covariance
  matrices are declared.

### Mock audit

`mock_sperry.myco` uses per-segment scalar state (axial flux,
water potential) threaded through network topology; it does **not**
assemble an explicit stiffness matrix. `mock_potkay.myco` similar.
The mocks predate matrix machinery; no ad-hoc matrix constructs
exist that need to be reconciled. Hydraulic networks today rely on
the SCC solver to implicitly materialize the Jacobian at runtime.

### Audit verdict on scope

Medium-to-large design effort, comparable to the collections or
e-graph-foundation chunks. Not a minor enhancement. Type-system
extension, operator overloading (dispatch on tensor shape and
structural facts), envelope generalization, probabilistic
integration (MVN/Wishart), and e-graph integration (shape
compatibility under merge) all require explicit design.

---

## 3. Design surface — what needs to be decided

### 3.1 Type constructor shape — LOCKED: Option C

**Decision (2026-04-20):** `Tensor<U, shape>` is the primitive;
`Vector<U, n>` and `Matrix<U, m, n>` are **shape-refined aliases**
atop it. Structural facts / refinements (Symmetric, PosDef, etc.)
are further refinements on `Matrix<U, n, n>`.

```
Tensor<U, shape>           # primitive; arbitrary rank
Vector<U, n>   := Tensor<U, (n,)>
Matrix<U, m, n> := Tensor<U, (m, n)>
Scalar<U>      := Tensor<U, ()>   # normative source spelling for rank 0
```

Rank-specific operations are statically scoped on the refinement:
`cholesky`, `solve`, `det` are defined on `Matrix<U, n, n>` only;
`norm(·, "2")` on `Vector<U, n>`; matmul on compatible
`Matrix<U, m, k> * Matrix<U, k, n>`.

**Rationale for C over A (unified Tensor only) or B (distinct
Vector / Matrix / Tensor primitives):**

- Unifies shape / unit / envelope / AD machinery at the primitive
  level (the "consider together" instinct) — one set of rewrite
  rules, one set of envelope propagation laws, one set of
  GPU-lowering entry points.
- Keeps rank-specific linear-algebra statically typed via
  refinements — invalid uses (e.g., `cholesky` on a rank-3 tensor)
  caught at compile time, not at lowering.
- Lets structural facts compose cleanly (`positive_definite(A)`
  entails square, symmetric, PSD, full-rank, and invertible facts).
- Matches the burn / JAX / PyTorch mental model without importing
  their runtime — the type system captures rank statically; the
  backend (chunk 06) handles execution.

**Scalar reconciliation — RESOLVED (2026-04-24).** `Scalar<U>` is
formally sugar for `Tensor<U, ()>`, while remaining the normative
source spelling for ordinary rank-0 quantities.

Users write:

```myco
temp: Scalar<kelvin>
```

The compiler elaborates:

```text
temp: Tensor<kelvin, ()>
shape(temp) = ()
rank(shape(temp)) = 0
```

Diagnostics preserve `Scalar<U>` unless shape reasoning is the point
of the diagnostic. There is no `Scalar <-> Tensor0` conversion edge:
the two are not distinct semantic types. This unifies shape, unit,
envelope, AD, distribution, `convert`, and `approximate` machinery on
one value substrate without making users write rank-0 tensor syntax
in ordinary models.

### 3.2 Heterogeneous-unit question — RESOLVED: matrix facts

Decision (2026-04-23): Myco does **not** add a second tensor-like
type (`LinearMap<From, To>`), a `basis` declaration, or user-marked
matrix role annotations. Heterogeneous-unit matrix meaning is carried
by compiler-facing graph facts over ordinary `Tensor` / `Matrix`
values.

**Background.** A Jacobian from multiphysics residual systems has
per-entry distinct units: `d force / d length`, `d force / d time`,
`d temperature / d length`, etc. A covariance matrix over mixed
observations has entries such as `kelvin * pascal`. A geometry metric
or Gram matrix has its own construction-specific unit law. Units alone
do not identify the mathematical meaning; the relation graph and
construction provenance do.

**Resolution.**

- `Tensor<U, shape>` remains the primitive; `Vector` and `Matrix`
  remain shape-refined aliases.
- Pure field-set contracts provide reusable axis signatures. No new
  `basis` syntax is added.
- The compiler records row / column axes, entry-unit laws, structural
  facts, construction provenance, and provider-validation evidence as
  matrix facts.
- Names such as "covariance", "Jacobian", "metric", "precision", and
  "Gram" are fact bundles / provenance patterns, not privileged source
  roles.
- Operations consume facts. If a required fact is unknown, planning
  reports an unmet obligation. There is no automatic semantic fallback
  or opaque handoff.

Example axis source:

```myco
contract Obs {
    temp: Scalar<kelvin>
    pressure: Scalar<pascal>
}
```

An empirical covariance relation can derive:

```
row_axes(Sigma) = Obs
col_axes(Sigma) = Obs
entry_unit(Sigma[temp, pressure]) = kelvin * pascal
symmetric(Sigma)
positive_semidefinite(Sigma)   // when the construction/proof supports it
```

A solver SCC can derive a Jacobian coefficient matrix with facts such
as:

```
jacobian_of(J, residuals, variables)
entry_unit(J[i,j]) = unit(residuals[i]) / unit(variables[j])
row_axes(J) = residual fields
col_axes(J) = variable fields
```

These facts are compiler / inspection objects. Users normally write
relations, constraints, and workflow bindings; they do not annotate
matrix roles. A constraint such as `positive_definite(A)` creates an
obligation to prove or validate the fact, not a grant of the fact.

This closes the type-signature branch of the heterogeneous-unit
question. Remaining chunk-05 work after the resolved sections below:
matrix literal syntax and final commitment text.

### 3.3 Envelope flavors for matrix-valued quantities — RESOLVED: parallel views

Decision (2026-04-23): matrix envelopes are **multi-view bundles** on
Layer-2 e-class metadata. The standard views are entry-wise, norm,
spectral, and structural. No view is canonical; the compiler does not
coerce one view into another unless a named rule proves the
implication.

- **Entry-wise bounds.** Each entry has a scalar envelope such as
  `A[i,j] in [lo, hi]`. Best for elementwise ops, sign checks,
  provider-validation diagnostics, and local bounds.
- **Norm bounds.** Bounds such as `||A||_2 <= c` or
  `||A - A_approx||_F <= eps`. Best for matmul perturbation,
  solver-error bounds, approximation accounting, and `condition_of`.
- **Spectral bounds.** Eigenvalue / singular-value intervals such as
  `lambda_min(A) >= a`, `lambda_max(A) <= b`,
  `sigma_min(A) >= a`, `spectral_radius(A) <= r`. Needed for
  Cholesky eligibility, covariance validity, stability, and Level III
  `condition_of`.
- **Structural certificates.** Exact facts such as `symmetric(A)`,
  `positive_definite(A)`, `diagonal(A)`, `graph_laplacian(A)`,
  `row_sum_zero(A)`, `block_diagonal(A, blocks)`, and
  `zero_pattern(A)`. Zero-numerical-tolerance view: either the
  property holds or the fact is refuted / unavailable.

Merge behavior is per view: entry-wise intervals join as interval
records; norm bounds remain named and derive tighter bounds only under
known rules; spectral intervals intersect where compatible; structural
certificates union with contradiction checks.

Primitive propagation is explicit. `A + B` consumes entry-wise and
norm views; `A * B` primarily consumes norm views; `cholesky(A)`
consumes structural + spectral facts; spatial lowering emits
structural facts such as `graph_laplacian`, `row_sum_zero`, and
`stencil_pattern`, plus spectral facts when the discretization proves
them.

Guardrail: entry-wise bounds do not automatically prove PSD, PSD does
not imply useful entry-wise bounds, norm bounds do not imply symmetry,
and symmetry does not imply positive definiteness. Cross-view
implications must be named compiler / stdlib rules.

### 3.4 Structural fact lattice — RESOLVED: implication facts, not enum subtypes

Decision (2026-04-23): matrix structure is an **implication lattice
over compiler facts**. Myco does not need a closed enum of matrix
kinds, and it does not treat `PositiveDefinite` / `Diagonal` /
`Orthogonal` as user-granted proof labels. Those names may exist as
stdlib refinement names, inspection vocabulary, or diagnostics, but
they lower to fact obligations and fact entailments.

A matrix e-class carries fact records with four pieces of data:

- predicate and parameters (`positive_definite(A)`,
  `banded(A, width)`, `zero_pattern(A, pattern)`,
  `entry_unit_law(A, law)`);
- domain (`real`, `complex`, square / rectangular shape, axes,
  units, scaling policy, construction preconditions);
- evidence (relation provenance, stdlib primitive contract, e-graph
  rewrite, provider validation, backend report, conditional proof);
- status (`proven`, `refuted`, `conditional`, `obligation`,
  `provider_validated`, `backend_reported`, `unknown`).

The lattice order is implication. Meet combines compatible facts and
normalizes consequences; join keeps only facts common to all incoming
alternatives unless the condition remains explicit. E-class merge
unions evidence, detects contradictions, and never promotes
`unknown` to `proven`.

Committed v2.1 entailments:

| fact or meet | entailed / normalized facts |
|---|---|
| `positive_definite(A)` in the real-matrix setting | `square(A)`, `symmetric(A)`, `positive_semidefinite(A)`, `full_rank(A)`, `invertible(A)`, `lambda_min(A) > 0` |
| `positive_semidefinite(A)` in the real-matrix setting | `square(A)`, `symmetric(A)`, `lambda_min(A) >= 0` |
| `diagonal(A)` | `square(A)`, `upper_triangular(A)`, `lower_triangular(A)`, off-diagonal `zero_pattern(A)`, `symmetric(A)` in the real setting |
| `scalar_diagonal(A)` | `diagonal(A)` and all diagonal entries equal |
| `identity(A)` | `scalar_diagonal(A)`, `positive_definite(A)`, `orthogonal(A)`, inverse identity facts |
| `upper_triangular(A) ∧ lower_triangular(A)` | `diagonal(A)` |
| `orthogonal(A)` | `square(A)`, `full_rank(A)`, `invertible(A)`, `inverse(A) = transpose(A)` |
| `permutation(A)` | `orthogonal(A)`, one-hot row / column pattern, sparse zero-pattern facts |
| `full_rank(A) ∧ square(A)` | `invertible(A)` |
| `transpose(A) * A` | symmetric PSD Gram/provenance facts when axes and units are compatible; PD only with `full_col_rank(A)` |
| `graph_laplacian(A) ∧ conservative_operator(A)` | `row_sum_zero(A)` plus graph/discretization provenance; symmetry / PSD / M-matrix facts require extra construction evidence |

Propagation rules are local and named:

- `transpose(A)` swaps axes and entry-unit laws, flips upper/lower
  triangular facts, and preserves applicable symmetry,
  definiteness, diagonal, orthogonal, and spectral facts.
- `A + B` preserves shared symmetry, diagonal, triangular direction,
  zero-pattern facts, and PSD only by the cone rule when both operands
  are PSD over the same axes and scaling policy.
- `A * B` composes axes and entry-unit laws, preserves same-direction
  triangular products and orthogonal products, and does not preserve
  positive definiteness unless a named congruence / product rule
  applies.
- `inverse(A)` consumes `invertible(A)` and preserves triangular,
  diagonal, orthogonal, and positive-definite facts under named
  inverse rules.
- Factorizations consume facts and emit facts. `cholesky(A)` consumes
  positive definiteness and factorable units, then emits
  `lower_triangular(L)`, `positive_diagonal(L)`, and `A = L * L^T`.

Sparse patterns are facts, not storage declarations. `zero_pattern`,
`banded`, `block_sparse`, `stencil_pattern`, and
`nearest_neighbor_coupling` can be compile-time facts,
provider-validated facts, or runtime-bounded facts depending on their
source. `CSR`, `CSC`, `COO`, and dense materialization are backend
representation choices tracked separately.

User-facing implication: writing `constraint positive_definite(A)`
creates an obligation. It does not mark `A` as positive definite.
The compiler must derive the fact, validate it at the workflow
boundary, carry it conditionally, or report the obligation as unmet.

### 3.5 `convert` scope on tensors — RESOLVED: semantic isomorphism / materialization / widening only

Decision (2026-04-23): tensor `convert` is narrow. It covers
meaning-preserving index isomorphisms, materialization of the same
mathematical object, and structural-refinement widening. It does not
cover precision changes, storage layout, device placement,
role-label changes, or approximate sparsification.

Chunk 04 O2.1 resolved `convert` invertibility verification via
bounded counterexample search. Tensor `convert` adds these rules:

- **Reshape / flatten.** `Vector<U, m*n> ↔ Matrix<U, m, n>` and
  related reshapes are legal only when the shape solver proves equal
  cardinality and the conversion names an index bijection. The
  bijection may be a stdlib canonical map or an explicit map in the
  conversion body. It transports axes, entry-unit laws, zero-pattern
  facts, and provenance through the new shape. Equal element count
  alone is not enough.
- **Sparse / dense materialization.** Sparse-to-dense is legal as
  materializing known zeros. Dense-to-sparse requires an explicit
  target pattern plus a proven or provider-validated `zero_pattern`
  fact for every entry outside the pattern. Thresholded
  sparsification, pruning, or over-approximate sparsity routes
  through `approximate`, not `convert`.
- **Structural-refinement widening.** A conversion may forget
  matrix facts without changing values: `Diagonal` to `Symmetric`,
  `PositiveDefinite` to `PositiveSemiDefinite`, or a named matrix
  type to the structural refinement it entails. Narrowing to a
  stronger refinement creates an obligation; it does not grant the
  fact.

Explicitly out of scope: numeric precision downcasts, storage order
(`CSR`, `CSC`, `COO`, row-major, column-major), host / GPU residency,
and matrix role relabels. Precision changes belong to `approximate`;
layout and device placement belong to backend / provider facts;
matrix roles are already represented as graph facts rather than
source-level types.

### 3.6 Shape refinements as type-level predicates — RESOLVED: staged solver

The locked tensor / matrix constructor shape requires the type system to express predicates like
`shape = (n, n)` for square matrices. Current refinement-type
machinery is scalar value predicates (`self >= 0`,
`self <= 1`).

Decision direction (2026-04-23): lock a broad shape-expression AST
now, with staged solver support. Do not lock a tiny shape language
that would need redesign for block matrices, batched tensors,
convolution-like formulas, or dynamic topology counts.

The shape model has four layers:

- **`DimExpr`.** Integer structural literals, `val` generics, axis
  lengths, provider-bound dimensions, arithmetic (`+`, `-`, `*`,
  exact division / divisibility), selected `min` / `max`, and
  topology-derived counts.
- **`ShapeExpr`.** Tuples plus indexing, rank, product, sum,
  transpose, concat, slice, insert/remove axis, flatten, reshape,
  and block partitions.
- **`ShapeConstraint`.** Equality, bounds, divisibility, product
  equality, matmul / reshape / stack / broadcast compatibility, and
  block-partition compatibility.
- **`ShapePhase`.** `static`, `provider_validated`,
  `runtime_bounded`, `dynamic_unknown`.

Guaranteed automatic solver subset for v2.1: tuple equality, rank,
indexing, product equality, transpose, concat / stack, and simple
affine dimension expressions where variables match syntactically.

Represented but not guaranteed automatically solved: floor / exact
division formulas for convolution-like operators, arbitrary nonlinear
arithmetic, dynamic topology dimensions, ragged row lengths, and
general block algebra. These stay expressible as obligations /
diagnostics so the abstraction does not have to change later.

Shape expressions are structural. They may appear in type parameters,
refinement predicates, stdlib primitive contracts, and diagnostics;
they are not runtime model values and relations cannot observe them
as ordinary numeric quantities.

### 3.7 Collections boundary — RESOLVED: explicit collection-axis extraction

`Matrix<U, m, n>` is a **distinct primitive** from
`[Scalar<U>; m, n]` (2D collection). Rationale:

- Linear-algebra semantics (decompositions, norms, solves,
  conditioning) are not collection-like.
- Tagged-handle machinery for heterogeneous argmax
  (`02_collections_iteration_report.md:299-329`) assumes scalar
  elements and doesn't generalize to matrix elements.
- GPU lowering for matrices routes through BLAS-style kernels
  (chunk 06); collection aggregation lowers through different
  kernels.
- Validity masks on dynamic collections are unnecessary overhead
  for fixed-shape matrices.

**Spec commitment:** collections and tensors are orthogonal. A
collection of tensors is meaningful (`[Matrix<U, m, n>; k]` — a
stack of matrices); a "matrix of collections" is not a primitive
construct.

Collections remain the right type for entity state in ecosystem
simulations (heterogeneous nodes, dynamic add/remove); tensors are
for numerical arrays with linear-algebra semantics.

The bridge is explicit collection-axis extraction. A stdlib relation
may gather a field from a collection into a tensor only by naming the
entity ordering, axis identity, field path, unit law, and missing /
inactive-entry policy. The extracted tensor carries facts such as:

```
axis(temp_vec, 0) = leaves ordered by leaf_id
entry_unit(temp_vec[i]) = kelvin
provenance(temp_vec) = collected_from(leaves.temperature)
```

There is no implicit `[Scalar<U>; n] -> Vector<U, n>` conversion and
no implicit "tensor axis is a collection" semantics.

### 3.8 Dynamic topology × matrix shapes — RESOLVED: ShapePhase plus regime-boundary handlers

Dynamic topology is not simply deferred. The Myco layer represents
it with `ShapePhase` facts (§3.6) and regime-boundary crossing
policies (§8.10 / §24.6 in spec). A shape-changing event does not
mutate a matrix in place; it crosses a boundary between topology
versions or executes in a runtime representation whose axis set is
explicitly dynamic.

Committed semantic modes:

- **`static`.** Shape known from source / generics.
- **`provider_validated`.** Shape known after workflow materializes a
  topology before planning.
- **`runtime_bounded` / `CapacityMask`.** Fixed maximum shape with
  alive masks and capacity records. Shape is stable; active set
  changes.
- **`event_replan`.** A topology-changing event creates a new
  topology version and a new member of an SCC family. The executor
  stops at the event boundary, applies the topology diff, recomputes
  axes / facts / sparsity / obligations, and re-lowers or dispatches
  a cached plan for the new version.
- **`dynamic_keyed`.** Axis sets are runtime maps keyed by entity IDs.
  This is a valid Myco semantic mode for CPU / host execution and
  dynamic sparse runtimes; compiled accelerator backends may reject
  or route through host / replan according to capability facts.
- **`dynamic_unknown`.** Shape is not sufficiently bounded or keyed
  for a selected backend / handler. Planning reports an unmet
  obligation or asks the workflow to choose a crossing policy.

This means v2.1 can model dynamic topology honestly without making
JAX-style static-shape execution the language semantics. Backends
advertise which modes they can lower efficiently. CPU reference
execution is the semantics-complete path for `dynamic_keyed`; GPU /
JIT backends may prefer capacity masks or event-boundary replanning.

An SCC may not silently change tensor shape in the middle of one
solve step. Shape-changing events must use an explicit handler:
`CapacityMask`, `EventReplan`, `DynamicKeyed`, or a future
backend-specific handler with equivalent semantics.

---

## 4. Primitive catalog — RESOLVED: names, signatures, and obligations

These primitives ship in the v2.1 stdlib as compiler-owned expression
atoms. Actual execution lives in chunk 06 (backend abstraction), but
the language-level surface and fact contracts are committed here.

Naming policy:

- Prefer standard math vocabulary in lowercase: `det`, `trace`,
  `transpose`, `adjoint`, `solve`, `norm`.
- Use `inverse(A)` as the canonical spelling; `inv(A)` is not the
  canonical surface.
- Matrix product uses ordinary `*` with shape / axis facts governing
  contraction. Elementwise multiplication, if needed, is a named
  primitive such as `hadamard(A, B)`, not `@`.
- Numeric matrix rank is `matrix_rank(A)` to avoid collision with
  shape rank (`rank(shape)`).
- Primitives consume facts and emit facts. A required unknown fact is
  an unmet obligation, not permission to pick a semantic fallback.

Committed primitive groups:

| primitive | required facts / signature shape | emitted facts / notes |
|---|---|---|
| `cholesky(A)` | `rank(A)=2`, `square(A)`, `symmetric(A)` or `hermitian(A)`, `positive_definite(A)`, `factorable_unit_law(A)` | `lower_triangular(L)`, `positive_diagonal(L)`, `A = L * transpose(L)` or Hermitian transpose, output unit law. PSD alone is not enough. |
| `lu(A)` | `rank(A)=2`, `square(A)`, `invertible(A)` or pivoting route | `(L, U, P)` with `P * A = L * U`, triangular facts, `permutation(P)`. |
| `qr(A)` | `rank(A)=2`, numeric entries, and scaling policy for heterogeneous units | `orthogonal(Q)`, `upper_triangular(R)`, `A = Q * R`, rank facts when rank-revealing route is selected. |
| `svd(A)` | `rank(A)=2`, numeric entries, and scaling policy for heterogeneous units | `(U, S, Vt)` with orthogonality facts, diagonal / nonnegative singular-value facts, and rank / spectral facts where classifiable. |
| `eigen(A)` | `square(A)`; symmetric / Hermitian facts for the real-symmetric route; Complex / backend facts for the general route | Eigenvalue / eigenvector facts, spectral-radius / eigenvalue bounds where classifiable. |
| `solve(A, b)` | `rank(A)=2`, `compatible_axes(A, b)`, `solvable(A, b)` | Solution axes / units, residual report, `condition_of(solve_block)` facts. Dispatch uses facts such as triangular, positive-definite, full-rank, or rank-deficient. |
| `solve_triangular(A, b)` | `lower_triangular(A)` or `upper_triangular(A)`, compatible axes, nonzero diagonal / solvability facts | Explicit direct-solve primitive; `solve` may route here when facts establish eligibility. |
| `least_squares(A, b)` | Rectangular or rank-deficient system facts, compatible axes, scaling policy | Solution / residual facts, rank / conditioning diagnostics. |
| `inverse(A)` | `square(A)`, `invertible(A)`, and materialization authorization | Inverse identities, inverse entry-unit law, condition facts. `inverse(A) * b` may rewrite to `solve(A, b)`. |
| `det(A)` | `square(A)` and determinant-capable unit / scalar facts | Determinant unit law and triangular-product simplifications. |
| `trace(A)` | `square(A)` and diagonal-entry unit comparability | Trace unit law; diagonal / block-diagonal simplifications. |
| `transpose(A)` | `rank(A)=2` | Swaps axes, transposes entry-unit law, flips upper / lower triangular facts, preserves applicable facts (§3.4). |
| `adjoint(A)` | Complex numeric support or real route where adjoint reduces to transpose | Conjugate-transpose facts; required by Hermitian primitives. |
| `norm(expr, kind)` | Supported kind (`"1"`, `"2"`, `"fro"`, `"inf"`), unit / scaling policy where needed | Norm envelope facts consumed by `condition_of` and approximation accounting. |
| `condition_of(expr)` | Shape, axis comparability, unit comparability, norm / scaling policy | `condition_estimate`, `condition_mode`, `condition_bound` when available. |
| `matrix_rank(A)` | `rank(A)=2`, numeric entries, tolerance / scaling policy | `rank_value(A)`, full-rank / nullspace facts when classifiable. |
| `gram(k, points)` | Kernel-domain compatibility and Gram construction facts | `gram_of(K,k,points)`, symmetry / PSD facts when provable, compact-support zero-pattern facts. |
| `zeros<U>(shape)` | Structural shape expression and unit parameter | Zero tensor with zero-pattern facts. |
| `ones(shape)` | Structural shape expression | Dimensionless all-ones tensor. |
| `identity(n)` | Structural square dimension | Dimensionless identity matrix; diagonal, orthogonal, positive-definite facts. |
| `diag(v)` | Vector input | Diagonal matrix with diagonal entries from `v`. |
| `diag_of(A)` | Matrix input | Vector of diagonal entries. |
| `stack`, `hstack`, `vstack` | Shape constraints from §3.6 | Tensor with derived shape, axis, and unit facts. |

Matrix literals remain the one syntax question in this cluster. The
primitive surface does not require a literal form; users can build
matrices through constructors, collection-axis extraction, providers,
or stdlib relations until literal syntax is locked.

---

## 5. Interactions with other v2.1 surfaces

- **Units (spec §4).** Heterogeneous-unit accounting is resolved by
  matrix facts: row/column axes, entry-unit laws, construction
  provenance, and provider-validation evidence. Homogeneous unit
  propagation remains the simple case.
- **Named types (spec §4).** Named-type stripping rules U1-U3
  (chunk 04) extend to structural-fact-preserving rules for
  tensor ops (`transpose(Symmetric) → Symmetric`, etc.).
- **Refinement types (spec §4.x).** Shape refinements §3.6 require
  extending refinement-type machinery from scalar value predicates
  to shape-level predicates. Matrix structural refinements §3.4
  lower to facts and obligations rather than user proof labels.
- **`convert` (spec §4.x).** Scope resolved in §3.5: reshape with
  index bijection, sparse / dense materialization with proven
  pattern facts, and structural-refinement widening are in scope;
  precision, storage order, device placement, and role relabels are
  out of scope.
- **Envelopes (chunk 04 Layer 2).** §3.3 — four flavors, merging
  rules, propagation per op.
- **Distributions (chunk 04 §11 Z-group).** Unblocks MVN (Z10
  Cholesky reparameterization), Wishart, InverseWishart. Their
  log_pdf expressions consume `det`, `trace`, `cholesky`, `solve`.
- **Condition analysis Level III (chunk 04 §11 O2.4).**
  `condition_of(solve_block)` is the Myco primitive; backends may
  lower it to `condest`-style runtime estimators. Chunk 04 locked the
  API shape (mode-tagged return);
  concretizing the runtime path depends on this chunk's primitive
  set and chunk 06's backend execution.
- **Kernels (chunk 03).** Gram matrices, low-rank approximations
  (K3). Kernel function `K(a, b)` remains scalar; `gram(K, points)`
  assembles an ordinary matrix carrying `gram_of`, `symmetric`, and
  `positive_semidefinite` facts when the kernel construction proves
  them. K2 separability rule (chunk 04 Bucket 3) directly consumes
  matrix tensor-product factorization.
- **Collections (chunk 02).** Distinct primitive per §3.7. Stacks
  of matrices (`[Matrix<U, m, n>; k]`) are meaningful; matrix of
  collections is not.
- **AD (`deriv` intrinsic).** `deriv(expr, Matrix) -> Matrix` /
  `deriv(Matrix, Scalar) -> Matrix` — symbolic rules for
  matrix-valued derivatives. Custom VJPs for decompositions
  (standard literature). Execution belongs to chunk 06 (AD
  ownership fork).
- **Neural controllers (`Controller` sources).** Data contracts with
  matrix-valued fields — mechanical extension once tensor types
  exist. Cross-backend interop belongs to chunk 06.
- **Events / topology.** §3.8 — dynamic matrix shapes use
  `ShapePhase` plus explicit regime-boundary handlers; backend
  execution support is capability-advertised.

---

## 6. Downstream unblocks

With this chunk shipped:

- Wishart / InverseWishart distributions (consume SPD / determinant /
  trace / factorable-unit matrix facts).
- Level III runtime `condition_of` concrete primitive target
  (though actual runtime path requires chunk 06).
- MVN log_pdf and Z10 Cholesky reparameterization.
- Gram matrices and low-rank kernel rewrites (K3 in chunk 04
  audit; chunk 03).
- Linear-system solves as first-class modeling primitive —
  hydraulic networks can declare stiffness matrices explicitly.
- Jacobian assembly for implicit-function SCC blocks.
- Finite-element mass / stiffness matrices as typed values.

---

## 7. Return path

Items in priority order (later items depend on earlier items
closing):

Completed: §3.6 shape-expression model, §3.3 envelope views, §3.4
structural fact lattice, §3.5 tensor `convert` scope, §3.7
collections boundary, §3.8 dynamic topology shape handling, and §4
primitive catalog. Scalar reconciliation is also closed: `Scalar<U>`
is rank-0 `Tensor<U, ()>` with scalar source spelling.

1. **Resolve matrix literal syntax (§4 Q11).** Does v2.1 ship a
   literal form, or constructors/providers only?
2. **Write the final v2.1 commitment text into the spec.**

Parallelizable with chunk 06 (backend abstraction) — chunk 06
needs this chunk's primitive list to have lowering targets; this
chunk doesn't need chunk 06's decisions to specify the type system.

This chunk and chunk 06 both must close before the remaining
matrix-dependent Z-group promotions (Wishart / InverseWishart and
Level III `condition_of` runtime) can actually ship. MVN consumes
the matrix fact model directly and depends on the relevant primitive
fact contracts being established for `Σ`.

---

## 8. Open questions (consolidated)

- **Q1.** Heterogeneous-unit question. RESOLVED 2026-04-23:
  compiler-facing matrix facts over ordinary tensors; no `LinearMap`
  type, no `basis` syntax, no user-marked matrix role annotations.
- **Q2.** Shape refinement language. RESOLVED 2026-04-23:
  broad shape-expression AST with staged solver support (§3.6).
- **Q3.** Envelope flavors. RESOLVED 2026-04-23: parallel
  entry-wise, norm, spectral, and structural views (§3.3).
- **Q4.** Structural fact lattice. RESOLVED 2026-04-23:
  implication facts/refinements, not enum subtypes or user proof
  labels (§3.4).
- **Q5.** `convert` scope. RESOLVED 2026-04-23: tensor convert
  covers reshape with index bijection, sparse / dense
  materialization with proven pattern facts, and
  structural-refinement widening; precision, storage order, device
  placement, and role relabels are out (§3.5).
- **Q6.** Scalar reconciliation. RESOLVED 2026-04-24:
  `Scalar<U>` is rank-0 `Tensor<U, ()>` with `Scalar` retained as
  the normative source spelling (§3.1).
- **Q7.** Sparse pattern facts. RESOLVED 2026-04-23:
  `zero_pattern` / `banded` / `block_sparse` are facts with evidence
  phase; storage representation remains backend-level (§3.4).
- **Q8.** Collections boundary. RESOLVED 2026-04-24:
  collections and tensors are orthogonal; bridges are explicit
  collection-axis extraction relations (§3.7).
- **Q9.** Dynamic topology × matrix shapes. RESOLVED 2026-04-24:
  `ShapePhase` plus regime-boundary handlers (`CapacityMask`,
  `EventReplan`, `DynamicKeyed`); no silent in-solve shape mutation
  (§3.8).
- **Q10.** Primitive catalog. RESOLVED 2026-04-24: committed names,
  signatures, fact contracts, and unmet-obligation behavior (§4).
- **Q11.** Matrix literal syntax — `[[1, 2]; [3, 4]]` or alternative.
  (§4)
