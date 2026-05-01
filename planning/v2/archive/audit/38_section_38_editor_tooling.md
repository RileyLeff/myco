# Audit Report — §38 Editor Tooling

Audited against corpus as of 2026-04-22.

---

## Absorbed

No corpus material requires absorption into §38. The section is a Part VII
stub whose purpose is to name the surface and defer it; it has no design
content to absorb.

---

## Superseded

No corpus material has been superseded by §38 or by any later decision that
would invalidate earlier editor-tooling text. No earlier tooling design exists
in the corpus to be replaced.

---

## Homeless

- **`planning/v2/v2.1_chunk_reports/10_package_dependencies.md`, line 197:**
  > "Tooling integration. Editor integration (LSP), documentation generation (`hypha doc`?), formatter, linter."

  Chunk 10 lists "tooling integration" as an open item within the package
  system's open-questions section, alongside workspace/Python interaction and
  registry story. The LSP and editor integration sub-items map directly to §38.
  The formatter and linter sub-items have no explicit home: `fmt` appears in
  §36's CLI stub as a subcommand (`myco fmt`), but the formatter and linter as
  implemented tools rather than CLI flags are not named in §38. Documentation
  generation maps to §39. The chunk 10 open item bundles all four sub-items
  under package-system concerns, but the split across §36, §38, and §39 is
  not acknowledged in any of those stubs, leaving the formatter and linter
  without a clear tracking home.

  `Recommend:` Add "Formatter and linter" to §38's body list (they belong with
  editor tooling more naturally than with the CLI subcommand stub in §36, and
  they are clearly distinct from the doc generator in §39). The chunk 10 open
  item can then be satisfied by cross-referencing §36, §38, and §39 rather
  than holding an unstructured bundle in the package report. No further action
  on chunk 10 itself is needed; the open item there is a planning note, not
  spec content.

---

## Conflicts

No direct contradictions between §38 and any corpus document. The corpus
contains no earlier editor-tooling design decisions that §38 could conflict
with. The chunk 10 bundling (see Homeless above) is a coverage gap, not a
contradiction.
