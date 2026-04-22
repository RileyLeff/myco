# Audit Report — §18 The Type Graph

Auditor: Claude (Sonnet 4.6)
Date: 2026-04-21
Section under audit: spec_new.md §18

---

## Absorbed

The following corpus content is already reflected in spec_new.md §18.

- **`planning/v2/v2.1_chunk_reports/07_type_graph_in_progress.md` §1.**
  > "They do. There are at least five distinct type-level relations in v2.1:
  > refinement lattice, conversion graph, definitional / alias equality,
  > contract / `dyn` satisfaction, generic instantiation."

  Absorbed as the §18 characterization: "The type graph is a separate substrate
  from the expression e-graph, carrying named-type relations (subtyping,
  conversion, conservation-group membership, refinement implications)."

- **`planning/v2/v2.1_chunk_reports/07_type_graph_in_progress.md` §2.**
  > "e-graph holds the equation, type graph provides the guard."

  The §18 summary captures this split: the type graph holds the named-type
  relations while the e-graph holds expressions. The three interaction points
  (structural view equalities, unit-conversion identities, refinement-gated
  rewrites) are the content that §18 defers to chunk 07.

- **`planning/v2/spec_dev_notes.md` (2026-04-21 batch).**
  > "Type graph ↔ expression e-graph bridge cross-reference (chunk 07) —
  > 2026-04-21 confirmed already covered by §18 STUB + §34 chunk 07 entry."

  Dev notes explicitly confirm that §18 STUB plus the §34 chunk-slotted
  tracking entry constitute the intended coverage for now.

- **`planning/v2/spec_dev_notes.md` (Must-add sections table).**
  > "2026-04-21: §0 Principles, §18 Type Graph (stub), §20 SCC Decomposition,
  > §21 Lowering (renumbered), §22 Plan Inspection ... all stubbed in
  > spec_new.md."

  The decision to make §18 a stub is intentional and recorded here.

- **`planning/v2/spec_new.md` §34 Chunk-Slotted Work.**
  The tracking entry "Chunk 07. Type-graph ↔ e-graph bridge." is the
  intended forward pointer that §18's stub leads to.

---

## Superseded

No corpus content relevant to §18 has been replaced by a newer decision that
would need to move to anti_spec.md. The following stale item already appears
in anti_spec.md and needs no further action.

- **`planning/v2/v2.1_chunk_reports/07_type_graph_in_progress.md` §1, §4
  (Q5) — `dyn` keyword.**
  > "Contract / `dyn` satisfaction. Which concrete types satisfy which
  > contracts; which `dyn` witnesses close over which concrete implementations."
  > "Dyn witness caching. `dyn` objects carry type-graph information at runtime."
  > "Q5. How does `dyn` interact with the type graph? Is a `dyn Contract`
  > a type-graph node or an e-class?"

  `dyn` was retired in anti_spec.md ("| `dyn` | `impl Contract` (static monomorph)
  + `some` (runtime sizing) | clean split of compile-time vs runtime heterogeneity |")
  and the retired open question is recorded there as well ("| `dyn` trait-object
  semantics vs sized | void — `dyn` retired |"). The chunk 07 report predates the
  retirement lock. Already in anti_spec.md; no further action needed.

---

## Homeless

These are corpus items relevant to §18 that are not accounted for in the
current §18 stub and are not in anti_spec.md. All are from the in-progress
chunk 07 report (an open chunk), so they are legitimately open design work.
They are flagged here only where a stable framing decision (not a design
answer) appears to be missing from §18 or from open_questions.md.

- **`planning/v2/v2.1_chunk_reports/07_type_graph_in_progress.md` §3
  — three coupling options.**
  > "Option A — Two graphs, explicit bridge ... Option B — One graph, types
  > are terms ... Option C — Two graphs, type graph compiled to e-graph rules
  > at elaboration ... No strong lean yet."

  The three-option framing is the primary organizing decision for chunk 07.
  It is not mentioned in §18, in §34's chunk 07 entry, or in open_questions.md.

  Recommend: Add to open_questions.md as a Tier 0 or Tier 1 item (it blocks
  all of chunk 07). The §18 stub itself does not need to reproduce it, but the
  question tracker should name the decision and its three options so the thread
  is not lost between context sessions.

- **`planning/v2/v2.1_chunk_reports/07_type_graph_in_progress.md` §7
  — open questions Q1-Q7 (excluding Q5, already handled above).**
  > "Q1. Is the type graph mechanized as option A, B, or C?"
  > "Q2. What precisely lives in the type graph vs. the expression e-graph?"
  > "Q3. Subtype semantics for refinements — predicate, cast, or witness-function?"
  > "Q4. Variance rules for generic parameters."
  > "Q6. Cost model for conversion-graph edges."
  > "Q7. Does the type graph support online derivation during e-graph saturation,
  > or is it fully compiled out (option C)?"

  None of Q1-Q4, Q6, Q7 appear in open_questions.md. They are tracked only in
  the chunk report itself. The chunk report's status is IN PROGRESS / STUB with
  no committed answers.

  Recommend: Add Q1-Q4, Q6, Q7 to open_questions.md (Tier 1 — they must resolve
  before chunk 07 can produce a spec-ready section). Q6 cross-references chunk 05
  Q7 and chunk 06, which is already noted in spec_dev_notes.md; that cross-link
  should appear in the open_questions entry.

- **`planning/v2/v2.1_chunk_reports/07_type_graph_in_progress.md` §6
  — dependency ordering.**
  > "This chunk is best tackled after chunks 04 / 05 / 06 land."

  The chunk 07 stub itself and §34's tracking entry do not mention the
  dependency ordering. If chunk 04 (in progress) stalls, the ordering note
  clarifies why chunk 07 cannot advance independently.

  Recommend: Add a brief dependency note to the §34 chunk 07 entry in
  spec_new.md, or to open_questions.md alongside Q1-Q7. Landing in §18 would
  be premature since §18 is a user-facing stub, not implementation guidance.

- **`planning/v2/spec_dev_notes.md` — "Conversion-graph cost model" foundational
  concept.**
  > "Conversion-graph cost model — §0.1 paragraph (open; unit conversions +
  > tensor reshapes + sparse/dense + subtype widening; tracked in §35, scoped
  > chunk 05 Q7 / chunk 07 Q6)."

  The §0.1 paragraph exists, but §18 does not forward-reference it. The
  cost-model question straddles both the type graph (what edges exist and their
  costs) and the e-graph extraction cost vector. A forward reference from §18 to
  the §0.1 paragraph and to §35 would close the loop for a reader coming in from
  the type-system direction.

  Recommend: Add a one-sentence forward reference in §18 to the §0.1
  Conversion-graph cost model paragraph and to §35 when the stub is fleshed out
  at chunk 07 time. Not urgent for the stub as written.

---

## Conflicts

- **Stale §18 cross-references inside spec_new.md itself.**

  Two locations in spec_new.md cite §18 for SCC / residual classification, but
  §18 is the Type Graph, not the classification machinery.

  - `spec_new.md` §0.1 (Conservation laws paragraph, line ~85):
    > "Conserved-group declarations (§3.7) produce compile-checked invariants
    > that thread through types, relation equality (§8), event firings (§10),
    > and residual classification (§18)."

    Residual classification lives in §19.3 (three-way overdetermination tag)
    and §20 (four-way SCC tag). §18 is the Type Graph stub. The cross-reference
    is incorrect.

  - `spec_new.md` §0.1 preamble / Part II intro (SCC decomposition paragraph,
    lines ~1915-1916):
    > "Each SCC carries its own classification (§18), residual flavor (§19),
    > and tolerance envelope (§16.4)."

    Again, SCC classification belongs to §20 (four-way tag) and §19.3
    (three-way tag). §18 is the Type Graph.

  Both of these are internal spec_new.md cross-reference errors introduced
  during the renumbering that moved Lowering from the old §18 slot to §21.
  The references were not updated when §18 became the Type Graph stub.

  Recommend: Replace "(§18)" with "(§20)" in both locations. The conservation-law
  paragraph should also add a note that conservation-group membership is a
  type-graph fact (correctly a §18 concern), while the enforcement thread at
  residual classification time belongs to §19.3 and §20. The SCC decomposition
  paragraph's "(§18)" should become "(§20)" straightforwardly — no additional
  nuance needed there.
