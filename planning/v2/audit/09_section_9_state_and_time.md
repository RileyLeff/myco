# Audit — §9 State and Time

Corpus surveyed: `planning/soul.md`, `planning/v2/spec.md`,
`planning/v2/spec_dev_notes.md`, `planning/v2/riley_project_note.md`,
`planning/v2/anti_spec.md`, `planning/v2/v2.1_in_progress.md`,
`planning/v2/open_questions.md`, and chunk reports 01-07.

§9 of `spec_new.md` covers: `initial:` / `temporal:` block placement,
`d(x) = expr` (ODE), `step(x) = expr` (discrete), `dt` provision, and
per-path uniqueness after generic expansion (§9.2).

---

## Absorbed

Content that already landed in spec_new.md §9.

- **`planning/v2/v2.1_in_progress.md` lines 438-444** — "`d(x)` declares a
  continuous rate (compiler owns integration); `step(x)` declares a discrete
  update (compiler applies RHS verbatim each tick, RHS reads prior values, LHS
  writes current)." Absorbed: §9 Summary and §9.1 restate both forms and their
  integration-ownership rule.

- **`planning/v2/v2.1_in_progress.md` lines 469-473** — "Module-scope
  `initial`/`temporal` forms are retired. The v2.0 spec's 'implicit top-level
  type' mechanism ... is removed." Absorbed: §9 Summary states "Module-scope
  relations reserved for truly cross-entity cases."

- **`planning/v2/v2.1_in_progress.md` lines 492-510 (locus-scoped temporals)**
  — "`temporal name on locus:` is legal by symmetry with `relation name on
  locus:`." Absorbed: the §9 Summary mentions `initial:` and `temporal:` blocks
  in type bodies and module-scope for cross-entity cases, which covers the
  scoping rule. (The locus-scoped variant is referenced in the v2.1_in_progress
  section as `temporal surface_drying on boundary where y = surface_y:`; the
  fly-on-a-horse worked example at line 1879 also exercises `temporal
  thermoregulation: d(body_temp) = ...` inside a type body.)

- **`planning/v2/open_questions.md` "Temporal Semantics — `d(x)` vs `step(x)` —
  RESOLVED"** — "Two surface forms with distinct semantics. `d(x) = expr` --
  continuous ODE. `step(x) = expr` -- discrete update." Absorbed: §9 repeats the
  resolution verbatim.

- **`planning/v2/open_questions.md` "`dt` as workflow-layer concern — RESOLVED"**
  — "For `d(·)`, `dt` is not referenced in the model -- the compiler owns the
  integration step size... For `step(·)`, the tick cadence is set at the workflow
  layer via `assume_constant('config.dt', …)`." Absorbed: §9.1 restates this in
  full.

- **`planning/v2/anti_spec.md` retirement table row** — "`[t+1]` / `[t]`
  temporal subscripts | `d(x) = expr` (ODE) / `step(x) = expr` (discrete) |
  subscripts conflated kinds." Absorbed: §9 Summary line "No `[t+1]` subscript
  surface."

- **`planning/v2/anti_spec.md` retirement table row** — "`rate()` | `d(x) =
  expr` | same." Not named in §9, but `rate()` retirement is already in
  anti_spec.md and §9 does not need to re-litigate it.

- **`planning/v2/anti_spec.md` retirement table row** — "module-scope
  `initial:` / `temporal:` per-type | in-type-body `initial:` / `temporal:` |
  module-scope kept only for truly cross-entity relations." Absorbed: §9 Summary
  first sentence matches.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  Section 4** — "temporal is not a special axis. It is just indexing that
  produces distinct terms... `y[1]`, `y[2]`, ... are distinct ground terms —
  they are not claimed equal anywhere." Absorbed: §9 Summary last sentence,
  "temporal indexing produces distinct e-graph ground terms (`y[1]`, `y[2]`, …)
  with structural relations between them (§16)."

---

## Superseded

Content that has been replaced by a newer decision in spec_new.md §9. Moved to
anti_spec.md unless noted otherwise.

- **`planning/v2/spec.md` §6.3 lines 1374-1376** — shows the old subscript form
  in a structural-introspection block: `seg.min_historical_pressure[t+1] =
  min(seg.min_historical_pressure[t], seg.core.water_potential[t])`. Superseded
  by `step(x) = expr` (§9 Summary: "No `[t+1]` subscript surface"). Already in
  anti_spec.md (`[t+1]` / `[t]` row). The spec.md occurrence is in a section
  flagged stale by anti_spec.md ("spec.md §2.5, §4.11, §7, §5.5, §8.5, §6.3,
  §12, §13.2-13.3, §14.6, Appendix A/C — supersede wholesale").

- **`planning/v2/spec.md` §6.3 lines 1614-1617** — "`x[t+1] = x[t] + dt *
  rate` would (1) hardcode the integration scheme into the model." This is an
  accurate rationale but written in the voice of justifying the old `[t+1]` form.
  In spec_new.md the rationale is present only implicitly (§9 states the form
  without defending it against the retired form). The `[t+1]` form is already
  retired in anti_spec.md; this spec.md prose belongs to the stale-flagged §6.3.

- **`planning/v2/spec.md` §2 lines 144-168** — module-scope `temporal` and
  `slot` declarations attached to a single top-level type via an "implicit
  top-level type" mechanism. Superseded by in-type-body scoping (§9 Summary).
  Already in anti_spec.md ("module-scope `initial:` / `temporal:` per-type"
  row). The spec.md §2 occurrence is stale-flagged.

- **`planning/v2/spec.md` §7 (lines 1652-1699)** — `slot` keyword, `[*]`
  wildcard, temporal edges `[t]` → `[t+1]` used in reachability scope
  description. Superseded wholesale. Already in anti_spec.md (`slot` /
  `learn_slot` / `bind_slot` row; `[*]` wildcard row; `[t+1]` / `[t]` row).

- **`planning/v2/spec.md` §8.5 structural introspection** (referenced in
  open_questions.md line 585) — "via structural introspection on the competing
  paths." Superseded; structural introspection is retired. The phrase lives in
  the stale-flagged §8.5. open_questions.md Spec Maintenance section calls for
  patching this. Not yet landed in spec_new.md §9 (does not intersect §9
  directly, but the scoping change from §9 is what made structural introspection
  obsolete for temporal blocks).

---

## Homeless

Corpus content that is relevant to §9, not accounted for in spec_new.md §9, and
not already committed to anti_spec.md.

- **`planning/v2/v2.1_in_progress.md` lines 475-482 (per-path uniqueness,
  expanded statement)** — "Each fully-qualified temporal quantity path receives at
  most one `initial` equation and at most one `temporal` declaration, counted
  *after* expansion across instances. Type-body declarations expand into
  per-instance equations (one per instantiation of the type); the per-path rule
  catches the case where a module-scope declaration and a type-body declaration
  (or two type-body declarations via nested types) both target the same resolved
  path. Compile error on duplicates, with a diagnostic identifying both sources."
  Spec_new.md §9.2 covers the generic-event expansion case but omits the
  type-body instance expansion case entirely. This is a stable decision (Status:
  settled in v2.1_in_progress) that was not ported. **Recommend: expand §9.2 to
  include type-body-per-instance expansion as a second source of duplicate
  obligation keys, not just generic event expansion.**

- **`planning/v2/v2.1_in_progress.md` lines 484-490 ("No override mechanism")**
  — "Myco has no implementation inheritance. One declaration per path, period...
  Contracts describe *interface shape*, not *state evolution* -- contracts cannot
  declare `initial`/`temporal`, and they have no override story for these forms."
  This is a settled scoping rule with correctness implications (a user who tries
  to override temporal evolution via a contract gets a compile error, not silent
  behavior). It is absent from spec_new.md §9. **Recommend: add a sentence to §9
  stating that contracts cannot carry `initial`/`temporal` declarations and that
  there is no override mechanism for state evolution; the compile-error-on-
  duplicate rule in §9.2 is a consequence of this, not a standalone fact.**

- **`planning/v2/v2.1_in_progress.md` lines 492-510 (locus-scoped temporals,
  full design)** — "`temporal name on locus:` is legal by symmetry with `relation
  name on locus:`. State evolution that applies only at a specific locus of a
  domain (e.g., surface evaporation at a soil domain's top boundary, distinct
  from bulk diffusion) is expressible directly." The spec_new.md §9 Summary
  mentions `initial:` and `temporal:` blocks in type bodies, but does not mention
  locus-scoped temporals at all. This is a settled, spec_new.md-adjacent design
  that already appears in the geometry report and worked examples. **Recommend:
  add a §9.3 (or subsection) for locus-scoped `temporal name on locus:` blocks,
  parallel to locus-scoped relations, and reference the geometry section for
  spatial operator semantics.**

- **`planning/v2/spec.md` §6.3 lines 1529-1534 (read/write semantics of
  `step(·)`)** — "Within a `step(·)` equation, unsubscripted RHS references read
  the *previous tick's* value, and the LHS writes the *current tick's* value...
  `step(a) = b` and `step(b) = a` together form a swap, not a cycle, because
  both RHSs read pre-tick values." This semantics statement is critical for
  simultaneous-update correctness and is absent from spec_new.md §9. The
  v2.1_in_progress temporal section (lines 1818-1821) mentions "RHS reads
  prior-tick values; LHS writes current-tick values" briefly, but the swap-not-
  cycle consequence is only in spec.md §6.3. **Recommend: include the
  read/write semantics rule explicitly in spec_new.md §9 (one sentence on
  pre-tick RHS, one sentence on the swap-not-cycle consequence).**

- **`planning/v2/spec.md` §6.3 lines 1536-1539 (mixing `d(·)` and `step(·)`)**
  — "Both forms may appear in the same model. They live in different semantic
  worlds: `d(·)` variables are advanced by the integrator between ticks; `step(·)`
  variables update at tick boundaries." This interaction rule is stable and absent
  from spec_new.md §9. **Recommend: add one sentence to §9 stating that `d(·)`
  and `step(·)` may coexist and that the compiler composes them without user-level
  coordination.**

- **`planning/v2/spec.md` §6.3 lines 1558-1602 (initial block full semantics)**
  — The four mutually-exclusive initialization mechanisms (`initial` block in
  `.myco`, `assume_initial`, `learn_initial`, `learn_trajectory`) and the
  compiler-error-on-missing-initialization diagnostic are fully specified in
  spec.md §6.3 but are entirely absent from spec_new.md §9. Spec_new.md §9 only
  mentions `initial:` blocks in passing in its Summary. This is a stable,
  locked design. **Recommend: add a §9.x covering the four initialization
  mechanisms, their mutual-exclusion rule, and the compiler diagnostic for
  unresolved initial conditions. This is the most substantial gap in §9.**

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  Section 4, lines 154-168 (dead-entity temporal semantics)** — "Things do not
  know they are dead. Entity existence at timestep T is implicit in whether
  relations reference it at T... t >= 1001: no new relations reference bunny
  #50's state." This is a settled consequence of the referential-truth design
  and touches temporal semantics (how temporal state propagates for dead
  entities). It is absent from spec_new.md §9. However, the primary home for
  this may be §10 (Events), where referential-truth semantics are addressed.
  **Recommend: verify §10 covers dead-entity temporal behavior; if not, add a
  note in §9 that persistent state for an entity that is no longer referenced by
  any active relation simply stops receiving updates (no tombstoning needed).**

---

## Conflicts

Direct contradictions between spec_new.md §9 and corpus documents.

- **spec_new.md §9 vs `planning/v2/spec.md` §6.3 on `dt` reservation.**
  Spec_new.md §9.1: "`dt` is not a reserved name in `.myco`, not a universal,
  not a special verb." The legacy spec.md §6.3 line 1552 says "It is not a
  reserved name" -- no direct contradiction. However, spec.md §6.3 line 1077
  (outside the temporal section, inside a carbon-dynamics context) contains
  the phrase "naturally in temporal equations: `carbon.C[t] + dt * (flux_expression)`
  where `dt` appears as an ordinary quantity." This usage treats `dt` as a
  regular world-model quantity that may appear in `.myco` arithmetic, consistent
  with spec_new.md §9.1. No actual conflict -- the two descriptions are
  compatible.

- **spec_new.md §9.2 title and §10 scope overlap.** Spec_new.md §9.2 is titled
  "Per-Path Uniqueness After Generic Expansion" and states "A generic event or
  relation ... expands at compile time to one concrete instance per T-satisfier."
  However, §10 (Dynamic Topology and Events) §10.2 covers "Generic Event
  Cartesian-Product Expansion" in full. The §9.2 content describes generic
  events using the phrase "obligation key" -- which is also the vocabulary of
  §10.2 ("Each expanded path has its own obligation key (§9.2)"). §10.2 cites
  §9.2 as the definition site for obligation keys, yet §9.2 does not actually
  define what an obligation key is; it only says duplicate keys are a compile
  error. This is a definition gap rather than an outright contradiction, but it
  creates a circular cross-reference. **Recommend: §9.2 should define
  "obligation key" explicitly (the canonical path string identifying a
  fully-expanded temporal/initial/relation instance), so §10.2's back-reference
  is grounded.**
