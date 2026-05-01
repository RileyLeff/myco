# Audit Report — §39 Documentation Generation and Website

Audited against corpus as of 2026-04-22.

---

## Absorbed

Corpus content that is already captured by spec_new.md §39.

- **`planning/v2/spec.md` §B.4 "Doc comments and documentation generation":**
  > "Support `///` doc comments on types, contracts, functions, and fields: ... Generate browsable HTML documentation for library packages (like `rustdoc`). Documentation should include: Type signatures with units / Contract interfaces and their implementations / Constraint listings / Cross-references between related items"

  The `///` doc-comment syntax and the rustdoc-style HTML generator with its four enumerated contents (type signatures with units, contract interfaces, constraint listings, cross-references) are the substantive design in spec.md. §39's stub text "Docstring conventions. Doc generator for user-defined types, contracts, events, universals" captures this at the right level of detail for a deferred stub. The `///` token is the syntax claim; the coverage list from spec.md §B.4 is what §39's "types, contracts, events, and universals" bullet condenses.

  `Recommend:` No action. §39 correctly characterizes the spec.md §B.4 material as a deferred item.

- **`planning/v2/spec_dev_notes.md` (Riley notes, near EOF):**
  > "ohhhh a package in myco is called a spore. hell yeah / - crates.io - like thing (bigmyco.com/spores) / rustdoc-like thing (bigmyco.com/spores/plant_hydraulics/docs) / i like svelte 5 for web development i'll probably use that / not an immediate concern / i have a placeholder up on bigmyco.com right now / site to host main myco docs + user"

  The bigmyco.com placeholder, Svelte 5 preference, per-spore docs URL structure, and "not an immediate concern" verdict are informal session notes. They are not spec prose. §39's "website layout: language reference, tutorials, API docs, examples" covers the same territory at an appropriate level of abstraction without encoding the technology choice (Svelte 5) or the URL pattern, which are implementation details not yet locked.

  `Recommend:` No action. The dev notes confirm §39's deferred status is intentional and matches Riley's own "not an immediate concern" assessment. Svelte 5 and bigmyco.com URL structure should stay in dev notes, not migrate into §39 prose.

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §39.

Nothing found. §39 is a deferred stub with no locked decisions to supersede.

---

## Homeless

Corpus content relevant to §39 that is not captured in spec_new.md §39 and not already committed to anti_spec.md.

- **`planning/v2/v2.1_chunk_reports/10_package_dependencies.md` §5 (open items):**
  > "Tooling integration. Editor integration (LSP), documentation generation (`hypha doc`?), formatter, linter."

  Chunk 10 lists documentation generation as an open tooling integration question under the `hypha` CLI, with `hypha doc` as a candidate sub-command spelling. §39 does not reference the CLI surface at all; the current stub says "doc generator" but gives no indication of whether this is invoked as `myco doc`, `hypha doc`, or some other command. The chunk 10 note is informal (the `?` signals it is unresolved), but the question of which CLI tool owns doc generation is a genuine open item that touches both §38 (CLI / REPL) and §39.

  `Recommend:` Add a brief note to §39 that the doc-generation CLI sub-command spelling (`myco doc` vs. `hypha doc`) is open and depends on the CLI / package-manager boundary settled in §38. The note should not encode the chunk 10 tentative spelling as a decision; it should flag the dependency so that §38 and §39 are filled in together.

- **`planning/v2/spec.md` §B.5 "Graph rendering architecture" (Mermaid reference):**
  > "plan.graph.to_mermaid() ... Mermaid string (for markdown/GitHub) / D2: modern text-to-diagram with better default styling. Good for documentation and presentations"

  The graph rendering section (spec.md §B.5) explicitly names Mermaid and D2 as rendering targets suited to documentation contexts. Neither target is referenced in §39. The relationship is loose: §B.5 is primarily about plan-graph visualization (§22 / §38 territory), but the documentation-and-presentations framing for D2 and the GitHub-markdown integration of Mermaid are directly relevant to the "website layout" and "examples" goals that §39 names. §39 currently treats documentation as a static HTML artifact (rustdoc-style), which understates the interactive / diagram-rich approach signaled in spec.md §B.5.

  `Recommend:` §39 may optionally note that the doc website can embed generated model-graph diagrams via the Mermaid rendering target defined in §B.5 / §22. This is a cross-reference note, not a new design commitment. If §39 prose remains a high-level stub, the connection can be deferred; flag it here so it is not silently dropped when §39 is filled in.

---

## Conflicts

Direct contradictions between spec_new.md §39 and any corpus document.

Nothing found. §39 is a 10-line deferred stub; it makes no claims specific enough to conflict with corpus material. The stub's coverage list (types, contracts, events, universals) extends spec.md §B.4's list (types, contracts, functions, fields) by substituting "events" and "universals" for "functions" and "fields." This is an expansion of scope, not a contradiction. "Events" and "universals" are first-class named constructs in the language that §B.4 did not enumerate (spec.md §B.4 predates both as settled constructs). The substitution is appropriate; no correction needed.
