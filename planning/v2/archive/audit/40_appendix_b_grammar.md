# Audit Report — Appendix B: Grammar / EBNF Summary

Audited against corpus as of 2026-04-22.

---

## Absorbed

Appendix B correctly positions itself as a deferred placeholder. The deferral
rationale is accurate: the surface language is not yet stable enough for a
normative EBNF. Multiple chunk reports confirm this state explicitly — chunk
11 lists "exact syntax" of `enum` / `match` / pattern forms as an open item
(v2.1_chunk_reports/11_sum_types_enums.md, lines 295-297), chunk 09 flags
node-path syntax as open (v2.1_chunk_reports/09_workflow_data_layer.md, lines
60, 194), and chunk 03 defers the rewrite-rule declaration surface
(v2.1_chunk_reports/03_kernels_in_progress.md, line 166). A normative grammar
committed today would be stale before it shipped.

The `spec_dev_notes.md` entry at line 525 confirms the provenance: "Grammar /
EBNF summary — 2026-04-21 stub added as Appendix B; left open for post-lock
pass." The placeholder was intentional.

---

## Superseded

Legacy `spec.md` has an "Appendix B" (lines 3957-4007) titled "Developer
Experience," covering VSCode syntax highlighting, LSP, and related tooling.
That appendix is entirely unrelated to a grammar/EBNF summary — it is the old
dev-experience appendix that was reorganized into Part VII (Developer
Experience, §36-§40) in spec_new.md. The "Appendix B" label has been reused
for a different purpose. No EBNF or grammar production material appears
anywhere in legacy spec.md; the only grammar reference there is a mention of a
"TextMate grammar for .myco files" (line 3964), which is a tooling artifact,
not a language grammar.

Nothing in legacy spec.md constitutes EBNF material that was dropped or that
Appendix B should absorb.

---

## Homeless

Several concrete syntax decisions have been made and are recorded in chunk
reports but are not tracked in Appendix B or elsewhere as grammar-precursor
material. These are not EBNF productions, but they are the locked points that
a future grammar pass will need:

1. **Relation invocation form.** Chunk 08 (v2.1_chunk_reports/08_relation_fix_whoops.md,
   lines 55-137) locks the full concrete syntax of parameterized-relation
   declarations and invocations: `relation name(param: Type, ...):` with
   equation body, statement-form-only invocation, no expression-position
   invocation, no missing-slot inference. The grammar rule is stated
   explicitly: "parentheses after a name always parse as invocation" (line
   132). This is a parse-level commitment not reflected in Appendix B.

   Recommend: When Appendix B is filled, chunk 08's relation-invocation rules
   are the first input. Consider a cross-reference note in the Appendix B
   placeholder pointing to chunk 08 as locked prior art.

2. **`impl` / `some` keyword disambiguation.** Chunk 02
   (v2.1_chunk_reports/02_collections_iteration_report.md, lines 40-45) locks
   the replacement of the overloaded `dyn` keyword with `impl` (static
   monomorphization) and `some` (runtime variable sizing). Both appear in
   Appendix A as type-former keywords. Their syntactic position rules
   (appearing inside collection brackets `[Type; some]`, `[Type<impl C>; N]`)
   are stated in code examples but not as grammar productions.

   Recommend: Capture the bracket-position rule for `impl` and `some` as an
   early grammar candidate when Appendix B is written.

3. **Method-style dispatch sugar.** Chunk 08 (lines 105-118) locks that
   `receiver.rel(args...)` desugars to `rel(receiver, args...)` for relations
   whose first parameter is typed `Self`. This is a syntactic transformation
   rule — a grammar-level concern.

   Recommend: Note alongside the relation-invocation production when Appendix
   B is drafted.

---

## Conflicts

Appendix B's scope statement lists "§2 through §14 (types, values, units,
functions, contracts, relations, constraints, events, geometry, stdlib calls,
workflow-boundary syntax)" as the intended coverage.

Two scope issues:

1. **"Workflow-boundary syntax" is §23, not §2-§14.** The parenthetical in
   the placeholder names "workflow-boundary syntax" as part of the §2-§14
   range, but §23 ("The .myco <-> Python Boundary") is Part III, outside Part
   I. This is a minor scope-description inconsistency rather than a content
   conflict — the intent is clear — but the parenthetical should read "§2-§14
   plus §23 workflow-boundary syntax" or simply drop the parenthetical
   enumeration, which risks going stale.

2. **§27 (Distribution Families) surface.** Appendix A already reserves
   distribution-family names as stdlib-reserved identifiers. The `~` operator
   and its contract-parameter keywords (`<Ito>`, `<Stratonovich>`) are also
   listed in Appendix A. If Appendix B intends to cover "stdlib calls" as
   noted in its parenthetical, the `~` binding operator and distribution-
   family call syntax belong in scope. The §2-§14 range excludes §27, so
   either the scope claim needs widening or "stdlib calls" should be narrowed
   to §6 function-call syntax only. No normative harm today since Appendix B
   makes no locked claims, but the scope description will need tightening
   before it is filled.
