# Audit Report — §28 Kernels

Audited against corpus as of 2026-04-22.

---

## Absorbed

Corpus content that already landed in spec_new.md §28.

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §2 ("What 'kernel' means here"):**
  > "A kernel is just a function `K: A × B → V` — typically from two points in some domain (or a point and a subdomain) to a scalar weight or value."
  > "**Decided:** kernels are not a new kind. Not a new keyword. Not a new block. They are ordinary `.myco` functions."

  Absorbed into §28 preamble and §28.1: "Kernels are ordinary functions from pairs of locus points to scalars: `fn k(x: Point<L>, y: Point<L>) -> Scalar<U>`. No separate `kernel` keyword, no separate type kind."

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §3 ("What was rejected"):**
  > "A new `kernel` keyword or kernel kind. Rejected in favor of 'kernels are just functions.'"
  > "A stdlib-only hierarchy like `SpatialKernel<Reduction, Profile>`. Rejected because it isn't universal."

  Absorbed into §28.1's "no separate `kernel` keyword, no separate type kind" and the capability-contract-based framing. The rejection is also reflected in `anti_spec.md`:
  > "`kernel` keyword or kernel kind | ordinary `fn` accepting two point arguments and returning a scalar | kernels are not a distinct construct; §6 states the design positively"

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §2 ("universality is required"):**
  > "The standard library will provide the common kernels (Matérn family, Gaussian, compact-support splines, etc.), but users must be able to write arbitrary ones."

  Absorbed into §28.1: stdlib ships "Matérn (ν = 1/2, 3/2, 5/2, ∞), squared-exponential (RBF), rational-quadratic, Wendland compact-support," and §28 preamble notes kernel-ness comes from capability contracts, not a closed taxonomy.

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §4 ("Ambient-locus problem"):**
  > "If a kernel needs a position in a larger domain ... how does it get ambient coordinates without hardcoding a global spatial frame?"
  > "**Answer:** this is solved by the horse/fly composition pattern already in v2.1. Parents expose scalar coordinate fields; children sample the parent via `.at()`. Visibility is downward-only. No new machinery needed."

  Absorbed into §28.2 "Ambient-Locus via Composition": kernels take `Point<L>` arguments where `L` is ambient at the call site.

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §5 ("Sparsity / characteristic length — deferred"):**
  > "Not yet fully locked. Revisit after the e-graph / cost / unified-machinery layer is drawn up."

  Absorbed into §28.3 as a chunk-03 deferral, with the additional observation that sparse representation belongs in matrix assembly (chunk 05 / B5), not kernel definition.

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §6 ("Integration semantics — deferred"):**
  > "Mixing continuous domains ... and discrete collections ... in the same kernel integrand is needed. Proposal floated: `integrate(expr for p in D)` ... Syntax not locked. Semantics not locked."

  Absorbed into §28.3 second bullet: "Kernel integration operators. Convolution, integration against a measure, and the various ways kernels interact with stochastic integrals."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` rewrite catalog K1:**
  > "K1. K(a,b) → 0 when distance(a,b) > L_char for compact-support/rapidly-decaying kernels (Gaussian at ±3σ, Matérn, spline compact support)."

  Absorbed into spec_new.md §32 K1 (the rewrite catalog section, lines 5309-5313), cross-referenced from §28.

- **`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §5 (kernels interaction):**
  > "Kernel function `K(a, b)` remains scalar; `gram(K, points)` assembles a `Symmetric<U, n, n>` or `PosSemiDef<U, n, n>`."

  Absorbed into §28.3's sparsity deferral to chunk 05 / B5 and into §30's commitment that the Cholesky primitive consumes `Matrix<_, PositiveDefinite>` from kernel Gram matrices.

- **`planning/v2/spec_new.md` §13.5 (Tier C opaque handoff cross-reference):**
  > "Higher-order distributions. Distributions over functions (Gaussian processes, etc.) route through kernel machinery (§28) rather than the parametric Tier 1 list."

  Absorbed as the Tier C handoff path §28.3 references for GP covariance use.

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §28. Should move to anti_spec.md if not already there.

- **`planning/v2/v2.1_in_progress.md:1686-1714` ("NEW: Coupling & Kernels"):**
  > "### Kernel coupling — `K(a, b) -> strength`. Soft spatial coupling between two domains or between a domain and discrete entities."
  > "### Coupling may be sugar for relation with kernel integration."

  Superseded by §28's "kernels are ordinary functions" framing. The `K(a, b) -> strength` surface as a distinguished coupling construct is replaced by kernels-as-ordinary-fns plus ambient-locus composition (§28.2). The coupling-as-sugar pattern is subsumed and deferred with the §28.3 integration operators.

  `Recommend:` The `v2.1_in_progress.md` "NEW: Coupling & Kernels" section is a legacy-doc stale chunk. Its three retired open questions (kernel-as-fn vs own declaration, learnable kernels via controller, sparsity plumbing) are all either closed or deferred in §28 and `03_kernels_in_progress.md`. Add an explicit entry to `anti_spec.md` "Stale in legacy docs" for this v2.1_in_progress section so future readers do not reimport it.

- **`planning/v2/spec.md:1347, 1686-1711, 1957`:**
  > "handled by kernel coupling, not the geometry system."
  > "## NEW: Coupling & Kernels ..."

  Superseded by §28 as above. These spec.md references are already covered by the anti_spec.md "spec.md ... supersede wholesale" entry for the relevant sections, but none of them explicitly name §28.

- **`planning/v2/open_questions_deprecated_use_spec_new.md:680-693` ("Coupling & Kernels"):**
  > "Is a kernel just a function used inside an `integrate` call, or does it need its own declaration? (Leaning toward: just a function — Approach 1.)"
  > "Is `coupling` a keyword, or just a pattern the compiler detects in kernel-weighted integrals?"

  Superseded. §28 commits to "kernels are ordinary functions"; the `coupling`-as-keyword question is closed by the same commitment. The deprecated open-questions doc is globally marked superseded by `spec_new.md` but this specific question cluster resolves cleanly and can be dropped.

---

## Homeless

Corpus content relevant to §28, not accounted for in spec_new.md §28, and not already committed to anti_spec.md. Highest-value bucket.

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §2 ("If the compiler needs to know something special ... that information lives as properties on the function"):**
  > "If the compiler needs to know something special about them (monotonicity, compact support, separability, stationarity, etc.), that information lives as properties on the function — in the same surface that already exists for `invertibility` / `differentiability` / `domain`."

  §28.1 names `PositiveDefinite`, `Stationary`, `Isotropic` as contracts but does not name `compact_support` as a contract. `anti_spec.md` retires user-declared `property` blocks in favor of "capability contracts + `constraint` blocks." §28 does not state which surface carries the compact-support / bandwidth fact. Chunk 03 §5 deferred this to post-substrate-lock; §28.3 now defers to chunk 05 (matrix assembly). Neither resolves whether compact-support is a capability contract on the function, a refinement type on the output, or a workflow-layer declaration.

  `Recommend:` Add a bullet to §28.3 noting that the surface for declaring compact support / characteristic length is open under chunk 03 / chunk 05, with the three candidate surfaces (contract, refinement, workflow) enumerated. Alternatively track in §35. Without this, Wendland's compact support has no declarative home.

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §8 (deferred open):**
  > "Whether `condition_weighted` closure policy gets resurrected with a `condition_of(expr)` intrinsic now that we're taking cost modeling seriously"

  Chunk 04 resolved this: `condition_of` ships with Levels I–III per `04_egraph_foundation_in_progress.md` §11 O2.4, and `anti_spec.md` confirms:
  > "`condition_weighted` deferred | resolved — ships via `condition_of` Levels I-III (chunk 04 O4.5)"

  §28 does not reference this resolution. The kernel-adjacent use of `condition_of` (ill-conditioned Gram matrices, e.g. near-collinear points, length-scale pathology) is a natural consumer; §30's Cholesky entry for `Matrix<_, PositiveDefinite>` is the hook.

  `Recommend:` Add a cross-reference in §28.3 or the §28 preamble to §14 `condition_of` for Gram-matrix conditioning diagnostics, and to §30 Cholesky / `solve` for the computational path. Low urgency but closes the loop.

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §8 (deferred open):**
  > "Syntax for compact support and piecewise-defined kernels"
  > "Whether separability is declared or inferred"
  > "Kernel composition (kernel of kernels) — out of scope until primary machinery is locked"

  §28.1 states "the usual operations on kernels preserve the contracts: sum ... product ... scaling ... and radial wrapping ... These closure rules are how the compiler derives kernel contracts from composition." This covers the closure-under-operations direction. It does not address:
  - compact-support declaration syntax
  - whether separability (K2 in the rewrite catalog) is user-declared or compiler-inferred
  - piecewise-defined kernels

  §32 K2 is marked OPEN and cross-references chunk 03 and §35, which is correct. §28 does not surface the declared-vs-inferred question where a reader would expect it.

  `Recommend:` Add a one-sentence forward reference in §28.1 or §28.3 to the K2 (separable decomposition) open-question cluster, naming the declared-vs-inferred axis. Chunk 04 locked K2 as "pending kernels-chunk resume" (sub-question 5 of O2.2) and §28 is the section the reader lands in.

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §7 ("Three-way optimization cut"):**
  > "1. Lossless — compiler-internal rewrites. ... 2. Lossy-as-model-claim ... 3. Lossy-as-tolerance ..."

  Chunk 04 §7 ("Relationship to the kernel report's three-tier cut") superseded this with the 2×3 framing (faithfulness × orientation). §28 does not reference either cut. The kernel rewrites (K1 compact-support truncation, K2 separability, K3 low-rank) are distributed across those 2×3 cells. §28's three-subsection structure (contracts / ambient locus / deferrals) omits the rewrite-class classification entirely, which is reasonable for a surface section, but the reader has no pointer from §28 to §32 K where the rewrites live.

  `Recommend:` Add a final paragraph or bullet to §28 cross-referencing §32 K (the kernel rewrite cluster) and §15.1 `approximate` blocks for the lossy-model / lossy-tolerance rewrite surface. Presently §28 reads as pure surface and leaves the rewrite hook implicit.

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §9 ("Notes to self"):**
  > "Riley flagged that equality in the overdetermination machinery can be strict or fuzzy, and fuzzy equality is sometimes useful — this is a feature of the unified machinery, not a bug."

  This is a live design note relevant to §28 insofar as kernel equivalences (e.g. Matérn(ν=1/2) ≡ exponential, RBF ≡ Matérn(ν=∞)) may fire as strict rewrites and other kernel approximations (Nyström ≡ full-rank within ε) as fuzzy rewrites. §28 does not document this. It is appropriately deferred to post-substrate-lock.

  `Recommend:` No immediate action. Track as an implied open for chunk 03 resume.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §8 bucket K3:**
  > "K3. Low-rank K → U·V^T (truncated SVD, Nyström, random Fourier features). Speculative — kernels report doesn't enumerate, but machinery demands they fit."

  Chunk 04 decided "Defer to v2.2: ... speculative kernel rewrites (K3 low-rank specifics)." `spec_new.md` §32 K3 is marked OPEN. §28 does not acknowledge K3 or low-rank Gram-matrix approximation even as a deferred item.

  `Recommend:` Add K3 (low-rank approximation / Nyström / random Fourier features) to §28.3 as a third deferred concern alongside sparsity and integration. Alternatively, widen §28.3's first bullet from "sparse / compact-support" to also name low-rank. The current §28.3 structure implies only two kernel-adjacent concerns defer; chunk 04 has three.

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §2 Riley note:**
  > "Riley Note: check what the deal is with properties"

  The `property` keyword is retired (`anti_spec.md`: "`property` declarations (`property sigma is PositiveDefinite`) | refinement types + capability contracts + `constraint` blocks"). §28.1 uses capability contracts, which is the replacement. The note is resolved.

  `Recommend:` No action. Note closed.

- **`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §5 ("K2 separability rule ... directly consumes matrix tensor-product factorization"):**
  > "K2 separability rule (chunk 04 Bucket 3) directly consumes matrix tensor-product factorization."

  §28.2 notes product loci `L = L_x × L_y` and the product-kernel decomposition at the function surface. It does not state that the Gram matrix of a separable kernel factors as a Kronecker product and that this factorization is the computational payoff. §30 covers linear algebra primitives but does not mention Kronecker / tensor-product matrices.

  `Recommend:` Either add a sentence to §28.2 noting that separable product kernels enable Kronecker-structured Gram matrices (tracked for chunk 05), or add a note in §30 that tensor-product / Kronecker structure is a deferred matrix refinement. Chunk 05 §3.4 does not currently list `Kronecker` in the structural-subtype lattice; this is an adjacent gap.

---

## Conflicts

Direct contradictions between spec_new.md §28 and any corpus document.

- **§28.1 `Isotropic` supertrait claim vs. `anti_spec.md` user-declared property retirement:**

  §28.1 states: "`Isotropic`. Guarantees `k(x, y) = k̂(‖x − y‖)` for some `k̂`. Supertrait `Stationary` plus rotation invariance." This is stated as a contract relationship. `anti_spec.md` retires "user-declared fn invertibility / differentiability / domain" in favor of "compiler derives from body composition + stdlib atom contracts." §28.1 echoes this: "Kernel-ness is a property of the function that the compiler verifies from body composition plus capability contracts on atoms."

  Not a direct contradiction, but the mechanism for deriving `Stationary` and `Isotropic` from a user-written kernel body is underspecified. §6 (referenced by §28.1) would need to state how rotation invariance is derived from a function body, or §28.1 would need to name stdlib atoms whose contracts carry these facts (e.g., `norm` atoms carry `Isotropic`, `-` carries `Stationary` under `Point<L>` subtraction).

  `Recommend:` Not a conflict to resolve in §28; flag for §6 / chunk 03 resume. The §28.1 text is consistent with the derivation principle but leaves the atom contracts implicit.

- **§28.3 "deferred to chunk 03" framing vs. §28 preamble "deferred to chunk 03":**

  §28 preamble: "Sparsity and integration operators are deferred to chunk 03." §28.3 heading: "Kernel Sparsity and Integration, Deferred to Chunk 03." Internal consistency. However, chunk 03 (`03_kernels_in_progress.md`) is itself in progress, and its §5 / §6 defer further to the e-graph substrate lock (chunk 04, now locked) and to the unified-machinery resumption. Chunk 05 (`05_matrices_in_progress.md` §5) additionally absorbs sparse-kernel representation. The §28 deferral "to chunk 03" is thus partially misleading: sparsity also defers to chunk 05 (B5 matrix types), and integration defers to chunk 04 + chunk 03 resume.

  `Recommend:` Tighten §28.3's "chunk 03" references to name the actual unblocking venues. Sparse Gram-matrix representation → chunk 05 (B5); integration operators → chunk 03 resumption post chunk 04 substrate lock; low-rank approximation → chunk 03 / v2.2. §28.3's first bullet already correctly names chunk 05 for sparse matrix representation, contradicting the section heading that claims chunk 03 owns it.

- **§28 preamble "internal substrate not [committed]" vs. chunk 04 locked-in-place status:**

  §28 preamble: "Chunk 03 unified-machinery thread is pending e-graph substrate lock; the surface shape below is committed, internal substrate not." Chunk 04 is no longer pending: `04_egraph_foundation_in_progress.md` §11 resolutions show O2.1, O2.3, O2.4, Y4, Z-group, and CC1–CC5 all LOCKED as of 2026-04-20. The e-graph substrate is committed per `anti_spec.md` and chunk 04.

  The preamble's "pending e-graph substrate lock" reads as stale. The kernel-specific unified-machinery thread (chunk 03 resume) is still open, but that is different from the substrate itself.

  `Recommend:` Rewrite §28 preamble's second paragraph to say: "The e-graph substrate is locked (§16, chunk 04). The kernel-specific unified-machinery resumption (rewrite rule K2 separability, K3 low-rank, integration operator semantics) is open and tracked in §35 / chunk 03." This clarifies what is and is not settled.
