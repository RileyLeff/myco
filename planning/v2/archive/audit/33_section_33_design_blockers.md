# Audit Report — §33 Design Blockers

Audited against corpus as of 2026-04-22.

---

## Absorbed

Corpus content that already landed in spec_new.md §33.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 "Design blockers B1-B5":**
  > "B1 — Opaque `log_pdf` stdlib policy. Does stdlib permit distribution families whose `log_pdf` is a structurally-opaque numerical evaluator (inverse-FFT of characteristic function, table-lookup, etc.) rather than a symbolic expression?"
  > "B2 — Joint distribution declaration syntax."
  > "B4 — Coupling machinery (non-independence in envelope)."
  > "B5 — Matrix / tensor types and linear-algebra primitives."
  > "B6 — Backend abstraction."

  All five B-blocker labels and their one-line characterizations are absorbed into §33's bullet list (lines 4814-4818).

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 settled items 34 and 36 (design-blocker enumeration and B3 absorption):**
  > "Design blockers enumerated and split (2026-04-20): B1 opaque log_pdf policy, B2 joint declaration syntax, B4 coupling machinery, B5 matrix types, B6 backend abstraction (absorbs former B3); each must resolve before corresponding deferred families ship."

  The five-blocker structure (with B3 absorbed into B6) is the §33 list structure. Absorbed.

- **`planning/v2/spec_dev_notes.md` (2026-04-21 renumbering entry):**
  > "Part VI §32 Design Blockers → §33."

  The renumbering was applied correctly; §33 now carries the design-blockers section.

---

## Superseded

Corpus content replaced by a decision now reflected in spec_new.md §33. Should move to anti_spec.md if not already there.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11, B3 entry:**
  > "B3 — Tier C PPL backend protocol. ABSORBED (2026-04-20) into B6 / chunk 06 backend abstraction."

  The old five-item list B1-B5 (where B3 was the PPL backend protocol) is superseded by the current six-item conception B1-B6 with B3 absorbed and B6 added. §33 correctly omits B3 from the live list, listing only B1, B2, B4, B5, and B6. The absorption is recorded in chunk 04 and in spec_dev_notes.md.

  `Recommend:` The anti_spec.md "Retired open questions" table does not yet carry a row for "B3 as standalone blocker (PPL backend protocol)." It should. The current anti_spec.md row for the MVN deferral notes the B5 gate but does not mention B3. Add a row: "B3 as PPL-protocol-only blocker | absorbed into B6 (backend abstraction chunk 06); B3 label retired | chunk 04 §11 2026-04-20 split."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11, "Recommended ordering for B1-B6":**
  > "B3 absorbed into B6 (backend abstraction chunk 06); B2 + B4 remain paired for a future chunk 07 (joint distributions / coupling). Dependencies suggest: 1. B5 (matrix types)... 2. B6 (backend abstraction) in parallel with B5... 3. B1 (opaque log_pdf policy) in parallel with B5 and B6... 4. B2 + B4 (future chunk 07) after B5 and B6."

  The dependency ordering note is design-process reasoning, not spec content. §33 correctly omits it. Superseded as spec content; it belongs in spec_dev_notes or the chunk report only.

---

## Homeless

Corpus content relevant to §33 that is not accounted for in spec_new.md §33 and is not already retired. This is the highest-value bucket.

### B1 — α-stable driving case and three design sub-questions

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:874-889` B1 detail:**
  > "Driving family: general α-stable (only characteristic function is closed; PDF requires numerical inverse-FFT)."
  > "Design items: (a) should stdlib ship these at all, or route through user-declared `approximate` blocks? (b) if yes, what autodiff infrastructure is required? (c) how do they interact with Tier C PPL backend routing?"
  > "Consequences: layer-3 `condition_of` can't analyze through opaque `log_pdf`; gradient-based inference (HMC, NUTS, VI) requires autodiff through the numerical evaluator."

  §33's B1 bullet (line 4814) reads only "Opaque log_pdf stdlib policy." The driving family, the three design items (stdlib admissibility, AD infrastructure, Tier C routing interaction), and the condition-analysis consequence are all named in chunk 04 but absent from §33. The §27 audit (`27_section_27_distribution_families.md:132-138`) already flagged this as a §33 gap.

  `Recommend:` Expand the §33 B1 entry to name: (i) the driving case (general α-stable, closed-form characteristic function only, PDF requires numerical inverse-FFT); (ii) the three design sub-questions: whether stdlib ships opaque families at all or routes them through `approximate` blocks, what autodiff infrastructure an opaque `log_pdf` requires, and how opaque families interact with Tier C PPL backend routing. Without this detail, un-deferring B1 requires re-reconstructing context from chunk 04.

### B2 and B4 — joint-syntax scope and coupling dependency chain

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:891-934` B2 and B4 detail:**
  > "Three candidate shapes surveyed (`couple(x, y) via ...`, `(x, y) ~ JointCopula(...)`, explicit `u ~ Uniform; x = inv_cdf(F_x, u)`)."
  > "B4 status: open. Will land in a future chunk 07 paired with B2. Blocked on B2 (syntax determines what coupling looks like in the graph), B5 (MVN-case needs matrix types), and B6 (Tier C handoff protocol)."

- **`planning/v2/v2.1_chunk_reports/08_relation_fix_whoops.md` (deferred item on distribution contract shape):**
  > "Distribution contract shape. Still open. Are `log_pdf` / `sample` stdlib-only callable exceptions or relation-shaped obligations? Needs to land before §13/§27 do. Deferred to a follow-up chunk on probabilistic programming."

  §33's B2 and B4 bullets (lines 4815-4816) each carry a single phrase. The three candidate syntax shapes surveyed for B2, B4's explicit blocked-on list (B2, B5, B6 in dependency order), and the distribution-contract-shape question from chunk 08 (which must resolve before B2/B4 joint syntax can land) are all absent from §33.

  `Recommend:` Expand §33 B2 to note the three candidate syntax shapes (coupled `~` expression, `JointCopula` application, explicit latent-uniform decomposition) so the next design session has a starting point. Expand §33 B4 to note its dependency chain: B4 is blocked on B2 (syntax first), B5 (MVN-coupling sub-case), and B6 (Tier C handoff protocol). Also note that the distribution-contract shape question (are `log_pdf`/`sample` relation-shaped obligations or stdlib-callable exceptions?) must resolve before B2 syntax can be locked; that question is tracked in chunk 08's deferred list but not cross-referenced from §33.

### B5 — heterogeneous-unit sub-questions and open-question list

- **`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §3.2 and §8 consolidated open questions:**
  > "Heterogeneous-unit question — `Tensor<U, shape>` only, `LinearMap<From, To>` only, or two-track? (§3.2; primary blocker)"
  > "Shape refinement language — how much shape arithmetic ships in v2.1? (§3.6)"
  > "Envelope flavors and their per-op propagation rules? (§3.3)"
  > "Structural subtype lattice — which ship in stdlib, how declared, what composition rules? (§3.4)"
  > "Scalar reconciliation — redefine `Scalar<U> := Tensor<U, ()>` or keep distinct with implicit conversion? (§3.1 / §3.6)"

  §33's B5 entry (line 4817) reads "Matrix heterogeneous-unit resolution." This names only the primary question from chunk 05. Chunk 05 §7 ("Return path") lists seven dependent sub-questions (§3.1 through §3.7) in priority order: heterogeneous-unit choice gates all others; shape refinements are prerequisite for structural subtypes; envelope flavors gate Level III `condition_of`; `convert` scope and scalar reconciliation follow. None of these sub-questions or their dependency ordering appear in §33.

  `Recommend:` Expand §33 B5 to enumerate the primary and subsidiary open items from chunk 05 in dependency order: (i) heterogeneous-unit resolution (`Tensor<U, shape>` only vs two-track with `LinearMap<From, To>`) — gates all others; (ii) shape refinement language (how much shape arithmetic ships in v2.1) — prerequisite for structural subtypes; (iii) envelope flavors (entry-wise, operator-norm, spectral, structural-bound-as-fact) and per-op propagation rules — gates Level III `condition_of`; (iv) structural subtype lattice (which ship in stdlib, declaration syntax, composition); (v) scalar reconciliation (`Scalar<U>` redefined as `Tensor<U, ()>` or kept distinct). Items iii-v are parallelizable once i and ii close.

### B6 — AD-ownership fork and PPL protocol detail

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §4.3 and §8 open questions:**
  > "AD ownership — the central fork: Option A (Myco owns AD), Option B (delegate to backend AD), Option C (hybrid). Lean: Option C. But this is a real fork that deserves explicit decision."
  > "Q1. AD ownership: Myco / delegate / hybrid?"
  > "Q2. Minimum backend trait API vs. capability-advertised optional?"
  > "Q3. Default fallback policy: error / host / emulate?"
  > "Q4. PPL backend protocol concrete form?"
  > "Q5. Opaque callable gradient-flow semantics?"

  §33's B6 entry (line 4818) reads "Backend abstraction (see Part V)." The cross-reference to Part V is correct (§31 in spec_new.md is the backend stub), but the central design fork — AD ownership (Myco-owned vs backend-delegated vs hybrid) — does not appear in §33 or in §31. The seven open questions from chunk 06 §8 are tracked only in that chunk report.

  `Recommend:` Expand §33 B6 to name the central fork (AD ownership: Myco-owned symbolic AD vs backend-delegated AD vs hybrid per chunk 06 §4.3; lean is hybrid) and the PPL backend protocol (what the compiler emits to a Tier C backend, what the backend returns). These are the two highest-stakes unknowns in B6; naming them in §33 surfaces the dependency for anyone planning B2/B4 work (which cannot close until B6's PPL protocol is locked).

### B3 missing from §33 — status confirmed

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:907-915` B3 absorption record:**
  > "B3 — Tier C PPL backend protocol. ABSORBED (2026-04-20) into B6 / chunk 06 backend abstraction."
- **`planning/v2/spec_dev_notes.md:533`:**
  > "§27 should note B3 was absorbed into B6 (one-liner; reviewers noticed the missing B3) — 2026-04-21 dropped. Per feedback-memory rule, history breadcrumbs do not belong in spec prose; reviewers looking for B3 can check dev_notes."

  B3 is absent from §33 by design: it was absorbed into B6 in the 2026-04-20 design-blocker split. The absence is not an errata. The dev_notes entry explains that the "B3 → B6" breadcrumb was deliberately dropped from spec prose per the no-history-in-spec rule. No action on §33 is needed for B3's absence.

  `Recommend:` The explanation belongs in this audit file rather than requiring future readers to re-trace the absorption. Anti_spec.md should carry the B3-absorbed row (see Superseded section above).

---

## Conflicts

Direct contradictions between spec_new.md §33 and corpus documents.

- **§33 B2 chunk assignment vs. corpus:**

  §33 does not name a chunk for B2. §34 (lines 4837-4839) assigns B2 + B4 to chunk 08:
  > "Chunk 08. B2 + B4 joint syntax / coupling; user-fn ban and parameterized-relation lock (design resolved, §6/§7/§8 prose pending application)."

  But chunk 04 §11 "Recommended ordering" (lines 979-1004) assigns B2 + B4 to "a future chunk 07":
  > "B2 + B4 (future chunk 07, joint distributions / coupling) after B5 and B6."

  The chunk-08 assignment in spec_new.md §34 is more recent and appears to reflect a renaming decision (chunk 07 became chunk 08 when the type-graph work was promoted to chunk 07). However, the chunk 04 report retains the "chunk 07" label and has not been updated to reflect the reassignment. The corpus is internally inconsistent on this point.

  `Recommend:` Confirm which chunk owns B2 + B4. If the answer is chunk 08 (as §34 states), update chunk 04 §11 "Recommended ordering" to say chunk 08. If the assignment shifted because chunk 07 now owns type-graph-to-e-graph bridge work (per §34), document the renaming in spec_dev_notes.md so the next design session does not re-litigate it.

- **§33 B4 dependency claim implicit in spec vs. corpus explicit ordering:**

  §33 lists B2 and B4 as parallel independent entries. Chunk 04 §11 (line 931) states explicitly: "Will land in a future chunk 07 paired with B2. Blocked on B2 (syntax determines what coupling looks like in the graph)." B4 is not independent of B2; it is blocked on B2. Listing them as co-equal bullets in §33 understates the dependency and could cause a designer to attempt B4 work before B2 syntax is locked.

  `Recommend:` Add a parenthetical to the §33 B4 bullet noting that B4 is blocked on B2 closing first. The parallel-bullet presentation in §33 should not imply parallel executability.
