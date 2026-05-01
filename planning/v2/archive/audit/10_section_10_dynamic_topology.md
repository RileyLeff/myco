# Audit: §10 Dynamic Topology and Events

**Section under audit:** `spec_new.md §10` (five subsections: 10.1–10.5)
**Corpus searched:** soul.md, spec.md, spec_dev_notes.md, riley_project_note.md,
anti_spec.md, v2.1_in_progress.md, open_questions.md, chunk reports 01–07.

---

## Absorbed

Content from corpus that is already reflected in spec_new.md §10.

- **`planning/v2/v2.1_in_progress.md` lines 1596–1632 ("Event execution model"):**
  > "Firing order is a simulation parameter, not language syntax. Default: declaration order in the `.myco` file."
  Absorbed into §10.1 verbatim in substance.

- **`planning/v2/v2.1_in_progress.md` lines 1544–1585 ("Generic events — first-class"):**
  > "Multi-parameter generic events expand over the cartesian product of their bounds."
  Absorbed into §10.2 (cartesian product expansion, obligation keys, firing-order dispatch).

- **`planning/v2/open_questions.md` lines 422–438 ("Generic events — commit to first-class sugar — RESOLVED"):**
  > "Multi-parameter generic events expand over the cartesian product of their bounds. Const generic events are permitted on the same footing as type generics."
  Absorbed into §10.2.

- **`planning/v2/open_questions.md` lines 440–449 ("Cross-container events — RESOLVED"):**
  > "An event is declared on the smallest container type whose scope contains all its input/output participant types. For cross-container events this is the nearest common ancestor container."
  Absorbed into §10.3.

- **`planning/v2/v2.1_in_progress.md` lines 1515–1538 ("Container scoping rule"):**
  > "cross-container events, the event lives on the nearest common ancestor container. When a participant type appears as a pool in multiple sibling children, the signature uses dotted paths"
  Absorbed into §10.3.

- **`planning/v2/open_questions.md` lines 451–462 ("Within-event conflict tiebreaking — RESOLVED — no language decision"):**
  > "three canonical policies (`declaration_order`, `shuffle(seed)`, `priority_by_scalar(quantity_path)`) ... None of this affects language design. Priority hints on event declarations ... were explicitly rejected."
  The three-case exhaustive analysis (structurally identical, conserved-field conflict, legitimately overdetermined) in §10.4 absorbs the spirit of this decision.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` lines 152–207 ("Referential truth and dynamic topology — LOCKED"):**
  > "Things do not know they are dead. Entity existence at timestep T is defined by whether any relation at T references the entity. No alive/dead flag. No tombstoning."
  Absorbed into §10 summary and §10.4 case 1 (e-graph merges structurally identical facts).

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` lines 172–207 ("Events add facts; never retract"):**
  > "Events only ever add new nodes and new equalities ... They never retract equalities added earlier."
  Absorbed into §10 summary ("Events add facts; no tombstoning, no retraction") and §10.5.

- **`planning/v2/spec_dev_notes.md` lines 241–246 (§10 changelog):**
  > "10.1 firing-order policy, 10.2 generic event expansion, 10.3 cross-container events (NCA rule), 10.4 within-event tiebreaking (three-case exhaustive under referential truth), 10.5 `replaces` monotonicity (default-suppression not retraction; arbitrary prior-claim retraction stays open in §34)."
  Confirms all five subsections were deliberately landed. Fully absorbed.

- **`planning/v2/spec_dev_notes.md` lines 303–305 (§16 changelog):**
  > "monotonicity invariant (append-only; `replaces` suppresses generation, does not retract; dead entities continue to exist equationally)"
  The §10.5 wording ("suppressing its emission, not by retracting a fact") is the spec-prose form of this absorbed decision.

- **`planning/v2/anti_spec.md` line 19 (retired keywords table):**
  > "`rule` keyword — replacement: `event`"
  The rename is complete. §10 uses `event` throughout; no trace of `rule`.

---

## Superseded

Content replaced by a newer decision in spec_new.md §10.

- **`planning/v2/v2.1_in_progress.md` lines 1624–1626 ("Within-event conflicts"):**
  > "Within-event conflicts (two predators want the same prey): deterministic tiebreak by index order, overridable from Python. The compiler can detect that two events could compete for entities of the same type and warn."
  Superseded by §10.4, which classifies within-event concurrency into three exhaustive cases (e-graph merge, junction balance, closure policy) and states "No additional within-event ordering construct is exposed." The index-order tiebreak language implied a runtime ordering mechanism; §10.4 shows why none is needed. Not yet in anti_spec.md.
  Recommend: add the stale "tiebreak by index order" framing to anti_spec.md under "Retired architectural framing" to prevent re-litigating it.

- **`planning/v2/v2.1_in_progress.md` lines 1797–1800 ("Dynamic topology lowering strategy"):**
  > "The `.myco` file says 'this can grow/shrink' (`dyn`), the experiment runner says 'but never more than 500.' Rule firing flips mask bits..."
  Uses the retired `dyn` keyword and the retired `rule` keyword. Both superseded: `some` replaced `dyn` (anti_spec.md), `event` replaced `rule` (anti_spec.md). The stale prose in v2.1_in_progress.md is already flagged in anti_spec.md line 80 as "stale versioning prose"; the specific retired-keyword occurrences on lines 1798 and 1799 are covered there.

---

## Homeless

Corpus content relevant to §10 that is neither in spec_new.md §10 nor committed to anti_spec.md.

- **`planning/v2/v2.1_in_progress.md` lines 1590–1594 ("`when` — event trigger condition"):**
  > "`when` — event trigger condition. Condition under which an event fires. Deterministic thresholds. Can reference quantities on the participants and on the container. Edge-triggered (fires at the moment the condition becomes true). Status: new, settled."
  The `when` clause is the guard syntax for events; §10 nowhere mentions `when` or edge-triggered semantics. The trigger mechanism is a stable, settled decision about how events fire, not an open design item.
  Recommend: add a §10.0 (or §10.1 note) describing `when` as the event trigger, covering deterministic thresholds and edge-triggered semantics. Without this, §10 describes firing order and parallelism for events that have no documented trigger surface.

- **`planning/v2/open_questions.md` lines 451–462 (within-event tiebreaking — RESOLVED):**
  > "three canonical policies (`declaration_order`, `shuffle(seed)`, `priority_by_scalar(quantity_path)`) will ship as Python library helpers; a unified policy API `policy(pending_firings, state) -> List[Firing]` covers both between-event ordering and within-event tiebreaks."
  The specific policy API shape (`policy(pending_firings, state) -> List[Firing]`) and the three named stdlib policies are stable decisions about the workflow-layer scheduling interface. §10.1 says firing order is "a simulation parameter set at workflow composition" and "workflow overrides via run-config," but gives no API surface. Users reading §10.1 cannot know what the Python-side interface looks like.
  Recommend: add a note in §10.1 or a cross-reference to the workflow section for the three stdlib policies and the `policy` callable API.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` lines 1289–1293 (O4.7):**
  > "O4.7 — Event-driven topology mutation. Events add nodes, edges, equivalences. This is covered at the semantic level by Section 5 (events add facts). Operationally, the e-graph's saturation must handle incremental additions without re-running from scratch. Implementation concern; not a design blocker but needs a note."
  This is an open implementation concern, not a settled spec decision. It is legitimately in-progress work in a chunk-04 in-progress document, so by the audit's rules it is not homeless. Noted for completeness.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` lines 1242–1252 (O4.1):**
  > "O4.1 — Obligation retraction (`replaces`). The `replaces` keyword demands deletion semantics; standard e-graph theory is monotonic. ... The referential-truth framing (Section 5) may cover `replaces` the same way it covers ecosystem entities, but the specific semantics of `replaces` in the spec needs a pass to confirm. Section 12 open."
  §10.5 in spec_new.md states that `replaces` suppresses compiler-generated defaults (not retraction) and refers "the harder case" to §35. However, §10.5 does not cite or acknowledge O4.1's open status or reference the e-graph monotonicity tension. This is in-progress design in chunk-04, so not homeless by the audit rule, but the §10.5 cross-reference to §35 is the right hook and appears to be landing it correctly.

- **`planning/v2/open_questions.md` lines 80–98 ("Claude's pre-audit additions — Events/topology change"):**
  > "Events / topology change. An event that births or kills entities: does this invalidate parts of the e-graph? Require reconstruction? Or is topology-change represented in the e-graph?"
  This question is a Tier 0 e-graph substrate question, not a §10 spec question. The §10 sections address the user-facing semantics (referential truth, no tombstoning). The e-graph implementation question is open but belongs to the §16/§35 substrate story, not §10. Not homeless relative to §10.

- **`planning/v2/v2.1_in_progress.md` lines 1540–1542 ("Heterogeneous dynamic collections"):**
  > "Events targeting `[T<impl Contract>; some]` must specify the concrete output type: `event oak_recruit: -> Tree<FarquharC3>`. This lets the compiler route to the correct type pool."
  This is the concrete-output-type requirement for heterogeneous collection events — a stable syntax rule settled in v2.1_in_progress and chunk report 02 (line 473 of chunk-02). §10 has no mention of this requirement. It is a stable syntax decision, not an open design item.
  Recommend: add a note in §10.2 (generic event expansion) or as a standalone §10.x that events targeting `impl`-typed collections must name the concrete output type; generic events over such collections are the desugaring.

- **`planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md` lines 473–506 ("Heterogeneous dynamic collections"):**
  > "event oak_recruit: -> Tree<FarquharC3> ... Option 2 (generic event) is acceptable as syntactic sugar ... the compiler monomorphizes it to N concrete events, one per in-scope implementation."
  Same stable decision as above. Both sources confirm it.
  Recommend: same as above — land this in §10.2 or a new §10.x.

---

## Conflicts

Direct contradictions between spec_new.md §10 and corpus documents.

- **§10.4 vs `planning/v2/v2.1_in_progress.md` lines 1624–1626:**
  spec_new.md §10.4: "No additional within-event ordering construct is exposed. ... within a single type, parallelism is the default and the three cases above cover every outcome."
  v2.1_in_progress.md: "Within-event conflicts (two predators want the same prey): deterministic tiebreak by index order, overridable from Python."
  These are directly contradictory. §10.4 says the three-case analysis is exhaustive and no ordering is needed. The in-progress doc says there is a runtime tiebreak by index order. The spec_new.md §10.4 analysis is the later, more carefully argued position (it explains why each case resolves without ordering), so spec_new.md takes precedence.
  Recommend: the stale tiebreak-by-index-order framing in v2.1_in_progress.md should be struck or annotated as superseded by the §10.4 three-case analysis. Add a row to anti_spec.md under "Retired architectural framing": "within-event index-order tiebreak | §10.4 three-case exhaustive analysis | ordering is not needed once cases are classified."

- **§10.1 vs `planning/v2/open_questions.md` lines 456–459:**
  spec_new.md §10.1: "Default is declaration order; workflow overrides via run-config."
  open_questions.md: "three canonical policies (`declaration_order`, `shuffle(seed)`, `priority_by_scalar(quantity_path)`) will ship as Python library helpers; a unified policy API `policy(pending_firings, state) -> List[Firing]`."
  Not a contradiction in substance, but §10.1 says "workflow override via run-config" without naming the API surface, while open_questions.md has a resolved, specific API shape. The gap is incomplete coverage rather than contradiction. §10.1 does not conflict with the resolved decision, but it should be updated to mention the stdlib policies and the callable API.
  Recommend: extend §10.1 to reference the three stdlib scheduling policies and the `policy(pending_firings, state) -> List[Firing]` callable interface, consistent with the RESOLVED entry in open_questions.md.
