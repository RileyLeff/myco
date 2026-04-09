# Myco

Myco is a compiler prototype for acausal scientific models with compile-time
binding, provider-aware planning, and Python/JAX artifact emission.

## Python development

This repo includes a `uv`-managed Python client around the `myco-py` PyO3
extension.

```bash
UV_PROJECT_ENVIRONMENT=venv uv sync
UV_PROJECT_ENVIRONMENT=venv uv run pytest
UV_PROJECT_ENVIRONMENT=venv uv run python examples/tiny_tree_demo.py
```

The initial Python surface is intentionally narrow:

- `myco.load_model_source(...)`
- `myco.load_model_path(...)`
- `myco.compile_demo_source(...)`
- `myco.compile_demo_path(...)`
- `myco.write_demo_path(...)`
