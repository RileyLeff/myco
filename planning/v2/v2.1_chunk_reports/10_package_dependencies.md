# 10 — Package Dependencies (Spores & Hypha)

Durable summary of the package / dependency story. Locks vocabulary
(`spore` = package, `hypha` = CLI), pins the approach (Cargo + uv
conventions, adapted as needed), and flags the items that need more
careful thought before shipping.

**Status: draft, partial lock. Vocabulary and overall shape are
committed. Resolver algorithm, version semantics, workspace model, and
several detail items are open and need more design work before ship.**

## Vocabulary

- **Spore** — a Myco package. Ecosystem-level unit of distribution.
  Equivalent role to a Rust "crate" or a Python "package." The word
  appears in documentation and error messages; it is not a `.myco`
  keyword.
- **Hypha** — the CLI tool (`hypha build`, `hypha add hydraulics`, etc.).
  Distinct from the language runtime, following the Rust
  (rustc/cargo) and Python (python/uv) precedent.
- **myco.toml** — per-package manifest. Metadata, dependencies,
  workspace members.
- **myco.lock** — resolved dependency tree, committed alongside
  `myco.toml`. Reproducibility guarantee.

## Principle

Follow Cargo and uv closely. Both are well-designed for their
ecosystems; reinventing dependency management is not where Myco's
novelty should live. Deviate only where the underlying model forces
us to.

Specifically:

- **Per-package manifest** (`myco.toml`) — Cargo-style.
- **Lockfile** (`myco.lock`) — Cargo-style, committed for applications,
  often not for libraries.
- **Workspace** — multi-spore repositories with a root `myco.toml`
  listing member paths. Cargo-workspace-style.
- **Semver-ish version constraints** — Cargo-style; exact semantics
  depend on how Myco handles breaking changes (open).
- **Lock-first resolution** — uv-style; if the lock is present and
  consistent, use it; only re-resolve on explicit update.
- **Offline-friendly** — uv-style; cached spores work without network.

When in doubt between Cargo and uv conventions, default to uv — its
more recent design has already addressed mistakes in earlier Python
tooling and is closer in spirit to Myco's
we-want-this-reproducible-and-fast stance.

## myco.toml (sketch)

```toml
[package]
name = "my_model"
version = "0.1.0"
authors = ["Riley Leff <rileyleff@gmail.com>"]
description = "Spatially explicit model of plant hydraulics"

[dependencies]
hydraulics = "1.2"
atmospherics = { version = "0.4", features = ["radiation-spectrum"] }
my_local_dep = { path = "../my_local_dep" }
canopy_types = { git = "https://github.com/...", rev = "abc123" }

[dev-dependencies]
test_utilities = "0.1"

[features]
default = ["gpu"]
gpu = []
cpu-only = []
```

Open whether `features`, `build-dependencies`, `profiles`, and other
Cargo-isms map cleanly onto Myco's compile model. Not a lot of the
Cargo feature set depends on a procedural language; most concepts
(dependencies, versions, features, workspaces) translate.

## myco.lock

Committed for applications (anything with an entry point), usually not
for libraries. Locked tree of resolved versions.

```
# myco.lock
version = 1

[[package]]
name = "my_model"
version = "0.1.0"
dependencies = ["hydraulics 1.2.4", "atmospherics 0.4.1"]

[[package]]
name = "hydraulics"
version = "1.2.4"
source = "registry+..."
checksum = "..."
```

Exact format TBD. Cargo's lockfile format is a reasonable starting
point; uv's `uv.lock` has some improvements (e.g., per-platform
resolution) worth adopting.

## Hypha CLI verbs (sketch)

```
hypha new <name>              # scaffold a new spore
hypha init                    # turn the current dir into a spore
hypha add <spore> [version]   # add a dependency to myco.toml
hypha remove <spore>          # remove a dependency
hypha build                   # compile the current spore
hypha check                   # typecheck without codegen
hypha run <entry>             # build + run an entry-point
hypha test                    # run tests
hypha lock                    # refresh myco.lock
hypha update [spore]          # update a specific spore or all
hypha publish                 # publish to a registry
hypha workspace list          # list workspace members
```

Exact verb menu and flags open.

## Workspaces

Cargo-workspace-style: a root `myco.toml` with a `[workspace]` section
listing member paths. Shared `myco.lock`, shared dependency resolution,
one-command build/test/check across members.

```toml
# root myco.toml
[workspace]
members = [
  "spores/hydraulics",
  "spores/atmospherics",
  "models/my_model",
]
```

Good for:
- Monorepo research setups (multiple spores + a consumer model)
- Developing a library + testing it against a consumer in parallel
- Path-dependencies across the tree without publishing

Open: how the workspace interacts with the Python workflow side. The
workflow is Python; does the workspace concept extend to Python
workspace conventions (uv workspaces), or do workspaces live purely at
the `.myco` layer with Python treated as a consumer?

## `.myco`-side imports

Module-level imports (file-to-file, within or across spores) use the
`use` statement already specified in §2:

```myco
use hydraulics::{VulnerabilityCurve, WeibullVC}
use atmospherics::radiation::{PhotosystemII, Absorptivity}
use super::leaf::Leaf
```

Spore-level imports are implicit once listed in `myco.toml`; `use
hydraulics::...` resolves to the declared `hydraulics` dependency. No
separate "extern crate"-style declaration in `.myco` source.

## Spore registry

A central registry (analogous to crates.io or PyPI). Open:

- Whether Myco needs a registry at all for v2.1, or whether Git-based
  dependencies are sufficient to start.
- Registry hosting (Anthropic? community? self-host?).
- Publishing workflow.
- Ownership / namespace rules.
- Yanking / security advisories.

Interim: Git-based dependencies (`{ git = "...", rev = "..." }`) cover
research use and avoid the registry question entirely.

## Open items (partial list, flagged for more work)

- **Resolver algorithm.** PubGrub (Cargo's current) vs uv's (pubgrub-
  inspired but different). Either is fine; pick one and implement.
- **Version semantics.** What counts as a breaking change in Myco?
  Changing a relation's parameter types obviously; removing a
  parameter obviously; but what about adding a new relation, or
  changing a stdlib atom's capability contract? Needs a semver policy
  spelled out.
- **Feature model.** Cargo features are powerful and confusing.
  Whether Myco needs them, and at what granularity, is open.
- **Build scripts / codegen.** Some spores may want compile-time
  codegen (e.g., generating `.myco` relations from a CSV of empirical
  constants). Open whether to support this, and how.
- **Workspace ↔ Python interaction.** Do Python workflows live inside
  the workspace as a separate member, or outside? How do Python
  imports (`import myco`, `from my_model import ...`) interact with
  the workspace?
- **Tooling integration.** Editor integration (LSP), documentation
  generation (`hypha doc`?), formatter, linter.
- **Cross-spore relation visibility.** Can a spore declare private
  relations that are only visible internally? Cargo's `pub(crate)` is
  the reference. Needs spelling out for Myco.
- **Registry story.** As above.
- **Platform / backend metadata.** A spore may require a specific
  backend (CUDA, CPU, etc.) or declare supported ones. Where does this
  live in `myco.toml`?

## Minimum viable package system for v2.1

The following subset is probably enough to get real work done while the
full story lands:

- `myco.toml` with `[package]`, `[dependencies]`
- Git and path dependencies (no registry required)
- `myco.lock` with basic deterministic resolution
- `hypha new`, `hypha add`, `hypha build`, `hypha check`, `hypha run`
- Workspaces (Cargo-style)
- `use` imports resolving via `myco.toml` declarations

Deferred to post-v2.1:

- Central registry
- Features
- Build scripts
- Advanced resolver features
- Publish workflow
- Cross-platform lockfile

## Status

Vocabulary locked (`spore`, `hypha`, `myco.toml`, `myco.lock`). Overall
approach locked (Cargo + uv, adapted). Details open. Needs dedicated
design work before any of this becomes normative spec text. Not
blocking for the core language lock; can land post-v2.1 if needed.
