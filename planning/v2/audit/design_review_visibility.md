# Design Review: Abolishing Visibility from Myco

**Scope.** Read-only review of `planning/v2/spec_new.md`.
**Question.** Should `pub` / `private` / `file-local` visibility be removed from the language?

---

## Part 1 — Enumeration of Occurrences

The full term search across `spec_new.md` produced eight line matches.
No occurrence of the literal `pub` keyword appears in any code example.
All occurrences are prose-level mentions. The complete list, grouped by
line:

| Line | Quoted text | Section | Concern addressed |
|------|-------------|---------|-------------------|
| 71 | `downward-only visibility, traceability, error-reporting tiers` | §0.1 Foundational Concepts (summary list) | Cross-scale topology hierarchy (entities vs composites). Different concept from module visibility. |
| 93 | `**Downward-only cross-scale visibility.** Composite types see their components. Components do not see their composite.` | §0.1 | Same cross-scale concept. A `Forest` can read per-`Tree` state; a `Tree` cannot inquire about its `Forest`. |
| 204 | `public/private/file-local visibility` | §2 Summary | Module-level access control. What the `pub` keyword defends. |
| 210 | `Visibility rules (public / private / file-local). Scope resolution rules` | §2 Body | Same as above; the bullet-list restatement of the §2 Summary. |
| 1054 | `Cross-scale visibility is downward-only.` | §11.4 Horse-and-Fly Composition (Summary) | Cross-scale topology. Same concept as §0.1. |
| 1066 | `Cross-scale visibility is downward only: the tree sees its patches; patches do not see the tree.` | §11.4 Body | Same. |
| 3690 | `Backend AD ownership (Part V §32, listed separately for visibility).` | §35 Open Items | "Visibility" used in the English sense of "keeping track of," not a language keyword. |
| 3758/3762 | `versions, publish, and lock ... Package registry layout and publishing workflow.` | §37 Dependency Management | "publish" used in the package-distribution sense, not module visibility. |

**Observations on the count.**

- Lines 71, 93, 1054, 1066 all refer to the *cross-scale topology rule*
  (downward-only field access between composite and component entities).
  This concept is orthogonal to module visibility. Its name collides with
  "visibility" by accident of English; it must not be conflated.
- Lines 3690, 3758/3762 are false positives (English reuse of the word).
- Lines 204 and 210 are the only two occurrences that address module-level
  access control. Both are in §2. Neither appears in a code example.
- The `pub` keyword itself is **never spelled** in any `.myco` code block
  in `spec_new.md`. Its presence is entirely declarative prose.

**The file-local tier.**

Lines 204 and 210 both list "file-local" as the third tier. As the §2
audit (`02_section_2_modules.md`) documents, no corpus document backs this
tier. It has no syntax, no semantics, and no worked example anywhere in the
spec or the design history. It is an unresourced claim.

---

## Part 2 — Classification

### Load-bearing for external-facing APIs

None found in `spec_new.md`.

The spec does include a deferred §37 (Dependency Management / Package
Registry), but that section is a stub. The question of what a published
`.myco` package exposes to downstream packages is explicitly unresolved.
If a package-registry story lands, visibility could become meaningful
here -- but that story is not in the spec yet. No occurrence in the
current text falls into this class.

### Load-bearing for implementation hiding

The two occurrences at lines 204 and 210 are the only candidates. They
assert that module visibility exists and has three tiers. The invariant
they ostensibly defend is: a symbol marked private in module A cannot be
named in module B's `use` declaration.

There is one additional load-bearing statement not in `spec_new.md` itself
but documented in the §2 audit: the legacy `spec.md §1.2` rule that
"the workflow layer (Python API) can bind any path in the model regardless
of visibility." This asymmetry -- `pub` governs `.myco`-to-`.myco`
references but does not constrain Python workflow bindings -- is a
non-trivial design decision. It is not restated anywhere in `spec_new.md`.

Because `spec_new.md` is still a skeleton and contains no worked examples
where `pub` appears on a declaration, the "implementation hiding" category
is **asserted but uninstantiated**.

### Incidental / cargo-culted

Both live occurrences (lines 204 and 210) qualify as incidental. They are
feature-list bullets with no constraint behind them in the current text.
The `pub` keyword has no grammar rule, no code example, and no cross-
reference to any other section. The three-tier claim adds a tier (file-
local) with zero backing. This is cargo-culted structure.

---

## Part 3 — Two Abolishment Designs

### Design A: No visibility concept at all

Every symbol declared in a module is accessible to any module that imports
it via `use`. The `pub` keyword is removed from the grammar. The file-
local tier disappears. There is no "private" default.

**What changes in the language:**

- `use path::to::symbol` works for any symbol in any reachable module.
  No access-control check at import resolution.
- Re-export semantics become trivial: any module can re-export any symbol
  from any dependency by naming it in its own namespace. No `pub use`
  concept needed; the question collapses to "does the importing module
  also export this name under its own path?"
- The DAG import constraint (circular imports disallowed) remains
  unchanged; it is structural, not access-driven.
- The cross-scale topology rule (§0.1, §11.4) is entirely unaffected.
  That rule governs field access within the type system at runtime, not
  module-scope name lookup.

**Spec sections that need edits:**

- §2: Delete lines 204 and 210 references to visibility tiers. Replace
  with a statement that all declarations are accessible to any importing
  module.
- §37 (Dependency Management): When this section fills in, it must not
  assume a `pub` qualifier exists. Package-level exposure policy would
  need a different mechanism (e.g., a manifest file lists exported paths,
  separate from the language itself) or no policy at all.
- No other section in the current skeleton is affected. §6 (Functions),
  §17 (Equality-Introducing Machinery), §29 (Units Library) contain no
  visibility references.

**User-facing examples that break:** None. No code example in `spec_new.md`
uses `pub`.

**Cross-cutting language concepts that depend on visibility:**

- Workflow binding: The implicit rule (from legacy corpus) that the
  workflow can bind any path "regardless of visibility" becomes vacuously
  true. No asymmetry to document.
- Name resolution: Scope resolution rules in §2 simplify to: a name is in
  scope if it is declared in the current module or appears in a `use`
  statement whose target exists in the DAG.
- Nothing in the workflow verbs (§24), the e-graph substrate, event
  semantics (§10), or the training layer (§22-23) references visibility.

**What this design loses:** The ability to signal "this symbol is an
implementation detail; do not depend on it." In a large shared library
this matters. In a small-modeling-language context where models are one
project deep, it matters much less.

### Design B: Implicit visibility tied to import

No `pub` keyword. A symbol is "importable" if and only if another module
references it via `use`. Symbols that are never referenced by any other
module are considered file-local by definition. No explicit declaration.

**What breaks:**

- **Circular-import detection is unaffected** (it is a graph property,
  not a visibility property).
- **Name resolution ordering becomes order-dependent.** The compiler must
  resolve all `use` statements before it can determine which symbols are
  "importable." In practice this means a two-pass compilation where pass
  one builds a dependency graph and pass two resolves names. This is not
  unusual, but it must be specified.
- **The "is this symbol importable" question is non-local.** Whether
  `mymodule::helper` is importable depends on whether any other module
  happens to import it. Removing a use-site in module B silently demotes
  a symbol in module A from importable to file-local. This is the primary
  hazard: the property is an emergent fact about the whole project, not a
  local declaration.
- **Tooling (LSP, doc generation §39) cannot easily distinguish public
  API from internal helper.** Without an explicit marker, the doc
  generator must either document everything or use heuristics.
- **Re-export semantics are undefined.** If module C does
  `use A::helper`, helper is importable from A. Does that make it
  importable from C's namespace too? Only if something imports it from C.
  The chain becomes recursive and hard to reason about without explicit
  declaration.

**Spec sections that need edits (Design B):**

- §2: Replace the three-tier declaration with a description of implicit
  visibility derivation. Add language about two-pass resolution.
- §37: Package registry cannot use implicit visibility for published APIs;
  a manifest mechanism is still required.
- §39 (Documentation Generation): Needs a policy for what gets
  documented.

**User-facing examples that break:** None currently (no examples in spec).
In practice, Design B would confuse users who want to mark a helper as
internal without deleting all its callers.

---

## Part 4 — Recommendation

**Recommended: Design A (no visibility concept), with one small addition.**

The addition: a doc-comment convention or naming convention signals
"implementation detail" without language enforcement. The convention costs
nothing to implement and satisfies the one real use case for private in
this domain (telling readers "do not depend on this"). Rustdoc achieves
this with `#[doc(hidden)]`; Myco could achieve it with a leading underscore
convention (`_helper_fn`) or a doc-comment tag, both of which are zero
compiler complexity.

**What Riley loses by abolishing visibility:**

- Compile-time enforcement that module consumers do not depend on internal
  helpers. If a downstream module references `_helper`, it works
  silently.
- A clean surface for the package-registry story. If Myco ever ships a
  crates.io-equivalent, the registry will need to know what a package
  exports. This can be solved with a manifest (list of exported paths in
  `Myco.toml` or equivalent) rather than a per-declaration keyword.

**What Riley gains:**

- Elimination of `pub` from the grammar entirely. Fewer keywords, simpler
  parser, one less concept to explain.
- No three-tier system where the third tier (file-local) has no design
  backing and would need to be invented.
- The workflow binding asymmetry (workflow ignores visibility, `.myco`
  imports respect it) disappears because there is no asymmetry to explain.
- The spec skeleton at §2 shrinks to two real rules: file-as-module, and
  DAG imports. Both are independently motivated and need no visibility
  concept to stand.
- Alignment with how the language is actually used: single-project models
  where the "internal" symbols are either obvious from context or
  disciplined by naming convention.

**Design B is not recommended.** Implicit visibility is worse than no
visibility concept at all. It makes a local declaration's status a global
property of the entire project, breaks doc generation, and introduces
subtle ordering hazards without any user benefit that Design A does not
also provide.

**On the file-local tier specifically:** it should be removed regardless
of which decision Riley makes on the broader visibility question. It has
no backing, no syntax, and no worked example. Its presence in §2 lines 204
and 210 is the audit's strongest concrete finding.
