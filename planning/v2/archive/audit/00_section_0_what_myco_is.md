# Audit: §0 "What Myco Is"

**Scope:** `spec_new.md` §0 and §0.1 (the full output of `just spec-section 0`).
**Corpus:** files listed in the audit brief.

---

## Absorbed

Content from the corpus that has landed in spec_new.md §0 / §0.1.

- **`planning/soul.md` principle 2** — "The workflow is a separate concern. What you assume, observe, and learn is not a property of the world." Landed as §0 Principle 1 (world-vs-experiment split) and Principle 2 (clean boundary).

- **`planning/soul.md` principle 3** — "The compiler does the work. Causal ordering, solver selection, code generation, algebraic inversion, loop detection — all of this is the compiler's job." Landed as §0 Principle 3 (compiler does work) and the derivative-and-projection inventory in §0.1 "Projection-free compiler."

- **`planning/soul.md` principle 4** — "Structure is the regularizer. Overdetermined relations, constraints, temporal dynamics, and cross-quantity coupling are not inconveniences to be simplified away. They are the signal." Landed as §0 Principle 4 (structure regularizes), with shape/units/refinements/conservation groups/contracts enumerated.

- **`planning/soul.md` principle 5** — "Generated code is the product. Myco emits ordinary source code... the source of truth is the `.myco` model and the binding." Landed as §0.1 concept "Generated code is the product" (plan = unit of execution, source = unit of reproduction; §22 inspection).

- **`planning/v2/spec.md` Design Philosophy** — "The model describes what is true about the world. The compiler figures out how to compute it. The user never annotates solution strategies, solver choices, or execution order." The execution-order / solver-choice commitment landed in §0 Principle 3 and §0.1 "Projection-free compiler."

- **`planning/v2/spec_dev_notes.md` §0.1 cross-cutting concepts list** (2026-04-21 batch) — all eleven named concepts (conservation laws, referential truth, downward-only visibility, traceability, error-reporting tiers, capability errors, three-layer scoping, determinism, world-vs-experiment axis, conversion-graph cost model, projection-free compiler, generated code is the product) are present in §0.1 verbatim.

- **`planning/v2/spec_dev_notes.md` soul principles mapping** — "Soul principle 2: workflow is separate from the model — already covered by §0 principles 1 and 2. Soul principle 3: the compiler does work — already covered by §0 principle 3. Soul principle 4: structure regularizes — already covered by §0 principle 4." All confirmed present.

- **`planning/v2/v2.1_in_progress.md` glossary entries** — "`.myco` layer — the world description... Relations are symmetric constraints, not assignments... Nothing in the `.myco` commits to execution order..." These are foundational framings that correspond directly to §0 Principles 1–3 and §0.1 world-vs-experiment axis entry.

- **`planning/v2/riley_project_note.md` "spore" concept** — distributable Myco packages for domain-specific content. Landed in spec_new.md §28-§29 area ("distributable packages on top of the stdlib"), not in §0 itself, but the scope note in §0 ("The long-term goal is a GPU ecosystem simulator...") at minimum acknowledges the project framing. See Conflicts below.

---

## Superseded

Content the corpus once stated but that spec_new.md §0 explicitly replaces or that anti_spec.md has already retired.

- **`planning/v2/spec.md` Design Philosophy** — "The compiler never silently trusts claims it cannot verify. If the compiler cannot prove a property, it errors with an actionable diagnostic. The user may explicitly acknowledge unverifiable properties, but silence is never consent." This escape hatch ("user may explicitly acknowledge") is superseded by the total annotation purge. `anti_spec.md` retires `#[verified_externally]` and states "no proof-escape-hatch annotations." Already in anti_spec.md; no action needed.

- **`planning/soul.md` principle 5 sub-claim** — "Myco emits ordinary source code, not a hidden runtime. The user owns the output — they can inspect it, import it, and run it with standard tools." The "emits ordinary source code" framing is superseded: spec_new.md §0.1 "Generated code is the product" now describes the compiled plan as the artifact, not raw emitted source. The spec §0.1 says "the compiled plan plus the workflow bindings" is the run-time artifact, which is a more accurate framing than "ordinary source code." Already resolved in §0.1; no corpus action needed.

---

## Homeless

Corpus content relevant to §0, not yet in spec_new.md §0, and not committed to anti_spec.md.

- **`planning/soul.md` principle 1** — "A `.myco` file should read like a description of what relationships hold in reality — not like a program that computes something. If you have to think about execution order while writing the model, the abstraction is leaking." The positive framing ("reads like a description of relationships") and the leak test ("if you have to think about execution order, the abstraction is leaking") appear nowhere in §0 or §0.1. The section covers what the compiler does but not the user-facing design criterion for the model language itself.
  `Recommend:` Add a sentence or brief paragraph to §0 (before or after the five principles) that captures this criterion: the `.myco` surface should read as world-description, not as a computation recipe. The leak-test framing is a useful authorial heuristic that belongs in the spec's positioning statement.

- **`planning/v2/spec.md` Design Philosophy** — "The `.myco` representation should approach the minimum description length of the science. If the implementation complexity vastly exceeds the description complexity, the gap is incidental complexity that belongs in the compiler, not in the model." This MDL framing does not appear anywhere in spec_new.md §0. It is a stable design principle, not open work.
  `Recommend:` Absorb into §0 as a sentence under Principle 3 ("Compiler does work") or as a standalone design criterion before the five principles. It sharpens the intent of the compiler-does-work principle from a different angle and is load-bearing for explaining why Myco's surface is intentionally lean.

- **`planning/v2/riley_project_note.md`** — "Things that DO belong in Myco core: general language surface... Things that do NOT belong in Myco core: any unit, function, model shape, or stdlib item that only makes sense in a plant-physiology / ecology / Riley's-research context." This scope discipline is documented only in `riley_project_note.md` and in the memory file; it is not stated in spec_new.md at all.
  `Recommend:` Add a short "Scope" sub-clause to §0 Scope paragraph stating that Myco is a general scientific modeling language; domain-specific libraries ship as distributable packages outside the core. This prevents future contributors from adding plant-physiology content as if it belongs in core. A single sentence suffices; the note does not need to name Riley's project.

- **`planning/v2/spec_dev_notes.md` structural note (2026-04-21)** — "Read-order / audience / level-of-detail convention stated in preamble — cut. Out of scope for spec pre-build; revisit once `.myco` exists and there's real audience feedback." This was consciously deferred, not forgotten. Not homeless; deliberately excluded. Noted here only to confirm.

- **`planning/v2/spec_dev_notes.md` versioning note** — "Versioning / stability policy for `.myco` (breaking-change discipline) — cut. Out of scope pre-build; policy is a post-implementation decision." Same as above; consciously deferred.

---

## Conflicts

- **`planning/v2/spec_new.md` §0 Scope** vs **`planning/v2/riley_project_note.md`** and memory rule `feedback_project_vs_language.md`: spec_new.md §0 reads "The long-term goal is a GPU ecosystem simulator with neural controllers, dynamic topology, and spatial explicitness. Myco is a precondition." The memory rule states "Myco is general; Riley's ecosystem project is a user, keep project-specific framing out of spec prose." The Scope paragraph positions a specific user's research project as Myco's "long-term goal," which is the project-specific framing the memory rule prohibits.

  - spec_new.md §0: `"The long-term goal is a GPU ecosystem simulator with neural controllers, dynamic topology, and spatial explicitness. Myco is a precondition."`
  - `riley_project_note.md`: `"The spore is a consumer of Myco, not part of it."`
  - Memory (`feedback_project_vs_language.md`): `"Myco is general; Riley's ecosystem project is a user, keep project-specific framing out of spec prose"`

  `Recommend:` Replace the "long-term goal" sentence with a statement of Myco's general ambitions: spatially explicit simulation, dynamic topology, neural-controller integration as language capabilities, without naming any specific research project as the goal. The sentence currently conflates a research application with a language design goal. Example replacement: "The language is designed to support spatially explicit simulation, dynamic topology, and integration with learned components as first-class modeling constructs."
