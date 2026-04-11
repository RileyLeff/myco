from pathlib import Path

import myco


ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "crates" / "myco-core" / "tests" / "fixtures" / "tiny_tree.myco"


def main() -> None:
    model = myco.load(FIXTURE)
    experiment = model.experiment(mode="train", horizon_steps=24)
    experiment.assume_series("vpd_scale", range(24))
    experiment.assume_series("soil_water", range(24))
    experiment.assume_constant("hydraulic_cond")
    experiment.assume_constant("g_max")
    experiment.learn_slot("controller")
    experiment.observe_dense("transpiration")

    try:
        experiment.compile()
    except myco.MycoError as err:
        print("compile failed with structured diagnostics:")
        for diagnostic in err.diagnostics:
            print(f"  {diagnostic.severity}: {diagnostic.message}")


if __name__ == "__main__":
    main()
