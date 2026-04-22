# Audit: §2 Modules, Imports, Scope

Section under audit: `spec_new.md §2 "Modules, Imports, Scope"`

---

## Absorbed

Content from the corpus that already landed in spec_new.md §2.

- **`planning/v2/spec.md` §1 "Modules"** — `module path::name` declaration, `use path::{items}` imports, `pub` visibility, DAG import constraint, `node name: Type` reserved for model modules, library vs model module distinction, `pub` controls inter-module visibility only (workflow can bind any path regardless). All of these decisions appear word-for-word as the conceptual spine of spec_new.md §2 ("File-as-module convention. Path-based imports. Visibility rules. Scope resolution rules").

- **`planning/v2/v2.1_in_progress.md` §"Module System"** — settled entries for `module path::name`, `use path::{items}`, `pub`, library vs model modules, and `node name: Type`. All absorbed. The note "Open: add the lib/bin analogy framing to the spec prose" is satisfied by spec_new.md §2's summary prose.

- **`planning/v2/v2.1_in_progress.md` line 469** — "Module-scope `initial`/`temporal` forms are retired." The retirement is captured in spec_new.md §9: "`initial:` and `temporal:` blocks live in type bodies. Module-scope only for truly cross-entity relations." Also reflected in the §2 summary's absence of module-scope temporal forms.

- **`planning/v2/spec.md` §1.2** — "`pub` controls inter-module visibility only. The workflow layer (Python API) can bind any path in the model regardless of visibility." Absorbed into spec_new.md §2's "public/private/file-local visibility" framing and cross-referenced in the Python/`.myco` split sentence.

- **`planning/v2/spec.md` §1.3** — "Circular imports are disallowed. The module dependency graph must be a DAG." Implied as settled; spec_new.md §2 does not restate it explicitly but the concept is uncontested across corpus.

- **`planning/v2/spec_dev_notes.md` §2026-04-20 gap-review** — item 2 flagged "`node` instantiation + module semantics underspecified." The spec_dev_notes note that §2 already covers modules/imports (Riley's addition). spec_new.md §2 keeps that coverage; `node` semantics are addressed in §3 (Types) not §2 itself, which matches the section-split intent.

---

## Superseded

Content replaced by a newer decision in spec_new.md §2.

- **`planning/v2/spec.md` §1.1** — library modules may contain "`dyn` contract references." `dyn` is retired (replaced by `impl`/`some`). Already in `anti_spec.md` ("| `dyn` | `impl Contract` (static monomorph) + `some` (runtime sizing) ..."). Skip.

- **`planning/v2/spec.md` §1 "Library modules that contain module-scope relations, temporal equations, or slots must have a top-level type"** — the "implicit top-level type" mechanism is retired by v2.1_in_progress.md ("Module-scope `initial`/`temporal` forms are retired") and encoded in anti_spec.md ("| module-scope `initial:` / `temporal:` per-type | in-type-body `initial:` / `temporal:` ..."). The `slot` keyword is also retired. Already in anti_spec.md. Skip.

- **`planning/v2/spec.md` §1.1** — `dyn` references in library modules ("may contain unresolved generics and `dyn` contract references"); superseded by `impl`/`some` split. Already in anti_spec.md. Skip.

- **`planning/v2/v2.1_in_progress.md` Module System note** — "Open: add the lib/bin analogy framing to the spec prose (tracked in riley_spec_notes.txt)." The tracker note is now moot because spec_new.md §2 includes the lib/model module distinction. No anti_spec.md entry needed; the open item is resolved.

---

## Homeless

Corpus content relevant to §2, not accounted for in spec_new.md §2, and not committed to anti_spec.md.

- **`planning/v2/spec.md` §1** — "Modules may re-export items from other modules." This is a concrete language rule about `pub use` or equivalent syntax. spec_new.md §2 lists "public/private/file-local visibility" but says nothing about re-exports. The `v2.1_in_progress.md` Module System entry also lists it as settled ("Imports. Modules may re-export."). The mechanism for re-export (explicit `pub use path::item`? implicit?) is never specified.

  Recommend: land in §2. The settled status is clear; the surface spelling needs one sentence.

- **`planning/v2/spec_new.md` §2 itself** — claims a three-tier visibility model: "public / private / file-local." The `file-local` tier does not appear in any corpus document. `spec.md` §1.2 defines only two tiers (private by default, `pub` to expose). `v2.1_in_progress.md` Module System also shows only two tiers ("Items private by default. `pub` controls inter-module visibility only"). The third tier is introduced in spec_new.md §2 without any design backing in the corpus.

  Recommend: open a new open_questions entry. Either the file-local tier is a new undecided design element (needs a decision on syntax, e.g. `pub(file)` or no keyword), or it should be collapsed back to two tiers.

- **`planning/v2/spec_dev_notes.md` lines 553-558 ("riley notes for open questions")** — "a package in myco is called a spore ... crates.io-like thing (bigmyco.com/spores) ... rustdoc-like thing." Package naming and distribution are relevant to what `use path::to::symbol` paths refer to and how the module resolver locates packages. spec_new.md §2 says nothing about package layout, `Myco.toml`, or the spore distribution model.

  Recommend: this is pre-implementation tooling, not a blocking language design question for §2. Open a new open_questions entry (Tier 2) capturing the "spore" name, the registry concept, and the filesystem-to-module-path mapping question. Do not add to anti_spec.md; it is not retired.

- **`planning/v2/v2_old/convo_backup.md` line 15599** — "The spec doesn't describe how module paths map to filesystem paths or how the module resolver works. Is it `plant/sperry.myco`? A directory `plant/sperry/`? This matters for the package registry story." This is a stable gap that predates the corpus reorganization and was never answered in any locked document.

  Recommend: consolidate with the spore/registry open question above into one new open_questions entry (Tier 2).

- **`planning/v2/spec.md` §1.3** — "Circular imports are disallowed. The module dependency graph must be a DAG. The compiler reports a cycle with the full import chain if one is detected." This is a concrete compiler behavior rule, settled status, not restated in spec_new.md §2.

  Recommend: land in §2. One sentence is sufficient; it is uncontested.

- **`planning/v2/v2.1_in_progress.md` lines 565-568** — "Spec §2 module-scope declarations: Retire the 'implicit top-level type' mechanism for module-scope `temporal`/`relation`/`initial`. State evolution belongs on types; `initial`/`temporal` are legal inside type bodies. Strike module-scope forms." This is a write-up instruction, not yet executed in spec_new.md §2. The retirement is in anti_spec.md and in §9's prose, but §2 itself contains no explicit statement that module-scope `initial`/`temporal` are disallowed.

  Recommend: add a single sentence to §2 stating that `initial:` and `temporal:` are not legal at module scope (pointing readers to §9 for the type-body form). The anti_spec.md entry already covers the retirement; §2 just needs the positive statement that scope is type-body-only.

---

## Conflicts

- **`planning/v2/spec_new.md` §2 vs `planning/v2/spec.md` §1.2 and `planning/v2/v2.1_in_progress.md` Module System** — spec_new.md §2 asserts a three-tier visibility model ("public / private / file-local"), while both legacy documents describe only two tiers.

  spec_new.md §2: "public/private/file-local visibility"

  spec.md §1.2: "Items are private by default. The `pub` keyword makes an item visible to importing modules."

  v2.1_in_progress.md: "Inter-module visibility. Items private by default. `pub` controls inter-module visibility only. **Status: settled.**"

  Recommend: resolve in §2. Either (a) document the syntax and semantics for the `file-local` tier and update v2.1_in_progress.md's settled entry, or (b) remove `file-local` from spec_new.md §2 as an undecided addition and open a Tier 2 question.
