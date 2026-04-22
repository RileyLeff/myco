# Audit: spec_new.md §4 — Values and Literal Policy

Section under audit covers the CC1 literal-numerics rule and its diagnostic
surface (§4.1). Corpus search covered `literal`, `CC1`, `universal`,
`assume_constant`, `value position`, `structural position`, `physical constant`,
`boolean`, `string literal`, `default value`, and related terms across all
listed corpus files.

---

## Absorbed

Corpus content that already landed in spec_new.md §4.

- **`planning/v2/spec_dev_notes.md` (CC1 decision block, 2026-04-20)** —
  "`.myco` permits literal numerics only in unit definitions, affine conversion
  bodies, and structural positions (shape tuples, indices, generic-parameter
  definitions). All values enter from the workflow." The spec_new.md §4 prose
  reproduces this three-exception enumeration verbatim.

- **`planning/v2/spec_dev_notes.md` (CC1 diagnostic surface, 2026-04-20)** —
  "The compiler diagnostic surface for CC1 violations is not specified.
  Flagged in the merged audit under 'Other Opens.'" The spec_dev_notes notes
  this is blocking for implementation, and spec_new.md §4.1 responds with the
  CC1 Diagnostic Surface subsection that specifies the error shape, the
  position-kind field, and the canonical workflow-verb pointer.

- **`planning/v2/spec_dev_notes.md` (CC1 implication for universals)** —
  "Universals declare *types*, not values." Absorbed into spec_new.md §4's
  framing that all numeric values enter from the workflow, with `assume_constant`
  or `assume_series` (§24) as the named supply verbs.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  §CC1 (LOCKED block)** — "`.myco` does not permit naked numeric values.
  Hard-coded numerics are a workflow concern, not a world claim." The three-
  exception enumeration from chunk 04 is absorbed; the philosophical lock
  ("structure vs values surfaces do not mix") is implicit in spec_new.md §4's
  summary.

- **`planning/v2/anti_spec.md` (retired items)** — Two directly relevant
  retired items:
  - "literal numerics in `.myco` value position | CC1: banned except in unit
    defs, affine conversion bodies, structural positions"
  - "universals carrying values (`universal R: Scalar<U> = 8.314`) | ...
    value from workflow | CC1 scope"
  Both are in anti_spec.md, consistent with spec_new.md §4's positive-only
  framing.

- **`planning/v2/anti_spec.md` (stdlib physical constants)** — "stdlib physical
  constants (`R`, `Avogadro`, etc.) | workflow-injected via `assume_constant` |
  physical constants are values; values live workflow-side." Consistent with
  §4's statement that all numeric values enter from the workflow.

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §4.

- **`planning/v2/spec.md` §4.11 (universals with inline values)** — "universals
  are valued inline in the `.myco` file ... `universal R: Scalar<J_mol_K> =
  8.314`" and "`param` declares an empirical parameter whose value comes from
  the workflow." Both `param` and inline-valued `universal` are superseded by
  CC1. Already in anti_spec.md (`param` retired; universals-with-values retired).
  No further action needed.

- **`planning/v2/spec.md` §4.11 (dimensionless literals exemption)** —
  "Dimensionless integers (0, 1, 2) and dimensionless ratios used in
  mathematical structure are exempt ... `1.6` as the stomatal diffusivity ratio,
  `0.25` as a fraction in an equation." This is a broader exemption than CC1
  allows. CC1 in spec_new.md restricts exemptions to structural positions (shape
  tuples, indices, arity), not arbitrary dimensionless ratios in relation bodies.
  The old spec's exemption for arbitrary dimensionless fractions like `1.6` and
  `0.25` in equations is superseded. Not yet in anti_spec.md. Should be added.

- **`planning/v2/v2.1_in_progress.md` §"Values & Literals" (inline universal
  value)** — "`universal R: Scalar<J_mol_K> = 8.314`. Status: settled." This
  is superseded by CC1; the inline value is no longer settled. Already covered
  by anti_spec.md retirement entry for universals-with-values.

- **`planning/v2/v2.1_in_progress.md` §"No bare dimensioned literals"** —
  "Compiler rejects any dimensioned number not attached to a `universal`.
  Dimensionless integers (0, 1, 2) and dimensionless ratios are exempt." The
  `universal`-attachment escape and the dimensionless-ratio exemption are both
  narrower or broader in conflicting ways relative to CC1. Superseded by the
  spec_new.md §4 rule (workflow-bound universals; only structural positions
  exempt). Already substantially covered by anti_spec.md; see also the Conflicts
  section below for the third-exception divergence.

---

## Homeless

Corpus content relevant to §4, not accounted for in spec_new.md §4, and not
already committed to anti_spec.md.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  §CC1 (third exception: π, e as named identifiers)** — Chunk 04's CC1 lock
  names the third exception as "Pure mathematical constants — π, e. Symbolic
  references in `.myco`; folded to float at lowering time. Not a value, a
  reference to a known irrational." The spec_dev_notes 2026-04-20 entry later
  reframes this: "Symbolic constants (π, e) are named identifiers in the
  numeric stdlib — they don't require an exception slot because they're not
  literals." spec_new.md §4 names the third exception as "structural positions
  (shape tuples, indices, generic-parameter definitions)" with no mention of
  π/e at all. The question of where π and e live (stdlib named identifiers, or
  a separate exception slot) has a stated answer in spec_dev_notes but is absent
  from §4.
  `Recommend:` Add a one-line note to §4 clarifying that π and e are stdlib
  named identifiers, not numeric literals, and therefore require no exception
  slot. This closes the open-question thread from chunk 04 and prevents future
  confusion about whether writing `pi` or `3.14159` in a `.myco` body is a CC1
  violation.

- **`planning/v2/open_questions.md` §"Literal constants in `.myco`" (Tier 2)**
  — This section still presents the question of whether `.myco` should have any
  literal values as open, with "motivations for keeping them" including
  dimensionless ratios in equations. Its presence in open_questions.md as a Tier
  2 question is inconsistent with CC1 being locked. The section has not been
  struck or marked RESOLVED.
  `Recommend:` Mark the open_questions.md §"Literal constants in `.myco`"
  entry as RESOLVED with a pointer to the CC1 lock in spec_dev_notes.md and
  spec_new.md §4. The resolution answers the question: no literal values except
  the three structural positions. The "keep them" motivation for dimensionless
  ratios is superseded by the stricter rule.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  §CC1 consequence for rewrite set** — "B2 (universal-to-literal substitution)
  ceases to be a compile-time fold. Workflow-bound values enter the e-graph as
  observation-style equalities at workflow composition." This is a compiler-
  semantics consequence of CC1 that is locked in chunk 04 but has no home in
  spec_new.md §4 or in §17 (equality-introducing machinery). It is a stable
  decision (locked, not open design work).
  `Recommend:` Add a cross-reference in spec_new.md §4 (or §17 workflow-constant
  injection) to note that workflow-bound constants enter the e-graph as
  observation-style equalities (V1-style), not as compile-time literal folds.
  This cements the B2 consequence and makes the §17 eight-source enumeration
  legible for workflow constants.

---

## Conflicts

Direct contradictions between spec_new.md §4 and corpus documents.

- **Third exception position: spec_new.md §4 vs chunk 04 CC1 lock**

  spec_new.md §4 names the third exception as:
  > "structural positions (shape tuples, indices, generic parameters)"

  chunk 04 CC1 lock (`04_egraph_foundation_in_progress.md`, line 1360) names
  the third exception as:
  > "Pure mathematical constants — π, e. Symbolic references in `.myco`;
  > folded to float at lowering time."

  spec_dev_notes.md (2026-04-20) reconciles these by treating π and e as stdlib
  identifiers (not literals requiring an exception slot) and naming the third
  exception as structural positions. spec_new.md §4 reflects the spec_dev_notes
  resolution. But chunk 04 was not updated to match, so chunk 04 and spec_new.md
  §4 enumerate different third exceptions without any cross-reference explaining
  the reconciliation.

  `Recommend:` No change to spec_new.md §4 (it reflects the later, authoritative
  spec_dev_notes decision). Add a note in spec_dev_notes.md or a comment in
  chunk 04 that the π/e third-exception from CC1 was superseded: π and e are
  stdlib named identifiers; the third structural exception slot was reassigned
  to shape tuples/indices/arity. This is a documentation consistency fix, not a
  design change.

- **Dimensionless-ratio exemption: spec.md §4.11 vs spec_new.md §4**

  spec.md §4.11:
  > "Dimensionless integers (0, 1, 2) and dimensionless ratios used in
  > mathematical structure are exempt. For example, `1.6` as the stomatal
  > diffusivity ratio, `0.25` as a fraction in an equation ... are all
  > permitted as bare dimensionless literals."

  spec_new.md §4:
  > "Three exception positions where literals are allowed: unit definitions,
  > affine conversion bodies, and structural positions (shape tuples, indices,
  > generic parameters)."

  A dimensionless ratio like `1.6` in a relation body is not in any of the
  three exception positions and is therefore a CC1 violation under spec_new.md
  §4. The old spec explicitly permitted it. This is a direct conflict. spec.md
  §4.11 is flagged as stale in anti_spec.md, but the specific dimensionless-
  ratio exemption is not yet listed there as retired.

  `Recommend:` Add an entry to anti_spec.md retiring the "dimensionless ratios
  in relation bodies are exempt" rule. The CC1 replacement: all non-structural
  literal numerics are value-position violations regardless of dimensionality;
  values enter from the workflow.
