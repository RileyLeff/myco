from myco.demos import run_tiny_tree_training_demo


def test_tiny_tree_training_demo_recovers_behavior():
    result = run_tiny_tree_training_demo(train_steps=120, learning_rate=0.05)

    assert result.final_loss < result.initial_loss * 0.2
    assert result.holdout_transpiration_mse < 1e-3
    assert result.holdout_water_mse < 1e-3
