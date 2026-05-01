# Audit: §15 Approximate Blocks

Corpus examined:
- `planning/soul.md`
- `planning/v2/spec.md`
- `planning/v2/spec_dev_notes.md`
- `planning/v2/riley_project_note.md`
- `planning/v2/anti_spec.md`
- `planning/v2/v2.1_in_progress.md`
- `planning/v2/open_questions.md`
- `planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md`
- `planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
- `planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md`

---

## Absorbed

Content that has landed in spec_new.md §15.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §9 — `approximate` block fields.**
  > "`approximate <expr_A> <op> <expr_B>`: `under:` ... `tolerance_class:` ... `error_bound:` ... `body:` ... `where:`"
  All five fields appear in spec_new.md §15.1, with descriptions matching chunk 04's semantics.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §9 — No global `approximate` scope.**
  > "No fuzzy-equality operator ... Inline fuzzy equality hides *why* the approximation is justified"
  Absorbed into spec_new.md §15.1: "No global `approximate` scope exists; approximation is always explicitly chosen."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §9 — Block nesting.**
  The chunk's description of scoped composition is absorbed into §15.1: "Blocks compose by nesting. Outside a block's `body`, the authorized rewrite does not fire."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §10 — Four-source lossiness derivation.**
  > "Auto-derived lossiness ... Symbolic residual analysis ... Interval arithmetic propagation ... Condition number estimation ... Declared `error_bound:`"
  Absorbed as the four sources in spec_new.md §15.2: stdlib atom contracts, approximation-block declarations, numeric type choices, backend emulation paths. (The compiler-derivation stack is partially reframed but the substance is the same commitment.)

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11, O2.3 — `approximate` tier ships in v2.1.**
  > "O2.3 — Baseline `approximate` tier. RESOLVED (2026-04-20): SHIP in v2.1."
  Absorbed: §15 exists as a committed section, not a stub.

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §7 — Three-tier optimization cut.**
  > "Three-way optimization cut proposed: 1. Lossless ... 2. Lossy-as-model-claim ... 3. Lossy-as-tolerance"
  Absorbed into spec_new.md §15.3's three-tier lossiness grouping (lossless / lossy-model / lossy-tolerance).

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §7 — Faithfulness axis.**
  > "Lossless ... Lossy-as-model-claim ... Lossy-as-tolerance"
  These three faithfulness tiers are the direct predecessors of §15.3's three-tier cut.

- **`planning/v2/spec_dev_notes.md` §15 stub record.**
  > "15.1 block syntax (`under` / `tolerance_class` / `error_bound` / `body` / `where`), 15.2 four-source lossiness derivation ... 15.3 three-tier cut (lossless / lossy-model / lossy-tolerance)."
  The spec_dev_notes record of what §15 should contain matches the written subsections exactly.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §10, O2.4 — `approximate` block stays as a user surface complementing auto-derivation.**
  > "The `approximate` surface and auto-derivation are not redundant; they are complementary."
  Absorbed into §15.2's framing: four sources are independent contributions joined by a lattice join, with explicit scoping to `approximate` block declarations as one source.

---

## Superseded

Content that has been replaced by a newer decision in spec_new.md §15.

- **`planning/v2/spec.md` §8.1 — Four-class invertibility metadata.**
  > "`bijective` / `injective_restricted` / `lossy` / `opaque`"
  This was the old surface for declaring operation lossiness. Superseded by capability contracts on stdlib atoms (spec_new.md §6 / §7) plus the `approximate` block for user-declared approximations (§15.1). Already listed in `planning/v2/anti_spec.md`: "four-class invertibility metadata ... capability contracts on fns."

- **`planning/v2/spec.md` §14.6 — `condition_weighted` in the closure-policy stdlib list.**
  > "`condition_weighted`: weights paths by numerical conditioning (section 8.5)."
  Superseded: `condition_weighted` was deferred in anti_spec.md and open_questions.md, then un-deferred via `condition_of` Levels I-III (chunk 04 O4.5). The superseding mechanism is §14.1 `condition_of` intrinsic + §8.7 Y4 closure policy, not a simple stdlib list entry. Already handled in anti_spec.md (the open_questions.md stale-items entry reads "chunk 03 §8 `condition_weighted` deferral — pre chunk-04").

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §9 — `approximate` as expression-level inline syntax.**
  > "`approximate <expr_A> <op> <expr_B>: under: ...`" (the expr-infix form)
  The chunk 04 draft uses an expression-to-expression inline syntax (`approximate A <-> B`). Spec_new.md §15.1 re-casts this as a block construct with a `body:` field that scopes the authorization. The block form is a deliberate change: it separates "which rewrite is authorized" from "where it applies." Not yet in anti_spec.md. Recommend: add a note to anti_spec.md that the expr-infix surface was considered and replaced by the `body:`-scoped block form.

---

## Homeless

Content relevant to §15 that is not accounted for in spec_new.md §15 and is not in anti_spec.md.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §7 — Orientation axis (bidirectional vs. unidirectional) as orthogonal to faithfulness.**
  > "The kernel report §7 proposed a three-tier cut (lossless / lossy-as-model / lossy-as-tolerance). That is one axis of two; orientation is the orthogonal axis the kernel report did not call out. The 2x3 framing is a superset."
  Spec_new.md §15's top-level summary claims a "2x2 matrix of approximation flavors: (lossy-model vs lossy-tolerance) x (univariate vs bivariate)." This is a different 2x2 than the chunk 04 framing, and neither the orientation axis (bidirectional/unidirectional) nor the "univariate vs bivariate" dimension is developed anywhere in §15.1, §15.2, or §15.3. The summary sentence is a placeholder that was never filled.
  Recommend: the stable decision from chunk 04 is the 2x3 matrix (faithfulness x orientation), not 2x2. Either fill §15 with the 2x3 content or replace the summary's 2x2 placeholder sentence with a forward reference to §17.6 (the rewrite-group catalog), where orientation is handled. The "univariate vs bivariate" framing is not defined anywhere and should be removed or defined.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §10, O2.4 — Envelope-narrowing makes a normally-lossy rewrite lossless in context.**
  > "A normally-lossy rewrite can become lossless in context when envelope metadata (Layer 2) proves the value is in a regime where the loss vanishes. Example: `Float64 -> Float32` is normally lossy-tolerance; if the e-class envelope proves `value ∈ [0, 1]`, the conversion is lossless in that context and fires without consuming the tolerance budget."
  This contextual-lossiness rule is a stable settled decision (chunk 04 §11 entry 12: "Envelope-narrowing affects contextual lossiness"). It does not appear anywhere in spec_new.md §15. The section describes four sources of lossiness but does not mention that the derived lossiness can be context-sensitive (i.e., envelope-narrowed).
  Recommend: add a short paragraph to §15.2 or §15.3 stating that envelope metadata (§16) can narrow a normally-lossy rewrite to lossless in context, and that the compiler uses this before consuming the tolerance budget.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §10, O2.4 — Declaration/derivation interaction semantics (three cases).**
  > "When both exist for the same expression, the compiler checks consistency: (a) Compiler proves exact — promote to Tier A silently ... (b) Compiler proves bound within user declaration — honor user's declaration ... (c) Compiler proves declaration violated — hard error."
  This is a stable, locked decision (O2.4, explicitly resolved 2026-04-20). It is entirely absent from spec_new.md §15. The section describes auto-derived lossiness as a lattice join but does not state what happens when a user `approximate` declaration and the compiler's derived bound disagree.
  Recommend: add a §15.2 or new §15.4 subsection covering declaration/derivation interaction with the three cases. This is load-bearing: without it, the section provides no answer to "what if I declare a bound the compiler can disprove?"

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §10, O2.4 — Multi-dimensional loss cost vector with named fields.**
  > "Loss is multi-dimensional ... `cost = { compute:, approximation:, condition:, truncation:, discretization: }` ... `loss_of(expr)` returns a struct of named fields, not a scalar."
  This settled decision (O2.4 part 5) connects directly to §15's auto-derived lossiness, since lossiness feeds the cost tuple that extraction uses. Spec_new.md §15.2 says "the compiler reports the aggregate lossiness per expression via inspection surfaces (§22)" but does not name the cost dimensions or the `loss_of` struct fields. The `loss_of` intrinsic is described in §14.2 but its connection to §15's lossiness derivation is not stated in §15 itself.
  Recommend: add a cross-reference in §15.2 to §14.2 `loss_of`, noting that the four lossiness sources feed into the named cost-field struct, not a single scalar.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §9 — Exactly one of `under` or `tolerance_class` is required.**
  > "exactly one of under / tolerance_class"
  The chunk 04 syntax table explicitly requires exactly one of these two fields per block. Spec_new.md §15.1 presents both fields in the block template and describes each, but does not state that they are mutually exclusive and that exactly one is required. A user reading §15.1 alone would not know whether both, neither, or exactly one is valid.
  Recommend: add a one-sentence rule to §15.1 stating that `under` and `tolerance_class` are mutually exclusive and exactly one is required. This is a stable syntactic constraint from the locked chunk 04 design.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §10, O2.4 — Sampling parameters are workflow-side.**
  > "Non-determinism, seed management, and sample budget are workflow concerns, not world claims — consistent with CC1 ... `run.config.loss_estimation = { 'sampling': { 'n_samples': ..., 'seed': ..., 'strategy': ... }, ... }`"
  The auto-derived lossiness stack includes a runtime sampling layer (layer 4). The workflow-side configuration surface for this layer is locked but absent from §15. This is a boundary specification (who controls sampling policy), which is the kind of stable decision §15 should carry.
  Recommend: add a sentence to §15.2 noting that sampling parameters for the runtime lossiness estimation layer are workflow-side configuration (`run.config.loss_estimation`), not `.myco` declarations, consistent with CC1. Cross-reference §24 workflow verbs.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` O2.3 — Named Tier B rewrites that fire under `approximate` blocks.**
  > "Unblocked rewrites: K1 (kernel compact-support truncation), M1 (first-order Taylor), M2 (high-order term drop), Z8 (delta method), Z9 (Fenton-Wilkinson)."
  Spec_new.md §15.1 names Delta method, Fenton-Wilkinson, CLT, and named smoothings as examples in the `under` field description, but does not link to the rewrite-group catalog in Appendix C. The connection between the `approximate` block's `under` field and the rewrite-group identifiers (e.g., Z8, Z9) is absent.
  Recommend: add a cross-reference from §15.1's `under` field description to Appendix C (rewrite-rule catalog), noting that `under` takes a rewrite-group identifier from that catalog. This is a usability-critical lookup path, not a new design decision.

---

## Conflicts

Direct contradictions between spec_new.md §15 and corpus documents.

- **spec_new.md §15 summary — "2x2 matrix" vs. chunk 04 §7 — "2x3 matrix".**
  spec_new.md §15 states:
  > "The 2x2 matrix of approximation flavors: (lossy-model vs lossy-tolerance) x (univariate vs bivariate). Syntax, semantics, envelope consequences."
  Chunk 04 §7 states:
  > "That is one axis of two; orientation is the orthogonal axis the kernel report did not call out. The 2x3 framing is a superset."
  and presents a 2x3 table with axes (Faithfulness: lossless / lossy-model / lossy-tol) x (Orientation: bi / uni). The spec_new.md summary uses a different 2x2 with different axes (lossy-model / lossy-tolerance) x (univariate / bivariate). The "univariate / bivariate" axis is undefined in any corpus document; the "bidirectional / unidirectional" orientation axis is the locked chunk 04 decision. These are mutually inconsistent framings of the same design space.
  Recommend: replace the summary's 2x2 sentence with either (a) the correct 2x3 framing from chunk 04 (faithfulness x orientation), or (b) a stub forward reference to §17.6 that defers the matrix exposition. Remove "univariate vs bivariate" until it has a defined meaning.

- **spec_new.md §15.2 — Four auto-derivation sources vs. chunk 04 §10 — Five auto-derivation layers.**
  spec_new.md §15.2 lists four lossiness sources:
  > "1. Stdlib atom contracts. 2. Approximation-block declarations. 3. Numeric type choices. 4. Backend emulation paths."
  Chunk 04 §10 describes five stacked derivation methods:
  > "1. Symbolic residual analysis. 2. Interval arithmetic propagation. 3. Condition number estimation. 4. Declared `error_bound:`. 5. Sampling."
  These are not the same list. Chunk 04's five items are compiler-derivation *methods* (how the compiler proves bounds). Spec_new.md §15.2's four items are *sources* that inject lossiness into an expression. These are orthogonal framings of the same system. The spec_new.md §15.2 list omits the derivation-method dimension entirely (i.e., it says lossiness enters from four doors but does not say how the compiler quantifies what comes through each door).
  Recommend: clarify that §15.2 describes lossiness *sources* (where it comes from), while the derivation *stack* (how the compiler quantifies each source's contribution) is described in §14 or a new §15.4. The two framings are complementary, but presenting one without the other leaves the section incomplete for the reader who asks "how tight is the compiler's bound?"
