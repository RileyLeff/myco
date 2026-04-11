from pathlib import Path

import myco
import pytest


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
    experiment.assume_series("vpd_scale", range(24))
    experiment.assume_series("soil_water", range(24))
    experiment.assume_constant("hydraulic_cond")
    experiment.assume_constant("g_max")
    experiment.assume_initial("water")
    experiment.assume_initial("carbon")
    experiment.learn_slot("controller")
    experiment.observe_dense("transpiration")

    summary = experiment.summary()
    artifact = experiment.compile(backend="jax")

    assert summary.assumption_count == 6
    assert summary.learning_count == 1
    assert summary.planned_slot_steps == 1
    assert summary.planned_temporal_steps == 1
    assert artifact.backend == "jax"
    assert artifact.suggested_filename == "tinytree_jax.py"
    assert artifact.metadata.compile_mode == "train"
    assert artifact.metadata.consistency_policy == "equation_only"
    assert artifact.metadata.constraint_runtime_policy == "project_learned_penalize_derived"
    assert artifact.metadata.loss_helpers_enabled is True
    assert artifact.metadata.learned_slots == ("controller",)
    controller = artifact.slot_interface("controller")
    assert controller is not None
    assert controller.inputs == (
        "water",
        "carbon",
        "vpd_scale",
        "soil_water",
        "hydraulic_cond",
        "g_max",
    )
    assert controller.outputs == ("stomata",)
    assert controller.input_arity == 6
    assert controller.output_arity == 1
    assert artifact.metadata.persistent_quantities == ("carbon", "water")
    assert "def total_loss(" in artifact.source

def test_compile_spec_role_builders_compile_real_spec():
    model = myco.load(FIXTURE)
    spec = myco.CompileSpec(mode="train", horizon_steps=24)
    spec.assume_series("vpd_scale", range(24))
    spec.assume_series("soil_water", range(24))
    spec.assume_constant("hydraulic_cond")
    spec.assume_constant("g_max")
    spec.assume_initial("water")
    spec.assume_initial("carbon")
    spec.learn_slot("controller")
    spec.observe_dense("transpiration")

    artifact = model.compile(spec, backend="jax")

    assert artifact.metadata.persistent_quantities == ("carbon", "water")
    assert artifact.metadata.learned_slots == ("controller",)


def test_experiment_explain_plan_returns_typed_paths():
    model = myco.load(FIXTURE)
    experiment = model.experiment(mode="train", horizon_steps=24)
    experiment.assume_series("vpd_scale", range(24))
    experiment.assume_series("soil_water", range(24))
    experiment.assume_constant("hydraulic_cond")
    experiment.assume_constant("g_max")
    experiment.assume_initial("water")
    experiment.assume_initial("carbon")
    experiment.learn_slot("controller")
    experiment.observe_dense("transpiration")

    explanation = experiment.explain_plan()

    assert "transpiration" in explanation.available_current
    assert any(path.source == "controller" for path in explanation.chosen_current)
    assert any(path.expression is not None for path in explanation.chosen_current)
    assert any(path.provenance_label is not None for path in explanation.chosen_current)
    assert any(
        alternative.source == "supply_transpiration"
        for alternative in explanation.alternatives
    )
    assert explanation.unresolved == ()


def test_experiment_explain_quantity_surfaces_alternatives_and_unresolved():
    model = myco.load(FIXTURE)

    resolved = model.experiment(mode="train", horizon_steps=24)
    resolved.assume_series("vpd_scale", range(24))
    resolved.assume_series("soil_water", range(24))
    resolved.assume_constant("hydraulic_cond")
    resolved.assume_constant("g_max")
    resolved.assume_initial("water")
    resolved.assume_initial("carbon")
    resolved.learn_slot("controller")
    resolved.observe_dense("transpiration")

    transpiration = resolved.explain_quantity("transpiration")
    assert transpiration.quantity == "transpiration"
    assert transpiration.observed is True
    assert transpiration.unresolved is False
    assert transpiration.chosen_current is not None
    assert transpiration.chosen_current.source == "demand_transpiration"
    assert transpiration.chosen_current.expression == "(stomata * vpd_scale)"
    assert transpiration.chosen_current.source_span is not None
    assert any(
        alternative.source == "supply_transpiration"
        for alternative in transpiration.alternatives
    )

    unresolved = model.experiment(mode="train", horizon_steps=24)
    unresolved.assume_series("vpd_scale", range(24))
    unresolved.assume_series("soil_water", range(24))
    unresolved.assume_constant("hydraulic_cond")
    unresolved.assume_initial("water")
    unresolved.assume_initial("carbon")
    unresolved.learn_slot("controller")
    unresolved.observe_dense("transpiration")

    g_max = unresolved.explain_quantity("g_max")
    assert g_max.unresolved is True
    assert g_max.chosen_current is None
    assert g_max.direct_binding is None


def test_structured_myco_error_exposes_diagnostics():
    model = myco.load(FIXTURE)
    experiment = model.experiment(mode="train", horizon_steps=24)
    experiment.assume_series("vpd_scale", range(24))
    experiment.assume_series("soil_water", range(24))
    experiment.assume_constant("hydraulic_cond")
    experiment.assume_constant("g_max")
    experiment.learn_slot("controller")
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
    assert spec.consistency_policy == "equation_only"
    assert len(spec.assumptions) == 6
    assert len(spec.learning) == 1
    assert artifact.backend == "jax"
    assert "import jax.numpy as jnp" in artifact.source


def test_legacy_compile_spec_fields_are_rejected():
    with pytest.raises(ValueError, match="legacy compile-spec fields"):
        myco.CompileSpec.from_dict(
            {
                "mode": "train",
                "horizon_steps": 24,
                "direct_bindings": [],
                "slot_bindings": [],
                "observations": [],
            }
        )


def test_experiment_can_set_consistency_policy():
    model = myco.load(FIXTURE)
    experiment = model.experiment(mode="train", horizon_steps=24)
    experiment.set_consistency_policy("off")
    experiment.assume_series("vpd_scale", range(24))
    experiment.assume_series("soil_water", range(24))
    experiment.assume_constant("hydraulic_cond")
    experiment.assume_constant("g_max")
    experiment.assume_initial("water")
    experiment.assume_initial("carbon")
    experiment.learn_slot("controller")
    experiment.observe_dense("transpiration")

    artifact = experiment.compile(backend="jax")

    assert "CONSISTENCY_POLICY = \"off\"" in artifact.source


def test_simulate_mode_omits_loss_helpers():
    model = myco.load(FIXTURE)
    experiment = model.experiment(mode="simulate", horizon_steps=24)
    experiment.assume_series("vpd_scale", range(24))
    experiment.assume_series("soil_water", range(24))
    experiment.assume_constant("hydraulic_cond")
    experiment.assume_constant("g_max")
    experiment.assume_initial("water")
    experiment.assume_initial("carbon")
    experiment.learn_slot("controller")

    artifact = experiment.compile(backend="jax")

    assert artifact.metadata.compile_mode == "simulate"
    assert artifact.metadata.loss_helpers_enabled is False
    assert "LOSS_HELPERS_ENABLED = False" in artifact.source
    assert "def obs_loss(" not in artifact.source
    assert "def total_loss(" not in artifact.source


def test_generated_artifact_validates_runtime_inputs():
    model = myco.load(FIXTURE)
    experiment = model.experiment(mode="train", horizon_steps=24)
    experiment.assume_series("vpd_scale", range(24))
    experiment.assume_series("soil_water", range(24))
    experiment.assume_constant("hydraulic_cond")
    experiment.assume_constant("g_max")
    experiment.assume_initial("water")
    experiment.assume_initial("carbon")
    experiment.learn_slot("controller")
    experiment.observe_dense("transpiration")

    artifact = experiment.compile(backend="python")
    module = artifact.to_module("tiny_tree_validation_test")

    with pytest.raises(KeyError, match="missing forcing step 0 value for 'soil_water'"):
        module.validate_rollout_inputs(
            {"water": -0.3, "carbon": 0.2},
            [{"vpd_scale": 0.5} for _ in range(24)],
            {"hydraulic_cond": 0.75, "g_max": 1.1},
            {"controller": lambda *args: 0.0},
        )
