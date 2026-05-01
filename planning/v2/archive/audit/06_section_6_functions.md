# Audit: §6 Functions — spec_new.md

**Section text (full, from `just spec-section 6`):**

> `fn` declarations with parametric generics. Body composition. Contracts
> apply to functions using the same composable machinery used for types
> and distribution families (see §7). Stdlib atoms (`exp`, `log`, `sin`,
> `sqrt`, …) declare capability contracts like `Invertible<_>`,
> `Differentiable`, `Monotone`; these drive e-graph rewrites (see §17
> merge sources). User functions carry no property-declaration surface;
> the compiler derives properties from body composition plus stdlib
> atom declarations. No annotation blocks, no `#[...]` attributes.

---

## Absorbed

Content from the corpus that already landed in spec_new.md §6.

- **`spec_dev_notes.md` (2026-04-20 Functions entry):** The entire Option B
  decision — "Stdlib atoms (`exp`, `log`, `sin`, `sqrt`, …) carry capability
  contracts like `Invertible<_>`, `Differentiable`, `Monotone`. User functions
  carry no property-declaration surface — the compiler derives properties from
  body composition plus the declared stdlib-atom facts. No annotation blocks,
  no `#[...]` attributes." — is reproduced verbatim as §6's normative text.

- **`spec_dev_notes.md` (2026-04-20 Functions entry):** The consequence that
  e-graph rewrites are driven by stdlib-declared contracts: "§17 merge sources:
  function-inverse rewrites fire from stdlib-declared contract satisfaction, not
  from user annotations." Absorbed into §6's cross-reference: "these drive
  e-graph rewrites (see §17 merge sources)."

- **`v2.1_in_progress.md` §Functions:** "`fn name<generics>(params) ->
  ReturnType { body }` — Registered functions with typed parameters." The
  settled status for `fn` declarations and function-level generics is reflected
  in §6's opening phrase "fn declarations with parametric generics."

- **`v2.1_in_progress.md` §Generics:** "Types and functions can be generic over
  contracts." Absorbed: §6 says "fn declarations with parametric generics" and
  cross-references §7 for contract machinery.

- **`anti_spec.md` (Retired annotations table):** "user-declared fn
  invertibility / differentiability / domain — compiler derives from body
  composition + stdlib atom contracts — no user property-declaration surface;
  refactor fn if compiler can't derive." Matches §6 exactly: "User functions
  carry no property-declaration surface."

- **`anti_spec.md` (Retired annotations table):** "all `#[...]` attribute
  annotations — nothing — `.myco` has no annotation surface." Absorbed: §6's
  "No annotation blocks, no `#[...]` attributes."

- **`anti_spec.md` (Retired annotations table):** "four-class invertibility
  metadata (`bijective` / `injective_restricted` / `lossy` / `opaque`) —
  capability contracts on fns." Absorbed: §6 names `Invertible<_>`,
  `Differentiable`, `Monotone` as the replacement surface.

---

## Superseded

Corpus content replaced by newer decisions in spec_new.md §6. Anti_spec.md
coverage noted where present.

- **`spec.md` §9.2 (User-registered functions):** User functions declare
  annotation blocks inline: `invertibility: bijective`, `differentiability:
  smooth`, `domain: slope > 0`. Superseded by §6's "no annotation blocks."
  Already in `anti_spec.md` ("user-declared fn invertibility / differentiability
  / domain — compiler derives from body composition + stdlib atom contracts").

- **`spec.md` §9.3 (Inverse verification):** Explicit user-supplied inverse
  functions (`fn inverse vulnerability_curve(...) -> Potential { ... }`) with
  two-level verification (symbolic + round-trip sanity check). The `#[verified_externally]`
  fallback path is part of this. Superseded: §6's user-composition-as-escape-hatch
  and no-annotation-surface. Already in `anti_spec.md` ("`#[verified_externally]`
  — nothing — no proof-escape-hatch annotations"; four-class metadata retired).

- **`spec.md` §8.1-8.4 (Operation Algebra):** Four invertibility classes
  (`bijective`, `injective_restricted`, `lossy`, `opaque`) and four
  differentiability classes (`smooth`, `subgradient`, `fragile`,
  `non_differentiable`) declared per-function as metadata fields. Superseded
  by capability contracts on stdlib atoms. Already in `anti_spec.md`
  ("four-class invertibility metadata").

- **`v2.1_in_progress.md` §Functions — "Annotation blocks":** "`invertibility:
  bijective`, `differentiability: smooth`. Declared per-function. Compiler uses
  these for symbolic analysis. Status: settled." Superseded. Already in
  `anti_spec.md` (annotation surface retired).

---

## Homeless

Corpus content relevant to §6, not accounted for in spec_new.md §6, and not
committed to `anti_spec.md`. Ordered by assessed impact.

- **Recommend:** `spec.md` §9.2 and `v2.1_in_progress.md` §Functions: the
  distinction between `fn` (functions) and `relation` is load-bearing for §6
  but never stated. Spec.md describes functions as "purely functional" evaluation
  (no persistent bindings, forward-only), while relations are "symmetric
  constraints" the compiler can invert or solve in any direction. §6 does not
  state what distinguishes a function from a relation, or when a user should
  reach for each. The `open_questions.md` names "Functions and contracts in the
  e-graph. A function call `f(x)` — distinct node, or participates in e-class
  merging when `f` is inverted?" as an open question; §6 gives no answer and
  does not flag the question. Assessment: a reader of §6 alone cannot tell
  whether `fn` is the right construct or whether a relation would do.

- **Recommend:** `spec.md` §9.2 (function-level generics) and `v2.1_in_progress.md`
  §Generics: `fn arrhenius<U: Unit>(value_25: Scalar<U>, ...) -> Scalar<U>` as
  the canonical example of a unit-polymorphic function. §6 says "parametric
  generics" but gives no example and no statement of how generic functions
  monomorphize at call sites. The settled decision is already in the corpus;
  it never made it into §6 text.

- **Recommend:** `spec.md` §9.4 (Importable function packages): Functions ship
  with library modules; `pub fn` controls inter-module visibility. §6 gives no
  statement about visibility or module-level packaging for functions. The
  decision is stable (§2 Modules covers `pub`; §6 should cross-reference or
  restate). Not in `anti_spec.md`.

- **Recommend:** `v2.1_in_progress.md` §Functions: "Compiler uses these
  [registered functions] for dimensional analysis, symbolic differentiation, and
  solver emission." §6 does not state the compiler roles for `fn` declarations.
  The spec summary block mentions "body composition" but not dimensional checking
  or the compiler's use of function bodies for symbolic differentiation. This is
  a stable settled decision not reflected in §6.

- **Recommend:** `spec_dev_notes.md` (2026-04-20 Functions entry, Implications):
  "Function inversion user recourse: if a user fn needs its inverse recognized
  and the compiler can't derive it, user refactors the fn into structurally
  composable pieces. No escape hatch." This is the actionable consequence of the
  no-annotation-surface policy. §6 states the policy but gives no recourse
  statement. Users who hit the wall have no guidance in §6.

- **Recommend:** `03_kernels_in_progress.md` §2: "Decided: kernels are not a
  new kind. Not a new keyword. Not a new block. They are ordinary `.myco`
  functions." This is a stable architectural decision (kernels-as-functions) that
  eliminates a potential misreading of §6 (would someone expect a `kernel`
  keyword?). Not in `anti_spec.md`. The chunk report is in-progress overall, but
  this specific decision is closed and recorded as "Decided."

- **Recommend:** `spec.md` §9.2 and `v2.1_in_progress.md`: Custom closure
  policies are "ordinary `.myco` functions." `open_questions.md` line 517:
  "Policies are ordinary `.myco` functions whose arguments are candidate values
  and user hyperparameters." §6 gives no statement that functions are the
  extensibility surface for closure policies (§8) — a connection that matters
  for understanding the role of `fn` in the language.

---

## Conflicts

Direct contradictions between spec_new.md §6 and corpus documents.

- **`v2.1_in_progress.md` §Functions — Annotation blocks status vs. §6:**
  `v2.1_in_progress.md` records annotation blocks (`invertibility: bijective`,
  `differentiability: smooth`) with status "settled." Spec_new.md §6 states
  "No annotation blocks." These are contradictory settled-status claims in the
  same corpus.
  - `v2.1_in_progress.md`: "`invertibility: bijective`, `differentiability:
    smooth`. Declared per-function. Compiler uses these for symbolic analysis.
    **Status: settled.**"
  - `spec_new.md §6`: "No annotation blocks, no `#[...]` attributes."
  - Recommend: `v2.1_in_progress.md` is stale on this point. The `spec_dev_notes.md`
    2026-04-20 entry explicitly supersedes it (Option B decision). No action
    needed on `spec_new.md §6`; `anti_spec.md` already carries the retirement.
    `v2.1_in_progress.md` is flagged in `anti_spec.md`'s stale-list as
    containing versioning prose — this is a specific stale entry that should
    not be imported.

- **`spec.md` §9.2 — explicit inverse function surface vs. §6:**
  `spec.md` §9.2 presents `fn inverse vulnerability_curve(...) -> Potential { ... }`
  as a normal user-facing construct. Spec_new.md §6 eliminates this surface
  entirely (no user property-declaration surface; compiler derives from body
  composition).
  - `spec.md` §9.2: "Optionally: an explicit inverse function" — documented
    as a settled user-facing feature.
  - `spec_new.md §6`: "User functions carry no property-declaration surface."
  - Recommend: `spec.md` §9.2 is stale wholesale — `anti_spec.md` flags
    `spec.md` §9.2-9.3 as superseded. No action on spec_new.md §6; the conflict
    is with a stale document.
