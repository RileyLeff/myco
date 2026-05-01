# Audit Report — §19 Residual Graph (Projection)

Audited against corpus as of 2026-04-21.

---

## Absorbed

Corpus content that already landed in spec_new.md §19.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §2:**
  > "The residual graph is a user-facing projection of the internal e-graph, not a separate thing."

  Absorbed into §19 preamble and §19.2: "The residual graph is a user-facing diagnostic view projected from the e-graph."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §10 ("Extraction cost becomes a tuple"):**
  > `cost = (compute_cost, approximation_loss, ...)`
  > "Extraction policy combines the tuple per workflow preferences: `compute_weight`, `loss_weight`, `loss_cap`."

  Absorbed into §19.1 as the multi-dimensional cost vector with precision, latency, memory, and approximation class dimensions.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 (O2.4, named-field generalization):**
  > "Loss is multi-dimensional. Extraction cost is not a scalar. Named dimensions: compute, approximation, condition, truncation, discretization."
  > "Aggregation to a scalar for ranking happens only at the final extraction step under workflow-configured weights."

  Absorbed into §19.1: "No default scalar weighting — the compiler does not assume one dimension dominates" and the Pareto-front / workflow-point-selection design.

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §7 ("Three-way optimization cut"):**
  > "1. Lossless ... 2. Lossy-as-model-claim ... 3. Lossy-as-tolerance ..."

  Absorbed into §19.1's "approximation class" cost dimension and the connection to `approximate` blocks (§15.1).

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §7 (faithfulness × orientation axes):**
  > "Default-on rewrites (lossless row: algebraic, unit-preserving, stdlib inverse round-trips) ... Default-off rewrites fire only under authorizing `approximate` blocks."

  Absorbed into §19.4: "Default-on rewrites saturate; default-off rewrites fire only inside authorizing `approximate` blocks with an error budget."

- **`planning/v2/open_questions.md` (Tier 0 audit, item 2 of the top-10 priority list):**
  > "Relate 'residual graph' to the e-graph. Is the residual graph the user-visible diagnostic projection of the saturated e-graph after extraction? If so, say so in spec §12."

  Absorbed: §19 now states this relationship explicitly.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 (settled item 3):**
  > "Residual graph is a projection of the e-graph, not separate."

  Absorbed into §19 preamble.

- **`planning/v2/spec.md` §12.3 (three-way component classification — redundant / provably inconsistent / underdetermined):**
  > "Computational redundancy ... Overconstrained residual (n_eq > n_unknown) ... Underdetermined residual (n_eq < n_unknown)"

  The §19.3 three-way overdetermination tag (`redundant` / `provably inconsistent` / `conditionally inconsistent`) is the spec_new.md restatement of this classification, absorbed and renamed.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11 (settled item 1):**
  > "E-graph is the internal equality substrate of Myco."

  §19 builds on §16 for this. The residual-graph-as-projection framing in §19 presupposes this commitment.

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §19. Should move to anti_spec.md if not already there.

- **`planning/v2/spec.md` §12 preamble and §12.1:**
  > "The residual graph is the core semantic object of the compiler."
  > "The residual graph contains: Variable nodes, Derived nodes, Residual factors, Slot nodes, Metadata."

  Superseded by §19 preamble: "The residual graph is a user-facing diagnostic view projected from the e-graph via cost-vector-guided extraction. It is not a canonical form: different workflow cost preferences yield different residuals." The old framing treated the residual graph as the core internal object; the new framing makes the e-graph the core and the residual graph the diagnostic projection.

  `Recommend:` The retired framing "residual graph as core semantic object" is already captured in anti_spec.md:
  > "residual graph as core semantic object | e-graph three-layer split (equational core / envelope metadata / adjacent keyed state); residual = user-facing projection | chunk 04 recommitment"
  No further action needed.

- **`planning/v2/spec.md` §12.2 path-selection heuristics:**
  > "Prefer `bijective`, `smooth` paths over `injective_restricted` or `fragile` ... Assign higher cost to inversions through ill-conditioned operations ... Reject inversions through `lossy` or `opaque` operations"

  Superseded by §19.1's cost-vector model, which replaces the legacy four-class invertibility metadata (`bijective` / `injective_restricted` / `lossy` / `opaque`) with the multi-dimensional cost tuple and capability contracts on functions (§6). The four-class metadata is already retired in anti_spec.md:
  > "four-class invertibility metadata (`bijective` / `injective_restricted` / `lossy` / `opaque`) | capability contracts on fns | same"
  No further action needed.

- **`planning/v2/spec.md` §12.3 "canonical evaluator" framing:**
  > "The planner picks a canonical evaluator using the operation algebra's cost model. This is compiler-internal and does not affect the science."

  Superseded by §19.1: extraction returns a Pareto front; workflow configuration selects a point. The idea that the compiler internally selects one canonical evaluator without user-facing cost-vector parameterization is replaced.

  `Recommend:` This specific phrasing is not yet in anti_spec.md. It should be added. However, it is an instance of the broader "residual graph as core semantic object" retirement already captured. Low urgency but worth a note if anti_spec.md is ever refined.

- **`planning/v2/open_questions.md` Tier 0 trajectory observation:**
  > "The residual graph as diagnostic: users should probably never see the e-graph directly. They see the residual graph ... as the factor graph surfaced via `explain_plan()`."

  Absorbed into §19 and §19.2. Now stable decision; the open-question framing is superseded.

---

## Homeless

Corpus content that is relevant to §19, not accounted for in spec_new.md §19, and not already committed to anti_spec.md. This is the highest-value bucket.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11, O4.3 — Per-residual training emission:**
  > "Overconstrained relations must survive extraction with their *original relation names* so training emission can expose them per-residual. Standard CSE-style canonicalization would collapse them. The e-graph can hold both forms; the extraction policy must be aware."

  This is a stable constraint on §19.2's "Sharing policy" and extraction mechanics: the extractor cannot freely apply common-subexpression elimination to overconstrained relations when training-classified SCCs are present. §19.2 notes "specific heuristics remain open under Tier 0 Phase 2 work" and cross-references §35, but does not state this constraint explicitly.

  `Recommend:` Add a bullet under §19.2 (or a note in §35 O4.3) stating that the extraction policy must preserve overconstrained relation names through extraction. This is a stable constraint from chunk 04 CC3 / O4.3, not open design work. It narrows the solution space for the §19.2 heuristics.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11, O2.4 — Named cost dimensions:**
  > `cost = { compute: ..., approximation: ..., condition: ..., truncation: ..., discretization: ... }`
  > "`loss_of(expr)` returns a struct of named fields, not a scalar."

  §19.1 describes cost dimensions as precision, latency, memory, and approximation class. The chunk 04 named-field form is more granular (and more settled) than §19.1's four-item list. The `condition`, `truncation`, and `discretization` dimensions in the chunk 04 report do not appear in §19.1.

  `Recommend:` Reconcile §19.1's cost dimensions with chunk 04's named-field `loss_of` struct. Either expand §19.1 to name the additional fields or add a forward reference to §14 where `loss_of` / `condition_of` are specified. This is settled design from O2.4, not open work.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §10, workflow extraction policy surface:**
  > "`run.config.extraction_policy = { compute_weight: 1.0, loss_weight: 10.0, loss_cap: 0.01 }`"
  > "`run.config.extraction_policy` and `run.config.loss_estimation` are v2.1 workflow verbs."

  §19.1 describes "workflow configuration selects a specific point (latency-first, precision-first, or weighted)" but does not name the workflow-side API (`run.config.extraction_policy`) or its fields. The locked API shape from O2.4 is absent from §19.

  `Recommend:` Add a reference in §19.1 to the workflow verb (or to whichever Part IV section specifies `run.config.extraction_policy`). Without it §19.1's description of policy selection hangs without a hook into the workflow layer.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §8 — Rational saturation termination concern:**
  > "Rational arithmetic grows denominators unboundedly (coprime additions). The e-graph needs a **precision cap** or **canonical-form simplification** to prevent non-terminating saturation. Specific policy is **Section 12 open**."

  §19.4 describes a rewrite-count cap as the termination mechanism for pathological cases, and states "Practical models do not approach the bound." The Rational-denominator growth problem is a distinct non-termination vector not guarded by the rewrite-count cap (the number of rewrites may be small but the cost per rewrite grows). §35 lists the open item ("Rational saturation termination") but §19.4 does not acknowledge it.

  `Recommend:` Add a caveat to §19.4's "Termination bound" bullet noting that rational constant folding during saturation has a separate termination concern addressed in §35 and §26.3. This is a known gap, not new design work.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11, O4.3 (CC3) — Extraction policy awareness of overconstrained relation names is a Phase 4 / §35 open item but the constraint itself is stable:**
  Covered in the first Homeless bullet. Not repeated.

- **`planning/v2/open_questions.md` Tier 0 top-10, item 8 — Closure policies over extraction timing:**
  > "Specify closure policies over extraction. Run before or after saturation; do policies see the full e-class or only the canonical evaluator?"

  §19.3 states that the "three-way overdetermination tag gates closure-policy meaning" and §8.7 covers policies, but neither §19 nor §8.7 resolves the timing question: do closure policies fire at saturation time (as rewrites) or at extraction time (as post-saturation selection)? §19.4's scheduling priority list ends with "closure-policy" as the last priority, implying they fire during saturation — but this is not stated explicitly.

  `Recommend:` Add to §19.4 (or §8.7) a sentence explicitly stating that Y-group closure policies are extraction-time, not saturation-time operations. This is a stable design decision implied by chunk 04 §11 ("closure-policy co-membership ... extraction picks among them"), not ongoing open work.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11, O2.2 CC3 / CC5 — Scheduling priority for site-scoped predicates:**
  > "Pole L'Hopital and `identify`-seam merges fire as site-scoped rewrites; no new rewrite category."
  > "Merges from explicit relation `=` and `identify` (sources 1 and 4, §17.1) fire first."

  §19.4's scheduling priority says merges from sources 1 and 4 fire first and "algebraic and unit-preserving rewrites next; conversion and closure-policy last." The chunk 04 CC5 decision that pole L'Hopital rewrites and `identify`-seam merges fire as site-scoped rewrites means they logically belong in the second priority tier (algebraic / unit-preserving), but this placement is not stated in §19.4.

  `Recommend:` Clarify in §19.4 that site-scoped structural predicates (pole L'Hopital, seam rewrites locked in CC5) fire in the algebraic/unit-preserving tier, not as a distinct tier. This is a specificity gap, not open design.

- **`planning/v2/open_questions.md` Tier 0 Q2.3 — Residual-to-e-graph round-trip for diagnostics (Phase 2 open item O2.Q3):**
  > "How extraction policy determines what the residual graph looks like at any moment ... round-trip for diagnostics: given a residual-graph node, how to materialize the full e-class it came from."

  §19.2 tracks this under "Open items tracked in §35 (Tier 0 Phase 2 Q3)." The cross-reference is correct. This is legitimately open design work for an in-progress chunk; not homeless.

  `Recommend:` No action — §19.2 already correctly defers to §35. Included here to document the review decision.

---

## Conflicts

Direct contradictions between spec_new.md §19 and any corpus document.

- **§19.4 scheduling priority vs. chunk 04 §11 extraction-time framing of closure policies:**

  §19.4 lists "conversion and closure-policy last" as the fourth scheduling priority tier, implying closure policies are rewrite-time operations applied during saturation. Chunk 04 §11 (settled item 7, §6 merge sources) states: "Closure-policy co-membership ... extraction picks among them per the closure rule. Merge happens regardless of policy." This means closure-policy co-membership (the merge) fires during saturation, but the *policy selection* (which candidate wins) happens at extraction, after saturation.

  §19.4 conflates two distinct operations. The co-membership merge fires during saturation (at some priority). The policy decision — which candidate the extractor picks — is an extraction-time, post-saturation operation, not a saturation-phase rewrite.

  `Recommend:` Rewrite the §19.4 scheduling priority bullet to distinguish: (a) closure-policy co-membership merges fire in the saturation phase at the lowest priority tier; (b) policy selection (Y1-Y6) is extraction-time, after saturation completes. This matches chunk 04's framing and removes the ambiguity.

- **§19.1 cost dimensions vs. chunk 04 O2.4 named-field dimensions:**

  §19.1 lists four cost dimensions: precision, latency, memory, approximation class. Chunk 04 O2.4 (settled) names five: `compute`, `approximation`, `condition`, `truncation`, `discretization`. The mismatch is not merely terminological: "memory" in §19.1 has no correspondent in the chunk 04 named-field list, and "condition", "truncation", and "discretization" in chunk 04 have no correspondent in §19.1. These are two different enumerations of the same concept, and at least some of the differences are substantive (condition number is a separate axis from precision; truncation error is distinct from approximation class).

  `Recommend:` Reconcile §19.1's dimension list with chunk 04 O2.4's named-field struct. The chunk 04 version is more recent and more carefully designed. If memory is a legitimate dimension, add it to the chunk 04 list; if condition / truncation / discretization are legitimate, add them to §19.1. The simplest resolution is to adopt the chunk 04 named-field form in §19.1 and add "memory" as a sixth field if needed.
