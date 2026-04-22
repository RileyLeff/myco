# Audit: §16 — The E-Graph

Auditing `spec_new.md §16` (subsections 16.1–16.4) against the corpus.

---

## Absorbed

Content from the corpus that already landed in spec_new.md §16.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §2**
  > "The e-graph is the internal equality substrate of Myco."
  > "The residual graph is a user-facing projection of the internal e-graph, not a separate thing."

  Absorbed into §16 opening summary ("The e-graph is Myco's internal equality substrate") and into §19 Residual Graph, which opens with "The residual graph is a user-facing diagnostic view projected from the e-graph via cost-vector-guided extraction."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §3 (three-layer scoping split)**
  > "Layer 1 — Equality substrate (the e-graph proper) ... Layer 2 — Envelope metadata attached to e-classes ... Layer 3 — Adjacent keyed structures (genuinely separate)"

  Absorbed into §16.1. The three concentric layers are named and described with substantially the same semantics.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §3 (non-equational constraints in Layer 2)**
  > "Non-equational constraints — inequalities (≥ 0), domain bounds, type-level predicates. Attached to the class carrying the constrained expression."

  Absorbed into §16.1 Layer 2: "Refinement bounds, distributional metadata from `~` (§13.8), capability advertisements from contracts (§7.2), observed samples (§13.9), tolerance envelopes (§16.4)."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §5 (referential truth and dead entities)**
  > "Events add facts; never retract ... Things do not know they are dead."

  Absorbed into §16.2: "Events add facts; they do not remove prior e-classes. Dead entities continue to exist equationally; their absence from subsequent ticks is a layer-3 keyed-state fact, not a layer-1 deletion."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §5 (replaces does not retract)**
  > Referential-truth framing applied to `replaces`.

  Absorbed into §16.2: "`replaces` (§8.10, §10.5) suppresses default generation; it does not retract an already-emitted fact."

- **`planning/v2/anti_spec.md` (retired architectural framing)**
  > "| residual graph as core semantic object | e-graph three-layer split (equational core / envelope metadata / adjacent keyed state); residual = user-facing projection | chunk 04 recommitment |"

  Absorbed: residual graph is now §19 (projection), and the three-layer split is §16.1. The anti_spec entry is already filed correctly as a retired framing.

- **`planning/v2/spec_dev_notes.md` (§16 changelog)**
  > "§16 E-Graph — 2026-04-21: 16.1 three-layer scoping split named as structural principle (cross-reference §0), 16.2 monotonicity invariant (append-only; `replaces` suppresses generation, does not retract; dead entities continue to exist equationally), 16.3 envelope ownership (stdlib + compiler rewrites + `observe` write; dispatch/extraction/diagnostics read; no invalidators), 16.4 envelope flavors (entry-wise / operator-norm / spectral / structural with per-flavor composition rules)."

  All four subsections described here are present in spec_new.md §16.

- **`planning/v2/open_questions.md` (Tier 0, "The v1 scoping sentence")**
  > "the e-graph is the equality core of Myco, not the entire semantic system. Temporal links, observations, provider bindings, provenance, and non-equational constraint metadata live in adjacent compiler structures keyed to that equality core."

  Absorbed into §16.1 preamble and the structural summary ("three concentric layers ... each layer has its own modification rules").

- **`planning/v2/anti_spec.md` (resolved open questions)**
  > "| `~` stochastic as e-graph merge | resolved — `~` is layer-2 distributional metadata, not a merge |"

  Absorbed into §16.1 Layer 2: "distributional metadata from `~` (§13.8)."

---

## Superseded

Content that has been replaced by a newer decision in spec_new.md §16.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §3 (temporal indexing in Layer 3)**
  > "Early drafts of this discussion put temporal indexing ... in Layer 3. This was wrong."
  > "Temporal indexing: `y[1]`, `y[2]`, `y[3]` are just distinct ground terms in Layer 1."

  Spec_new.md §16.1 Layer 3 lists "y[1], y[2], ... ; per-event copies; identity-tagged instances" as Layer 3 occupants. This conflicts with the chunk 04 correction. See Conflicts section for the full entry.

  Note: no anti_spec.md entry exists for this temporal-indexing-in-Layer-1 correction. The chunk 04 document is in-progress and the question may still be live. See Conflicts.

- **`planning/v2/spec.md` (residual graph as core semantic object)**
  > "spec §12.3 'computational redundancy: algebraically equivalent evaluators of the same solved component'" and "spec §12.3 'canonical evaluator'"

  These phrases assumed the residual graph was the equality substrate. Superseded by the three-layer split. The anti_spec.md already records "residual graph as core semantic object" as retired, so this is correctly handled.

- **`planning/v2/spec.md` §8.5**
  > "structural introspection on the competing paths"

  Superseded: structural introspection is retired (anti_spec.md). Closure policies receive only values and hyperparameters. The anti_spec.md already records this. No action needed.

---

## Homeless

Corpus content relevant to §16, not accounted for in spec_new.md §16, and not committed to anti_spec.md.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §11, O4.1**
  > "O4.1 — Obligation retraction (`replaces`). The `replaces` keyword demands deletion semantics; standard e-graph theory is monotonic. Options: In-graph with versioning and rebuild on retraction (expensive). Adjacent obligation-keyed metadata per v1 §6.2 (Layer 3). Reframe `replaces` as 'add a superseding fact, old fact stops being selected by extraction.'"

  Spec_new.md §16.2 states that `replaces` suppresses default generation and does not retract, but §16.2 does not acknowledge that O4.1 is still unresolved for the harder case (user-relation retraction, not just compiler-default suppression). The spec_dev_notes entry for §10.5 records "arbitrary prior-claim retraction stays open in §34," but §16.2 carries no cross-reference to that open status. A reader of §16.2 alone would not know this is an open design question with three candidate resolutions.

  `Recommend:` Add a cross-reference in §16.2 to §35 (or wherever the open is tracked), noting that the case handled by §16.2 is compiler-generated defaults only and that broader retraction semantics remain open. This is a stable partial decision that should be distinguished from the still-open case in the section itself, not just in dev_notes.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §3, Layer 2 (provenance as envelope content)**
  > "Provenance — which relation, which file, which line introduced each node. Union rule under merge: the merged class lists all sources."

  Spec_new.md §16.3 lists the readers of envelope facts as "Diagnostics / `mycoc explain` (§22) reads every envelope fact and surfaces provenance," which implies provenance is envelope content. But §16.3 does not name provenance explicitly as an envelope-fact type with its own merge rule (union under merge). The merge rule for provenance is not stated anywhere in §16.

  `Recommend:` Add provenance to the Layer 2 enumeration in §16.1 (with union-under-merge rule) and note it in §16.3 writers/readers. This is a settled chunk-04 decision that belongs in §16 proper.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §3, Layer 2 (faithfulness and orientation tags on merge edges)**
  > "Faithfulness and orientation tags on merge edges (see Section 8)."

  The 2x3 faithfulness x orientation matrix is a settled chunk-04 decision. Faithfulness tags (lossless / lossy-model / lossy-tolerance) are referenced in §15.3 and §16.4, but neither §16.1 nor §16.2 names these tags as Layer 2 envelope content with their own composition rules. The tags travel with merge edges; their Layer 2 home is not stated.

  `Recommend:` Add faithfulness and orientation tags to the Layer 2 enumeration in §16.1. The full 2x3 matrix lives in §15/Appendix C; §16 needs only to name these as Layer 2 occupants. Stable chunk-04 decision.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §5 (SCC decomposition results as Layer 3)**
  > "SCC decomposition results — derived from the e-graph at lowering time. Distinct artifact."

  Spec_new.md §16.1 Layer 3 does not list SCC decomposition results as an occupant. SCC decomposition is described in §20, but the layering claim (SCC results are a Layer 3 artifact keyed to the e-graph) is a settled chunk-04 decision absent from §16.

  `Recommend:` Add SCC decomposition results to the Layer 3 enumeration in §16.1 with a cross-reference to §20. Stable decision.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §5 (provider bindings / "coloring book" as Layer 3)**
  > "Provider bindings / coloring — workflow-time knowns/unknowns state. The 'coloring book' metaphor: the `.myco` file is a book of outlines; the workflow colors it in. Not on the graph, but keyed by e-class identity."

  Spec_new.md §16.1 Layer 3 lists "y[1], y[2], ...; per-event copies; identity-tagged instances" but does not mention workflow provider bindings (known/unknown state, the result of `assume_*` / `learn_*` / `bind_controller` verbs). This is a stable chunk-04 decision on what lives in Layer 3.

  `Recommend:` Add workflow provider bindings (known/unknown state) to the Layer 3 enumeration in §16.1. Stable decision.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §5 (stochastic sampling traces as Layer 3)**
  > "Stochastic sampling traces — the runtime history of `~` draws. Live in a trace structure keyed by draw-node identity."

  Not listed in §16.1 Layer 3.

  `Recommend:` Add stochastic sampling traces to the Layer 3 enumeration in §16.1. Stable decision from chunk 04; cross-reference §13.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §5 (runtime event-trigger state as Layer 3)**
  > "Runtime event-trigger state — event semantics operate on the graph (mutating it by adding facts) but the scheduler and trigger state is adjacent."

  Not listed in §16.1 Layer 3.

  `Recommend:` Add runtime event-trigger state to the Layer 3 enumeration in §16.1. Stable decision; cross-reference §10.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §10 (extraction cost becomes a tuple)**
  > "cost = (compute_cost, approximation_loss, ...)"
  > "Extraction policy combines the tuple per workflow preferences."

  The cost-tuple for extraction is elaborated in §19.1 (Extraction Cost Model). However, §16 does not characterize extraction cost as a Layer 2 concern — envelope lossiness facts are what feed the cost tuple. The connection between Layer 2 envelope content and the extraction cost tuple is implicit (§16.4 states tolerance envelopes are Layer 2; §19.1 uses them as cost inputs) but never stated as a principle in §16.

  `Recommend:` This gap is minor and adequately covered by cross-referencing §19.1 from §16.4. Not high priority for §16 itself. Open a note in §35 if desired, but this is not a blocking gap.

- **`planning/v2/spec_dev_notes.md` (§31.1 backend emulate mode)**
  > "31.1 capability advertising + 3 fallback modes (error / host / emulate) with fallback scoped per-run via `run.config.backend`; emulate mode's substitutions enter the approximation-error layer (§16)."

  Spec_dev_notes records that backend emulate-mode substitutions "enter the approximation-error layer (§16)." Spec_new.md §16 does not explain what "approximation-error layer" means or that backend emulation is a fourth source of lossiness feeding into the Layer 2 envelope. (§15.2 does list "backend emulation" as a lossiness source, which covers the pipeline; but the Layer 2 mechanism by which it enters is not stated in §16.)

  `Recommend:` Add a sentence in §16.3 or §16.2 noting that backend emulate-mode substitutions register as lossy-tolerance envelope facts (Layer 2), with a cross-reference to §31.1. Stable decision recorded in spec_dev_notes; should be stated explicitly in §16.

- **`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §3.3 (four envelope flavors are open for matrices)**
  > "### 3.3 Envelope flavors for matrix-valued quantities — OPEN"
  > "Probably all four are needed; they merge differently under different ops. Key design questions: Canonical form ... Storage / merging rules ... Propagation rules per op."

  Spec_new.md §16.4 names the four tolerance-envelope flavors (entry-wise, operator-norm, spectral, structural) and states composition rules for each. Chunk 05 explicitly marks these same four flavors as OPEN questions for matrix-valued quantities. The composition rules in §16.4 ("entry-wise bounds compose by summation under triangle inequality; operator-norm by sub-multiplicativity; spectral by Weyl-style inequalities; structural by set intersection") appear to be committed general rules, but chunk 05 flags the storage, merging, and propagation rules for tensors as unresolved.

  `Recommend:` Add a caveat in §16.4 noting that the composition rules are stated for the scalar-envelope case; the tensor-valued extension (merge storage, per-op propagation) is an open design question tracked in chunk 05. This prevents §16.4 from being read as fully settled when the matrix extension is not. This is relevant to §16 not just §3; it belongs here.

---

## Conflicts

Direct contradictions between spec_new.md §16 and corpus documents.

- **Temporal indexing: Layer 1 (chunk 04) vs. Layer 3 (spec_new.md §16)**

  **spec_new.md §16.1 Layer 3:**
  > "Adjacent keyed state (layer 3). Structures indexed by temporal subscript, event firing, or identity tag, but holding e-class references internally. `y[1]`, `y[2]`, …; per-event copies; identity-tagged instances."

  **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` §4:**
  > "`y[t]` and `y[t+1]` are distinct ground terms — they are not claimed equal anywhere. They coexist peacefully as distinct nodes."
  > "temporal is not a special axis. It is just indexing that produces distinct terms."
  > "Early drafts of this discussion put temporal indexing ... in Layer 3. This was wrong."

  These directly contradict each other. Chunk 04 explicitly corrected a "Layer 3" framing for temporal indexing to "Layer 1 distinct ground terms." Spec_new.md §16 still uses the earlier (chunk-04-corrected-away) framing.

  `Recommend:` Update §16.1 Layer 3 to remove `y[1]`, `y[2]`, ... from its list of occupants. Temporal subscripts produce distinct ground terms in Layer 1, not a Layer 3 dispatch table. The Layer 3 description should be updated to list the actual Layer 3 occupants that chunk 04 settled (provider bindings, stochastic sampling traces, SCC results, runtime event-trigger state, observations before injection). The "dispatch table" metaphor in §16.1 may be rescued for event-firing copies and identity-tagged instances, but temporal subscripts are not the right example.
