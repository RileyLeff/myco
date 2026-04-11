from pathlib import Path

import myco


ROOT = Path(__file__).resolve().parents[1]
MODEL_PATH = ROOT / "crates" / "myco-core" / "tests" / "fixtures" / "tiny_tree.myco"


def main() -> None:
    model = myco.load(MODEL_PATH)
    experiment = model.experiment(mode="train", horizon_steps=24)
    experiment.assume_series("vpd_scale", range(24))
    experiment.assume_series("soil_water", range(24))
    experiment.assume_constant("hydraulic_cond")
    experiment.assume_constant("g_max")
    experiment.assume_initial("water")
    experiment.assume_initial("carbon")
    experiment.learn_slot("controller")
    experiment.observe_dense("transpiration")

    plan = experiment.explain_plan()
    print("chosen current paths:")
    for path in plan.chosen_current:
        print(
            f"  - {path.output} <= {path.source} "
            f"({path.direction}, cost={path.cost})"
        )

    transpiration = experiment.explain_quantity("transpiration")
    print("\ntranspiration:")
    print(
        f"  chosen current: {transpiration.chosen_current.source} "
        f"({transpiration.chosen_current.direction})"
    )
    if transpiration.alternatives:
        print("  alternatives:")
        for alternative in transpiration.alternatives:
            print(
                f"    - {alternative.source} "
                f"({alternative.direction}, cost={alternative.cost})"
            )


if __name__ == "__main__":
    main()
