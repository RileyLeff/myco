from pathlib import Path

import myco


FIXTURE = (
    Path(__file__).resolve().parents[1]
    / "crates"
    / "myco-core"
    / "tests"
    / "fixtures"
    / "tiny_tree.myco"
)


def test_load_model_path_returns_summary():
    payload = myco.load_model_path(str(FIXTURE))
    model = payload["model"]

    assert model["name"] == "TinyTree"
    assert model["quantity_count"] == 8
    assert "stomata" in model["quantity_names"]


def test_compile_demo_path_emits_jax_artifact():
    payload = myco.compile_demo_path(str(FIXTURE), backend="jax")
    artifact = payload["artifact"]
    experiment = payload["experiment"]

    assert artifact["backend"] == "jax"
    assert artifact["model_name"] == "TinyTree"
    assert "import jax.numpy as jnp" in artifact["source"]
    assert experiment["planned_slot_steps"] == 1


def test_write_demo_path_writes_artifact(tmp_path: Path):
    output = tmp_path / "tiny_tree_demo.py"
    written = myco.write_demo_path(str(FIXTURE), backend="python", output_path=str(output))

    assert Path(written) == output
    assert output.exists()
    assert "def step(state, forcing, constants, slot_providers, dt):" in output.read_text()
