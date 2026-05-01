# Myco

Myco is currently a specification-first programming language project.
The canonical product is the v2 language specification:

- [planning/v2/spec.md](planning/v2/spec.md)
- [planning/v2/anti_spec.md](planning/v2/anti_spec.md)
- [planning/soul.md](planning/soul.md)

The previous toy Rust/Python prototype has been removed so it does not
misrepresent the scope or direction of the project. Implementation work
will restart from the canonical v2 spec.

This repository intentionally keeps only bare workspace shells for now:

- `Cargo.toml` is an empty Cargo workspace.
- `pyproject.toml` is an empty uv workspace.
- `scripts/` contains spec-navigation and verification helpers.
- `website/` is left as-is.

Useful spec commands:

```bash
just spec-index
just spec-section 8
just spec-summary 28
just spec-verify  # currently reports the known spec style backlog
```
