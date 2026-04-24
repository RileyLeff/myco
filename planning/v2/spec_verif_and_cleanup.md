- no riley project specific stuff
  - e.g. plant ecophys-specific stuff lives in a dedicated lib (called a spore, tentatively) external to the myco core project, i can implement that stuff separately, it doesn't belong here.

- no versioning in the spec
  - we're pre-alpha, if we say "2.1" or whatever i find that agents start to worry about what stuff belongs in 2.1 vs 2.2 vs 2.3, which is the wrong mentality. right now i want to get the spec correct and thorough up front so we don't have any surprise refactors later and minimize carnage moving forward.

- no legacy stuff or "we reorganized this" in the spec. i don't want the spec to reflect the entire history of thought and organization on the project, if something is stale or subseded, we don't want it in there. if something moved to a different section, it doesn't matter that it used to be in a previous section.

- if we encounter something that we DON'T want to be in the spec, list it in anti_spec.md so we don't have to revisit.

- mitigate duplication to the extent possible for token efficiency

- check chunk reports against spec new very thoroughly

- check each section of spec new against spec, v2.1, open questions, chunk reports. open to chunking this where a subagent is responsible for a few sections or something.

- consider token efficiency of spec

- do full within-spec review: any redundancy or conflict? anything marked as deferred to future that we really want to be in the open questions that we need to resolve before implementation?

- do full review of spec_dev_notes, spec.md, v2.1 in progress. anything in there that is currently homeless that hasn't made it into either spec or anti spec.md?

## Pre-ship mock rewrite obligations

Both `mocks/mock_sperry.myco` and `mocks/mock_potkay.myco` predate
the CC1 literal-numerics lock and other v2.1 surface decisions. Before
any release, they need a rewrite pass so the canonical examples
reflect the locked spec:

- **Strip literal values from universals.** Any `universal X =
  <number>` form in the mocks violates CC1 and §3.1. Rewrite as
  `universal X: Scalar<U>` and move the numeric value to the Python
  workflow via `assume_constant`.
- **Sperry specifically:** audit every `universal` declaration and
  move values to a paired `workflow_sperry.py` file alongside.
- **Potkay specifically:** same universal pass plus migration of any
  old `slot` / `[t+1]` timestep syntax to the locked `step(y) =
  expr` form (§10, §16).
- **Cross-check against anti_spec.md.** Any construct the mocks use
  that now lives in anti_spec.md (e.g. `slot`, `#[...]` annotations,
  `dyn` escape, `data contract` as a distinct kind) must be rewritten
  to the replacement surface.
- **Re-run through `mycoc check` once the compiler exists.** The
  mocks should compile without diagnostics against the locked spec;
  any remaining drift surfaces as compile errors at that point.

Mocks are canonical examples, so they double as a smoke test of the
locked surface. Keeping them aligned is a release gate.

## Canonicalization workflow

Seven-phase arc to move from the current multi-doc planning tree to a
single canonical spec plus role-specific scaffolds.

### Phase 0 — Tooling and index

- Add a section index at the top of spec_new.md (one line per numbered
  section with jump-anchor links).
- Add a 2-3 sentence summary at the top of each numbered section.
- Create a `justfile` with retrieval commands backed by uv-runnable
  Python scripts in `scripts/`:
  - `just spec-section 3.9` fetches one subsection in full.
  - `just spec-section 3.9 --summary` fetches only the summary header.
  - `just spec-roles compiler` generates the compiler scaffold
    (Phase 7).
  - `just spec-index` prints the index.
  - `just spec-verify` runs the integrity check (Phase 2 logic),
    optionally scoped to named sections.
- Scripts use PEP 723 inline metadata headers so `uv run
  scripts/foo.py` resolves dependencies automatically. Justfile
  recipes are thin wrappers.
- Update CLAUDE.md / AGENTS.md with a pointer to `just spec-*` as the
  default retrieval path and the edit-spec checklist below.

### Phase 1 — Per-section integration audit

For each numbered section in spec_new.md:

- Spawn a Haiku subagent with a consistent prompt template.
- Inputs: the target section in spec_new.md, the corresponding region
  in old spec.md, matching content in v2.1_in_progress.md, relevant
  chunk report sections.
- Output: structured four-bucket report.
  - **absorbed** — content that already landed.
  - **superseded** — content replaced by newer decision; should move
    to anti_spec.md.
  - **homeless** — content that exists somewhere but has not been
    absorbed or retired.
  - **conflicts** — direct contradictions between docs.
- Batch reports into groups of 4-5 sections for human adjudication.
- Apply fixes section-by-section.

Parallelism matters: ~20 concurrent Haiku subagents complete in the
time of one. Use a shared output directory under `tmp/audit/` so
results land predictably.

### Phase 2 — Cross-cutting integrity pass

Full-context check on the Phase-1-cleaned spec_new.md. One Sonnet
general-purpose subagent reads the entire doc and reports:

- Forward references to sections that changed number.
- Cross-refs to retired concepts (§X sends the reader to §Y, but §Y
  was absorbed or moved).
- Terms defined twice with different wordings.
- Anti-spec items still mentioned in the positive as if extant.
- Items marked "open" in §35 that are actually resolved elsewhere.
- Summary-vs-body drift (Phase 0 summaries no longer match the body
  prose after downstream edits).

One agent, full spec in context, one structured report out.

### Phase 3 — External parallel review

Run `riley-skills:review` in parallel mode against the Phase-2 spec:
Codex + Gemini + Claude subagent concurrently. Target: design
soundness, missing cross-refs, token-efficiency surprises, obvious
gaps a first-time reader hits.

Token budget: exclude chunk reports from dirgrab since they are about
to be archived. Spec alone should fit all three models.

Merged findings, human triage.

### Phase 4 — Archive and rename

- Move old spec.md to `planning/v2/archive/spec_old.md`.
- Move v2.1_in_progress.md to `planning/v2/archive/staging.md`.
- Keep chunk reports in place. Prepend a top-of-file marker to each:
  "Absorbed into spec.md §X" for closed chunks, "Design venue for open
  items: O4.1, O4.3, ..." for chunks still holding live design work.
- Rename spec_new.md to spec.md.
- Commit as a mechanical rename with no content change.

### Phase 5 — Fill thin preambles

Decided-but-underspecified content. Sections to fill:

- §2 Modules, Imports, Scope
- §6 Functions
- §20 SCC Decomposition
- §22 Plan Inspection
- §25 Training Emission
- §29 Units Library
- The five under-covered workflow verbs in §24 (`assume_constant`,
  `assume_series`, `learn_constant`, `learn_initial`,
  `learn_trajectory`, `observe`)

Source material sits in archived staging and chunk reports. Batch in
groups of 2-3, human adjudication per batch.

### Phase 6 — Close design opens

Chunks 03 (kernels), 04 (O4.x remainders), 05 (matrices B5), 06
(backend), 07 (type graph). Not part of canonicalization strictly;
queued after it.

### Phase 7 — Role-specific scaffolds

Generate `impl_guide_compiler.md`, `impl_guide_workflow.md`,
`impl_guide_backend.md` via `just spec-roles <role>`. The generator
script reads the spec index, pulls full sections relevant to the
role, pulls summaries for other sections, and concatenates. Size
target: 15-20k tokens per scaffold.

### Agent assignments

| Phase | Work | Agent |
|---|---|---|
| 0 | Index, summaries, justfile, scripts | Human + Claude (foreground) |
| 1 | Per-section audit (~20 units) | Haiku subagents in parallel |
| 1 | Adjudicate reports | Human |
| 2 | Full-spec integrity pass | Sonnet subagent |
| 3 | External review | `riley-skills:review` parallel mode |
| 4 | Archive and rename | Claude, mechanical |
| 5 | Fill preambles | Human + Claude per batch |
| 6 | Close design opens | Human-led design sessions |
| 7 | Scaffold generation | Scripted |

## Edit-spec checklist

When an agent edits the canonical spec, apply this checklist before
the edit is considered complete. The list lives in CLAUDE.md /
AGENTS.md so every agent picks it up at session start.

- Do not add historical breadcrumbs ("was X", "moved from Y", "in
  previous version").
- Do not add version references ("v2.1", "v2.2", "ships in vX.Y").
- Do not use em dashes in prose. Do not use "not X, Y" framings.
- Check whether the section summary at the top of the section still
  matches the body. Update the summary if the body changed.
- Check whether the section index needs an update (new subsection
  added, subsection renumbered).
- Grep for incoming cross-references to the edited section. Update
  any anchors that moved.
- If the edit retires content, move the retirement entry to
  anti_spec.md.
- If the edit defers content, add or update an entry in
  open_questions.md under the correct tier.
- If the edit touches a rewrite rule, reconcile Appendix C.
- If the edit changes a locked item's semantics, check whether any
  cross-cutting concept in §0.1 needs adjustment.
- After the edit, run `just spec-verify` scoped to the edited
  sections and their immediate cross-refs.

## Phase 1 execution plan (locked)

This is the detailed Phase 1 plan, recorded here so it survives
context compaction. If you resume Phase 1 in a fresh session, use
this as the source of truth.

### Model and mode

- Five parallel Sonnet subagents per batch (not Haiku; Riley trusts
  Sonnet's recommendations).
- Agents are READ-ONLY. They do not edit any file. They may grep,
  read, and write their own report file under `planning/v2/audit/`.
- Riley adjudicates all findings. Fixes are accumulated across all
  batches and applied in one coordinated pass at the end of Phase 1.

### Scope

All top-level sections of `spec_new.md`, including trackers,
placeholders, and appendices. Do not skip anything. Relevant content
may be lurking even in stub sections, and we want the audit to
surface it.

Section count: §0, §1, and §2 through §40, plus Appendix A / B / C.
Total: 44 top-level sections.

### Corpus each subagent greps

- `planning/soul.md`
- `planning/v2/spec.md` (old spec)
- `planning/v2/spec_dev_notes.md`
- `planning/v2/riley_project_note.md`
- `planning/v2/anti_spec.md`
- `planning/v2/v2.1_in_progress.md`
- `planning/v2/open_questions.md`
- `planning/v2/v2.1_chunk_reports/*.md` (all seven)

### Per-section report format

Four buckets, written to
`planning/v2/audit/<NN>_section_<slug>.md`:

- **absorbed** — content that already landed in spec_new.md §<N>.
- **superseded** — content in the corpus that has been replaced by a
  newer decision and should move to anti_spec.md (if not already
  there).
- **homeless** — content in the corpus that is relevant to this
  section, is not accounted for in spec_new.md §<N>, and is not
  already committed to anti_spec.md. This is the most important
  bucket; it is what we would lose by canonicalizing.
- **conflicts** — direct contradictions between spec_new.md §<N>
  and the corpus.

For any finding, the agent may append one line starting
`Recommend:` with a proposed disposition (absorb text X into §Y,
move to anti_spec under heading Z, add entry to open_questions, etc.).
These are suggestions only; Riley decides.

### Chunk-report treatment

- Reports `01_geometry_design_report.md`,
  `02_collections_iteration_report.md`, and
  `05_matrices_in_progress.md` are closed. Treat absent
  content from these as candidate homeless or superseded.
- Reports `03_kernels_in_progress.md`,
  `04_egraph_foundation_in_progress.md`,
  `06_backend_abstraction_in_progress.md`,
  `07_type_graph_in_progress.md` are still open design venues.
  Content in these that is legitimately open and absent from
  spec_new.md is NOT homeless; it belongs to the in-progress work
  tracked under §33 / §34. Only flag as homeless if it is a stable
  decision that has not been reflected.

### Batches

Nine batches of five (last batch four):

| Batch | Sections |
|---|---|
| 1 | §0, §1, §2, §3, §4 |
| 2 | §5, §6, §7, §8, §9 |
| 3 | §10, §11, §12, §13, §14 |
| 4 | §15, §16, §17, §18, §19 |
| 5 | §20, §21, §22, §23, §24 |
| 6 | §25, §26, §27, §28, §29 |
| 7 | §30, §31, §32, §33, §34 |
| 8 | §35, §36, §37, §38, §39 |
| 9 | §40, Appendix A, Appendix B, Appendix C |

Each batch launches as five parallel subagents in a single message.
Riley reviews the five reports after each batch before the next
launches.

### Subagent prompt template

```
You are auditing §<ID> ("<TITLE>") of the canonical Myco v2 spec at
/Users/rileyleff/Documents/dev/myco/planning/v2/spec_new.md.

Your job is READ-ONLY. Do not edit any file. Write your report to
/Users/rileyleff/Documents/dev/myco/planning/v2/audit/<NN>_section_<slug>.md.

Corpus to grep for relevant content:
- planning/soul.md
- planning/v2/spec.md
- planning/v2/spec_dev_notes.md
- planning/v2/riley_project_note.md
- planning/v2/anti_spec.md
- planning/v2/v2.1_in_progress.md
- planning/v2/open_questions.md
- planning/v2/v2.1_chunk_reports/*.md

Use `just spec-section <ID>` to fetch the section under audit.

Report format (markdown, four sections):

## Absorbed
Corpus content that already landed in spec_new.md §<ID>. One bullet
per item with corpus file path and a short quote.

## Superseded
Corpus content that has been replaced by a newer decision in
spec_new.md §<ID>. Should move to anti_spec.md if not already there.
One bullet per item with corpus file path, short quote, and the
spec_new.md §<ID> text that supersedes it. If already in
anti_spec.md, say so and skip.

## Homeless
Corpus content that is RELEVANT to §<ID>, is NOT accounted for in
spec_new.md §<ID>, and is NOT already committed to anti_spec.md.
This is the highest-value bucket. One bullet per item with corpus
file path, short quote, and a brief assessment of whether it should
land in §<ID>, move to anti_spec.md, or open a new open_questions
entry. Prefix the assessment with `Recommend:`.

For chunk reports 03-07 (in-progress), content that is legitimately
open design work absent from spec_new.md is NOT homeless. Only flag
as homeless if it is a stable decision that never made it into
spec_new.md.

## Conflicts
Direct contradictions between spec_new.md §<ID> and any corpus
document. One bullet per conflict with both sides quoted. Include a
`Recommend:` line for how to resolve.

Style rules for your report:
- No em dashes in your own prose.
- No version strings (v2.1, etc.).
- No historical breadcrumbs ("was X", "absorbed into Y", "previously").
- Quotes from corpus may contain any of these; leave them as-is
  inside blockquotes.
```

### Exit criteria

Phase 1 is complete when:

- All 44 reports exist under `planning/v2/audit/`.
- Riley has adjudicated each report and marked the disposition of
  each finding (accept / reject / defer to open_questions / move
  to anti_spec).
- The accumulated fix list has been applied to spec_new.md,
  anti_spec.md, and open_questions.md in one coordinated pass.
- `just spec-verify` passes with zero findings.
