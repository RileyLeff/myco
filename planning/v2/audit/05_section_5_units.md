# Audit: §5 Units — spec_new.md

Audited against the corpus listed in the task brief. §5 as fetched contains
three subsections: 5.1 (convert four variants), 5.2 (round-trip verification),
5.3 (`value_in` operator).

---

## Absorbed

Content from the corpus that already landed in spec_new.md §5.

- **`planning/v2/spec.md` §4.7 — four convert forms:** "Unit and named-type
  conversions come in four forms" covering bare bidi (`<->`), bare one-way
  (`->`), parameterized bidi, parameterized one-way. §5.1 absorbs this
  taxonomy.

- **`planning/v2/v2.1_in_progress.md` §174-225 — convert surface prose:**
  > "Units flow through convert bodies the same way they flow through any
  > relation — the compiler does dimensional analysis on the body. Bare
  > converts require the same dimension on both sides."
  The same-magnitude-alias rule in §5.1 ("declares A and B as same-magnitude
  aliases") captures this constraint.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  O2.1 — round-trip verification:** The chunk 04 commitment to trust
  `convert <->` declarations and run bounded counterexample search is fully
  reflected in §5.2 ("compiler verifies inverse consistency via bounded
  counterexample search within the participating types' refinement bounds.
  Counterexample found is a compile error").

- **`planning/v2/spec.md` §4.5 / §4.4 — `value_in` as escape hatch:**
  > "`value_in` is the only way to exit the dimension system. It strips the
  > dimension entirely — the result is `Scalar<ratio>` (dimensionless)."
  §5.3 absorbs the use-position framing (interop with unit-naive stdlib atoms,
  external-library arguments) and the dimensional-compatibility requirement.

- **`planning/v2/spec_dev_notes.md` line 221-222 — §5 status note:**
  > "§5 Units — 2026-04-21: 5.1 convert four variants, 5.2 round-trip
  > verification (O2.1), 5.3 `value_in` operator all stubbed."
  Confirms the three subsections are intentionally scoped to these topics.

---

## Superseded

Corpus content replaced by a decision now in spec_new.md §5. Items already in
`anti_spec.md` are noted and skipped.

- **`planning/v2/spec.md` §9.2 — four invertibility metadata classes
  (`bijective` / `injective_restricted` / `lossy` / `opaque`) used for
  converts:** The old spec used per-function invertibility annotations. §5.1's
  four-variant `convert` surface supersedes this with structural declaration
  forms. Already in `anti_spec.md` ("four-class invertibility metadata ...
  capability contracts on fns") — skip.

- **`planning/v2/spec.md` §4.6 — "Automatic unit inference for bare literals
  in typed contexts":**
  > "if a field is declared as `Scalar<MPa>` and the user writes `psi = -1.5`,
  > the literal is dimensionless and the compiler errors."
  This was a note *against* implicit unit inference. Still valid; the
  anti-inference stance is assumed but not restated in §5. Not a superseded
  item but may be homeless (see below).

- **`planning/v2/spec.md` §7.2 — slot-ABI unit conversion at boundary
  ("converts the slot's output from base units to declared units"):** The slot
  construct is retired; the boundary-conversion prose has no §5 successor.
  Already handled by the slot retirement in `anti_spec.md` — skip.

- **`planning/v2/v2.1_in_progress.md` universals-with-values
  (`universal R: Scalar<J_mol_K> = 8.314`):** Superseded by CC1 (no literal
  values in `.myco` value position). The old spec.md §4.6 showed this form.
  Addressed in `anti_spec.md` ("universals carrying values ... declaration
  only; value from workflow") — skip.

---

## Homeless

Corpus content that is relevant to §5, not present in spec_new.md §5, and not
committed to `anti_spec.md`. Each item is a stable decision absent from §5's
current stub.

- **`planning/v2/spec.md` §4.1-4.3 — base unit declarations, derived units,
  `Scalar<U>` definition:**
  > "A `base_unit` declaration introduces a new orthogonal axis in the
  > dimension exponent vector."
  > "Derived units are defined as products, quotients, and scalar multiples
  > of existing units."
  > "`Scalar<U>` is the built-in parameterized type meaning 'a real number
  > measured in unit U.'"
  These foundational definitions are not in §5. They are either in §3 Types
  (if `Scalar<U>` lives there) or need a §5.0 preamble. The current §5 stub
  starts at `convert` without stating what a unit or base dimension is.
  `Recommend:` add a §5.0 subsection covering `base_unit`, derived unit
  algebra, dimension exponent vectors, and `Scalar<U>` as the unit-parameterized
  quantity primitive — or explicitly forward-reference §3 if those live there.

- **`planning/v2/spec.md` §4.4 — affine unit semantics:**
  > "Affine units cannot be freely multiplied or divided. `20°C * 2` is not
  > `40°C` — the expression requires conversion to the absolute unit (Kelvin)
  > before multiplication."
  > "Addition and subtraction of two affine quantities is permitted (the offsets
  > cancel for subtraction, producing a temperature *difference*)."
  This is a stable design decision with behavioral consequences for modelers.
  `Recommend:` add as §5.4 or fold into a §5.0 unit-system preamble.

- **`planning/v2/spec.md` §4.5 — base-unit internal representation:**
  > "Internally, all math happens in base units. Declared units are a
  > user-facing layer."
  The `value_in` subsection (§5.3) references "unit-naive stdlib atoms" without
  explaining *why* the distinction matters. The base-unit-internal-storage rule
  is the reason.
  `Recommend:` add one sentence to §5.3 (or a §5.0 preamble) stating this
  invariant.

- **`planning/v2/spec.md` §4.6 — workflow binding unit parameter:**
  > "`experiment.assume_series('atm.temperature', data_in_kelvin, unit='K')`
  > — if provided and the dimension matches, the binding layer converts."
  This is a settled interface for cross-unit data injection at workflow time.
  Not in §5 and not in §29 (Units Library). It belongs either here or in the
  workflow-binding section.
  `Recommend:` note in §5.3 (as a second `value_in`-adjacent use position) or
  add as §5.5 covering workflow-boundary unit handling.

- **`planning/v2/spec.md` §4.6 — no implicit unit inference:**
  > "Automatic unit inference for bare literals in typed contexts: if a field
  > is declared as `Scalar<MPa>` and the user writes `psi = -1.5`, the literal
  > is dimensionless and the compiler errors."
  This is a firm language policy. Not stated in §5.
  `Recommend:` add as a one-liner policy note in §5 (the compiler does not
  infer units from context; annotated literals are the required form).

- **`planning/v2/spec.md` §4.7 — named-type coercion rules (arithmetic strips
  names, affine subtraction strips to base unit, same-type addition preserves
  name):**
  > "Arithmetic operations on named types produce anonymous dimensional
  > types — the named type is stripped."
  > "`CarbonPool = WaterPool` is a compile error even though both have
  > dimension `Amount`."
  These named-type arithmetic and comparison rules are stable decisions. They
  may belong in §3 Types rather than §5, but they are currently nowhere in
  spec_new.md.
  `Recommend:` confirm placement (§3 or §5); the open_questions.md Tier 2
  item ("extend named-type rules to cover relations and comparisons") shows
  this is known but unplaced.

- **`planning/v2/spec.md` §4.6 — expression unit annotations syntax:**
  > "`(0.1579 + 0.0017 * T_c) mol_m2_s` — syntactic sugar for multiplication
  > by the unit's scale factor."
  A concrete surface-syntax rule for attaching units to dimensionless
  parenthesized expressions. Not present in §5.
  `Recommend:` add to §5.0 or §5.4 (unit annotation syntax on literals and
  parenthesized expressions).

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  §CC-C (unit normalization rewrite group):**
  > "C1. Literal-with-unit → base SI: `0 degC ↔ 273.15 K`, `0.75 MPa ↔
  > 750000 Pa`"
  > "C4. Dimensionless 0 / 1 collapse across unit signatures"
  These are settled e-graph rewrite rules for unit normalization. They are
  implementation detail but also user-visible (they determine when two
  expressions are unit-equal). The rewrite group catalog is tracked under
  §33/§17.5 in spec_new.md. Not homeless at the spec level, but §5 should
  forward-reference where unit-equality merges are specified.
  `Recommend:` add a forward-reference sentence in §5 pointing to the
  equality-introducing machinery (§17) for where unit normalization rewrites live.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  D-group — named-type normalization in the e-graph:**
  > "D1. Bare `convert FishMass <-> DetritusMass` treats them as same-magnitude
  > in the e-graph (different name) when both share a conserved parent."
  > "D3. Inverse convert round-trip: `convert(convert(x)) → x` when verified
  > per O2.1`"
  These D-group rules give the e-graph semantics of `convert` declarations.
  They are open design work in chunk 04 (an in-progress report), so by the
  task brief's rule they are NOT flagged as homeless. Noted for cross-reference.

- **`planning/v2/open_questions.md` Tier 0 — "Units and the e-graph":**
  > "If two expressions are equal in the e-graph, must they have compatible
  > units? Can unit inference use e-class merging?"
  This is an explicitly open design question, not a stable decision. Not
  homeless by the task brief's rule. Noted for awareness.

- **`planning/v2/v2.1_chunk_reports/05_matrices_in_progress.md` §3.2 —
  heterogeneous-unit question:**
  > "Heterogeneous-unit question — OPEN (primary blocker)"
  Explicitly open. Not homeless.

- **`planning/v2/spec_dev_notes.md` §430-433 — §29 Units Library scope:**
  > "Committed scope is SI base, SI-derived, derived-unit algebra, and
  > affine-conversion machinery. Domain-specific libraries ... ship as
  > distributable packages consuming core units."
  This scoping statement belongs in §29 (Units Library), not §5. Not homeless
  for §5. Noted for §29 audit.

---

## Conflicts

Direct contradictions between spec_new.md §5 and corpus documents.

- **`convert A <-> B` (bare) vs. conservation-group requirement.** §5.1 states:
  > "Required for conservation-group siblings (§3.7). No body."
  `planning/v2/v2.1_in_progress.md` §174-225 shows bare bidi converts used
  freely between any same-dimension named types, with no mention of a
  conservation-group prerequisite:
  > "convert FishMass <-> DetritusMass" — presented as a general mechanism
  > for same-dimension named-type compatibility, not as a conservation-only tool.
  The corpus does not restrict bare bidi `convert` to conservation-group
  siblings. §5.1's parenthetical "(Required for conservation-group siblings)"
  reads as a use-case note, but the word "Required" could be read as imposing a
  restriction in both directions (only conservation-group siblings may use it).
  `Recommend:` clarify whether bare `<->` is *always* legal between same-
  dimension types or *restricted* to conservation-group siblings. If it is
  always legal and the conservation-group case is merely the canonical motivating
  example, reword §5.1 to remove the ambiguity. If it is restricted, the
  v2.1_in_progress examples (non-conservation-group bare converts) need updating.

- **`value_in` return type.** §5.3 says:
  > "extracts the raw numeric magnitude of a quantity in a named unit"
  and
  > "Unit must be dimensionally compatible with the receiver."
  `planning/v2/spec.md` §4.5 states:
  > "`value_in` is the only way to exit the dimension system. It strips the
  > dimension entirely — the result is `Scalar<ratio>` (dimensionless)."
  §5.3 omits the return type. The old spec.md is explicit: the result is
  `Scalar<ratio>`. If `Scalar<U, T>` (two-parameter form from chunk 04) is
  adopted, the return type is `Scalar<dimensionless, T>`. Either way, §5.3
  should state the return type.
  `Recommend:` add the return type to §5.3. Decide whether it tracks the
  `T` parameter of the receiver or defaults to `Float64`.
