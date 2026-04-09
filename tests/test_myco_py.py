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
SPEC_FIXTURE = Path(__file__).resolve().parents[1] / "examples" / "tiny_tree_spec.json"


def test_load_model_path_returns_summary():
    model = myco.load_model_path(str(FIXTURE))

    assert model.name == "TinyTree"
    assert model.quantity_count == 8
    assert "stomata" in model.quantity_names


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

    assert summary.planned_slot_steps == 1
    assert summary.planned_temporal_steps == 1
    assert artifact.backend == "jax"
    assert artifact.suggested_filename == "tinytree_jax.py"
    assert "def total_loss(" in artifact.source


def test_structured_myco_error_exposes_diagnostics():
    model = myco.load(FIXTURE)
    experiment = model.experiment(mode="train", horizon_steps=24)
    experiment.bind_data_series("vpd_scale", range(24))
    experiment.bind_data_series("soil_water", range(24))
    experiment.bind_constant("hydraulic_cond")
    experiment.bind_constant("g_max")
    experiment.bind_slot("controller", kind="learned")
    experiment.observe_dense("transpiration")

    try:
        experiment.compile(backend="jax")
    except myco.MycoError as err:
        assert len(err.diagnostics) >= 1
        assert "requires an explicit initial-state binding" in err.diagnostics[0].message
    else:
        raise AssertionError("expected compile to fail with structured diagnostics")


def test_load_spec_and_compile_from_file():
    spec = myco.load_spec(SPEC_FIXTURE)
    artifact = myco.compile_spec_path(FIXTURE, SPEC_FIXTURE, backend="jax")

    assert spec.mode == "train"
    assert spec.horizon_steps == 24
    assert artifact.backend == "jax"
    assert "import jax.numpy as jnp" in artifact.source
