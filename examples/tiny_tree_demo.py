from pathlib import Path

import myco


ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "crates" / "myco-core" / "tests" / "fixtures" / "tiny_tree.myco"


def main() -> None:
    payload = myco.compile_demo_path(str(FIXTURE), backend="jax")
    model = payload["model"]
    experiment = payload["experiment"]
    artifact = payload["artifact"]

    print(f"model: {model['name']}")
    print(f"quantities: {model['quantity_count']}")
    print(f"planned slots: {experiment['planned_slot_steps']}")
    print(f"suggested filename: {artifact['suggested_filename']}")
    print("artifact preview:")
    for line in artifact["source"].splitlines()[:8]:
        print(f"  {line}")


if __name__ == "__main__":
    main()
