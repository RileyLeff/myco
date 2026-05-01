# Audit: §1 Canonical Glossary

**Audited section:** `spec_new.md §1`
**Section state:** Skeleton. §1 contains only a term list:
`variable`, `relation`, `event`, `controller` (workflow-only), `initial`,
`temporal`, `data contract`, `locus`, `workflow`, `e-class`, `envelope`,
`universal`, `approximate`, `observe`. No definitions are written yet.

---

## Absorbed

Glossary entries whose term names appear in §1 and whose definitions exist
in the corpus without contradiction.

- **`variable`** — `planning/v2/v2.1_in_progress.md` lines 26-28:
  > "a named, typed entity declared as a field on a type or as a universal.
  Variables participate in relations. The `.myco` does not distinguish
  'computed' vs 'supplied' variables — every variable is on the same footing
  until a workflow binds sources."
  Term appears in §1 list; corpus definition is coherent and non-conflicting.

- **`relation`** — `planning/v2/v2.1_in_progress.md` lines 31-33:
  > "a symmetric constraint among variables. The compiler may invert a
  relation to solve for any participating variable depending on what the
  workflow pins down. Relations do not assign."
  Matches §1 listing without conflict.

- **`event`** — `planning/v2/anti_spec.md`: `rule` keyword retired in favor of
  `event`. Term appears correctly in §1. Definition settled in `open_questions.md`
  as first-class dynamic-topology trigger.

- **`controller` (workflow-only)** — `planning/v2/v2.1_in_progress.md` lines 48-51:
  > "Controller — workflow vocabulary for the callable attached to a variable
  by `bind_controller`. Not a `.myco` kind."
  §1 correctly parenthesizes `(workflow-only)`; matches corpus.

- **`universal`** — `planning/v2/v2.1_in_progress.md` lines 140-147 and
  `spec_new.md §3` one-liner:
  > "Module-scope typed names (`universal R: Scalar<J_mol_K>`) that every
  consumer in a run shares. Value comes from the workflow."
  Term in §1; corpus definition stable.

- **`workflow`** — `planning/v2/v2.1_in_progress.md` lines 22-23:
  > "the workflow supplies sources for variables whose values should come
  from outside the `.myco` relations. A single `.myco` supports many workflows."
  In §1; no conflict.

- **`e-class`** — `planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  lines 47-49 (Phase 2, Q1 locked):
  > "Each e-class represents a set of expressions claimed (or proven) equal."
  In §1; definition locked in chunk 04.

- **`envelope`** — `spec_new.md §0.1`:
  > "Layer 2 is envelope metadata: distributional, differentiability,
  invertibility, and observation facts keyed by e-class identity."
  In §1; consistent with chunk 04 Layer-2 description.

- **`locus`** — `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md`
  lines 76, 99-122: named structural singularities declared inside a `geometry`
  block (e.g., `locus pole where r = 0`). Settled in chunk 01. In §1.

- **`observe`** — `planning/v2/spec_dev_notes.md` lines 313-315:
  disambiguation note: "`observe` write" (layer-2 envelope fact) is distinct
  from workflow verb. Both senses are settled. Term in §1.

---

## Superseded

Glossary entries or corpus definitions for retired terms.

- **`data contract`** — §1 lists this term. `anti_spec.md` line 25 retires it:
  > "`DataContract` / 'data contract' as distinct contract kind | plain
  contracts satisfied by a type's output fields | workflow-layer visibility
  (`bind_controller(path, fn, contract)`) enforces access; no failure case
  found where a plain contract + output-type annotation is insufficient"
  The term is already in `anti_spec.md`. The §1 skeleton still lists it.
  Recommend: drop `data contract` from the §1 term list when definitions
  are written; it has been retired to `anti_spec.md`.

---

## Homeless

Corpus content that is stable, relevant to a canonical glossary, not
covered by §1, and not in `anti_spec.md`.

- **`node`** — `planning/v2/v2.1_in_progress.md` lines 95-96:
  > "`node name: Type` — instantiation. Creates a concrete instance in a
  model module. `node tree: SperryTree<...>`."
  This is a first-class language keyword central to the lib/bin split
  (library defines types; model modules instantiate with `node`). Missing
  from §1 entirely. `spec_dev_notes.md` flags `node` instantiation as a
  top-2 convergence gap.
  Recommend: add `node` to §1 with a one-line definition distinguishing
  it from `type` (type = shape descriptor; node = concrete instance in a
  model module).

- **`domain`** — `planning/v2/v2.1_in_progress.md` line 498:
  > "`type Soil : Domain<G = Euclidean<Dim = 2>> as (x: Scalar<m>, y: Scalar<m>)`"
  `Domain<G>` is the mechanism by which a type acquires spatial structure
  and spatial operators (`grad`, `diverg`, etc.). Absent from §1. Settled
  in chunk 01 and `v2.1_in_progress.md` §11.
  Recommend: add `domain` (or `Domain<G>`) to §1 as the spatial-structure
  supertype; one-line description noting that `geometry` describes kind of
  space, `Domain<G>` attaches it to a type.

- **`geometry`** — `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md`
  lines 43-53:
  > "A geometry is a first-class language construct declared with the
  `geometry` keyword. It describes the kind of space (metric structure,
  topology class, named singularities) but not any specific space."
  Fully settled in chunk 01; absent from §1.
  Recommend: add `geometry` to §1 alongside `locus` since `locus` is
  meaningless without a host `geometry`.

- **`~` (stochastic relational operator)** — `planning/v2/v2.1_in_progress.md`
  lines 584-603:
  > "`~` — stochastic relational operator. Parallel to `=` (deterministic
  equality)... `~` is used wherever a stochastic relationship holds in the
  world."
  `anti_spec.md` retires only the claim that `~` is an e-graph merge; the
  operator itself is first-class and locked. The lock is in `open_questions.md`
  (Tier 2 "Probabilistic inference integration" — RESOLVED). Absent from §1.
  Recommend: add `~` (stochastic relation) to §1 alongside `relation`.

- **`SCC` (strongly connected component)** — `planning/v2/v2.1_in_progress.md`
  lines 945-963: SCC detection and hierarchical SCC decomposition are
  central to the compiler substrate; "four-way SCC tag" and SCC-based
  lowering decisions appear throughout the spec. `spec_dev_notes.md` line 204
  names §20 SCC Decomposition as a must-add section. The term is used
  repeatedly across §0.1, §16, §19, §20, §21 without a glossary entry.
  Recommend: add `SCC` to §1; one line: "a maximal set of variables whose
  relations form a cycle; the compiler's unit of solver dispatch."

- **`residual graph`** — `planning/v2/anti_spec.md` line 55:
  > "residual graph as core semantic object | e-graph three-layer split
  (...); residual = user-facing projection"
  The term is retired as an independent semantic object but still used
  as the name for the user-visible diagnostic projection. `anti_spec.md`
  redirects it; `spec_dev_notes.md` lines 327-332 stub §19 Residual Graph.
  The term appears throughout the spec without a glossary definition.
  Recommend: add `residual graph` to §1 as "the user-visible diagnostic
  projection of the e-graph after extraction; not a separate substrate."

- **`binding verb`** — `planning/v2/v2.1_in_progress.md` lines 43-46:
  > "Binding verb — a workflow-side Python call that supplies a source for a
  variable. The v2.1 catalog: `assume_constant`, `assume_series`,
  `learn_constant`, `learn_initial`, `learn_trajectory`, `bind_controller`,
  `bind_topology`, `observe`."
  Fully stable; unlocks the reader's understanding of every workflow section.
  Absent from §1.
  Recommend: add `binding verb` (or `workflow verb`) to §1 with the
  eight-verb catalog named.

- **`bound variable` / `free variable`** — `planning/v2/v2.1_in_progress.md`
  lines 35-41:
  > "Bound variable (workflow vocabulary) — a variable for which the workflow
  has supplied a source... Free variable (workflow vocabulary) — a variable
  the workflow has not yet supplied a source for."
  These terms appear repeatedly in workflow binding discussions and are
  needed to understand the `residual graph` and `SCC` entries. Not in §1.
  Recommend: add both; mark as workflow vocabulary (parallel to `controller`).

- **`impl` / `some`** — `planning/v2/v2.1_chunk_reports/02_collections_iteration_report.md`
  lines 36-66: `impl` = heterogeneous element types (compile-time monomorphized);
  `some` = runtime variable sizing. Both appear in spec_new.md §3 stubs and
  throughout the language surface. Absent from §1.
  Recommend: add both to §1 as a paired entry distinguishing compile-time
  heterogeneity (`impl`) from runtime sizing (`some`).

- **`initial` / `temporal`** — listed in §1 but with no corpus definitions
  quoted here since they are skeleton entries. However, the settled definitions
  are in `open_questions.md` (RESOLVED) and `v2.1_in_progress.md`:
  `initial` = initial-condition block on a type body; `temporal` = state
  evolution block (hosts `d(x) = expr` or `step(x) = expr`). Both are stable
  but not yet written into §1. These are in the §1 term list already; flagged
  here only to note the definitions exist and should be drawn from the corpus.

- **Principles from `soul.md`** — none of the five soul.md principles
  (`The model is the science`, `The workflow is a separate concern`,
  `The compiler does the work`, `Structure is the regularizer`,
  `Generated code is the product`) appear as named glossary terms. They are
  stated in §0 but not collected in §1. The spec already refers back to them
  by name in §0.1. Whether they should anchor §1 entries is a question of
  glossary scope, but they could be cross-referenced as named principles
  rather than full definitions.
  Recommend: low-priority; soul.md principles are adequately housed in §0.1.

---

## Conflicts

- **`data contract` listed in §1 vs retired in `anti_spec.md`.**
  `spec_new.md §1` lists `data contract` as a current glossary term.
  `anti_spec.md` line 25 retires it explicitly with replacement:
  > "`DataContract` / 'data contract' as distinct contract kind | plain
  contracts satisfied by a type's output fields"
  The §1 term list and `anti_spec.md` directly contradict each other on
  whether this is a live concept.
  Recommend: remove `data contract` from the §1 term list. If the
  underlying concept (a contract used as `bind_controller`'s visibility
  argument) needs a name, use "output contract" or describe it inline under
  `controller`.

- **`approximate` listed in §1 vs its corpus definition as a block keyword.**
  §1 lists `approximate` as a standalone term. The corpus
  (`spec_dev_notes.md` line 278, `v2.1_in_progress.md`) consistently treats
  `approximate` as a block construct (`approximate { ... }`) rather than a
  freestanding concept. `spec_dev_notes.md` §15 stubs it as "`approximate`
  blocks." There is no corpus definition of `approximate` as a concept
  separate from its block syntax.
  Recommend: rename the §1 entry to `` `approximate` block `` and define it
  as the construct that enables a default-off rewrite set and tolerance
  tagging, rather than as a bare noun.
