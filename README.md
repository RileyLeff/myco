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
UV_PROJECT_ENVIRONMENT=venv uv run python examples/diagnostics_demo.py
```

After changing the Rust extension crate, rebuild the editable package with:

```bash
UV_PROJECT_ENVIRONMENT=venv uv sync --reinstall-package myco
```

The initial Python surface is intentionally narrow:

- `myco.load_model_source(...)`
- `myco.load_model_path(...)`
- `myco.compile_demo_source(...)`
- `myco.compile_demo_path(...)`
- `myco.write_demo_path(...)`

There is also an initial real experiment/binding surface:

```python
from pathlib import Path

import myco

fixture = Path("crates/myco-core/tests/fixtures/tiny_tree.myco")

model = myco.load(fixture)
experiment = model.experiment(mode="train", horizon_steps=24)
experiment.bind_data_series("vpd_scale", range(24))
experiment.bind_data_series("soil_water", range(24))
experiment.bind_constant("hydraulic_cond")
experiment.bind_constant("g_max")
experiment.bind_initial_state("water")
experiment.bind_initial_state("carbon")
experiment.bind_slot("controller", kind="learned")
experiment.observe_dense("transpiration")

artifact = experiment.compile(backend="jax")
artifact.write()
```

The Python package now has a conventional structure:

- `myco.api`: high-level model / experiment API
- `myco.types`: typed summaries, specs, bindings, and artifacts
- `myco.errors`: structured diagnostics and `MycoError`
