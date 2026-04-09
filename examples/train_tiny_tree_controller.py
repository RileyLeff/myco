from myco.demos import run_tiny_tree_training_demo


def main() -> None:
    result = run_tiny_tree_training_demo()
    print(f"initial loss: {result.initial_loss:.6f}")
    print(f"final loss: {result.final_loss:.6f}")
    print(f"initial obs loss: {result.initial_obs_loss:.6f}")
    print(f"final obs loss: {result.final_obs_loss:.6f}")
    print(f"holdout transpiration MSE: {result.holdout_transpiration_mse:.6f}")
    print(f"holdout water MSE: {result.holdout_water_mse:.6f}")


if __name__ == "__main__":
    main()
