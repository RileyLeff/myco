# Audit Report — §36 Command-Line Interface

Audited against corpus as of 2026-04-22.

---

## Absorbed

Corpus content already reflected in spec_new.md §36.

- **`planning/v2/v2.1_chunk_reports/10_package_dependencies.md` "Minimum viable package system for v2.1":**
  > "hypha new, hypha add, hypha build, hypha check, hypha run"
  > "Vocabulary locked (spore, hypha, myco.toml, myco.lock). Overall approach locked (Cargo + uv, adapted). Details open."

  The `check` and `run` subcommands named in §36's stub list are consistent with the MVS listing in chunk 10. The stub's posture — listing subcommands without locking flags, exit codes, or output conventions — matches chunk 10's "Details open" status. The deferral framing is correctly applied.

- **`planning/v2/spec_new.md` §23.4 (lines 3827-3828):**
  > "Tooling distinction: `mycoc check` catches tier-1 errors; workflow composition surfaces tier-2."

  §36 lists `check` as a subcommand. §23.4 has already made a partial commitment about what `check` does (tier-1 error detection). §36 does not contradict this; it simply leaves the full flag and exit-code surface open, which is correct for a stub.

- **`planning/v2/spec_new.md` §22 (lines 3668, 3674):**
  > "The `mycoc explain` CLI exposes the compiled plan for auditing, debugging, and verifying compilation choices."
  > "inspectable via `mycoc explain` (and related CLI surfaces, §36)"

  §36 lists `explain` as a subcommand and §22 already uses it as a concrete cross-reference. The stub correctly includes `explain` and defers the surface details to when the CLI is specified. The §22 cross-reference to `§36` is consistent with the stub's role as the future CLI spec home.

---

## Superseded

- **`planning/v2/spec.md` Appendix B.3 (lines 3987-3993): `myco fmt` invocation and Appendix B.8 (lines 4087-4107): `myco repl`, `myco plan --dot`:**
  > "Run as `myco fmt` or on save in the editor."
  > "`$ myco repl sperry/mechanics.myco`"
  > "`myco plan --dot | dot -Tsvg > plan.svg`"

  The legacy spec treats the entire user-facing CLI as a single `myco` binary with subcommands `fmt`, `repl`, `plan`, `add`, `publish`, `search`. Chunk 10 (locked vocabulary) splits this into two distinct binaries: `mycoc` for the compiler and `hypha` for the package manager, following the `rustc`/`cargo` and `python`/`uv` precedents. The legacy single-binary framing is superseded by this split. The legacy verbs `repl`, `plan --dot`, `add`, `publish`, and `search` have no clear home in the current two-binary model and cannot be imported from spec.md during any consolidation pass.

  `Recommend:` No new anti_spec.md entry is needed; spec.md is covered wholesale as stale. Flag the single-`myco`-binary model as superseded here so consolidation passes do not re-import it.

---

## Homeless

### H1 — `mycoc`/`myco` split: §36 calls it the `myco` CLI, but the compiler binary is `mycoc` throughout spec_new.md

- **`planning/v2/spec_new.md` §0.1 (lines 103, 110), §22 (line 3668), §23.4 (lines 3803, 3812, 3828), §4.1 (lines 556, 561), §32 (line 5122):**
  > "`mycoc explain` (§22)"
  > "`mycoc` compile errors catch type, unit, contract, and structural problems"
  > "`mycoc check` catches tier-1 errors"
  > "Violations surface as `mycoc` compile errors"
  > "will emit a `mycoc` parse error"

  §36 (lines 5059, 5063) calls the CLI "`the myco CLI`" and lists its subcommands (`compile`, `run`, `check`, `fmt`, `explain`). Every other reference to the compiler frontend in spec_new.md uses `mycoc`, not `myco`. The two-binary split locked in chunk 10 (`mycoc` for compiler, `hypha` for package manager) implies that the compiler binary is named `mycoc`, not `myco`. A stub that calls this surface "the `myco` CLI" is internally inconsistent with the rest of the spec.

  There is a plausible interpretation — `myco` as the brand wrapper (analogous to `cargo`, which wraps `rustc`) — but no corpus document states this interpretation explicitly, and the §37 audit (line 82-84 of `37_section_37_dependency_mgmt.md`) distinguishes `hypha` from "`the myco compiler CLI in §36`" using `myco` as a label, not `mycoc`. The two audits use different names for the same binary.

  `Recommend:` Decide whether the compiler-facing CLI binary is named `mycoc` (consistent with all spec_new.md usage) or `myco` (consistent with §36 as written and the legacy spec). If `mycoc`, update §36 to use `mycoc` throughout. If `myco` is a user-facing wrapper that delegates to `mycoc`, state that explicitly in §36 so the naming is intentional and not an oversight. Coordinate with §37 to ensure both stubs use the same label for the compiler CLI.

### H2 — `mycoc explain` has a specific subcommand surface committed in chunk 04, not named in §36

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` (line 475):**
  > "Diagnostics (`mycoc explain path_A --vs path_B` prints both loss bounds)."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` (line 1076):**
  > "`mycoc explain` shows both modes with labels."

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md` Phase 2 Q3 (lines 1173-1180):**
  > "How `mycoc explain` navigates between the two views."
  > "How user-facing error messages reference equivalence classes."
  > "Round-trip for diagnostics: given a residual-graph node, how to materialize the full e-class it came from."

  §36 lists `explain` as a subcommand in one phrase with no further detail. Chunk 04 has committed a specific flag (`--vs path_A path_B` for comparing alternative extraction paths and their loss bounds) and identified three open design items for the subcommand: how it navigates between e-graph and residual graph views, how error messages reference e-classes, and how to round-trip from a residual-graph node to its originating e-class. These are not trivial display choices; they depend on the e-graph / residual-graph architecture in §15. They are load-bearing design questions for `explain`, not implementation detail.

  `Recommend:` Add a note to §36 that `explain` has a partially committed surface: the `--vs path_A path_B` flag for comparing alternative extraction paths (sourced from chunk 04 §5) and three open design items (e-graph vs residual-graph navigation, e-class referencing in error messages, round-trip diagnostic materialization) pending the chunk 04 Phase 2 Q3 resolution. This prevents the §36 stub from being read as a uniform blank slate and cross-references the correct design home for `explain`'s open items.

### H3 — `mycoc check` is already a committed verb with a defined scope, not just a named subcommand

- **`planning/v2/spec_new.md` §23.4 (lines 3827-3828):**
  > "Tooling distinction: `mycoc check` catches tier-1 errors; workflow composition surfaces tier-2."

- **`planning/v2/spec_dev_notes.md` (line 529):**
  > "`mycoc check` smoke gate" — named as a pre-ship checklist item.

  `check` appears in §36's subcommand list as if it is uniformly deferred alongside `compile`, `run`, `fmt`, and `explain`. But §23.4 has already committed the scope of `check`: it runs the compiler through the tier-1 error surface (type, unit, contract, structural errors) without workflow binding. This is a locked behavioral commitment that belongs in §36 as a note on the `check` subcommand, not in §23.4 alone.

  `Recommend:` Add a sentence to §36 noting that `check` is a committed verb (runs compiler error detection without codegen or workflow binding, catches tier-1 errors as defined in §23.4), with flag conventions and exit codes still open. This makes §36 more useful as a stub by distinguishing what is committed about `check` from what remains open.

### H4 — `hypha explain` appears in chunk 08, naming a different binary from `mycoc explain`

- **`planning/v2/v2.1_chunk_reports/08_relation_fix_whoops.md` (lines 448-451):**
  > "`hypha explain`, IDE hover, and plan-inspection surfaces expose derived properties with proof chains or explicit 'unknown because ...'. Never a source-level annotation."

  This passage uses `hypha explain` where other corpus documents use `mycoc explain`. It is ambiguous whether chunk 08 means: (a) the `explain` subcommand of `hypha` (the package manager CLI) as a separate `hypha`-specific verb that inspects package metadata; or (b) a typo or early-draft label for `mycoc explain`. If (a), then the CLI surface has three binaries or subcommand groups: `mycoc` (compiler), `hypha` (package manager), and `hypha explain` (an inspection verb that may overlap with `mycoc explain`). If (b), the chunk 08 passage is simply inconsistent with the rest of the corpus.

  `Recommend:` Clarify whether `hypha explain` is a distinct verb from `mycoc explain`. If they are the same surface with a naming inconsistency in chunk 08, note the inconsistency in §36. If `hypha explain` is a legitimate separate verb (exposing derived properties for package-level inspection, rather than per-compilation plan inspection), it belongs in §37 rather than §36, and §36 should cross-reference it.

### H5 — `mycoc`/`hypha` two-binary split is not stated in §36

- **`planning/v2/v2.1_chunk_reports/10_package_dependencies.md` "Vocabulary" (lines 14-20):**
  > "Hypha — the CLI tool (hypha build, hypha add hydraulics, etc.). Distinct from the language runtime, following the Rust (rustc/cargo) and Python (python/uv) precedent."

- **`planning/v2/audit/37_section_37_dependency_mgmt.md` (lines 82-84):**
  > "The distinction between the `myco` CLI (§36) and the `hypha` CLI (§37) is not visible in either stub, and a reader has no indication that the packaging tool has its own distinct CLI binary."

  §36 describes a single CLI surface without mentioning that the developer toolchain has (at minimum) two distinct binaries: the compiler-facing CLI (`mycoc` per spec_new.md usage, or `myco` per §36 wording) and the package manager (`hypha`). The §37 audit has already flagged the reverse problem: §37 does not name `hypha`. The two stubs in combination give no reader a complete picture of the toolchain shape.

  `Recommend:` Add a sentence to §36 stating that the Myco developer toolchain has two distinct CLI binaries: the compiler CLI (the subject of §36) and `hypha` (the package manager, §37). Cite the `rustc`/`cargo` and `python`/`uv` precedents as motivation, consistent with chunk 10 "Principle" section. This does not require locking any open items; it just makes the toolchain shape visible in the section most readers will consult when asking "what commands does Myco have?"

### H6 — `mycoc explain` traceability role is a committed foundational concept, not referenced from §36

- **`planning/v2/spec_new.md` §0.1 (lines 101-107):**
  > "Every e-class merge, rewrite application, and workflow-injected value carries a provenance record accessible via `mycoc explain` (§22). Workflow-constant injections (§17) are tagged separately from compiler rewrites... Provenance is durable across plan serialization."

- **`planning/v2/spec_dev_notes.md` (line 495):**
  > "Traceability / provenance — §0.1 paragraph (cross-refs §22 mycoc explain, §17 merge tags, §13.9 observation tags; provenance durable across plan serialization)."

  The provenance and traceability role of `mycoc explain` is declared a foundational concept in §0.1 with a cross-reference to §22. §36 is pointed to by §22 ("related CLI surfaces, §36") as the home for CLI surface specification, but §36 does not acknowledge its role as the eventual spec home for the `explain` subcommand's provenance and traceability semantics. A reader consulting §36 to understand what `explain` does will find only "explain" in a subcommand list with no context.

  `Recommend:` Add a cross-reference from §36 back to §22 (plan inspection) and §0.1 (traceability foundational concept), noting that `explain` is the CLI entry point for provenance inspection and that its behavioral semantics are partially committed in §22 and §0.1. This gives §36 its proper position in the cross-reference graph rather than being a dead-end stub.

---

## Conflicts

### C1 — §36 calls the CLI "`the myco CLI`"; spec_new.md uses `mycoc` for compiler operations throughout

`planning/v2/spec_new.md` §36 (lines 5059, 5063):
> "The `myco` CLI spans compile, run, check, fmt, explain, and related subcommands"
> "The `myco` CLI: compile, run, check, fmt, explain, and related subcommands."

`planning/v2/spec_new.md` §23.4 (lines 3803, 3812, 3828):
> "`mycoc` compile errors"
> "`mycoc` compile errors. Structural problems in the `.myco`..."
> "`mycoc check` catches tier-1 errors"

`planning/v2/spec_new.md` §0.1 (lines 103, 110):
> "accessible via `mycoc explain` (§22)"
> "`mycoc` compile errors catch type, unit, contract, and structural problems"

`planning/v2/spec_new.md` §4.1 (lines 556, 561):
> "Violations surface as `mycoc` compile errors"

`planning/v2/spec_new.md` §32 (line 5122):
> "will emit a `mycoc` parse error"

The section that names and describes the CLI calls it "`myco`". Every other section that uses the CLI in a concrete example calls it "`mycoc`". The two names cannot refer to the same binary without a stated relationship. The legacy spec (spec.md Appendix B) used `myco` as the unified CLI name, but chunk 10 locked the two-binary split (`mycoc` + `hypha`) following `rustc`/`cargo` precedent. §36 has not been updated to reflect this split.

`Recommend:` Update §36 to use `mycoc` as the compiler CLI name, consistent with the rest of spec_new.md. If `myco` is intended as a user-facing shim that wraps `mycoc` (analogous to a build-tool front-end that delegates to the compiler binary), document that relationship explicitly in §36 rather than leaving both names in use without explanation. The current state — `myco` in §36, `mycoc` everywhere else — will cause implementors to build the wrong binary name.
