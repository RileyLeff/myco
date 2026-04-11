# Myco

Myco is a compiler prototype for acausal scientific models with compile-time
binding, provider-aware planning, and Python/JAX artifact emission.

## Python development

This repo includes a `uv`-managed Python client around the `myco-py` PyO3
extension.

The Python package uses an isolated package root under `python/`, which is
effectively a source layout for this mixed Rust/Python repo. I prefer this over
a flat root-level package because it keeps packaging boundaries explicit and
avoids accidental imports from unrelated repo files.

```bash
UV_PROJECT_ENVIRONMENT=venv uv sync
UV_PROJECT_ENVIRONMENT=venv uv run pytest
UV_PROJECT_ENVIRONMENT=venv uv run python examples/tiny_tree_demo.py
UV_PROJECT_ENVIRONMENT=venv uv run python examples/diagnostics_demo.py
UV_PROJECT_ENVIRONMENT=venv uv run python examples/spec_file_demo.py
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
experiment.assume_series("vpd_scale", range(24))
experiment.assume_series("soil_water", range(24))
experiment.assume_constant("hydraulic_cond")
experiment.assume_constant("g_max")
experiment.assume_initial("water")
experiment.assume_initial("carbon")
experiment.learn_slot("controller")
experiment.observe_dense("transpiration")

artifact = experiment.compile(backend="jax")
artifact.write()
```

The older `bind_*` helpers still exist as compatibility aliases, but the intended
workflow vocabulary is now:

- `assume_*` for directly supplied values
- `observe_*` for evidence
- `learn_*` for trainable components

The Python package now has a conventional structure:

- `myco.api`: high-level model / experiment API
- `myco.types`: typed summaries, specs, bindings, and artifacts
- `myco.errors`: structured diagnostics and `MycoError`

Compile specs can also be round-tripped as JSON files:

```python
import myco

spec = myco.load_spec("examples/tiny_tree_spec.json")
artifact = myco.compile_spec_path(
    "crates/myco-core/tests/fixtures/tiny_tree.myco",
    "examples/tiny_tree_spec.json",
    backend="jax",
)
```
