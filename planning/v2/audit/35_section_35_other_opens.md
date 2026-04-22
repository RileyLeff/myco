# Audit Report — §35 Other Opens

Audited against corpus as of 2026-04-22.

---

## Absorbed

Corpus content already reflected in spec_new.md §35.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 (O4.1):**
  > "`replaces` obligation retraction. The `replaces` keyword demands deletion semantics; standard e-graph theory is monotonic. Options: In-graph with versioning and rebuild on retraction (expensive). Adjacent obligation-keyed metadata per v1 §6.2 (Layer 3). Reframe `replaces` as 'add a superseding fact, old fact stops being selected by extraction'."
  > "Three options still open. Section 12 open."

  §35 names O4.1 by label ("three candidate semantics still open") and cross-references §8.10, §10.5, §15, and §16. The three-option structure from chunk 04 is the provenance. Absorbed; cross-refs adequate.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 (O4.3):**
  > "Per-residual exposure for training emission. Overconstrained relations need to survive extraction with their original relation names so the training emission can per-residual-expose them. Standard CSE-style canonicalization would collapse them. The e-graph can hold both forms; the extraction policy must be aware. Section 12 open."

  §35 line 4873-4875 describes this accurately: "CC3 cross-cut: overconstrained relations must survive extraction with original names so training can expose per-residual loss terms; tension with strict algebraic collapse." Absorbed.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 (O4.6):**
  > "Heterogeneous `argmax` tagged handles. v2.1 novel frontier; not in v1. The e-graph needs a story for heterogeneous e-class membership (different types claimed equal under a tagged-handle framing). Section 12 open."

  §35 line 4876-4877 names "O4.6 heterogeneous `argmax` tagged handles (closure-policy extensibility for collections with tagged alternatives)." Absorbed.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 (O4.7):**
  > "Event-driven topology mutation. Events add nodes, edges, equivalences. Operationally, the e-graph's saturation must handle incremental additions without re-running from scratch. Implementation concern; not a design blocker but needs a note."

  §35 line 4877-4879 names "O4.7 event-driven topology mutation (incremental saturation strategy when events add nodes, edges, or locus structure mid-run)." Absorbed.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 (O4.8):**
  > "Spatial operator lowering. Kernels, integrals, convolutions. Deferred to the kernel thread resumption."

  §35 line 4879-4882 names "O4.8 spatial operator lowering (rewrite group P1 architectural call: e-graph rewrite versus pre-e-graph codegen; geometry chunk 01 cross-ref)." Absorbed.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §8 ("GPU-incompatibility of BigFloat and Rational"):**
  > "GPU-incompatibility of BigFloat and Rational. Hard-error on GPU target? Fall-back-to-CPU with warning? (Leaning hard-error to avoid silent performance catastrophes.)"

  §35 lines 4867-4869 name "GPU-incompatibility of BigFloat and Rational (cross-refs §26.1 numeric table, §26.3 Rational termination caveat, §31.1 backend fallback modes)." Absorbed.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §10 / §11 item 17-18 (extraction cost as tuple; `loss_of` / `condition_of` intrinsic):**
  The chunk 04 report resolves O2.4 and locks `cost_of` as a five-field struct. §35 lines 4927-4933 carry "Memory as a `cost_of` field" as a still-open question (whether `memory` is a sixth field). Consistent with the chunk 04 lock on five fields and the chunk 12 report's identification of `memory` as outside O2.4's scope. Absorbed correctly as an ongoing open.

- **`planning/v2/spec_dev_notes.md` (line 501):**
  > "Conversion-graph cost model — §0.1 paragraph (open; unit conversions + tensor reshapes + sparse/dense + subtype widening; tracked in §35, scoped chunk 05 Q7 / chunk 07 Q6)."

  §35 line 4869-4870 names "Conversion-graph cost model" as an open item with the right chunk provenance. Absorbed.

- **`planning/v2/open_questions_deprecated_use_spec_new.md` §Tier 2 Distribution sections (Tier 2, Tier 3 machinery):**
  The deprecated open-questions file names joint-declaration syntax (B2), coupling (B4), copulas, Wishart/InverseWishart/LKJ (gated on B5), and higher-order distributions through kernels as Tier 2. §35 lines 4909-4916 reproduce this framing accurately, including the "multivariate subset that admits factorization ... already ships in Tier 1" qualification. The Tier 3 section (GPs, DPs, CRP, etc.) matches the deprecated open-questions "Deferred — Revisit After More v2.1 Design Locking" section's treatment. Absorbed.

- **`planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` §7.4:**
  > "`softmax` appeared in the argmax smooth-selection example. Should it be a builtin or a user-defined function? Probably stdlib. Low priority."

  §35 lines 4884-4888 cover `softmax` and weighted-sum aggregation as open, cross-referencing §8.7 `soft_select` and noting that the ergonomic surface is not locked. Absorbed.

- **`planning/v2/v2.1_chunk_reports/11_sum_types_enums.md` "Open items":**
  > "Exact syntax. Pattern matching power. Exhaustiveness diagnostics. Generics interaction. Event-triggered variant transitions. Relation bodies on enum-typed fields. Lifted arithmetic sugar. Serialization / workflow binding. Discriminant representation."

  §35 lines 4998-5018 ("Mode B: per-instance heterogeneous contract binding") captures the chunk 11 story accurately: enum as the resolution path, compile-time vs runtime specialization, and the open items (exact syntax, event-triggered transitions, workflow binding surface, v2.1 scope). Cross-refs to chunk 11, sum types §3.10 stub, §7, §12 are consistent with chunk 11's "Relationship to other chunks" section. Absorbed.

- **`planning/v2/v2.1_chunk_reports/10_package_dependencies.md` "Open items":**
  > "Resolver algorithm. Version semantics. Feature model. Build scripts / codegen. Workspace ↔ Python interaction. Cross-spore relation visibility. Registry story. Platform / backend metadata."

  §35 lines 5020-5030 ("Package dependency story") captures the locked vocabulary (`spore`, `hypha`, `myco.toml`, `myco.lock`), the Cargo+uv direction, and the eight open sub-items in terms that match the chunk 10 list. Absorbed.

- **`planning/v2/open_questions_deprecated_use_spec_new.md` §Workflow Verb Taxonomy:**
  The deprecated file raises whether `bind_controller` is more naturally grouped with `learn_*` and whether the four-verb partition clarifies or obscures. §35 does not carry this item, but it is not a §35 gap — it was an "elegance / future work" question whose resolution is tracked under §24. Not homeless for §35; see Homeless bucket below if it belongs here.

- **`planning/v2/v2.1_chunk_reports/08_relation_fix_whoops.md` (deferred to §35):**
  > "Whether `memory` is a sixth field of `cost_of` or a backend-specific annotation. Approximation-cost composition for stacked lossy rewrites. Condition-cost representation for multi-output operations. Enumeration of the canonical stdlib atom set. Separate chunk."

  All four items appear explicitly in §35 at lines 4926-4996: "Memory as a `cost_of` field," "Approximation cost composition," "Condition cost representation for multi-output operations," and "Stdlib canonical inventory." The chunk 08 report delegated these four to §35; §35 carries them. Absorbed.

- **CC5 resolution (chunk 04 §12 CC5 and the 2026-04-22 data-path lock):**
  §35 lines 4935-4951 contains the "CC5 site-gated strict rewrites: data path resolved" narrative including the X1/X2 split, the Layer-3 site-record mechanism, and the cross-geometry pollution proof. The anti_spec.md "Retired architectural framing" table (lines 69-71) captures the X-category bundling retirement and the eight-merge-sources reframing. The chunk 04 CC5 section is the provenance. Absorbed.

- **`planning/v2/open_questions_deprecated_use_spec_new.md` §Events (all resolved entries):**
  Items marked RESOLVED in the deprecated file (generic events, cross-container events, within-event tiebreaking, solver convergence during early training, constraint enforcement, closure policy semantic interface) are not carried as opens in §35. Correct — they are locked decisions now in spec_new.md §10, §8.7, §24, etc.

---

## Superseded

Corpus content replaced by a newer decision now reflected in §35 or elsewhere in spec_new.md.

- **`planning/v2/open_questions_deprecated_use_spec_new.md` §Closure policy — `condition_weighted` deferred (lines 525-530):**
  > "`condition_weighted` deferred beyond v2.1. Conditioning-aware weighting requires either a `condition_of(expr)` compiler intrinsic (parallel to `deriv`) or a compiler-provided black box — both have real cost. Most v2.1 workflows reconcile overdetermined systems via controller plus consistency loss rather than via conditioning-aware closure. Revisit post-v2.1 if demand emerges."

  Superseded by chunk 04 §11 item 32: "`condition_weighted` un-deferred and ships in v2.1; closes O4.5." Spec_new.md §8.7 lists Y4 `condition_weighted` as a shipping closure policy. Anti_spec.md already records: "`condition_weighted` deferred | resolved — ships via `condition_of` Levels I-III (chunk 04 O4.5)." The adjudication log (batch 5, S1) marks this void. §35 does not carry `condition_weighted` as a deferred item.

  `Recommend:` No new anti_spec.md entry needed; the existing row covers it. The deprecated open-questions file is already deprecated wholesale.

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §8 — `condition_weighted` deferral:**
  > "Whether `condition_weighted` closure policy gets resurrected with a `condition_of(expr)` intrinsic now that we're taking cost modeling seriously."

  Anti_spec.md line 97 already flags: "chunk 03 §8 `condition_weighted` deferral — pre chunk-04." Superseded; §35 correctly omits this.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11, O4.2 (Pole L'Hopital / `identify`-seam as structural-predicate-gated rewrites):**
  > "O4.2 — Pole L'Hopital and `identify`-seam as structural-predicate-gated rewrites. RESOLVED (2026-04-20) via Section 12 CC5."

  §35 does not carry O4.2 as an open; CC5 is correctly shown as resolved (lines 4935-4951). Anti_spec.md "Retired architectural framing" rows for X-category bundling and "structural-predicate-gated" name supersede the old framing. No action needed.

- **`planning/v2/open_questions_deprecated_use_spec_new.md` §Literal constants in `.myco` (marked RESOLVED):**
  > "RESOLVED. CC1 (spec §4, anti_spec 'Dropped features') bans literal numerics in value position."

  §35 line 4866 names "Literal-constants diagnostic surface (CC1 enforcement messages; shape in §4.1)" as an open item — specifically about the diagnostic surface, not about the CC1 policy itself. The policy is resolved; the diagnostics surface (error message format) is still open. The §35 framing is correct in keeping the diagnostic-surface question while not re-opening the CC1 policy. No supersession conflict.

---

## Homeless

Corpus content relevant to §35 that is not accounted for in §35 and is not retired.

### H1 — O4.1 three-option semantics not named

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` lines 1242-1252:**
  > "O4.1 — Obligation retraction (`replaces`). The `replaces` keyword demands deletion semantics; standard e-graph theory is monotonic. Options: (1) In-graph with versioning and rebuild on retraction (expensive). (2) Adjacent obligation-keyed metadata per v1 §6.2 (Layer 3). (3) Reframe `replaces` as 'add a superseding fact, old fact stops being selected by extraction' (a non-retraction framing analogous to the referential-truth answer for events). The referential-truth framing (Section 5) may cover `replaces` the same way it covers ecosystem entities, but the specific semantics of `replaces` in the spec needs a pass to confirm."

  §35 mentions O4.1 and says "three candidate semantics still open" but does not name the three candidates. A reader of §35 who needs to re-engage with this open item must locate chunk 04 to find the actual option list. The parallel with the referential-truth framing for events — which is now canonically documented in §10 and §16 — is also absent. Since O4.1 is a Tier 0 Phase 4 item (per chunk 04's phasing), the option list belongs in §35 as context.

  `Recommend:` Expand the §35 O4.1 entry to name the three options: (1) in-graph versioning with rebuild on retraction (expensive, breaks monotonicity); (2) adjacent Layer-3 obligation-keyed metadata (monotonic; Layer 3 already exists per §16.1); (3) referential-truth reframe ("add a superseding fact, extraction no longer selects the old one"). Note that option (3) parallels the §10 / §16.1 event-facts framing but needs a dedicated pass to confirm equivalence.

### H2 — O4.3 tension with algebraic collapse not scoped

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` lines 1263-1267:**
  > "Per-residual exposure for training emission. Overconstrained relations need to survive extraction with their original relation names so the training emission can per-residual-expose them. Standard CSE-style canonicalization would collapse them. The e-graph can hold both forms; the extraction policy must be aware."

  §35 lines 4872-4875 correctly identify the CSE-collapse tension, but do not state that the e-graph can hold both forms simultaneously and that the resolution is an extraction-policy constraint (not a rewrite-rule ban). The resolution path — "extraction policy must be aware" — is in chunk 04 and in the §20 / §25 cross-refs on spec_new.md lines 3313-3314 and 3412-3413, but §35 does not name the extraction-policy constraint as the design locus. Without this, a designer reading §35 cold might attempt a rewrite-rule solution rather than an extraction-policy solution.

  `Recommend:` Add a sentence to the §35 O4.3 item: "Resolution locus is the extractor's selection policy (§19), not a ban on algebraic rewrites; the e-graph holds both the extracted form and the original-named form as co-members of the e-class, and the policy selects the named form when training-emission mode is active."

### H3 — Workflow verb taxonomy open not in §35

- **`planning/v2/open_questions_deprecated_use_spec_new.md` §Workflow Verb Taxonomy (lines 788-811):**
  > "The v2.1 workflow ships eight binding verbs... That grouping may or may not survive the clarified framing now that 'controller' is not a `.myco` kind and `bind_topology` supplies structure rather than values. Specifically: Is `bind_controller` actually a `bind` verb in the same sense as `bind_topology`, or is it more naturally grouped with `learn_*` as 'supply a trainable source'? Does the four-verb partition clarify or obscure the fact that each verb simply supplies a source of a particular flavor?"

  This open is not carried in §35. It is a §24 concern (workflow verbs) but the deprecated file explicitly said "Revisit the taxonomy once callable-binding, topology-binding, and prior-binding (deferred) have all been designed." The adjudication log does not record a decision on this item. It is not retired in anti_spec.md. The §24 audit (`24_section_24_eight_verbs.md`) does not appear to have landed a resolution on the grouping question. With the relation/fn lock (chunk 08) now settled and the dumb-data Python principle locked (chunk 09), the grouping question is answerable and belongs somewhere. §35 is the natural parking spot for "open design taxonomy question about the workflow layer."

  `Recommend:` Add a short open to §35: "Workflow binding verb taxonomy. The eight verbs are load-bearing; the grouping question (whether `bind_controller` belongs with `bind_topology` as 'supply structure/callable' or with `learn_*` as 'supply a trainable source') remains unresolved. Not blocking; revisit after §24 workflow verbs are fleshed out."

### H4 — Cross-backend callable interop not in §35

- **`planning/v2/open_questions_deprecated_use_spec_new.md` §Compiler Internals (Tier 2) (lines 776-784):**
  > "Cross-backend callable interop. §31.6 locks that Myco commits to no primary backend; §23.3 locks that trained callables reuse across workflows via plain contracts. What's unresolved: if workflow A trains a callable on backend X (e.g., PyTorch), can workflow B bind the same callable when running on backend Y (e.g., JAX)? Weight-format translation, gradient-plumbing compatibility, and advertised-capability reconciliation all need to be specified. The single-backend-per-run policy (§32.1) caps intra-run scope; cross-run interop is the open question."

  - **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md` line 287:**
    > "Cross-backend callable interop is v2.2+."
  - **`planning/v2/spec_dev_notes.md` line 526:**
    > "cross-backend callable interop added to Tier 2 Compiler Internals."
  - **`planning/v2/audit/adjudication.md` lines 397, 419:**
    `ACC` items flagging that §23 and §24 should note this caveat with a forward-ref to §35.

  The adjudication log records this as `ACC` (accepted, not yet written into spec_new.md). The open does not appear in §35. The chunk 06 lean is v2.2+; the spec_dev_notes say it was added to Tier 2. This is a genuine missing §35 item.

  `Recommend:` Add a §35 entry: "Cross-backend callable interop. §31.6 commits to no primary backend; §23.3 commits that trained callables reuse across workflows via plain contracts. Within a single run, the single-backend-per-run policy (§32.1) bounds scope. Across runs: if workflow A trains a callable on backend X and workflow B runs on backend Y, weight-format translation, gradient-plumbing compatibility, and advertised-capability reconciliation are all open. Lean: v2.2+. Add cross-refs in §23.3 and §32.1."

### H5 — Rational saturation termination not in §35

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §8 and §11 (numeric-specific items deferred from Section 8):**
  > "Rational saturation termination. Precision cap or canonical-form simplification policy. Specific numeric cap / heuristic undecided."

  The GPU-incompatibility of BigFloat and Rational is in §35 (line 4867-4869), but Rational saturation termination is not. Both items appear together in chunk 04's "Open — numeric-specific items deferred from Section 8" list. The §26 audit may have picked this up, but it is not in §35's current text. It affects the e-graph compiler correctness and is not a backend concern — it belongs in §35 alongside the BigFloat/Rational GPU item.

  `Recommend:` Add to the GPU-incompatibility item in §35: "Also open: Rational saturation termination — the e-graph uses Rational internally for constant folding; coprime-denominator growth can produce non-terminating saturation; a precision cap or canonical-form simplification policy is needed (§26.3, §15 equational-core)."

### H6 — Sequential inference for HMMs not in §35

- **`planning/v2/open_questions_deprecated_use_spec_new.md` "Sequential inference for time-varying discrete latents (HMMs)" (lines 853-869):**
  > "A latent discrete variable with Markov transitions over time (mode state per timestep, phenological stage, regime-switching in SDEs) requires forward-backward, Viterbi, or particle filter inference — not compile-time marginalization. The v2.1 compiler detects the pattern and errors with guidance. Full design covers: syntactic recognition of Markov-structured latent discrete chains; which inference algorithms to generate; integration with the continuous-parameter inference loop; whether PPL machinery (Pyro's `markov`, NumPyro's `contrib.funsor`) covers enough to lean on, or whether Myco emits its own forward-backward."

  This item is in the deprecated open-questions file's "Deferred — Revisit After More v2.1 Design Locking" section, and spec_dev_notes line 526 confirms it was meant to land in open_questions for later sorting ("MVN / HMM / VI backends already in existing Deferred block"). It is not in §35. It is not retired in anti_spec.md. The §13 audit does not show it as resolved. It is a genuine design surface — the compiler must recognize the pattern and error gracefully, and the full story involves forward-backward / Viterbi / particle filter algorithm selection.

  `Recommend:` Add to §35: "Sequential inference for time-varying discrete latents (HMMs). The v2.1 compiler detects Markov-structured latent discrete chains and errors with guidance; the full design (syntactic recognition, algorithm selection among forward-backward / Viterbi / particle filter, integration with continuous-parameter inference, PPL backend routing) is deferred. No chunk assigned; design not yet scoped."

### H7 — Workflow-side capability overrides not in §35

- **`planning/v2/open_questions_deprecated_use_spec_new.md` "Workflow-side capability overrides" (lines 891-906):**
  > "The no-`.myco`-annotations principle pushes all tolerance / approximation / override decisions to the workflow layer. Concrete verbs still to design: Accept large enumeration states (override the default compile-time capability error). Choose inference backend and surface capability mismatches as errors at workflow composition. Approximate-inference switches when exact methods are infeasible. Per-residual projection flavor (`hard_clip`, `sigmoid`, `soft_clip`) selection."

  This open is in the deprecated file's "Deferred" section. It is not in §35. The per-residual projection flavor sub-item is partially addressed by the constraint-enforcement resolution (now in open_questions_deprecated §Constraint enforcement strategy), but the broader capability-override API — particularly the "accept large enumeration states" and "choose inference backend" overrides — is not addressed in §35 or anywhere in spec_new.md as a named open. These are workflow-layer verbs that must be designed before §24 is normative.

  `Recommend:` Add to §35: "Workflow-side capability overrides. The no-annotations principle pushes tolerance and approximation decisions to the workflow layer. Open concrete verbs: (1) accept-large-enumeration-states override (disable compile-time capability error); (2) inference-backend selection and capability-mismatch surface; (3) approximate-inference switches for infeasible exact methods. Intended to land with §24 (workflow verbs) during Phase 1 batch 5."

---

## Conflicts

Direct contradictions between spec_new.md §35 and corpus documents.

### C1 — `condition_weighted` described as deferred in §35's summary line

- **spec_new.md §35, Summary (lines 4855-4860):**
  > "CC1 diagnostics, GPU-incompatibility of exact numeric types, chunk 04 carryovers (per-residual loss, heterogeneous `argmax`, event-driven topology, spatial operator lowering)"

  This does not mention `condition_weighted` — which is correct.

- **spec_new.md §8.7 (line 971) and Appendix C (line 5456):**
  Y4 `condition_weighted` is listed as a shipping closure policy, backed by `condition_of` Levels I-III.

- **`planning/v2/anti_spec.md` line 86:**
  > "`condition_weighted` deferred | resolved — ships via `condition_of` Levels I-III (chunk 04 O4.5)"

  No conflict exists in §35 itself on this point. The conflict (if any) is historical between open_questions_deprecated and spec_new.md, already adjudicated. Confirmed clean in §35.

### C2 — CC5 entry in §35 describes a resolved item without explicit resolution marker

- **spec_new.md §35, lines 4935-4951:**
  The CC5 block is titled "CC5 site-gated strict rewrites: data path resolved" and describes a fully resolved design (Layer-3 site records, X1/X2 split, cross-geometry pollution proof). It reads as completed prose, not as an open item.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §12 CC table:**
  > "CC5 | Pole L'Hopital / `identify`-seam gating | LOCKED — option 2 (site-scoped rewrite predicates)"

  The conflict: §35 is titled "Other Opens" (a section of open design questions), but the CC5 block describes a fully locked item with no remaining open sub-questions. It occupies six of the roughly twenty-five paragraphs in §35 without contributing an actionable open. Readers scanning §35 for open items will find a resolved design narrative where they expect an open question. This is a presentation inconsistency, not a design contradiction.

  `Recommend:` Either (a) move the CC5 block from §35 to the section of spec_new.md that owns the X1/X2 rewrite rules (Appendix C or §17), where it belongs as resolved design content, or (b) retitle the CC5 block within §35 as a resolved item note (e.g., "CC5 resolved: site-gated strict rewrites") and shorten it to a two-sentence summary with a cross-reference to Appendix C X1/X2 and §16.1. The current paragraph length is disproportionate for a resolved item in an "opens" section.

### C3 — O4.3 tension with algebraic collapse partially stated in §35 but resolution locus absent

- **spec_new.md §35 lines 4872-4875:**
  > "O4.3 per-residual training emission (CC3 cross-cut: overconstrained relations must survive extraction with original names so training can expose per-residual loss terms; tension with strict algebraic collapse; §20 rewrite group O1)."

- **spec_new.md lines 3313-3314:**
  > "the CC3 / O4.3 constraint, so training-emission diagnostics (§25) can expose per-residual loss contributions; §35 O4.3"

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` lines 1263-1267:**
  > "The e-graph can hold both forms; the extraction policy must be aware."

  The §35 entry for O4.3 names the tension but does not name the resolution locus (extraction policy at §19). The §20 cross-reference to "rewrite group O1" suggests rewrite-rule intervention, but chunk 04 is explicit that the locus is the extraction policy, not a rewrite suppression. A designer reading §35 and §20 together without chunk 04 may pursue the wrong mechanism.

  `Recommend:` Amend the §35 O4.3 sentence to add: "resolution locus is the §19 extraction policy (not a rewrite-rule ban), which must select the original-named form when training-emission mode is active."

---

**Stale-doc-only conflicts (not tabled).** `spec.md` §14.6 lists `condition_weighted` as a stdlib closure policy in its old form, predating the chunk 04 un-deferral; that is a wholesale-stale-doc item handled by the archival plan, not a §35 conflict. `v2.1_in_progress.md` lines 1015-1020 carry the `condition_weighted` deferral prose; same archival-plan handling. `open_questions_deprecated_use_spec_new.md` §Closure policy section (lines 525-530) retains the deferral language; the file is deprecated wholesale and no edits are required to it.
