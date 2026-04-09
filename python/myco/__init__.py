from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterable, Literal

from ._myco_py import (
    MycoError,
    compile_demo_path as _compile_demo_path,
    compile_demo_source as _compile_demo_source,
    compile_path_with_spec_json as _compile_path_with_spec_json,
    compile_source_with_spec_json as _compile_source_with_spec_json,
    load_model_path as _load_model_path,
    load_model_source as _load_model_source,
    prepare_experiment_path_json as _prepare_experiment_path_json,
    prepare_experiment_source_json as _prepare_experiment_source_json,
    write_demo_path,
)

Mode = Literal["simulate", "fit", "train"]
DirectBindingKind = Literal["data_series", "constant", "initial_state"]
InitialStateSource = Literal["constant", "data", "learned"]
SlotBindingKind = Literal["data_series", "constant", "learned"]
LossKind = Literal["mse", "huber"]
ObservationScheduleKind = Literal["dense_per_step", "sparse"]
Backend = Literal["python", "jax"]


@dataclass(slots=True)
class DirectBinding:
    quantity: str
    kind: DirectBindingKind
    steps: list[int] = field(default_factory=list)
    source: InitialStateSource | None = None

    def to_dict(self) -> dict[str, object]:
        payload: dict[str, object] = {
            "quantity": self.quantity,
            "kind": self.kind,
        }
        if self.steps:
            payload["steps"] = list(self.steps)
        if self.source is not None:
            payload["source"] = self.source
        return payload


@dataclass(slots=True)
class SlotBinding:
    slot: str
    kind: SlotBindingKind

    def to_dict(self) -> dict[str, object]:
        return {"slot": self.slot, "kind": self.kind}


@dataclass(slots=True)
class Observation:
    quantity: str
    loss: LossKind = "mse"
    schedule: ObservationScheduleKind = "dense_per_step"
    steps: list[int] = field(default_factory=list)

    def to_dict(self) -> dict[str, object]:
        payload: dict[str, object] = {
            "quantity": self.quantity,
            "loss": self.loss,
            "schedule": self.schedule,
        }
        if self.steps:
            payload["steps"] = list(self.steps)
        return payload


@dataclass(slots=True)
class CompileSpec:
    mode: Mode
    horizon_steps: int
    direct_bindings: list[DirectBinding] = field(default_factory=list)
    slot_bindings: list[SlotBinding] = field(default_factory=list)
    observations: list[Observation] = field(default_factory=list)

    def to_dict(self) -> dict[str, object]:
        return {
            "mode": self.mode,
            "horizon_steps": self.horizon_steps,
            "direct_bindings": [binding.to_dict() for binding in self.direct_bindings],
            "slot_bindings": [binding.to_dict() for binding in self.slot_bindings],
            "observations": [observation.to_dict() for observation in self.observations],
        }


@dataclass(slots=True)
class Artifact:
    model_name: str
    backend: Backend
    suggested_filename: str
    source: str

    @classmethod
    def from_payload(cls, payload: dict[str, object]) -> "Artifact":
        return cls(
            model_name=str(payload["model_name"]),
            backend=str(payload["backend"]),  # type: ignore[arg-type]
            suggested_filename=str(payload["suggested_filename"]),
            source=str(payload["source"]),
        )

    def write(self, path: str | Path | None = None) -> Path:
        output = Path(path) if path is not None else Path(self.suggested_filename)
        output.write_text(self.source)
        return output


@dataclass(slots=True)
class Model:
    source: str
    summary: dict[str, object]
    path: Path | None = None

    @classmethod
    def from_source(cls, source: str) -> "Model":
        payload = _load_model_source(source)
        return cls(source=source, summary=payload["model"], path=None)

    @classmethod
    def from_path(cls, path: str | Path) -> "Model":
        source_path = Path(path)
        payload = _load_model_path(str(source_path))
        return cls(
            source=source_path.read_text(),
            summary=payload["model"],
            path=source_path,
        )

    def experiment(self, mode: Mode, horizon_steps: int) -> "Experiment":
        return Experiment(model=self, spec=CompileSpec(mode=mode, horizon_steps=horizon_steps))

    def compile(self, spec: CompileSpec, backend: Backend = "jax") -> Artifact:
        payload = _compile_source_with_spec_json(self.source, _dump_spec(spec), backend)
        return Artifact.from_payload(payload["artifact"])


@dataclass(slots=True)
class Experiment:
    model: Model
    spec: CompileSpec

    def bind_data_series(self, quantity: str, steps: Iterable[int]) -> "Experiment":
        self.spec.direct_bindings.append(
            DirectBinding(quantity=quantity, kind="data_series", steps=list(steps))
        )
        return self

    def bind_constant(self, quantity: str) -> "Experiment":
        self.spec.direct_bindings.append(
            DirectBinding(quantity=quantity, kind="constant")
        )
        return self

    def bind_initial_state(
        self,
        quantity: str,
        source: InitialStateSource = "constant",
    ) -> "Experiment":
        self.spec.direct_bindings.append(
            DirectBinding(quantity=quantity, kind="initial_state", source=source)
        )
        return self

    def bind_slot(self, slot: str, kind: SlotBindingKind = "learned") -> "Experiment":
        self.spec.slot_bindings.append(SlotBinding(slot=slot, kind=kind))
        return self

    def observe_dense(self, quantity: str, loss: LossKind = "mse") -> "Experiment":
        self.spec.observations.append(
            Observation(quantity=quantity, loss=loss, schedule="dense_per_step")
        )
        return self

    def observe_sparse(
        self,
        quantity: str,
        steps: Iterable[int],
        loss: LossKind = "mse",
    ) -> "Experiment":
        self.spec.observations.append(
            Observation(
                quantity=quantity,
                loss=loss,
                schedule="sparse",
                steps=list(steps),
            )
        )
        return self

    def summary(self) -> dict[str, object]:
        payload = _prepare_experiment_source_json(self.model.source, _dump_spec(self.spec))
        return payload["experiment"]

    def compile(self, backend: Backend = "jax") -> Artifact:
        payload = _compile_source_with_spec_json(
            self.model.source,
            _dump_spec(self.spec),
            backend,
        )
        return Artifact.from_payload(payload["artifact"])


def load_model_source(source: str) -> dict[str, object]:
    return _load_model_source(source)


def load_model_path(path: str | Path) -> dict[str, object]:
    return _load_model_path(str(path))


def load(source_or_path: str | Path) -> Model:
    path = Path(source_or_path)
    if path.exists():
        return Model.from_path(path)
    return Model.from_source(str(source_or_path))


def compile_demo_source(source: str, backend: Backend = "jax") -> Artifact:
    payload = _compile_demo_source(source, backend)
    return Artifact.from_payload(payload["artifact"])


def compile_demo_path(path: str | Path, backend: Backend = "jax") -> Artifact:
    payload = _compile_demo_path(str(path), backend)
    return Artifact.from_payload(payload["artifact"])


def data_series(quantity: str, steps: Iterable[int]) -> DirectBinding:
    return DirectBinding(quantity=quantity, kind="data_series", steps=list(steps))


def constant(quantity: str) -> DirectBinding:
    return DirectBinding(quantity=quantity, kind="constant")


def initial_state(
    quantity: str,
    source: InitialStateSource = "constant",
) -> DirectBinding:
    return DirectBinding(quantity=quantity, kind="initial_state", source=source)


def slot(slot_name: str, kind: SlotBindingKind = "learned") -> SlotBinding:
    return SlotBinding(slot=slot_name, kind=kind)


def observe_dense(quantity: str, loss: LossKind = "mse") -> Observation:
    return Observation(quantity=quantity, loss=loss, schedule="dense_per_step")


def observe_sparse(
    quantity: str,
    steps: Iterable[int],
    loss: LossKind = "mse",
) -> Observation:
    return Observation(
        quantity=quantity,
        loss=loss,
        schedule="sparse",
        steps=list(steps),
    )


def _dump_spec(spec: CompileSpec) -> str:
    return json.dumps(spec.to_dict())


__all__ = [
    "Artifact",
    "CompileSpec",
    "DirectBinding",
    "Experiment",
    "Model",
    "MycoError",
    "Observation",
    "SlotBinding",
    "compile_demo_path",
    "compile_demo_source",
    "constant",
    "data_series",
    "initial_state",
    "load",
    "load_model_path",
    "load_model_source",
    "observe_dense",
    "observe_sparse",
    "slot",
    "write_demo_path",
]
