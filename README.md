# Myco

Myco is a compiler for workflow-neutral scientific models with explicit
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

## Recommended Python Workflow

The main user path is:

1. load a structural `.myco` model
2. create an experiment for one workflow
3. `assume_*`, `observe_*`, and `learn_*`
4. compile to an ordinary Python or JAX artifact

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
module = artifact.to_module("tiny_tree_artifact")
```

The workflow vocabulary is:

- `assume_*` for supplied values
- `observe_*` for evidence
- `learn_*` for trainable components

Compiled artifacts expose typed metadata and slot contracts:

```python
import myco

artifact = myco.compile_spec_path(
    "crates/myco-core/tests/fixtures/tiny_tree.myco",
    "examples/tiny_tree_spec.json",
    backend="jax",
)

print(artifact.metadata.compile_mode)
print(artifact.metadata.persistent_quantities)
print(artifact.metadata.learned_slots)
print(artifact.slot_interface("controller"))
```

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

For a longer walkthrough of the same TinyTree flow, see
[docs/end_to_end_walkthrough.md](docs/end_to_end_walkthrough.md).
