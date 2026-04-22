# External Review: Relation/Fn Fix

External design-soundness review of the ban-user-fn + parameterized-relations lock (chunk 08). Run against `planning/v2/` (excluding `v2_old/`) plus `planning/soul.md`. Two models in parallel: Codex (GPT-5.4) and Gemini. Prompt asked for (1) a surface sweep beyond the list in chunk 08, and (2) yea/nay on five design choices and five sub-questions, plus any additional questions the reviewers thought we should be asking.

Both reviews ran READ-ONLY. Neither model touched any files.

## Consensus

Both models converged on the same answers for every design choice and every sub-question we posed.

**Five design choices — all Yea:**
1. Undirected syntax (no `-> ReturnType`)
2. No `use` / `call` keyword at invocation (Codex adds a qualifier — see below)
3. Flat positional args, named args later as sugar
4. Generics via the same machinery as types/contracts
5. Contract "method" collapses into required parameterized relation

**Five sub-questions — joint answers:**
1. `self` is a **convention**, not a syntactic marker. Structural rule: contract-required relations take the implementor instance as first parameter, typed `Self`; the spelling `self` is recommended but not semantic.
2. Free variables in parameterized-relation bodies: **ban** caller-scope capture. Only formal parameters, imported names, other declared relations, universals, and stdlib items are referenceable.
3. Recursion: **ban** in v2.1 (both direct and indirect). Use `temporal` or algebraic cycles for legitimate feedback; syntactic unfolding recursion explodes the e-graph.
4. Compiler-derived property visibility: **inspection-only** via tooling (`mycoc explain`, IDE hover). Never a source-level annotation. Output is a proof chain or an explicit "unknown because…".
5. Namespacing / shadowing: **qualified paths + explicit import aliases**. Unqualified collisions are hard errors.

Both models grounded their answers in `soul.md` (equational claims, compiler-derives-not-user-asserts, one reuse mechanism).

## Divergences and qualifiers

**Codex qualifier on Design Choice #2 (no `use`/`call` keyword):** Agree, **but only if invocation is statement-form, not expression-form**. If `photosynthesis(...)` is only a relation-body item (not a subexpression), the grammar is clean and the extra keyword buys nothing. If invocation can appear in expression position, you've recreated functions under a different keyword and need the disambiguator back.

This is the single most load-bearing unpinned question the reviewers surfaced. It should be decided before the surface sweep.

## New surfaces flagged (additive to chunk 08's list)

Chunk 08 listed: §6, §7, §8, §17.1, merge-source count, §28, anti_spec.md, mock files. Reviewers added the following.

### spec_new.md

| Location | What changes |
|---|---|
| Part I summary (L196) | Top-level inventory still says modelers write "functions"; global framing needs to become relation/parameterized-relation language. |
| §3.1 | "Functions are evaluating fns" framing needs to specify stdlib-axiomatic only; user reuse is strictly equational. |
| §8.8 Y5 (L817) | Custom closure policy still described as an ordinary `.myco` function. Two issues: (a) framing is obsolete; (b) Codex flags that custom Y5 may belong workflow-side rather than relation-side. |
| §11.1 Spatial operators | Reframe `grad`, `diverg`, etc., as axiomatic primitives with fixed capability contracts, not "functions". |
| §13.8 Distributions | Categorize `log_pdf` and `sample` explicitly: stdlib-only callable exceptions or relation-shaped obligations. Pressure-point; needs decision before §13/§27 land. |
| §14 Intrinsics | Formally categorize `deriv`, `integrate`, `condition_of` as axiomatic primitives, not user-reusable functions. |
| §24.1–§24.2 (L2775) | `bind_controller(path, fn, ...)` — parameter name `fn` collides with the retired user surface. Rename to `callable` in prose and API docs. |
| §28.1–§28.3 (L3301) | Kernels still specified as `fn k(x, y) -> ...` plus function-property language. Deeper reframe than "no kernel keyword" — rename section to something like "Relation Curation." |
| Appendix A (L3894) | `fn` still listed as declaration keyword; `self` still "reserved but unassigned"; stdlib atoms said to be shadowable by user functions. |
| Appendix B (L3943) | Grammar placeholder treats "functions" as a future user-surface construct. |
| Appendix C / Y5 (L4215) | Rewrite catalog hard-codes Y5 as "any `.myco` function"; will reintroduce the retired surface unless updated. |

### open_questions.md (deprecated, but still mined)

| Location | What changes |
|---|---|
| Tier 0 (L74) | "Functions and contracts in the e-graph" — live question is now stdlib-atom opacity, not user `fn`. |
| Closure policy semantic interface (L512) | Still says custom policies are ordinary `.myco` functions. |
| Coupling & Kernels (L680) | Asks whether a kernel is "just a function"; live question is relation surface + stdlib axiom boundary. |
| Type system example (L409) | Named-generic-arg note still teaches `fn arrhenius<U: Unit>` as the canonical example. |
| Callable reuse across studies (L343) | Still uses `bind_controller(path, fn, Tree)` language. |

### Chunk reports

| Location | What changes |
|---|---|
| `03_kernels_in_progress.md` §§2–3, 7–8 | "Kernels are ordinary `.myco` functions" sentences are superseded; "no kernel keyword" decision itself survives. |
| `06_backend_abstraction_in_progress.md` §4.5 (L270) | Same `bind_controller(..., fn, ...)` ambiguity persists in backend docs. |

### spec.md (legacy)

| Location | What changes |
|---|---|
| §3.4 / §3.4.1 (L438) | Legacy contract invocation explicitly "function-like"; default impls method-shaped. If still mined, needs full relation-shaped rewrite. |
| §9.2 / §9.4 (L2093) | Still the source of many later examples and audit notes; Codex recommends marking explicitly unsafe for import. |
| Appendix A worked examples (L3839) | Example index celebrates function-like contract invocation, generic functions, and registered helper functions. |

### anti_spec.md

Currently retires annotations, not the whole user-`fn` surface. Add explicit entries for:
- User-defined `fn` declarations
- Kernels-as-functions framing
- Y5 as `.myco function`
- Contract function-like invocation / method wording
- Explicit capability annotations (`Invertible`, `Differentiable`, `Monotone`) declared in user code

### Mocks

Both `mock_sperry.myco` and `mock_potkay.myco` need declaration-level and expression-level rewrites, not just annotation stripping. Header comments ("contracts invoked as functions") also need updating — the files will mis-teach the new surface even before the `pub fn` blocks are touched.

### Audit reports

| Location | What changes |
|---|---|
| `audit/06_section_6_functions.md` (L91) | Still treats `fn` vs `relation` as a live design question; chunk 08 resolves this for the user surface. |
| `audit/adjudication.md` Batch 2 §6 (L120) | Same. |
| `audit/13_section_13_probabilistic_programming.md` (L104) | Proposed `Distribution<U>` contract block still written as `fn log_pdf` / `fn sample` / `fn reparameterized_sample`. |

## New questions the reviewers raised

Beyond the ones chunk 08 asks, reviewers flagged four additional questions that should be pinned before the surface sweep.

1. **Expression-position vs statement-position invocation (Codex).** Chunk 08 currently mixes the two. Lock statement-position only with an explicit result parameter, or you've recreated functions under a different keyword. This is the most load-bearing unpinned question.

2. **Local-intermediate syntax (Codex, follow-on to #1).** If invocation is statement-position only, what's the sanctioned way to introduce a fresh local result slot — for example, `arrhenius_scale(..., local_rate)` where `local_rate` is a one-shot intermediate? Needs a deliberate story.

3. **Y5 custom closure policies may not belong in `.myco` at all (Codex).** Built-in Y1–Y6 stay in-language; user-defined Y5 should move to the workflow layer. Experiment-side, not world-side.

4. **Variable/relation name-collision rule (Gemini).** If a type has field `f` *and* an internal relation `f(...)`, the grammar needs a disambiguation rule. One candidate: parentheses always denote a relation invocation, never a field access. Pin this now while syntax is plastic.

## Recommended order of operations

1. **Decide expression-vs-statement-position first.** This changes whether `use`/`call` keyword is needed and what the grammar looks like. Surface sweep depends on it.
2. **Decide Y5 placement.** If custom Y5 moves to workflow layer, §8.8, §24, Appendix C, open_questions "Closure policy semantic interface," and chunk 06's §4.5 all need coordinated edits rather than independent ones.
3. **Decide Distribution contract policy (§13).** Stdlib-callable exception or relation-shaped obligation. Both paths are defensible but they branch the anti_spec rules and the §13 prose.
4. **Then run the surface sweep** with the expanded list above plus chunk 08's original list.

## Inventory of models

- Codex (GPT-5.4, via `codex exec`, read-only sandbox): completed; produced a surface list grounded in specific line numbers and an additional-questions section.
- Gemini (via `gemini` CLI, sandbox, text output): completed; shorter surface list, cleaner yea/nay structure, raised the name-collision question.

Both reviews are preserved in the review temp directory for this session (not under version control); this report captures everything actionable.
