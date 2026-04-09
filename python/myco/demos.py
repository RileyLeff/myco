from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

import jax
import jax.numpy as jnp
import optax

from .api import compile_path
from .types import CompileSpec, constant, data_series, initial_state, observe_dense, observe_sparse, slot


DEFAULT_TINY_TREE_MODEL_PATH = (
    Path(__file__).resolve().parents[2]
    / "crates"
    / "myco-core"
    / "tests"
    / "fixtures"
    / "tiny_tree.myco"
)


@dataclass(frozen=True, slots=True)
class TrainingResult:
    initial_loss: float
    final_loss: float
    initial_obs_loss: float
    final_obs_loss: float
    holdout_transpiration_mse: float
    holdout_water_mse: float
    learned_params: dict[str, jax.Array]


def build_tiny_tree_training_spec(horizon_steps: int) -> CompileSpec:
    spec = CompileSpec(mode="train", horizon_steps=horizon_steps)
    spec.direct_bindings.extend(
        [
            data_series("vpd_scale", range(horizon_steps)),
            data_series("soil_water", range(horizon_steps)),
            constant("hydraulic_cond"),
            constant("g_max"),
            initial_state("water"),
            initial_state("carbon"),
        ]
    )
    spec.slot_bindings.append(slot("controller", kind="learned"))
    spec.observations.extend(
        [
            observe_dense("transpiration"),
            observe_sparse("water", range(0, horizon_steps, 8)),
        ]
    )
    return spec


def compile_tiny_tree_training_module(
    horizon_steps: int,
    model_path: str | Path = DEFAULT_TINY_TREE_MODEL_PATH,
):
    artifact = compile_path(model_path, build_tiny_tree_training_spec(horizon_steps), backend="jax")
    return artifact.to_module("tiny_tree_training_artifact")


def make_forcing_series(horizon_steps: int, phase: float = 0.0) -> dict[str, jax.Array]:
    t = jnp.linspace(0.0, 2.0 * jnp.pi, horizon_steps)
    return {
        "vpd_scale": 0.65 + 0.15 * jnp.sin(t + phase),
        "soil_water": -0.18 - 0.04 * jnp.cos(0.5 * t + phase),
    }


def make_constants() -> dict[str, jax.Array]:
    return {
        "hydraulic_cond": jnp.asarray(0.75),
        "g_max": jnp.asarray(1.1),
    }


def make_initial_state() -> dict[str, jax.Array]:
    return {
        "water": jnp.asarray(-0.32),
        "carbon": jnp.asarray(0.2),
    }


def make_true_params() -> dict[str, jax.Array]:
    return {
        "w": jnp.asarray([1.35, 0.15, -1.1, 1.0, 0.2, 0.0]),
        "b": jnp.asarray(-0.1),
    }


def make_initial_learned_params() -> dict[str, jax.Array]:
    return {
        "w": jnp.asarray([0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
        "b": jnp.asarray(0.0),
    }


def controller_from_params(params: dict[str, jax.Array]):
    def controller(water, carbon, vpd_scale, soil_water, hydraulic_cond, g_max):
        features = jnp.stack(
            [water, carbon, vpd_scale, soil_water, hydraulic_cond, g_max]
        )
        raw = jnp.dot(params["w"], features) + params["b"]
        return g_max * jax.nn.sigmoid(raw)

    return controller


def make_observations(history: dict[str, jax.Array]) -> dict[str, dict[str, jax.Array]]:
    horizon_steps = int(history["transpiration"].shape[0])
    return {
        "transpiration": {
            "values": history["transpiration"],
            "mask": jnp.ones((horizon_steps,), dtype=bool),
        },
        "water": {
            "values": history["water"],
            "mask": (jnp.arange(horizon_steps) % 8) == 0,
        },
    }


def run_tiny_tree_training_demo(
    *,
    horizon_steps: int = 64,
    train_steps: int = 200,
    learning_rate: float = 0.05,
    consistency_weight: float = 0.01,
    dt: float = 0.02,
    seed: int = 0,
    model_path: str | Path = DEFAULT_TINY_TREE_MODEL_PATH,
) -> TrainingResult:
    del seed  # deterministic initialization for now

    module = compile_tiny_tree_training_module(horizon_steps, model_path=model_path)
    constants = make_constants()
    initial_state = make_initial_state()
    forcing_train = make_forcing_series(horizon_steps, phase=0.0)
    forcing_holdout = make_forcing_series(horizon_steps, phase=0.7)
    true_params = make_true_params()
    true_slot_providers = {"controller": controller_from_params(true_params)}

    _, history_train_true = module.rollout(
        initial_state,
        forcing_train,
        constants,
        true_slot_providers,
        jnp.asarray(dt),
    )
    train_observations = make_observations(history_train_true)

    learned_params = make_initial_learned_params()
    optimizer = optax.adam(learning_rate)
    opt_state = optimizer.init(learned_params)

    def loss_components(params):
        slot_providers = {"controller": controller_from_params(params)}
        _, history = module.rollout(
            initial_state,
            forcing_train,
            constants,
            slot_providers,
            jnp.asarray(dt),
        )
        return module.loss_components(
            history,
            train_observations,
            forcing_series=forcing_train,
            constants=constants,
            slot_providers=slot_providers,
        )

    def training_loss(params):
        components = loss_components(params)
        return (
            components["obs_loss"]
            + consistency_weight
            * (components["consistency_loss"] / jnp.asarray(max(horizon_steps, 1)))
            + components["soft_penalty_loss"]
        )

    initial_components = loss_components(learned_params)
    initial_loss = float(training_loss(learned_params))
    initial_obs_loss = float(initial_components["obs_loss"])

    for _ in range(train_steps):
        loss, grads = jax.value_and_grad(training_loss)(learned_params)
        updates, opt_state = optimizer.update(grads, opt_state, learned_params)
        learned_params = optax.apply_updates(learned_params, updates)

    final_components = loss_components(learned_params)
    final_loss = float(training_loss(learned_params))
    final_obs_loss = float(final_components["obs_loss"])

    learned_slot_providers = {"controller": controller_from_params(learned_params)}
    _, history_holdout_true = module.rollout(
        initial_state,
        forcing_holdout,
        constants,
        true_slot_providers,
        jnp.asarray(dt),
    )
    _, history_holdout_learned = module.rollout(
        initial_state,
        forcing_holdout,
        constants,
        learned_slot_providers,
        jnp.asarray(dt),
    )

    holdout_transpiration_mse = float(
        jnp.mean(
            (history_holdout_learned["transpiration"] - history_holdout_true["transpiration"])
            ** 2
        )
    )
    holdout_water_mse = float(
        jnp.mean((history_holdout_learned["water"] - history_holdout_true["water"]) ** 2)
    )

    return TrainingResult(
        initial_loss=initial_loss,
        final_loss=final_loss,
        initial_obs_loss=initial_obs_loss,
        final_obs_loss=final_obs_loss,
        holdout_transpiration_mse=holdout_transpiration_mse,
        holdout_water_mse=holdout_water_mse,
        learned_params=learned_params,
    )
