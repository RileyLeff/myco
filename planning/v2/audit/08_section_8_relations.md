# Audit: §8 Relations and Equality

Corpus scanned: `planning/soul.md`, `planning/v2/spec.md`,
`planning/v2/spec_dev_notes.md`, `planning/v2/riley_project_note.md`,
`planning/v2/anti_spec.md`, `planning/v2/v2.1_in_progress.md`,
`planning/v2/open_questions.md`, chunk reports 01-07.

---

## Absorbed

Corpus content that already landed in spec_new.md §8.

- **`planning/v2/v2.1_in_progress.md` (Relations & Equations section):**
  > "Relation — a symmetric constraint among variables. The compiler may invert
  > a relation to solve for any participating variable depending on what the
  > workflow pins down. Relations do not assign."
  Absorbed into §8 summary ("Relations are world-claims that the compiler
  treats as equational merges") and §8.1 constraint/relation distinction.

- **`planning/v2/v2.1_in_progress.md` (Closure policies, `constraint` section):**
  > "Inline constraint sugar: `water: Potential { self <= 0 MPa }` is sugar
  > for a named constraint at containing type scope."
  Absorbed into §8.5 Inline Relation and Constraint Sugar.

- **`planning/v2/v2.1_in_progress.md` (`let` binding semantics):**
  > "`let` bindings: named subexpressions for readability, not mutation"
  Absorbed into §8.2 `let` Bindings in Relation Bodies.

- **`planning/v2/v2.1_in_progress.md` (Runtime `where` and smoothing):**
  > "Myco treats smoothing as a model claim, not a workflow rewrite: if the
  > user wants a smooth transition, they write the smooth form in the `.myco`
  > file. The compiler never silently rewrites semantics based on compile mode."
  > Stdlib ships `smooth_threshold`, `smooth_max`, `smooth_min`, `smooth_pw`.
  Absorbed into §8.9 Smoothing as a Model Claim.

- **`planning/v2/v2.1_in_progress.md` (Three-way overdetermination
  classification):**
  > "The compiler distinguishes three cases when a system has more equations
  > than unknowns: redundant/consistent, provably inconsistent (hard compile
  > error), conditionally inconsistent (runtime assertion)."
  Absorbed into §8.6.

- **`planning/v2/v2.1_in_progress.md` (Closure policies stdlib list):**
  > "`weighted_average`, `soft_select(preference, sharpness)`, `hard_select`
  > ... All three are ordinary `.myco` functions users could write themselves."
  Absorbed into §8.7 Y1-Y3.

- **`planning/v2/v2.1_in_progress.md` (`replaces` with obligation keys):**
  > "`relation leaky_junction on junction replaces balance(axial_flux): ...`"
  Absorbed into §8.10 Generated-Defaults and Obligation Keys.

- **`planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` (locus-scoped
  overrides with obligation keys):**
  > "`relation tank_dynamics on junction replaces balance(axial_flux): ...`"
  Absorbed into §8.10.

- **`planning/v2/open_questions.md` (§8 coverage note):**
  > "§8 Relations and Equality: 8.1-8.5 stubbed ... 8.6 system-level
  > overdetermination classification, 8.7 closure policies Y1-Y6 (includes
  > Y6 C(N,M)), 8.8 Y5 user-defined, 8.9 smoothing as model claim, 8.10
  > generated-defaults with obligation keys."
  All ten subsections confirmed absorbed.

- **`planning/v2/anti_spec.md` (closed open questions):**
  > "`condition_weighted` deferred — resolved — ships via `condition_of`
  > Levels I-III (chunk 04 O4.5)"
  Absorbed into §8.7 Y4 description and §8.8 `condition_of` cross-reference.

- **`planning/soul.md` principle 4:**
  > "Overdetermined relations, constraints, temporal dynamics, and cross-quantity
  > coupling are not inconveniences to be simplified away. They are the signal."
  Absorbed into §8 summary framing ("Overdetermination is not an error").

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §8. Items
already listed in `anti_spec.md` are noted and skipped.

- **`planning/v2/spec.md` §8.5 / §14.6 (structural introspection for closure
  policies):**
  > "The metadata is available to both standard library closure policies and
  > user-defined ones via structural introspection on the competing paths."
  > (spec.md line ~2073-2074)
  > "The `condition_weighted` policy uses condition-number estimates to weight
  > paths." (spec.md §14.2 / §14.6)
  Superseded by spec_new.md §8.7-§8.8: Y5 policies receive "candidate values
  plus user-supplied hyperparameters" only; no structural introspection.
  Already in `anti_spec.md` (`structural introspection (<: predicate,
  §5.5/§8.5) | nothing | closure policies see values + hyperparameters only`).
  **Already covered; skip.**

- **`planning/v2/v2.1_in_progress.md` (`condition_weighted` deferral):**
  > "`condition_weighted` deferred. ... Ship `condition_weighted` post-v2.1
  > if demand emerges."
  Superseded by spec_new.md §8.7 Y4: `condition_weighted` ships, backed by
  `condition_of` Levels I-III (§14). Also noted in `anti_spec.md` closed
  questions. **Already covered; skip.**

- **`planning/v2/v2.1_chunk_reports/03_kernels_in_progress.md` §8 (closure
  policy section's deferral of `condition_weighted`):**
  > "Whether `condition_weighted` closure policy gets resurrected with a
  > `condition_of(expr)` intrinsic now that we're taking cost modeling
  > seriously" (open question list, §8)
  Superseded by spec_new.md §8.7 Y4 and chunk 04 lock. `anti_spec.md`
  explicitly flags: "chunk 03 §8 `condition_weighted` deferral — pre chunk-04."
  **Already covered; skip.**

- **`planning/v2/spec.md` §4.11 and surrounding prose (bare dimensioned
  literals in relations):**
  > "No bare dimensioned literals in relations. The compiler rejects them."
  This governs relation bodies. Superseded by CC1 (chunk 04), which locks
  the literal-numerics policy more precisely. `anti_spec.md` flags
  `spec.md §4.11` as stale. Not directly a §8 claim but affects relation
  bodies. **Already covered; skip.**

---

## Homeless

Corpus content relevant to §8 that is not accounted for in spec_new.md §8
and is not committed to `anti_spec.md`. This is the highest-value bucket.

- **`planning/v2/spec.md` §4.7 (named-type equality and comparison in relation
  context):**
  > "Named-type compatibility for equality and comparison. Both sides of `=`,
  > `>=`, `<=`, `>`, `<` must be named-type-compatible: either the same named
  > type, or one side is anonymous scalar."
  > "Without this rule, the type system would catch `CarbonPool + WaterPool`
  > in arithmetic but silently allow equating them in a relation."
  `spec_dev_notes.md` records "Named-type equality / comparison rules
  (DEFERRED — decide §3 vs §7 later)". §8 is the natural home for the rule
  that governs `=` inside relation bodies, but spec_new.md §8 is silent on
  named-type constraints at the relation level. The decision was deferred, not
  retired.
  **Recommend:** once the §3 vs §7 placement question resolves, add a
  cross-reference from §8 to wherever the named-type equality rule lands. If
  it lands in §8, the rule itself needs drafting. Not yet homeless in the sense
  of a stable decision omitted, but the gap is real and should be tracked.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  §6 (merge sources enumerated):**
  > "Explicit relation equations. Every `a = b` in a relation.
  > Non-negotiable." (merge source #1)
  > "Closure-policy co-membership. When two relations both define the same
  > quantity, both definitions become co-members of one e-class; the closure
  > policy selects among them at extraction time. Merge happens regardless of
  > policy." (merge source #7)
  These are settled decisions in chunk 04 (marked LOCKED) that directly
  govern what `relation =` means semantically in §8. Spec_new.md §8 describes
  closure policies and overdetermination classification but never states the
  foundational invariant: every `=` in a relation body introduces an e-class
  merge, and co-defining relations share one e-class. §8's framing ("equational
  merges") gestures at this but never commits the rule explicitly.
  **Recommend:** add one sentence to §8 (or a cross-reference to §16) that
  names the e-graph merge as the mechanism behind "equational merges." The
  chunk 04 decision is stable and should not remain implicit in §8.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  §3 / §6 (non-equational constraints live in Layer 2 envelope, not Layer 1):**
  > "Non-equational constraints — inequalities, domain bounds, type-level
  > predicates. Attached to the class carrying the constrained expression.
  > Intersection rule under merge."
  This is a settled chunk 04 decision (decision #6 in the settled table) that
  explains why `constraint` blocks behave differently from `relation` blocks.
  Spec_new.md §8.1 says constraints "don't merge e-classes; they restrict the
  admissible solution set" but does not state the positive claim: they live as
  Layer 2 envelope metadata on the e-class. Without that, the "don't merge"
  claim reads as a fact without a mechanism.
  **Recommend:** add a cross-reference from §8.1 to §16 (E-Graph) for the
  Layer 2 envelope placement.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  §9 / §10 (the `approximate` block and lossy-model equalities):**
  > "It has no syntactic surface for lossy-model or lossy-tolerance, despite
  > discussing them in prose and depending on them implicitly for closure
  > policies, kernel optimization, and numerical conditioning. This is a real
  > gap the commitment must address."
  > "Proposed `approximate` block: one keyword, four cells of the 2x3."
  This is in-progress design (chunk 04 marks `approximate` tier as shipping
  in v2.1, decision #29). The relation surface in §8 currently describes only
  lossless `=`. The `approximate` block, when locked, will be a relation-body
  construct and belongs alongside `relation`, `constraint`, and inline sugar.
  Per the prompt instructions, in-progress open design absent from spec_new.md
  is NOT homeless. Flagged here for completeness only; not a find.

- **`planning/v2/spec.md` §3.4.1 (contract default relations as a relation
  form):**
  > "A default relation is included if and only if the implementation does not
  > provide its own relation for that output. This is simple fallback, not
  > conflict resolution."
  This is a stable, settled decision (marked "Status: settled" in
  `v2.1_in_progress.md`). It governs a distinct form of relation (contract
  defaults with fallback semantics, distinct from overdetermination). Spec_new.md
  §8 does not mention this. It may belong in §3 (Types), but §8 is the section
  on relation semantics and the fallback rule is a semantic claim about how
  relation bodies interact.
  **Recommend:** verify that contract default relation fallback semantics are
  covered in spec_new.md §3 (Types/Contracts). If not present there, this is a
  genuine homeless stable decision that needs a home in either §3 or §8.

- **`planning/v2/v2.1_in_progress.md` (`where` preconditions on `convert`
  bodies):**
  > "`where` preconditions work on converts. The three-way overdetermination
  > classification applies: provably true → elide check, provably false →
  > compile error, undecidable → runtime assertion."
  `open_questions.md` confirms three layers of `where`: §8.3 compile-time
  narrowing, §12.7 collection filtering, §23.1 workflow-composition `where`.
  Spec_new.md §8.3 covers the `if` / `else` vs `where x is T` distinction
  cleanly. However, the `where`-on-`convert` application of the
  three-way classification is not mentioned in §8.3 or in §8.6. It is a
  third application of the same classification machinery and may warrant a
  cross-reference.
  **Recommend:** add a note to §8.6 or §8.3 that the three-way classification
  also applies to `where` preconditions on `convert` bodies (already specified
  in v2.1_in_progress.md; needs a home in spec_new.md).

---

## Conflicts

Direct contradictions between spec_new.md §8 and corpus documents.

- **`condition_weighted` shipped vs. deferred:**
  spec_new.md §8.7 lists Y4 `condition_weighted` as a shipping closure policy,
  "backed by `condition_of` Levels I-III (§14)."
  `planning/v2/v2.1_in_progress.md` (lines 1015-1021) still reads:
  > "`condition_weighted` deferred. ... Ship `condition_weighted` post-v2.1
  > if demand emerges."
  The anti_spec closed-questions table resolves this ("resolved — ships via
  `condition_of` Levels I-III (chunk 04 O4.5)"), and the dev_notes confirm the
  resolution. The conflict is between spec_new.md §8.7 and the unedited prose
  still present in `v2.1_in_progress.md`. The source of truth is spec_new.md;
  the `v2.1_in_progress.md` prose is stale but `anti_spec.md` already flags it.
  **Recommend:** no action needed in spec_new.md §8. The `v2.1_in_progress.md`
  deferral prose should be struck or annotated as superseded in that document,
  but that is a maintenance task on the legacy doc, not a §8 defect.

- **`spec.md` §14.2 / §14.6 claims `condition_weighted` is in the stdlib and
  accesses metadata via structural introspection:**
  > "The `condition_weighted` policy in `myco::closure` provides conditioning-
  > aware blending. See section 8.5 for how the operation algebra informs the
  > policies." (spec.md §14.2)
  > "user-defined ones via structural introspection on the competing paths."
  > (spec.md §14.6 / line ~2073)
  spec_new.md §8.7 says `condition_weighted` is Y4, shipped, with policies
  receiving "candidate values plus hyperparameters" only (no introspection).
  This is a direct contradiction between spec.md and spec_new.md §8. The
  anti_spec already lists `spec.md §14.6` as stale wholesale, and structural
  introspection as retired. The conflict lives in the legacy doc, not in
  spec_new.md.
  **Recommend:** no action needed in spec_new.md §8. Confirm `spec.md §8.5`,
  `§14.2`, and `§14.6` are fully superseded and not being imported into any
  new section.
