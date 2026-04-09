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
    artifact = myco.compile_demo_path(str(FIXTURE), backend="jax")

    assert artifact.backend == "jax"
    assert artifact.model_name == "TinyTree"
    assert "import jax.numpy as jnp" in artifact.source


def test_write_demo_path_writes_artifact(tmp_path: Path):
    output = tmp_path / "tiny_tree_demo.py"
    written = myco.write_demo_path(str(FIXTURE), backend="python", output_path=str(output))

    assert Path(written) == output
    assert output.exists()
    assert "def step(state, forcing, constants, slot_providers, dt):" in output.read_text()


def test_experiment_builder_compiles_real_spec():
    model = myco.load(FIXTURE)
    experiment = model.experiment(mode="train", horizon_steps=24)
    experiment.bind_data_series("vpd_scale", range(24))
    experiment.bind_data_series("soil_water", range(24))
    experiment.bind_constant("hydraulic_cond")
    experiment.bind_constant("g_max")
    experiment.bind_initial_state("water")
    experiment.bind_initial_state("carbon")
    experiment.bind_slot("controller", kind="learned")
    experiment.observe_dense("transpiration")

    summary = experiment.summary()
    artifact = experiment.compile(backend="jax")

    assert summary["planned_slot_steps"] == 1
    assert summary["planned_temporal_steps"] == 1
    assert artifact.backend == "jax"
    assert artifact.suggested_filename == "tinytree_jax.py"
    assert "def total_loss(" in artifact.source
