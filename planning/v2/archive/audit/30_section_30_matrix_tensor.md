# Audit Report — §30 Matrix and Tensor Primitives

Audited against corpus as of 2026-04-22.

Section 30 is a stub: it commits the stdlib function surface (cholesky, lu,
qr, svd, eigen, solve, inverse, det) and the opaque-primitive / capability-
contract framing, while deferring the type layer to §3.9 pending chunk 05
(B5). Most interesting corpus material lives in the Homeless bucket because
chunk 05 open questions are not yet closed and therefore cannot land in
spec_new.md.

---

## Absorbed

Corpus content already reflected in spec_new.md §30.

- **`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §4 (primitive
  list, tentative):**
  The chunk 05 report enumerates `cholesky`, `lu`, `qr`, `svd`, `eigen`,
  `solve`, `solve_triangular`, `least_squares`, `norm`, `condest`, `rank`,
  `det`, `trace`, `inv`, `transpose`, `zeros`, `ones`, `identity`, `diag`,
  `diag_of`, `stack`, `hstack`, `vstack`. The §30 committed list covers
  exactly the six decompositions (`cholesky`, `lu`, `qr`, `svd`, `eigen`) plus
  `solve`, `inverse`, and `det` — the subset whose downstream dependencies
  (MVN Z10, kernel Gram matrices, SCC solver dispatch) are already load-bearing
  in the current spec. The chunk 05 report is the provenance for this selection.

- **`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §3.4
  (structural subtype lattice) and `planning/v2/spec_new.md` §3.9:**
  The dispatch rule in §30 — "`solve(A, b)` dispatches on the structural
  subtype of `A` (triangular solve, Cholesky back-substitution, general LU)
  via the §3.9 lattice" — is sourced from chunk 05 §3.4 and already
  cross-referenced to §3.9. Provenance clear.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  B5 blocker summary (lines 936-955):**
  The statement that each primitive "wraps backend kernels and is opaque at
  the e-graph layer, with invariants declared by capability contract" reflects
  the chunk 04 B5 summary framing: primitives provide lowering targets for
  chunk 06 (backend abstraction); type-system decisions remain open in chunk
  05. The opaque-primitive characterization is consistent with chunk 04's
  "first-class matrix/tensor types with decompositions, norms, solves,
  structural subtypes" framing.

- **`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §1:**
  The three cross-references in §30's second paragraph (MVN reparameterization
  at §13.6 / Z10; kernel Gram-matrix machinery at §28; SCC solver dispatch)
  reflect the chunk 05 motivating list: "MVN internal reparameterization
  (Z10 rewrite)", "Gram matrices in kernel methods (chunk 03)", and "linear-
  system solves as first-class modeling primitive." Provenance clear.

- **`planning/v2/open_questions_deprecated_use_spec_new.md` "MultivariateNormal
  and multi-dimensional distributions" section (lines 871-878):**
  The MVN deferral note ("Deferred pending vector/matrix/container story lock")
  is the immediate predecessor to the current §30 + §13.6 framing, which
  resolves the deferral by committing the Cholesky surface. This corpus item
  is absorbed; the open-question framing is superseded by §13.6.

---

## Superseded

Corpus content replaced by decisions in spec_new.md §30 or related locked
sections. Should be added to anti_spec.md if not already present.

- **`planning/v2/spec.md` lines 2808-2821 (SCC solver classification):**
  The legacy spec describes solver dispatch as an ad-hoc planner behaviour:
  "Linear: solve with direct linear algebra (LU decomposition)"; "JAX:
  `jax.lax.custom_root` or Newton-Raphson loop with `jax.jacfwd`"; "Rust:
  standard NR with LU decomposition." This framing treats LU as a
  back-end-specific emission detail, not a named stdlib primitive with a
  structural-subtype dispatch rule. §30 supersedes this by promoting `lu`,
  `solve`, and the §3.9 subtype lattice to first-class language surface;
  solver selection by structural subtype is now a spec commitment, not a
  planner heuristic. The backend-specific emission (burn-style trait) belongs
  to Part V / chunk 06, not to solver naming.

  `Recommend:` The anti_spec.md "Retired architectural framing" section
  already covers "compiler auto-selected solver | workflow selects". The
  legacy §12 SCC solver dispatch wording is covered by anti_spec.md
  "Stale in legacy docs: spec.md §12, §13.2-13.3." No new anti_spec.md
  entry is required, but spec.md lines 2808-2821 should not be imported
  during any future consolidation pass.

- **`planning/v2/spec.md` line 2330 (incidence matrix as compiler-internal
  detail):**
  The legacy spec references "the incidence matrix of P" as an internal
  construct for SCC detection. §30 and §3.9 treat matrix types as user-
  visible language primitives, not compiler-internal objects. The incidence-
  matrix reference in spec.md is a legacy implementation note; it should not
  be elevated to user-visible surface.

  `Recommend:` No anti_spec.md entry needed; the stale-legacy-docs note
  covers spec.md wholesale. Document here for the consolidation pass.

- **`planning/v2/open_questions_deprecated_use_spec_new.md` lines 871-878
  (MVN deferral):**
  Superseded by §13.6. Already noted in Absorbed. Included here to mark the
  open-question framing as retired.

---

## Homeless

Corpus content relevant to §30 that is not yet in spec_new.md and is not
retired in anti_spec.md. This is the primary action bucket for a stub section.

### H1 — Opaque-primitive e-graph semantics: no body, contract-derived invariants

`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §3.1 (type
constructor, Option C rationale) states: "Unifies shape / unit / envelope / AD
machinery at the primitive level — one set of rewrite rules, one set of
envelope propagation laws, one set of GPU-lowering entry points." And §1:
"linear algebra primitives that do not currently exist in v2.1."

`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` lines
936-955 describe the opaque-primitive concern:

> "Opaque callables (`bind_controller`, neural nets, external Python). Black-
> box nodes — no internal structure the e-graph can rewrite. How does
> extraction handle 'this subtree is not rewritable'?"

§30 states the primitives "are opaque at the e-graph layer" and their
invariants "are declared by contract, not derived from body composition."
This is correct framing but leaves the mechanism unspecified. The chunk 04
report's open question about how the extractor handles opaque subtrees is
directly applicable: the linear-algebra primitives are the primary example
of opaque nodes with capability-contract-declared invariants. The spec does
not state what the e-graph does when it encounters an opaque node (no rewrite
rules fire, the node is treated as an e-class leaf, capability contracts are
asserted as metadata on the output e-class). This mechanism belongs either
in §30 or in §15 (e-graph substrate) with a forward reference from §30.

`Recommend:` Add a paragraph to §30 (or a note in §15) stating the opaque-
node handling rule: a linear-algebra primitive call is an e-graph leaf node
(no rewrite rules fire on its internal structure); its output e-class carries
the declared capability contracts as class-level metadata; rewrite rules
elsewhere may use those metadata facts (e.g. `solve(A, b)` fires the
Cholesky route when the `PositiveDefinite` fact is present on A's class).
This is a stable design implication of the capability-contract / opaque-
kernel framing already committed in §30; it just needs to be stated.

### H2 — `inverse(A) * b -> solve(A, b)` rewrite: source and scope

§30 states: "The compiler rewrites `inverse(A) · b` to `solve(A, b)` by
default to avoid explicit inversion in numeric code." This is a default-on
rewrite rule. The corpus does not specify which rewrite group this belongs to
(§17's D / E / X / Y / Z taxonomy), whether it is faithful/lossless (it is
for generic A) or carries an approximation penalty (it is not approximation,
it is algebraically equivalent), or what conditions disable it.

`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §7
(the 2x3 faithfulness x orientation matrix) establishes that "default-on
rewrites (lossless row: algebraic, unit-preserving, stdlib inverse round-
trips)" fire during saturation. The `inverse(A) * b -> solve(A, b)` rewrite
is exactly a "stdlib inverse round-trip" — it is the linear-algebra analogue
of the scalar inverse rewrites. Its source label (D-group algebraic,
or a new matrix-specific subgroup) is unspecified in both §30 and chunk 05.

`Recommend:` Classify `inverse(A) * b -> solve(A, b)` explicitly as a
default-on D-group algebraic rewrite (parallel to scalar inverse rewrites in
§6 / §17). Add a forward reference from §30 to §17 for this. The
classification is implied by the framing but not stated; without it, a
compiler implementer cannot determine the rewrite group or scheduling tier.

### H3 — `det(A)` unit signature: `Scalar<U^n>` open in chunk 05

`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §4 lists:

> "`det(Matrix<U, n, n>) -> Scalar<U^n>`"

§30 specifies `det` as "Determinant. On `Matrix<_, Triangular>` this reduces
to diagonal product; on general `A` it routes through LU." The return type
is not specified. The unit signature `Scalar<U^n>` (where n is the matrix
dimension) is a concrete consequence of the heterogeneous-unit question
(§3.2 in chunk 05, still open). For a homogeneous-unit matrix with entries
of unit U, `det` has unit `U^n`. The spec does not state this, leaving
`det`'s type signature incomplete.

Similarly, `trace(Matrix<U, n, n>) -> Scalar<U>` is in the chunk 05 §4
primitive list but not in §30's committed list at all, despite being required
by Wishart / InverseWishart log_pdf (chunk 04 Z-group, lines 735-743).

`Recommend:` Add a note to §30 acknowledging that return-type unit
signatures for `det` and `trace` are gated on the §3.2 heterogeneous-unit
resolution. Add `trace` to §30's committed list with a "(unit TBD per
chunk 05 §3.2)" annotation, since it is a downstream dependency of the
Wishart distributions already committed in principle (chunk 04 Z-group).

### H4 — `condest` / matrix operator norm: committed in chunk 04, absent from §30

`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` lines
1051-1058:

> "Level III runtime condition_of: actual runtime condition number of the
> assembled coefficient matrix via `condest` on the assembled matrix at the
> current operating point. ... Level III runtime machinery is gated on B5
> (matrix types, chunk 05). Chunk 05 must ship `condest` / σ_max / σ_min as
> matrix operator-norm primitives."

`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §4 includes:

> "`condest(Matrix<U, n, n>) -> Scalar<dimensionless>` — 1-norm condition
> estimator (Higham). Consumed by Level III `condition_of` per chunk 04."

`condest` is not in §30's committed list. It is committed by chunk 04 O2.4
(locked, lines 534-535 of chunk 04 changelog) as a required primitive for
Level III `condition_of`. This is not an open question — it is a locked
dependency. Its omission from §30 is a stub gap.

`Recommend:` Add `condest(A)` to §30's committed primitives list, noting
that it returns `Scalar<dimensionless>` (condition number is dimensionless)
and that it is consumed by `condition_of(expr)` Level III (§14.4). This is
a settled commitment from chunk 04 O2.4, not open design.

### H5 — `norm`, `rank`, `least_squares`: committed in chunk 05, absent from §30

`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §4 includes
`norm`, `rank`, and `least_squares` in the tentative primitive list. None
appear in §30. `norm` is needed by `condest` internally and by any
spectral-envelope computation (chunk 05 §3.3). `least_squares` is a natural
complement to `solve` for rectangular systems. `rank` is a diagnostic
primitive.

These are tentative (not locked as settled), so their absence from §30 is
consistent with the stub scope. However, the chunk 05 report marks them as
"should ship in v2.1 stdlib once type-system questions close."

`Recommend:` Add a forward-looking note in §30 that `norm`, `rank`,
`least_squares`, `trace`, and the constructor family (`zeros`, `ones`,
`identity`, `diag`, `stack`) are tentative additions pending chunk 05
closure, citing `05_matrices_in_progress.md` §4. This scopes the stub
accurately and prevents future readers from assuming §30's committed list
is the complete stdlib surface.

### H6 — Structural-subtype stripping rules: named in chunk 05, not in §30 or §3.9

`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §3.4:

> "`transpose(Symmetric) -> Symmetric`; `transpose(Triangular) ->
> other-triangular`; `inverse(PosDef) -> PosDef`; `A · Aᵀ -> PosSemiDef`.
> These are rewrite rules in the e-graph (Group D-style, named-type
> preserving). Must be enumerated."

These stripping / propagation rules are part of the §3.9 structural subtype
lattice story but appear in neither §3.9 nor §30. They are the matrix analogue
of the named-type stripping rules U1-U3 from chunk 04, and the chunk 05
report explicitly calls for their enumeration.

`Recommend:` Add a note to §3.9 (deferred items) or §30 that structural-
subtype stripping rules for common operations (`transpose`, `inverse`,
matmul, outer product) are open items in chunk 05 §3.4, to be enumerated
before the lattice is finalized. This keeps the reader from assuming the
§3.9 dispatch table is complete.

### H7 — Sparse / Kronecker structure: chunk 05 scope call needed, not deferred

`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §3.4:

> "Sparse pattern as type. `Sparse<U, n, pattern>` — is `pattern` a
> compile-time value (type-level matrix of booleans) or a runtime fact?
> If compile-time, it needs the type system to support matrix-of-boolean
> type-level values. If runtime, sparsity is an envelope fact, not a
> type refinement."

`planning/v2/spec_new.md` §3.9 defers: "`CSR` vs `CSC` vs `COO` vs
`block-sparse` — the structural property `Sparse` is an abstract marker;
the concrete storage is a backend-level choice tracked in chunk 06."

These are two distinct open questions that §3.9 conflates. The storage-
format question (CSR vs CSC) is correctly a chunk 06 backend concern.
But the type-vs-envelope question for sparsity pattern is a chunk 05 type-
system question: whether `Sparse` in the lattice carries a compile-time
pattern parameter or is a runtime envelope fact determines whether the
`Sparse` entry in the §3.9 table is correct as written. §30's committed
`solve` primitive dispatches on structural subtype including `Sparse` /
`Banded<b>` (via the §3.9 lattice), so this question is load-bearing for
§30's dispatch rule.

Kronecker structure (separable product matrices) is mentioned in
`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §5:
"K2 separability rule (chunk 04 Bucket 3) directly consumes matrix
tensor-product factorization." The K2 rewrite for separable kernel Gram
matrices is a known downstream use of Kronecker structure, but neither
§28 nor §30 states how it interacts with the structural subtype lattice.

`Recommend:` Separate the two open items in §3.9's `Sparse` deferral
note: (1) storage format (backend, chunk 06 — correctly deferred); (2)
whether sparsity pattern is a type parameter or envelope fact (type system,
chunk 05 — the open question that affects §30's dispatch rule). Add a cross-
reference to chunk 03's K2 separability rule for Kronecker structure, noting
that the matrix tensor-product factorization needed for K2 rewrites is a
chunk 05 design item.

### H8 — Dynamic matrix shapes: open question in open_questions_deprecated, absent from §30

`planning/v2/open_questions_deprecated_use_spec_new.md` lines 725-733:

> "Dynamic matrix shapes. Fixed-shape `Matrix<N, M>` is well-defined in
> the §3.9 structural subtype lattice. Dynamic-shape `Matrix<?, ?>` (shape
> unknown at compile time, bound by the workflow) needs a worked-out story:
> how the shape-refinement system interacts with the lattice, how shape-
> dependent dispatch resolves at workflow composition vs runtime, and what
> the error surface looks like when a runtime shape violates a structural
> constraint."

This open question is not retired in anti_spec.md and is not addressed in
§30 or §3.9. Chunk 05 §3.8 ("Dynamic topology x matrix shapes") defers
dynamic-shape matrices to v2.2, recommending: "For v2.1: tensor shapes are
compile-time known. Document this limitation explicitly."

§30 is silent on this scope call. A reader of §30 cannot determine whether
the `solve(A, b)` primitive accepts a workflow-bound dynamic-shape A or
only a compile-time-known-shape A.

`Recommend:` Add a scope note to §30 (or §3.9) stating that v2.1 tensor
shapes are compile-time known; dynamic-shape matrices are deferred to v2.2
per chunk 05 §3.8. This closes the open-question framing in
`open_questions_deprecated_use_spec_new.md` for v2.1 scope purposes.

### H9 — Scalar reconciliation: Scalar<U> := Tensor<U, ()> decision not in spec_new.md

`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §3.1:

> "Reconciliation with existing Scalar<U>: Lean: (i) — redefine
> `Scalar<U> := Tensor<U, ()>`. Unification is cleaner and the ergonomic
> surface stays the same."

`planning/v2/spec_new.md` §3.8 ("Scalar and Tensor Reconciliation") states:

> "Open: whether `Scalar<U>` is formally sugar for `Tensor<U, ()>`
> (shape-zero tensor) or a distinct primitive with coercion rules."

This open question is identified in both chunk 05 and §3.8 but carries a
"lean: (i)" recommendation in chunk 05 that has not propagated to §3.8 as
a lock. It is not retired in anti_spec.md. This is directly relevant to
§30's function signatures: if `Scalar<U> := Tensor<U, ()>` then `det`
returns a `Tensor<U^n, ()>` and the §30 signatures are well-typed; if they
are distinct, a coercion rule is needed.

`Recommend:` Close the §3.8 open question by recording the chunk 05 lean
in a "direction" note, or flag it explicitly as a chunk 05 blocking item
so it does not remain as an unresolved open question floating in §3.8.

### H10 — Envelope flavors for matrix quantities: open in chunk 05, forward-ref missing from §30

`planning/v2/spec_new.md` §3.9 defers:

> "Envelope flavors for matrix quantities. Whether matrix-valued quantities
> participate in the layer-2 envelope metadata system (§17) in the same way
> scalars do, or need specialized envelope machinery."

`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §3.3 details
four flavors (entry-wise bounds, operator-norm bounds, spectral bounds,
structural bounds as facts) with the observation that "probably all four
are needed; they merge differently under different ops." This is load-bearing
for Level III `condition_of` (spectral bounds on `condest` output), MVN
Cholesky reparameterization (positive-diagonal structural fact on L), and
Gram-matrix PSD guarantee (PSD as a structural fact on the kernel's Gram
output).

§30 is silent on envelope behavior of its outputs. The `cholesky` primitive
returns `Matrix<_, LowerTriangular>` — but what envelope facts does the
output carry? The chunk 05 framing implies it carries a structural-bounds
fact (positive-diagonal) which is the basis for the PD-encoded-by-L
optimization in §13.6.

`Recommend:` Add a note to §30 that envelope propagation rules for each
primitive (what structural or spectral facts the output carries) are open
items in chunk 05 §3.3, to be specified when the four envelope flavors are
closed. Cross-reference §13.6 as the concrete example (cholesky output
carries a positive-diagonal structural fact).

---

## Conflicts

Direct contradictions between spec_new.md §30 and the corpus.

### C1 — `cholesky` input type: §30 says `Matrix<_, PositiveDefinite>`, chunk 05 says `PosDef<U, n>`

`planning/v2/spec_new.md` §30 line 4566:

> "`cholesky(A)`. Lower-triangular factor `L` such that `L · Lᵀ = A`
> for `A: Matrix<_, PositiveDefinite>`."

`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §4:

> "`cholesky(PosDef<U, n>) -> LowerTriangular<U, n>`"

`planning/v2/spec_new.md` §3.9 line 454:

> "| `PositiveDefinite` | `xᵀ A x > 0` for all `x ≠ 0` | yes |"

The §3.9 lattice uses `PositiveDefinite` as a structural property name (a
predicate applied to a `Matrix<_, _>`), while §30 writes `Matrix<_, PositiveDefinite>`
in argument position. The chunk 05 report uses `PosDef<U, n>` as a
standalone type alias. The three spelling conventions are inconsistent:
- §3.9: `PositiveDefinite` as a lattice element (property name)
- §30: `Matrix<_, PositiveDefinite>` (property as second type argument)
- chunk 05: `PosDef<U, n>` (standalone alias)

The naming conflict is minor but concrete: `PositiveDefinite` vs `PosDef`.
The type constructor form `Matrix<_, PositiveDefinite>` used in §30 implies
the structural property is the second type argument to `Matrix`, but §3.9
does not specify the `Matrix` type constructor's argument position for
structural properties.

`Recommend:` Align the naming convention across §30 and §3.9 before chunk 05
closes. The §3.9 full-name form (`PositiveDefinite`) is more readable and
matches the table; the chunk 05 alias (`PosDef`) is shorter. Pick one and
use it consistently. Also clarify the type-constructor syntax: is it
`Matrix<U, n, n, PositiveDefinite>` (property as fourth argument) or
`Matrix<U, n, n> where { structural: PositiveDefinite }` or a named alias
`PositiveDefinite<U, n>`? §30 currently uses `Matrix<_, PositiveDefinite>`
which implies a two-argument form inconsistent with the three-argument
`Matrix<U, m, n>` declared in chunk 05 §3.1.

### C2 — `inverse` naming: §30 uses `inverse(A)`, chunk 05 uses `inv(A)`

`planning/v2/spec_new.md` §30 line 4579:

> "`inverse(A)`. Direct inversion for documentation and small cases..."

`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §4:

> "`inv(Matrix<U, n, n>) -> Matrix<U^(-1), n, n>` — flagged in docs as
> worse-conditioned than `solve` for most uses."

The two documents use different names for the same primitive (`inverse` vs
`inv`). The chunk 05 note also specifies the return type as `Matrix<U^(-1),
n, n>`, which §30 omits entirely. The return-type omission compounds the
naming discrepancy: `inverse` returning `Matrix<_, _>` without specifying
the unit of the inverse is a type-signature gap.

`Recommend:` Pick one name (`inv` is more consistent with the short-
lowercase-math-vocabulary convention stated in chunk 05 §4, and matches
LAPACK / NumPy convention) and record it. Add the return-type unit
`Matrix<U^(-1), n, n>` to §30's signature, with a note that the exact
unit-inversion semantics depend on the heterogeneous-unit resolution
(chunk 05 §3.2).

### C3 — `eigen` complex case: §30 defers to §26.1, chunk 05 only handles symmetric

`planning/v2/spec_new.md` §30 lines 4572-4575:

> "`eigen(A)`. Eigenvalue / eigenvector pair for square `A`. Real-
> symmetric specialization returns real eigenvalues and orthonormal
> eigenvectors; general case defers to complex eigenvalues pending
> §26.1 `Complex` lock."

`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §4:

> "`eigen(Symmetric<U, n>) -> (Vector<U, n>, Orthogonal<U, n, n>)`"

The chunk 05 report commits `eigen` only for `Symmetric` input (the
real-eigenvalue case), treating general `eigen` as implicitly out of scope
pending `Complex`. §30 describes `eigen` as accepting "square `A`" with a
deferral for the general case, implying the general-case signature exists
but is incomplete — a slightly different framing from chunk 05's restriction
to `Symmetric` input.

This is a minor scope framing discrepancy rather than a contradiction, but
it should be resolved: either §30 should restrict `eigen` to `Symmetric`
input matching chunk 05, or it should explicitly state that general `eigen`
is a committed stub pending §26.1.

`Recommend:` Align §30 with chunk 05 by restricting the v2.1 committed form
of `eigen` to `Symmetric<U, n>` input. Move the general-square-A form to
a "(pending §26.1 Complex lock)" note, making the scope call explicit.

---

*End of audit report.*
