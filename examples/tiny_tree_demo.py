from pathlib import Path

import myco


ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "crates" / "myco-core" / "tests" / "fixtures" / "tiny_tree.myco"


def main() -> None:
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

    print(f"model: {model.summary['name']}")
    print(f"quantities: {model.summary['quantity_count']}")
    print(f"planned slots: {summary['planned_slot_steps']}")
    print(f"suggested filename: {artifact.suggested_filename}")
    print("artifact preview:")
    for line in artifact.source.splitlines()[:8]:
        print(f"  {line}")


if __name__ == "__main__":
    main()
