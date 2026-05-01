# Audit Report — §37 Dependency Management and Package Registry

Audited against corpus as of 2026-04-22.

---

## Absorbed

Corpus content already reflected in spec_new.md §37.

- **`planning/v2/v2.1_chunk_reports/10_package_dependencies.md` "Status" header and §"Minimum viable package system for v2.1":**
  > "Needs dedicated design work before any of this becomes normative spec text. Not blocking for the core language lock; can land post-v2.1 if needed."

  §37's placement in Part VII ("Developer Experience, Deferred") and its stub presentation — listing topics without pinning any design decision — correctly reflects chunk 10's stated posture that full spec-level prose is deferred. The section exists as a surface holder, consistent with the chunk 10 status assessment.

- **`planning/v2/v2.1_chunk_reports/10_package_dependencies.md` "Vocabulary" section:**
  > "Spore — a Myco package... Hypha — the CLI tool... myco.toml — per-package manifest... myco.lock — resolved dependency tree..."

  The §37 **Summary** line ("How `.myco` packages declare dependencies, resolve versions, publish, and lock") correctly implies the four vocabulary terms by topic coverage. The terms are named explicitly in the §33 open-questions block at line 5020-5030 ("Package dependency story"), so the vocabulary lock has a home in spec_new.md even though §37 itself does not name the four terms directly.

- **`planning/v2/v2.1_chunk_reports/10_package_dependencies.md` "Principle" section:**
  > "Follow Cargo and uv closely... reinventing dependency management is not where Myco's novelty should live."

  The §33 open-questions block (lines 5020-5023) states: "the overall shape follows Cargo + uv conventions (chunk 10)." That block is not part of §37, but it is the correct location for locked principles in spec_new.md's structure. §37 being a stub with no conflicting principle is consistent.

- **`planning/v2/v2.1_chunk_reports/10_package_dependencies.md` "Workspaces" section cross-reference to Python:**
  > "Open: how the workspace interacts with the Python workflow side."

  §37 Summary: "Interacts with but stays distinct from the Python workflow layer's package system." This one-liner captures the locked boundary posture (distinct layers) and correctly leaves the interaction mechanics open, consistent with chunk 10's open item on workspace-Python interaction.

- **`planning/v2/spec_new.md` §2 cross-reference:**
  > "Package-level dependencies (cross-spore imports, version resolution, workspace layout) are a separate concern from file-as-module scoping and are covered in `v2.1_chunk_reports/10_package_dependencies.md`."

  §2 correctly delegates to chunk 10, which §37 is the eventual spec home for. The cross-reference chain (§2 -> chunk 10 -> §37) is internally coherent.

---

## Superseded

No content in §37 or chunk 10 has been superseded by a newer decision elsewhere in the corpus. The vocabulary and Cargo+uv principle from chunk 10 are live locked decisions with no replacement. The stub body of §37 does not contain any design claim old enough to have been revised.

---

## Homeless

Corpus content relevant to §37 not yet reflected in §37 itself and not committed to anti_spec.md.

- **`planning/v2/v2.1_chunk_reports/10_package_dependencies.md` four locked vocabulary terms absent from §37.**

  Chunk 10's "Vocabulary" section locks four terms: `spore` (package), `hypha` (CLI), `myco.toml` (manifest), `myco.lock` (lockfile). These are committed decisions, not open work. §37 currently names none of them. The terms appear in the §33 open-questions block (lines 5020-5022) but that block is an open-questions summary, not a normative section, and carries the open items alongside the locked ones without distinguishing them. A stub section in a deferred part is the appropriate place to at least state what is already locked about the area, so that the stub serves as a useful placeholder and not a blank surface.

  `Recommend:` Add a **Vocabulary (locked)** paragraph to §37 naming the four terms and their roles. One sentence each suffices. This does not require resolving any open item; it simply moves the locked vocabulary from the §33 open-questions block into the section whose subject it directly describes.

- **`planning/v2/v2.1_chunk_reports/10_package_dependencies.md` Cargo+uv convention lock absent from §37.**

  Chunk 10 "Principle" section locks the overall shape: follow Cargo and uv, deviate only where the underlying model forces it. This is a committed design decision. §37 contains no mention of this principle. The §33 block at line 5022 says "the overall shape follows Cargo + uv conventions (chunk 10)" but that appears in an open-questions summary, not in §37 where the packaging design lives.

  `Recommend:` Add a **Approach (locked)** line to §37 stating that the dependency system follows Cargo and uv conventions adapted for Myco, and that it deviates from those conventions only where the Myco model forces it. A single sentence is sufficient; the full rationale lives in chunk 10.

- **`planning/v2/v2.1_chunk_reports/10_package_dependencies.md` open items list not surfaced in §37.**

  Chunk 10 "Open items" section lists nine distinct open questions: resolver algorithm (PubGrub vs uv's variant), version semantics (what counts as a breaking change for Myco constructs), feature model (whether and at what granularity), build scripts and codegen, workspace-Python interaction, tooling integration, cross-spore relation visibility (`pub(crate)`-style private relations), registry story, and platform/backend metadata in the manifest. The §33 block (lines 5023-5028) names six of these nine: resolver algorithm, version semantics, feature model, build-script/codegen surface, workspace-Python interaction, cross-spore relation visibility, registry story, and platform/backend metadata. (The §33 block actually names eight; tooling integration is the one omitted there, appearing separately in §38.) §37 itself names none of them -- it describes the open area with "Version resolution. Package registry layout and publishing workflow. Lockfile format." without naming specific open design questions.

  A stub section that names specific open items is more useful than one that names topic areas, because the former tracks what must be resolved before §37 can graduate from stub to normative prose.

  `Recommend:` Add an **Open items** list to §37 enumerating the chunk 10 open items. The §33 block's enumeration (lines 5023-5028) can serve as the source; copy and expand it into §37's body so the section is self-contained. Cross-reference chunk 10 as the canonical reference.

- **`planning/v2/v2.1_chunk_reports/10_package_dependencies.md` minimum-viable subset not named in §37.**

  Chunk 10's "Minimum viable package system for v2.1" section lists the subset needed to get real work done: `myco.toml` with `[package]` and `[dependencies]`, git and path dependencies, basic deterministic `myco.lock`, five `hypha` verbs (`new`, `add`, `build`, `check`, `run`), Cargo-style workspaces, and `use` imports resolving via `myco.toml`. It separately lists what is deferred post-v2.1: central registry, features, build scripts, advanced resolver features, publish workflow, cross-platform lockfile. Neither the minimum viable subset nor the post-v2.1 deferral list appears anywhere in §37 or in spec_new.md outside chunk 10 itself.

  `Recommend:` Add a **Minimum viable scope (v2.1)** paragraph to §37 naming the committed MVP subset and the explicitly deferred post-v2.1 items. This gives the section a concrete boundary and prevents the stub from being treated as uniformly open.

- **§34 omits chunk 10 (confirmed by the §34 audit).**

  The §34 audit (`planning/v2/audit/34_section_34_chunk_slotted_work.md`) found that chunk 10 is not enumerated in §34 and recommended adding a Chunk 10 entry. §37 is the eventual spec-prose home for chunk 10's content, but §34 is where in-progress chunks are tracked. The §37 audit confirms this finding: §37 exists as a stub, but there is no §34 entry pointing to chunk 10 as the active in-progress canonical reference. The two sections are consistent (both are stubs or absences), but both should be addressed together.

  `Recommend:` When the §34 Chunk 10 entry recommended by the §34 audit is added, ensure it cross-references §37 as the eventual spec home. No additional action on §37 itself is needed beyond the vocabulary/approach/open-items/MVP additions above.

- **`planning/v2/v2.1_chunk_reports/10_package_dependencies.md` `hypha` CLI verb menu not named in §37.**

  Chunk 10 "Hypha CLI verbs (sketch)" section lists twelve verbs (`new`, `init`, `add`, `remove`, `build`, `check`, `run`, `test`, `lock`, `update`, `publish`, `workspace list`) as a sketch. Verb menu and flags are open. §36 covers the `myco` CLI; §37 covers the `hypha` CLI but currently does not name `hypha` at all. The distinction between the `myco` CLI (§36) and the `hypha` CLI (§37) is not visible in either stub, and a reader has no indication that the packaging tool has its own distinct CLI binary.

  `Recommend:` Add a sentence to §37's body noting that the packaging CLI is `hypha` (distinct from the `myco` compiler CLI in §36), consistent with the Rust `rustc`/`cargo` and Python `python`/`uv` precedents that chunk 10 cites. The full verb menu need not appear; naming `hypha` as the CLI and noting it is distinct from `myco` is sufficient for a stub.

---

## Conflicts

Direct contradictions between spec_new.md §37 and corpus documents.

- **§37 uses "`.myco` packages" rather than "spores" throughout, inconsistent with the locked vocabulary.**

  §37 body (lines 5068, 5072, 5074) uses "`.myco` packages" as the unit of distribution. Chunk 10 "Vocabulary" section locks `spore` as the distribution unit: "Spore — a Myco package. Ecosystem-level unit of distribution." The spec_new.md §2 cross-reference (line 232) uses "cross-spore imports" and "spore" correctly. The §23 Python-boundary section (line 3708-3710) uses "Spore authors ship one artifact (`.myco` sources plus `myco.toml`)." The §33 block (line 5020) uses "spore for packages." §37 is the only section in spec_new.md that uses "`.myco` packages" as a noun phrase where "spore" should appear.

  The mismatch is minor but creates an inconsistency in the one section nominally dedicated to packaging. A reader reading §37 cold does not learn that the distribution unit has a name.

  `Recommend:` Replace "`.myco` packages" in §37 body lines 5068, 5072, and 5074 with "spores" (or "`.myco` spores" on first use if clarity requires the qualifier). This makes §37 consistent with every other section in spec_new.md that touches the distribution unit.

- **§37 Summary says "resolve versions, publish, and lock" without distinguishing locked from open items, implying uniform open status that contradicts chunk 10.**

  The §37 Summary ("How `.myco` packages declare dependencies, resolve versions, publish, and lock") presents dependency declaration, version resolution, publishing, and locking as co-equal open topics. But chunk 10 distinguishes them: `myco.lock` format and the lockfile role are part of the locked approach (Cargo-style, committed for applications); the resolver algorithm is open; publishing is entirely open (registry not required for v2.1). Presenting all four as uniform future work misrepresents chunk 10's partial-lock status.

  `Recommend:` The Summary line can remain brief, but the body should signal the locked/open split. Adding the vocabulary and locked-approach paragraphs recommended under Homeless (above) would resolve this implicitly by showing what is already committed alongside what remains open.
