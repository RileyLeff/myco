# Audit Report — §34 Chunk-Slotted Work

Audited against corpus as of 2026-04-22.

---

## Absorbed

Corpus content that already landed in spec_new.md §34.

- **`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §3.1:**
  > "Shape polymorphism direction locked (Option C — `Tensor<U, shape>` primitive with `Vector<U, n>` / `Matrix<U, m, n>` as shape-refined aliases)"

  Absorbed into §34 Chunk 05 bullet: "Matrix details (heterogeneous units, envelope flavors, subtype lattice, shape refinements, scalar reconciliation)." Shape polymorphism lock is the settled portion; the remaining four open questions in §34 match chunk 05's §7 return-path list items 1, 3, 4, 5.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` §1:**
  > "burn-style `trait Backend { type Tensor; type Distribution; fn matmul(...); fn sample(...); ... }`. Every backend-dependent op routes through the trait."

  Absorbed into §34 Chunk 06 bullet: "Backend abstraction." The stub's framing — unified trait, workflow-selects, backend-agnostic `.myco` — is the direction §34 tracks as open chunk work.

- **`planning/v2/v2.1_chunk_reports/07_type_graph_in_progress.md` §3:**
  > "Option A — Two graphs, explicit bridge / Option B — One graph, types are terms / Option C — Two graphs, type graph compiled to e-graph rules at elaboration"

  Absorbed into §34 Chunk 07 bullet verbatim: "Three-option coupling framing (A two graphs with explicit bridge / B unified term-e-graph / C type graph compiled to e-graph rules at elaboration) and Q1-Q7 tracked in `planning/v2/v2.1_chunk_reports/07_type_graph_in_progress.md`."

- **`planning/v2/v2.1_chunk_reports/08_relation_fix_whoops.md` "The lock" section:**
  > "Ban user fn. Full stop." / "Introduce parameterized relations as the user-facing reuse mechanism." (all five design choices and five sub-questions marked Locked)

  Absorbed into §34 Chunk 08 bullet: "user-`fn` ban and parameterized-relation lock (design resolved, §6 / §7 / §8 prose pending application)." The "prose pending application" language correctly tracks the surface-sweep work enumerated in chunk 08's "Surfaces the follow-up sweep must touch" section.

- **`planning/v2/v2.1_chunk_reports/11_sum_types_enums.md` "What this locks" section:**
  > "Motivation locked (four converging pressures; contracts insufficient alone). Shape locked (tagged variants, exhaustive match, static/dynamic lowering, compose with contracts). Syntax open."

  Absorbed into §34 Chunk 11 bullet: "Motivation and shape locked (§3.10 stub); exact syntax, pattern-matching power, event-triggered variant transitions, lifted-arithmetic sugar, and workflow binding surface open." The five open items in §34 Chunk 11 match chunk 11's "Open items" list exactly.

- **`planning/v2/v2.1_chunk_reports/12_cost_field_unification.md` "The divergence" section:**
  > "§14.2 `loss_of` ... §19.1 extraction cost vector ... Chunk 04 O2.4 `cost_of` ... Same word 'cost' across three surfaces, three different field sets, no cross-reference in spec prose today."

  Absorbed into §34 Chunk 12 bullet: "Three divergent field sets, no cross-reference today." The three options (a/b/c) and the load-bearing questions in the chunk 12 report are the open design work §34 defers to `12_cost_field_unification.md`.

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §34. Should move to anti_spec.md if not already there.

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §2:**
  > "kernels are not a new kind. Not a new keyword. Not a new block. They are ordinary `.myco` functions."

  Superseded by chunk 08's "Kernels as parameterized relations" lock. The old framing ("kernels are ordinary `.myco` functions") predates the user-fn ban; chunk 08 replaces it with "kernels are parameterized relations." The anti_spec.md already captures the kernel-keyword retirement but uses the interim-framing "ordinary `fn` accepting two point arguments" which is now also stale (chunk 08 bans user `fn`). The chunk 08 report's surface-sweep list explicitly notes: "`03_kernels_in_progress.md` — retire 'kernels are ordinary `.myco` functions'."

  `Recommend:` Update anti_spec.md's kernel-kind row to replace "ordinary `fn` accepting two point arguments and returning a scalar" with "ordinary parameterized relation." The current anti_spec.md entry retires the keyword but uses the pre-chunk-08 replacement framing, which is now superseded by the fn-ban. This is a small targeted edit to one table row, not a structural change to anti_spec.md.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 (recommended ordering for B1-B6):**
  > "B2 + B4 remain paired for a future chunk 07 (joint distributions / coupling)."

  The chunk 04 blockers section recommended that B2 + B4 become chunk 07 for joint-distributions/coupling. That slot was subsequently reassigned: chunk 07 became the type-graph/e-graph bridge, and B2 + B4 became chunk 08's joint-syntax sub-problem. §34 Chunk 08's description "B2 + B4 joint syntax / coupling" reflects the updated assignment. The old "future chunk 07 = joint distributions" framing in chunk 04 is superseded.

  No action needed in anti_spec.md; this is a numbering-slot reassignment internal to chunk 04's planning notes, not a spec construct. Noted here to document the discrepancy for anyone reading chunk 04 cold.

---

## Homeless

Corpus content that is relevant to §34, not accounted for in spec_new.md §34, and not already committed to anti_spec.md.

- **Chunk 09 (`planning/v2/v2.1_chunk_reports/09_workflow_data_layer.md`) is not enumerated in §34.**

  Chunk 09 is a durable locked-principle chunk: "Python does not know Myco types" (locked); node catalog, path-based addressing, bind/observe/run verb families (shape pinned, exact API open); Python value providers distinct from `.myco` distributions (locked). The chunk has substantial open items: exact node-path syntax, node catalog typing, full observe output format menu, Mode B mechanism, parameter inference/calibration API. It explicitly calls for a "follow-up sweep across §24 (workflow verbs) and §31 (Python API) once the exact syntax lands."

  §34 enumerates seven chunks but omits chunk 09. Chunk 09 is not retired in anti_spec.md; it is active in-progress work with locked principles and open details.

  `Recommend:` Add a **Chunk 09** entry to §34: "Workflow data layer. Principle locked (Python as dumb data-provenance layer; node catalog; bind / observe / run verb families); exact node-path syntax, node-catalog type representation, observe output format menu, parameter inference/calibration API, and Mode B mechanism open. Canonical reference: `planning/v2/v2.1_chunk_reports/09_workflow_data_layer.md`." Without this entry, the chunk 09 open items have no tracking home in §34's scan surface.

- **Chunk 10 (`planning/v2/v2.1_chunk_reports/10_package_dependencies.md`) is not enumerated in §34.**

  Chunk 10 is a partial-lock chunk: vocabulary (`spore`, `hypha`, `myco.toml`, `myco.lock`) locked; overall Cargo+uv approach locked; resolver algorithm, version semantics, workspace model, feature model, build scripts, registry, and workspace-Python interaction all open. The chunk explicitly states: "Not blocking for the core language lock; can land post-v2.1 if needed." Nevertheless it is live in-progress work with a concrete stub and open design items.

  §34 omits chunk 10. Chunk 10 is not retired in anti_spec.md.

  `Recommend:` Add a **Chunk 10** entry to §34: "Package dependencies. Vocabulary and overall approach locked (spore, hypha, Cargo+uv conventions adapted); resolver algorithm, version semantics, feature model, workspace-Python interaction, and registry story open. Not blocking core language lock; can land post-v2.1. Canonical reference: `planning/v2/v2.1_chunk_reports/10_package_dependencies.md`." The "not blocking" qualifier distinguishes it from chunks whose open items gate v2.1 correctness.

- **Chunk 03 resume condition in §34 is possibly stale: "Kernels, resume after substrate lock."**

  The §34 Chunk 03 entry reads "Kernels, resume after substrate lock." The stated gate is "substrate lock" — meaning chunk 04 must be locked first. However, chunk 04's own status header reads "IN PROGRESS" and its section 12 summary table shows CC1-CC5 with individual LOCKED entries but the overall chunk 04 report itself has not been closed or marked DONE. The spec_new.md sections that absorbed chunk 04 content (§15 e-graph substrate, §16 adjacent keyed state, §17 rewrite rules, §19 extraction, Appendix C) are written and audited in Phases 1-5 of the batch audit (commits through `f791ada`).

  The chunk 04 report's own §12 "What is settled vs what is still open" lists: CC1 LOCKED, CC2 ELIMINATED, CC3 TRACKED (open as O4.3), CC4 LOCKED, CC5 LOCKED. The open items O4.1, O4.3, O4.6 are explicitly still open. These open items do not block kernel chunk resumption — they concern training emission, obligation retraction, and heterogeneous argmax, none of which are part of kernel design.

  The kernel chunk's own return path (chunk 03 §7): "1. Draft the v2.1 commitment to e-graphs internally, restoring the v1 commitment and stating the residual-graph / e-graph relationship cleanly." That commitment text now exists in spec_new.md §15-§17, and corresponding audit batches have landed. The substrate commitment that chunk 03 was waiting for appears to be present in spec_new.md; what remains open in chunk 04 (O4.1, O4.3, O4.6) is not a kernel prerequisite.

  `Recommend:` Update §34 Chunk 03 entry to reflect that the substrate commitment language is now present in spec_new.md (§15-§17) and that the gate "substrate lock" is satisfied for chunk 03's purposes. The revised entry might read: "Kernels. Substrate commitment now in spec_new.md §15-§17; chunk 03 can resume. Primary open items: unified-machinery design (cost model, rewrite-rule declaration surface, tolerance plumbing per chunk 03 §7), sparsity/characteristic-length, integration semantics." This removes the stale gate condition and replaces it with the actual open design work. Do not retroactively mark chunk 04 as "locked" — its report is still IN PROGRESS and the open O4.x items are genuine. The gate phrasing "after substrate lock" applies narrowly to chunk 03's dependency on the e-graph commitment, not on chunk 04's own open items.

- **Chunk 03's dependency on chunk 05 is not stated in §34.**

  Chunk 03 §7 step 3 explicitly includes: "Gram matrices and low-rank kernel approximations (K3: SVD, Nyström, random Fourier features) require matrix decompositions." Chunk 05 §5 states: "Kernels (chunk 03). Gram matrices, low-rank approximations (K3). Kernel function `K(a, b)` remains scalar; `gram(K, points)` assembles a `Symmetric<U, n, n>` or `PosSemiDef<U, n, n>`. K2 separability rule (chunk 04 Bucket 3) directly consumes matrix tensor-product factorization." The K2 and K3 kernel rewrites in chunk 04's Bucket 3 are explicitly gated on chunk 03 returning. The rewrite catalog says "K2 — Gated on Kernels chunk report" and "N1 — Gated on Kernels chunk §6."

  §34 does not state that chunk 03's K3 sub-work depends on chunk 05 (Gram matrices require matrix types). For most of chunk 03's scope (unified machinery design, integration semantics, compact-support kernels) this dependency is absent. But the K2/K3/N1 rewrite rules remain gated on both chunks, and a reader of §34 has no signal that chunk 03 and chunk 05 have an intersecting scope.

  `Recommend:` Add a note to the §34 Chunk 03 entry: "K2 separability and K3 low-rank-approximation sub-topics depend on chunk 05 matrix primitives (Gram-matrix assembly, SVD). Primary unified-machinery design does not."

- **Chunk 07 dependency on chunk 04 is described as ongoing, but chunk 04's relevant decisions are already present in spec_new.md.**

  §34 Chunk 07 states: "Depends on chunks 04 (expression e-graph substrate), 05 (refinement-lattice examples from matrix types), and 06 (backend-dependent conversion-edge costs)." For chunk 04 specifically, the dependency is on the expression e-graph substrate. The relevant decisions — three-layer split, merge sources, rewrite taxonomy — are now in spec_new.md §15-§17. The chunk 04 open items that remain (O4.1 obligation retraction, O4.3 per-residual emission, O4.6 heterogeneous argmax) are not prerequisites for chunk 07's type-graph/e-graph bridge design.

  This is a precision gap in the §34 dependency statement: "Depends on chunk 04" is true but overly broad, implying chunk 07 cannot start until chunk 04 is fully closed, when in fact only the substrate commitment portion is needed and that portion is in spec_new.md §15-§17.

  `Recommend:` Refine the §34 Chunk 07 dependency clause to: "Depends on the expression e-graph substrate commitment (now in spec_new.md §15-§17; chunk 04 open O4.x items do not block), chunk 05 (refinement-lattice examples from matrix types), and chunk 06 (backend-dependent conversion-edge costs)." This allows chunk 07 to start without waiting for chunk 04's remaining O4.x items to close.

- **Chunk 12 depends on chunk 04's `cost_of` named-field lock (O2.4), which is RESOLVED.**

  Chunk 12 report's "The divergence" section states: "Chunk 04 O2.4 locks `cost_of(expr)` as `{compute, approximation, condition, truncation, discretization}`." O2.4 is marked RESOLVED (2026-04-20) in chunk 04. §34 Chunk 12 does not mention this dependency at all — it states the unification problem but gives no signal that one of the three divergent inventories is already locked by a resolved decision.

  This matters because option (a) in chunk 12 ("One unified struct with O2.4's five fields") is constrained by a locked decision, while options (b) and (c) accommodate O2.4 as-is. A reader of §34 who does not also read chunk 12 and chunk 04 has no indication that O2.4's field list is a committed input to the unification decision.

  `Recommend:` Add a sentence to §34 Chunk 12: "Chunk 04 O2.4's five-field `cost_of` struct (`compute, approximation, condition, truncation, discretization`) is a locked input to the unification; option (a) in the chunk 12 report would absorb `loss_of` and the §19.1 vector into this struct."

---

## Conflicts

Direct contradictions between spec_new.md §34 and any corpus document.

- **§34 summary line omits chunk 12 from its enumeration.**

  §34's **Summary** paragraph reads: "Outstanding design chunks: chunk 05 matrix details, chunk 06 backend abstraction, chunk 07 type-graph to e-graph bridge, chunk 08 joint syntax and coupling, chunk 03 kernels (resumes after substrate lock), chunk 11 sum types / enums." Chunk 12 is listed in the body bullets but not in the summary. The summary names six chunks; the body has seven. This is a straightforward internal inconsistency in §34 itself.

  `Recommend:` Add "chunk 12 cost-field unification" to the §34 Summary sentence. The summary should enumerate all seven body bullets. No corpus material is needed; this is a within-§34 editorial gap.

- **§34 Chunk 08 label conflict: "B2 + B4 joint syntax / coupling" vs. chunk 08's actual scope.**

  §34 labels chunk 08 as "B2 + B4 joint syntax / coupling; user-`fn` ban and parameterized-relation lock." The chunk 08 report (`08_relation_fix_whoops.md`) is entirely about the user-fn ban and parameterized-relation lock — it contains no joint-distribution syntax design. The B2 + B4 joint-distribution content was the *prior planned scope* for what was originally "chunk 07" before the type-graph chunk displaced it. After the renumbering (chunk 07 became type-graph; B2+B4 became chunk 08), chunk 08 was written as the fn-vs-relation fix, not as the joint-distribution syntax design.

  The chunk 08 report's status section says: "Deferred: Y5 closure-policy extensibility — follow-up chunk; Distribution contract shape (`log_pdf` / `sample`) — follow-up chunk." Neither of these deferred items is B2 (joint declaration syntax) or B4 (coupling machinery). B2 and B4 remain open in §33's blocker list. The chunk 08 report does not claim to address them.

  So §34 Chunk 08's "B2 + B4 joint syntax / coupling" label is a stale carry-over from the pre-renumbering plan that assigned B2+B4 to the slot now occupied by chunk 08. The actual chunk 08 content is the fn-ban / parameterized-relation lock exclusively.

  `Recommend:` Remove "B2 + B4 joint syntax / coupling;" from the §34 Chunk 08 label. The accurate description is: "Chunk 08. User-`fn` ban and parameterized-relation lock (design resolved, §6 / §7 / §8 prose pending application). Canonical reference: `planning/v2/v2.1_chunk_reports/08_relation_fix_whoops.md`." B2 and B4 remain open blockers in §33, not assigned to any current chunk report. If a future chunk is written for B2+B4, §34 should receive a new chunk 13 (or similar) entry at that time.
