from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

from ._myco_py import (
    compile_demo_path as _compile_demo_path,
    compile_demo_source as _compile_demo_source,
    compile_path_with_spec as _compile_path_with_spec,
    compile_source_with_spec as _compile_source_with_spec,
    explain_plan_path as _explain_plan_path,
    explain_plan_source as _explain_plan_source,
    explain_quantity_path as _explain_quantity_path,
    explain_quantity_source as _explain_quantity_source,
    load_model_path as _load_model_path,
    load_model_source as _load_model_source,
    prepare_experiment_path as _prepare_experiment_path,
    prepare_experiment_source as _prepare_experiment_source,
    write_demo_path,
)
from .errors import bridge_call
from .types import (
    Artifact,
    Backend,
    CompileSpec,
    DirectBinding,
    ExperimentSummary,
    ModelSummary,
    Observation,
    PlanExplanation,
    QuantityExplanation,
    SlotBinding,
    load_spec,
)


@dataclass(slots=True)
class Model:
    source: str
    summary: ModelSummary
    path: Path | None = None

    @classmethod
    def from_source(cls, source: str) -> "Model":
        payload = bridge_call(_load_model_source, source)
        return cls(source=source, summary=_model_summary(payload), path=None)

    @classmethod
    def from_path(cls, path: str | Path) -> "Model":
        source_path = Path(path)
        payload = bridge_call(_load_model_path, str(source_path))
        return cls(
            source=source_path.read_text(),
            summary=_model_summary(payload),
            path=source_path,
        )

    def experiment(self, mode: str, horizon_steps: int) -> "Experiment":
        return Experiment(model=self, spec=CompileSpec(mode=mode, horizon_steps=horizon_steps))

    def compile(self, spec: CompileSpec, backend: Backend = "jax") -> Artifact:
        payload = bridge_call(_compile_source_with_spec, self.source, spec.to_dict(), backend)
        return _artifact(payload)


@dataclass(slots=True)
class Experiment:
    model: Model
    spec: CompileSpec

    def assume_series(self, quantity: str, steps) -> "Experiment":
        self.spec.direct_bindings.append(
            DirectBinding(quantity=quantity, kind="data_series", steps=list(steps))
        )
        return self

    def bind_data_series(self, quantity: str, steps) -> "Experiment":
        return self.assume_series(quantity, steps)

    def assume_constant(self, quantity: str) -> "Experiment":
        self.spec.direct_bindings.append(DirectBinding(quantity=quantity, kind="constant"))
        return self

    def bind_constant(self, quantity: str) -> "Experiment":
        return self.assume_constant(quantity)

    def assume_initial(self, quantity: str, source: str = "constant") -> "Experiment":
        self.spec.direct_bindings.append(
            DirectBinding(quantity=quantity, kind="initial_state", source=source)
        )
        return self

    def bind_initial_state(self, quantity: str, source: str = "constant") -> "Experiment":
        return self.assume_initial(quantity, source=source)

    def learn_initial(self, quantity: str) -> "Experiment":
        return self.assume_initial(quantity, source="learned")

    def bind_slot(self, slot: str, kind: str = "learned") -> "Experiment":
        self.spec.slot_bindings.append(SlotBinding(slot=slot, kind=kind))
        return self

    def learn_slot(self, slot: str) -> "Experiment":
        return self.bind_slot(slot, kind="learned")

    def set_consistency_policy(self, policy: str) -> "Experiment":
        self.spec.consistency_policy = policy
        return self

    def observe_dense(self, quantity: str, loss: str = "mse") -> "Experiment":
        self.spec.observations.append(
            Observation(quantity=quantity, loss=loss, schedule="dense_per_step")
        )
        return self

    def observe_sparse(self, quantity: str, steps, loss: str = "mse") -> "Experiment":
        self.spec.observations.append(
            Observation(quantity=quantity, loss=loss, schedule="sparse", steps=list(steps))
        )
        return self

    def summary(self) -> ExperimentSummary:
        payload = bridge_call(_prepare_experiment_source, self.model.source, self.spec.to_dict())
        return _experiment_summary(payload)

    def explain_plan(self) -> PlanExplanation:
        payload = bridge_call(_explain_plan_source, self.model.source, self.spec.to_dict())
        return PlanExplanation.from_payload(payload)

    def explain_quantity(self, quantity: str) -> QuantityExplanation:
        payload = bridge_call(
            _explain_quantity_source, self.model.source, self.spec.to_dict(), quantity
        )
        return QuantityExplanation.from_payload(payload)

    def compile(self, backend: Backend = "jax") -> Artifact:
        payload = bridge_call(
            _compile_source_with_spec,
            self.model.source,
            self.spec.to_dict(),
            backend,
        )
        return _artifact(payload)


def load(source_or_path: str | Path) -> Model:
    path = Path(source_or_path)
    if path.exists():
        return Model.from_path(path)
    return Model.from_source(str(source_or_path))


def load_model_source(source: str) -> ModelSummary:
    return _model_summary(bridge_call(_load_model_source, source))


def load_model_path(path: str | Path) -> ModelSummary:
    return _model_summary(bridge_call(_load_model_path, str(path)))


def compile_demo_source(source: str, backend: Backend = "jax") -> Artifact:
    return _artifact(bridge_call(_compile_demo_source, source, backend))


def compile_demo_path(path: str | Path, backend: Backend = "jax") -> Artifact:
    return _artifact(bridge_call(_compile_demo_path, str(path), backend))


def prepare_experiment_source(source: str, spec: CompileSpec) -> ExperimentSummary:
    return _experiment_summary(bridge_call(_prepare_experiment_source, source, spec.to_dict()))


def prepare_experiment_path(path: str | Path, spec: CompileSpec) -> ExperimentSummary:
    return _experiment_summary(bridge_call(_prepare_experiment_path, str(path), spec.to_dict()))


def explain_plan_source(source: str, spec: CompileSpec) -> PlanExplanation:
    return PlanExplanation.from_payload(
        bridge_call(_explain_plan_source, source, spec.to_dict())
    )


def explain_plan_path(path: str | Path, spec: CompileSpec) -> PlanExplanation:
    return PlanExplanation.from_payload(
        bridge_call(_explain_plan_path, str(path), spec.to_dict())
    )


def explain_quantity_source(
    source: str, spec: CompileSpec, quantity: str
) -> QuantityExplanation:
    return QuantityExplanation.from_payload(
        bridge_call(_explain_quantity_source, source, spec.to_dict(), quantity)
    )


def explain_quantity_path(
    path: str | Path, spec: CompileSpec, quantity: str
) -> QuantityExplanation:
    return QuantityExplanation.from_payload(
        bridge_call(_explain_quantity_path, str(path), spec.to_dict(), quantity)
    )


def compile_source(source: str, spec: CompileSpec, backend: Backend = "jax") -> Artifact:
    return _artifact(bridge_call(_compile_source_with_spec, source, spec.to_dict(), backend))


def compile_path(path: str | Path, spec: CompileSpec, backend: Backend = "jax") -> Artifact:
    return _artifact(bridge_call(_compile_path_with_spec, str(path), spec.to_dict(), backend))


def compile_spec_path(
    model_path: str | Path,
    spec_path: str | Path,
    backend: Backend = "jax",
) -> Artifact:
    return compile_path(model_path, load_spec(spec_path), backend=backend)


def _model_summary(payload: dict[str, object]) -> ModelSummary:
    return ModelSummary.from_payload(payload["model"])


def _experiment_summary(payload: dict[str, object]) -> ExperimentSummary:
    return ExperimentSummary.from_payload(payload["experiment"])


def _artifact(payload: dict[str, object]) -> Artifact:
    return Artifact.from_payload(payload["artifact"])
