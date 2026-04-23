# Myco v2.1 — Matrix / Tensor Types Design Report (IN PROGRESS)

**Date:** 2026-04-20
**Authors:** Riley Leff, Claude (Opus 4.7)
**Reviewers:** None yet
**Status:** IN PROGRESS. Shape polymorphism direction locked (Option
C — `Tensor<U, shape>` primitive with `Vector<U, n>` / `Matrix<U, m, n>`
as shape-refined aliases). Heterogeneous-unit question, envelope
flavors, and structural subtype lattice all open. Backend / AD / GPU
lowering concerns factored out into chunk 06.

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
  norm (`condest` / σ_max / σ_min).
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
- **No structural subtypes** for Symmetric, PSD, Diagonal,
  LowerTriangular, Banded, Orthogonal.
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
structural subtype), envelope generalization, probabilistic
integration (MVN/Wishart), and e-graph integration (shape
compatibility under merge) all require explicit design.

---

## 3. Design surface — what needs to be decided

### 3.1 Type constructor shape — LOCKED: Option C

**Decision (2026-04-20):** `Tensor<U, shape>` is the primitive;
`Vector<U, n>` and `Matrix<U, m, n>` are **shape-refined aliases**
atop it. Structural subtypes (Symmetric, PosDef, etc.) are further
refinements on `Matrix<U, n, n>`.

```
Tensor<U, shape>           # primitive; arbitrary rank
Vector<U, n>   := Tensor<U, (n,)>
Matrix<U, m, n> := Tensor<U, (m, n)>
Scalar<U>      := Tensor<U, ()>   # reconciliation with existing Scalar TBD
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
- Lets structural subtypes compose cleanly (`PosDef` refines
  `Matrix<U, n, n>` which refines `Tensor<U, (n, n)>`).
- Matches the burn / JAX / PyTorch mental model without importing
  their runtime — the type system captures rank statically; the
  backend (chunk 06) handles execution.

**Reconciliation with existing Scalar<U>.** `Scalar<U>` today is a
dimensioned number, not a rank-0 tensor. Options:

- (i) Redefine `Scalar<U> := Tensor<U, ()>`. Unifies everything;
  breaks no user code (Scalar continues to behave the same).
- (ii) Keep `Scalar<U>` distinct; provide bidirectional implicit
  conversion with `Tensor<U, ()>`. Less disruptive internally;
  requires clear conversion rules.

Lean: (i) — unification is cleaner and the ergonomic surface stays
the same. Users write `Scalar<meters>`, compiler sees
`Tensor<meters, ()>`.

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
question. Remaining chunk-05 work is fact propagation: envelope
flavors, shape refinements, structural fact lattice, sparse-pattern
facts, and primitive fact contracts.

### 3.3 Envelope flavors for matrix-valued quantities — OPEN

Chunk 04 §3 layered state: equality substrate / envelope metadata /
adjacent keyed. Envelopes for scalars are intervals `[lo, hi]`. For
tensors:

- **Entry-wise bounds.** Each entry has a scalar envelope. Simple;
  survives elementwise ops cleanly. Fails under matmul (matrix
  norms aren't subadditive over elementwise bounds).
- **Operator-norm bounds.** `‖A‖_p ≤ ε` for some norm. Composes
  under matmul (submultiplicative). Loses entry-level information.
- **Spectral bounds.** Eigenvalue / singular-value intervals
  `λ_min(A) ≥ a`, `λ_max(A) ≤ b`. **Needed for Level III
  `condition_of`.** Composes under some ops (Cholesky preserves
  positive definiteness), fails under others.
- **Structural bounds as facts.** "This matrix is PosDef" is itself
  an envelope fact — not a numerical interval but a structural
  claim. Enables specialized solver dispatch at lowering.

Probably all four are needed; they merge differently under different
ops. Key design questions:

- **Canonical form.** Is there a canonical envelope representation
  that all four flavors project into? Probably not — they capture
  genuinely different information.
- **Storage / merging rules.** When two e-classes merge and one has
  entry-wise bounds and another has spectral, what's the combined
  envelope?
- **Propagation rules per op.** A matmul / solve / inverse /
  decomposition each has a different envelope transformation table.
  Stdlib must specify these.

### 3.4 Structural subtype lattice — OPEN

Common structural subtypes and their refinement relationships:

```
Matrix<U, n, n>
├── Symmetric<U, n>              (A = Aᵀ)
│   ├── PosDef<U, n>             (+ positive eigenvalues)
│   └── PosSemiDef<U, n>         (+ non-negative eigenvalues)
├── Diagonal<U, n>               (off-diagonal = 0)
│   ├── Scaled<U, n>             (Diagonal + constant)
│   └── Identity<U, n>           (Diagonal + all-ones)
├── Triangular<U, n>
│   ├── UpperTriangular<U, n>
│   └── LowerTriangular<U, n>
├── Orthogonal<U, n>             (AᵀA = I)
│   └── Rotation<U, n>           (Orthogonal + det = +1)
├── Sparse<U, n, pattern>
└── Banded<U, n, bandwidth>
```

Design questions:

- **Which ship in v2.1 stdlib?** Probably Symmetric, PosDef,
  Diagonal, Triangular (upper/lower), Orthogonal, Sparse. Rotation
  / PosSemiDef / Banded / Scaled can be user-defined refinements.
- **Declaration syntax.** Likely uses existing refinement-type
  machinery generalized to shape: `type PosDef<U, n> := Symmetric<U, n>
  where { all(eigenvalues(self) > 0) }`. Needs clarity on whether
  structural predicates like "symmetric" are checkable at compile
  time (for literal matrices) or only at runtime (for assembled
  matrices).
- **Composition.** Can a user write `SymmetricPosDef<U, n>` that
  refines both? Probably subsumed by `PosDef` (which implies
  Symmetric), but the general composability question applies to
  user-defined refinements.
- **Stripping rules.** `transpose(Symmetric) → Symmetric`;
  `transpose(Triangular) → other-triangular`;
  `inverse(PosDef) → PosDef`; `A · Aᵀ → PosSemiDef`. These are
  rewrite rules in the e-graph (Group D-style, named-type
  preserving). Must be enumerated.
- **Sparse pattern as type.** `Sparse<U, n, pattern>` — is `pattern`
  a compile-time value (type-level matrix of booleans) or a runtime
  fact? If compile-time, it needs the type system to support
  matrix-of-boolean type-level values. If runtime, sparsity is an
  envelope fact, not a type refinement.

### 3.5 `convert` scope on tensors — OPEN (needs scope call)

Chunk 04 O2.1 resolved `convert` invertibility verification via
bounded counterexample search. Tensor conversions:

- **Reshape** (`Vector<U, m·n> ↔ Matrix<U, m, n>`). Trivially
  lossless at the math layer; symbolic reduction proves identity.
  Definitely in scope.
- **Precision** (`Matrix<Float64, m, n> ↔ Matrix<Float32, m, n>`).
  Lossy; already covered by `approximate` block surface from chunk 04
  §9. Language-level `convert` probably refuses this; the
  `approximate` block handles it.
- **Storage order** (row-major ↔ col-major). Codegen detail, not a
  language concern. Out of scope.
- **Dense ↔ sparse.** Semantically equal when sparsity pattern
  matches actual zero entries; semantically lossy when pattern
  over-approximates. Bounded counterexample search can't prove
  sparsity-pattern equivalence generally. Proposal: sparse-to-dense
  is lossless (just materialize zeros); dense-to-sparse requires
  explicit pattern declaration and is either provably-lossless (no
  entries outside pattern are nonzero) or rejected.
- **Named-type ↔ structural subtype** (`DistanceMatrix<U, n> ↔
  Symmetric<U, n>`). Named types that are structurally refinements
  use bare `convert` (relabel). Consistent with existing scalar
  pattern.

**Explicit in-scope for v2.1:** reshape, sparse↔dense with
pattern declaration, named↔structural relabel.

**Explicit out-of-scope:** precision conversion (routes through
`approximate`), storage-order (codegen detail).

### 3.6 Shape refinements as type-level predicates — OPEN (prerequisite)

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

### 3.7 Collections boundary — CLARIFICATION

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

**Explicit statement for the spec:** collections and tensors are
orthogonal. A collection of tensors is meaningful
(`[Matrix<U, m, n>; k]` — a stack of matrices); a "matrix of
collections" is not a sensible construct.

Collections remain the right type for entity state in ecosystem
simulations (heterogeneous nodes, dynamic add/remove); tensors are
for numerical arrays with linear-algebra semantics.

### 3.8 Dynamic topology × matrix shapes — DEFERRED

Growing adjacency matrices as entities join/leave the ecosystem
(via events) is a real v2.2+ problem. Interactions:

- Matrix shape parameters become runtime quantities under dynamic
  topology.
- Reshape events need to preserve existing entries, pad new ones.
- SCC decomposition of a matrix whose shape changes mid-run is
  nontrivial.

**For v2.1:** tensor shapes are compile-time known. Document this
limitation explicitly; defer dynamic-shape matrices to future
work. Ecosystem adjacency / topology is expressed via collections
and node-level state, not via growing matrices.

---

## 4. Primitives to commit (tentative)

These should ship in v2.1 stdlib once type-system questions §3.2 /
§3.3 / §3.4 / §3.6 close. Actual execution lives in chunk 06
(backend abstraction).

**Decompositions (on `Matrix<U, n, n>` or appropriate refinement):**
- `cholesky(PosDef<U, n>) -> LowerTriangular<U, n>`
- `lu(Matrix<U, n, n>) -> (LowerTriangular<U, n>,
  UpperTriangular<U, n>, Permutation<n>)`
- `qr(Matrix<U, m, n>) -> (Orthogonal<U, m, m>, UpperTriangular<U, m, n>)`
- `svd(Matrix<U, m, n>) -> (Orthogonal<U, m, m>, Diagonal<U, r>,
  Orthogonal<U, n, n>)` where `r = min(m, n)`
- `eigen(Symmetric<U, n>) -> (Vector<U, n>, Orthogonal<U, n, n>)`

**Solves:**
- `solve(Matrix<U, n, n>, Vector<U, n>) -> Vector<U, n>` — dispatches
  on structural subtype (Cholesky for PosDef, triangular solve for
  Triangular, LU otherwise).
- `solve_triangular(Triangular<U, n>, Vector<U, n>) -> Vector<U, n>`
- `least_squares(Matrix<U, m, n>, Vector<U, m>) -> Vector<U, n>`

**Norms and diagnostics:**
- `norm(Tensor, kind: "1" | "2" | "fro" | "inf") -> Scalar<U>`
- `condest(Matrix<U, n, n>) -> Scalar<dimensionless>` — 1-norm
  condition estimator (Higham). Consumed by Level III
  `condition_of` per chunk 04.
- `rank(Matrix<U, m, n>) -> Scalar<dimensionless>`

**Basic ops:**
- `det(Matrix<U, n, n>) -> Scalar<U^n>`
- `trace(Matrix<U, n, n>) -> Scalar<U>`
- `inv(Matrix<U, n, n>) -> Matrix<U^(-1), n, n>` — flagged in docs
  as worse-conditioned than `solve` for most uses.
- `transpose(Tensor<U, (m, n)>) -> Tensor<U, (n, m)>`
- `adjoint` — conjugate-transpose for complex (v2.2+ if complex
  numbers defer).
- Matmul as `@` operator: `Matrix<U, m, k> @ Matrix<V, k, n> ->
  Matrix<U·V, m, n>`.
- Matrix-vector, matrix-scalar.

**Constructors:**
- `zeros(shape) -> Tensor<U, shape>`
- `ones(shape) -> Tensor<dimensionless, shape>`
- `identity(n) -> Diagonal<dimensionless, n>`
- `diag(Vector<U, n>) -> Diagonal<U, n>`
- `diag_of(Matrix<U, n, n>) -> Vector<U, n>`
- `stack`, `hstack`, `vstack` — with shape-arithmetic on the output
- Matrix literals — syntax TBD (`[[1, 2]; [3, 4]]` or similar).

**Open question.** Exact naming, argument order, error modes all
TBD. Naming convention should match scalar stdlib (short, lowercase,
math-vocabulary).

---

## 5. Interactions with other v2.1 surfaces

- **Units (spec §4).** Heterogeneous-unit accounting is resolved by
  matrix facts: row/column axes, entry-unit laws, construction
  provenance, and provider-validation evidence. Homogeneous unit
  propagation remains the simple case.
- **Named types (spec §4).** Named-type stripping rules U1-U3
  (chunk 04) extend to structural-subtype-preserving rules for
  tensor ops (`transpose(Symmetric) → Symmetric`, etc.).
- **Refinement types (spec §4.x).** Shape refinements §3.6 require
  extending refinement-type machinery from scalar value predicates
  to shape-level predicates. Structural subtypes §3.4 build on
  this.
- **`convert` (spec §4.x).** Scope call per §3.5 — reshape +
  sparse↔dense with pattern + named↔structural relabel in scope;
  precision + storage-order out of scope.
- **Envelopes (chunk 04 Layer 2).** §3.3 — four flavors, merging
  rules, propagation per op.
- **Distributions (chunk 04 §11 Z-group).** Unblocks MVN (Z10
  Cholesky reparameterization), Wishart, InverseWishart. Their
  log_pdf expressions consume `det`, `trace`, `cholesky`, `solve`.
- **Condition analysis Level III (chunk 04 §11 O2.4).** Matrix
  `condest` is the primitive; `condition_of(solve_block)` lowers to
  it. Chunk 04 locked the API shape (mode-tagged return);
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
- **Events / topology.** §3.8 — dynamic matrix shapes deferred
  to v2.2.

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

1. **Close §3.6 (shape refinements).** Prerequisite for §3.4
   structural subtypes. Lean: broad shape-expression AST with a
   staged solver; v2.1 automatic support starts with tuple equality,
   rank, indexing, product equality, transpose, concat / stack, and
   simple affine expressions.
2. **Close §3.3 (envelope flavors).** Gates Level III
   `condition_of` machinery. Four flavors, merging rules,
   per-op propagation tables.
3. **Close §3.4 (structural subtype lattice).** Stdlib subtypes +
   declaration syntax + stripping rules.
4. **Close §3.5 (convert scope).** Mostly a scope call with small
   design consequences.
5. **Close §3.7 (collections boundary).** Written above as
   clarification; needs one-paragraph commitment in spec.
6. **Close §3.8 (dynamic topology deferral).** Documentation call.
7. **Draft primitive list §4 concretely.** Names, signatures,
   errors. Well-trodden; low design risk.
8. **Write the v2.1 commitment text into the spec.**

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
- **Q2.** Shape refinement language — how much shape arithmetic
  ships in v2.1? (§3.6)
- **Q3.** Envelope flavors and their per-op propagation rules?
  (§3.3)
- **Q4.** Structural subtype lattice — which ship in stdlib, how
  declared, what composition rules? (§3.4)
- **Q5.** `convert` scope — reshape / sparse / precision / storage
  order / named↔structural: which are in / out? (§3.5)
- **Q6.** Scalar reconciliation — redefine `Scalar<U> := Tensor<U,
  ()>` or keep distinct with implicit conversion? (§3.1)
- **Q7.** Sparse pattern as type-level value — compile-time matrix
  of booleans or runtime envelope fact? (§3.4)
- **Q8.** Matrix literal syntax — `[[1, 2]; [3, 4]]` or alternative.
  (§4)
