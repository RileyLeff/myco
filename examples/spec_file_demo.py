from pathlib import Path

import myco


ROOT = Path(__file__).resolve().parents[1]
MODEL_PATH = ROOT / "crates" / "myco-core" / "tests" / "fixtures" / "tiny_tree.myco"
SPEC_PATH = ROOT / "examples" / "tiny_tree_spec.json"


def main() -> None:
    spec = myco.load_spec(SPEC_PATH)
    artifact = myco.compile_spec_path(MODEL_PATH, SPEC_PATH, backend="jax")

    print(f"loaded spec mode: {spec.mode}")
    print(f"horizon: {spec.horizon_steps}")
    print(f"artifact backend: {artifact.backend}")
    print(f"suggested filename: {artifact.suggested_filename}")


if __name__ == "__main__":
    main()
