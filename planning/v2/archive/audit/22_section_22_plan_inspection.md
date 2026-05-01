# Audit Report — §22 Plan Inspection

Audited against corpus as of 2026-04-22.

§22 is 26 lines, a skeletal placeholder. Most corpus content on plan inspection
is homeless.

---

## Absorbed

Corpus content that already landed in spec_new.md §22.

- **`planning/v2/v2.1_in_progress.md:1032-1035` ("Plan inspection"):**
  > "Compilation plan as inspectable artifact. Shows SCCs, solver strategies,
  > symbolic resolutions, numerical fallbacks. Status: settled."

  Absorbed into §22's framing: "compiled plan for auditing, debugging, and
  verifying compilation choices."

- **`planning/v2/spec.md` §14.5 preamble (lines 3171-3177):**
  > "The residual graph is an inspectable artifact. After compilation, the user
  > can examine what strategies the compiler chose... This is the primary
  > discovery mechanism for configurable behavior and the primary diagnostic
  > tool for understanding the model's structural properties."

  The core commitment that the compiled plan is inspectable is absorbed into
  §22's "inspectable via `mycoc explain`... for users who want to audit the
  plan, debug behavior, or verify compilation choices."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:475`:**
  > "Diagnostics (`mycoc explain path_A --vs path_B` prints both loss bounds)."

  The `mycoc explain` surface naming is absorbed into §22's CLI entry point.

- **`planning/v2/v2.1_chunk_reports/09_workflow_data_layer.md:38-43`
  ("Python does not see"):**
  > "Relation bodies or their equational content ... E-graph structure, rewrite
  > choices, extraction plans."

  The principle that Python is a data layer, not a model layer, and that the
  compiled plan is an artifact rather than a substrate, aligns with §22's
  "compiled program is an output artifact, not the source of truth."

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §22. Should move to
anti_spec.md if not already there.

- **`planning/v2/spec.md` §14.5 "residual graph as inspectable artifact" framing
  (line 3173):**
  > "The residual graph is an inspectable artifact."

  Superseded by the §19 recommitment that the residual graph is a projection of
  the e-graph, not an artifact in its own right. §22 correctly frames plan
  inspection as "exposes the compiled plan," leaving the residual-vs-e-graph
  question to §19. The "residual graph as core semantic object" retirement is
  already in anti_spec.md.

  `Recommend:` No action on §22 itself. The residual-as-inspectable framing is
  subsumed by the broader residual-graph retirement already captured.

- **`planning/v2/spec.md` Appendix B.6 "Plan visualization" CLI:**
  > "myco plan --dot | dot -Tsvg > plan.svg"

  §22 names `mycoc explain` rather than `myco plan`. The `mycoc`/`myco` CLI
  naming split (compiler binary vs. general driver) is settled in §36 but the
  legacy `myco plan` invocation is superseded by `mycoc explain`.

  `Recommend:` Not in anti_spec.md. Low urgency since Appendix B is already
  flagged as "supersede wholesale" in anti_spec.md stale list; the specific
  `myco plan` subcommand naming is implicitly retired. Could be noted during
  a future anti_spec.md sweep.

---

## Homeless

Corpus content that is relevant to §22, not accounted for in spec_new.md §22,
and not already committed to anti_spec.md. This is the highest-value bucket
given §22's skeletal state.

- **`planning/v2/spec.md` §14.5 (lines 3184-3202) — Plan report contents:**
  > "The plan reports: Component classification ... Symbolic resolutions ...
  > Hierarchical decompositions ... Numerical fallbacks ... Slot bindings ...
  > Execution order ... Temporal state ... Resolution frontier."

  §22 says the plan is inspectable but does not enumerate what the plan surface
  contains. This list is settled ("Status: settled" per v2.1_in_progress.md:1035)
  but absent from §22. The taxonomy (SCCs, symbolic resolutions, solver
  strategies, etc.) is the content of the inspection surface.

  `Recommend:` Add a bullet list under §22 naming the plan-report contents.
  Translate slot-era terminology to v2.1 equivalents (slot bindings become
  `bind_controller` attachments per the slot retirement in anti_spec.md). This
  is settled content, not open design.

- **`planning/v2/spec.md` §14.5 (lines 3204-3215) — Per-quantity knowledge
  envelope queries:**
  > "`envelope = artifact.plan.knowledge(\"leaf.water_potential\")` ...
  > `envelope.realization`, `envelope.free_variables`, `envelope.bounds`,
  > `envelope.obligations`, `envelope.resolver_sets`, `envelope.provenance`"

  §17 establishes the envelope metadata system and names `mycoc explain` as a
  consumer (§spec_new:2991 "Diagnostics / `mycoc explain` reads every envelope
  fact"). §22 does not describe the per-quantity query surface that lets a user
  pull envelope content for a specific path. The envelope fields (bounds,
  obligations, provenance) are settled in §17.

  `Recommend:` Add a §22 subsection on per-quantity envelope inspection, or a
  forward reference to §17 / §31 for the workflow-side API shape. §17 confirms
  `mycoc explain` reads envelope facts; §22 should name the query surface.

- **`planning/v2/spec.md` §14.5 (lines 3217-3229) — Hypothetical reasoning:**
  > "`plan_b = artifact.plan.with_assumption(\"soil.water_potential\", -0.5)` ...
  > plan re-evaluation with additional constraints — the planner reruns from
  > the augmented binding set."

  Plan re-evaluation under a hypothetical binding is a settled feature in the
  legacy §14.5 text with no equivalent in spec_new.md. It supports the
  "experimental design" use case ("if I collect this measurement, how much
  additional information does the model give me?"). Entirely absent from §22.

  `Recommend:` Decide whether hypothetical plan re-evaluation ships in v2.1. If
  yes, add it to §22. If no, retire the feature to anti_spec.md. Currently it
  exists in legacy corpus with "settled" status but has no landing site.

- **`planning/v2/spec.md` §14.5 line 3238-3239 — SQL `EXPLAIN` analogy:**
  > "The plan is analogous to a SQL `EXPLAIN` — it shows the execution strategy
  > without changing the semantics."

  The analogy is a useful framing device settled in the legacy spec. §22
  mentions `mycoc explain` but does not name the analogy or the principle
  ("inspection reveals what was decided, and overrides are available").

  `Recommend:` Optional. The SQL-`EXPLAIN` analogy is pedagogically useful and
  trivially portable; include it in §22 prose if desired. Not load-bearing.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:1171-1181`
  — Phase 2 Q3 residual/e-graph navigation:**
  > "How extraction policy determines what the residual graph looks like at any
  > moment. Which diagnostics reference the residual graph versus the e-graph.
  > How `mycoc explain` navigates between the two views. How user-facing error
  > messages reference equivalence classes. Round-trip for diagnostics: given a
  > residual-graph node, how to materialize the full e-class it came from."

  This is explicitly open design work for §22's scope. §19.2 references it
  under "Tier 0 Phase 2 Q3" but §22 does not name the open items that govern
  its own surface. The `mycoc explain` navigation between residual and e-graph
  views is a §22 concern, not a §19 concern.

  `Recommend:` Add to §22 a note cross-referencing §35 Phase 2 Q3 for the
  residual/e-graph navigation and round-trip items. These are the open design
  questions that most directly govern what §22 ships.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:1076`
  — Dual-mode `condition_of` rendering:**
  > "`mycoc explain` shows both modes with labels."

  The settled O2.4 decision that `condition_of` surfaces both algorithmic and
  problem conditioning (with labels) in `mycoc explain` output is a concrete
  commitment on §22's surface. Not mentioned in §22.

  `Recommend:` Add a bullet to §22 (or an §17 cross-reference) stating that
  `mycoc explain` labels dual-mode facts (algorithmic vs. problem conditioning,
  and analogously for any other dual-mode envelope content). Settled, narrow.

- **`planning/v2/spec.md` §14.5 line 3198 — Execution order in plan:**
  > "Execution order: the topologically sorted sequence of computation steps
  > within a timestep"

  The per-timestep execution trace is settled content of the plan report. §22
  does not name execution ordering as an inspectable surface even though it is
  essential for debugging integration behavior and SCC scheduling.

  `Recommend:` Part of the plan-report-contents bullet list recommended above.
  No separate action.

- **`planning/v2/spec.md` §14.5 line 3194-3195 — Numerical fallback reporting:**
  > "Numerical fallbacks: which `integrate` expressions require runtime
  > quadrature, what strategy was chosen, and how to override it."

  §22 mentions "verifying compilation choices" but does not specify that the
  choices include runtime-fallback selection and override paths. This surface
  is the user's entry point to configuring numerical solvers.

  `Recommend:` Part of the plan-report-contents bullet list. No separate action.

- **`planning/v2/v2.1_in_progress.md:1300` — Obligation-key replacement
  visibility:**
  > "Obligation keys (`balance(axial_flux)`) are stable semantic identifiers,
  > not user-chosen relation names. Plan inspection shows which default was
  > replaced."

  The specific commitment that plan inspection surfaces auto-generated-default
  replacement (e.g., `replaces balance(axial_flux)`) is settled but has no
  §22 landing.

  `Recommend:` Add to §22's plan-report-contents enumeration: "which
  auto-generated defaults were overridden via `replaces`, keyed by obligation
  identifier." Settled per v2.1_in_progress.md.

- **`planning/v2/spec.md` §13.1 (lines 2978-2980) "Plan representation":**
  > "The plan is backend-agnostic. From the closed residual graph, the emitter
  > derives executable code."

  The backend-agnosticism of the plan is a settled precondition for §22 (the
  plan is inspectable independently of which backend is selected). §22's
  "output artifact" framing implies this but does not state it. Backend
  abstraction is now in Part V.

  `Recommend:` Optional cross-reference from §22 to Part V stating that plan
  inspection is backend-agnostic; the plan is inspected before lowering to a
  specific backend.

- **`planning/v2/spec.md` Appendix B.5 / B.6 — Plan visualization and graph IR:**
  > "The compiler emits a backend-agnostic graph intermediate representation —
  > a JSON format with nodes, edges, clusters, and metadata (SCC membership,
  > solver strategy, path selection, constraint type, etc.). Thin adapters
  > render this IR to different targets."
  > Renderers: Graphviz, D2, Mermaid, Cytoscape.js.

  The visualization surface (graph rendering IR, multiple renderers, CLI
  invocation) is a §22-adjacent concern. spec_new.md places visualization
  outside the core spec (Part VII §38 Editor Tooling is a stub; §22 says
  nothing about visualization). Appendix B is flagged as "supersede wholesale"
  in anti_spec.md, so the specific Graphviz/D2/Mermaid/Cytoscape enumeration
  is implicitly retired.

  `Recommend:` If visualization ships at all in v2.1, §22 should reference §38
  (or wherever it lands). If visualization is Part VII deferred, that deferral
  should be named in §22 explicitly so users know inspection is text-only
  until Part VII work lands. Current §22 is silent on whether graphical
  inspection is supported.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:1087-1094`
  — Workflow configuration for `mycoc explain` / loss estimation:**
  > "`run.config.loss_estimation = { ... \"policy\": \"strict\" or \"permissive\" }`"

  The workflow-side verb that configures what `mycoc explain` surfaces (strict
  vs. permissive policy for unprovable approximation claims) is settled in
  chunk 04 but has no §22 reference.

  `Recommend:` Cross-reference from §22 to the workflow-side `run.config`
  surface for inspection policy. Ensures users know inspection fidelity is
  configurable.

---

## Conflicts

Direct contradictions between spec_new.md §22 and any corpus document.

- **None identified.**

  §22 is short enough and abstract enough that no corpus content contradicts
  it directly. All tensions with corpus are omissions (Homeless), not
  contradictions. The `myco plan` vs `mycoc explain` CLI-name drift (noted
  under Superseded) is a terminological drift in deprecated Appendix B, not a
  live conflict.
