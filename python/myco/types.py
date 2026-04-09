from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterable, Literal

Mode = Literal["simulate", "fit", "train"]
DirectBindingKind = Literal["data_series", "constant", "initial_state"]
InitialStateSource = Literal["constant", "data", "learned"]
SlotBindingKind = Literal["data_series", "constant", "learned"]
LossKind = Literal["mse", "huber"]
ObservationScheduleKind = Literal["dense_per_step", "sparse"]
Backend = Literal["python", "jax"]


@dataclass(frozen=True, slots=True)
class ModelSummary:
    name: str
    quantity_count: int
    relation_count: int
    slot_count: int
    external_count: int
    state_count: int
    node_count: int
    temporal_count: int
    quantity_names: tuple[str, ...]
    relation_names: tuple[str, ...]
    slot_names: tuple[str, ...]

    @classmethod
    def from_payload(cls, payload: dict[str, object]) -> "ModelSummary":
        return cls(
            name=str(payload["name"]),
            quantity_count=int(payload["quantity_count"]),
            relation_count=int(payload["relation_count"]),
            slot_count=int(payload["slot_count"]),
            external_count=int(payload["external_count"]),
            state_count=int(payload["state_count"]),
            node_count=int(payload["node_count"]),
            temporal_count=int(payload["temporal_count"]),
            quantity_names=tuple(payload["quantity_names"]),
            relation_names=tuple(payload["relation_names"]),
            slot_names=tuple(payload["slot_names"]),
        )


@dataclass(frozen=True, slots=True)
class ExperimentSummary:
    name: str
    direct_binding_count: int
    slot_binding_count: int
    observation_count: int
    planned_slot_steps: int
    planned_equation_steps: int
    planned_temporal_steps: int
    alternative_path_count: int
    unresolved_current_count: int

    @classmethod
    def from_payload(cls, payload: dict[str, object]) -> "ExperimentSummary":
        return cls(
            name=str(payload["name"]),
            direct_binding_count=int(payload["direct_binding_count"]),
            slot_binding_count=int(payload["slot_binding_count"]),
            observation_count=int(payload["observation_count"]),
            planned_slot_steps=int(payload["planned_slot_steps"]),
            planned_equation_steps=int(payload["planned_equation_steps"]),
            planned_temporal_steps=int(payload["planned_temporal_steps"]),
            alternative_path_count=int(payload["alternative_path_count"]),
            unresolved_current_count=int(payload["unresolved_current_count"]),
        )


@dataclass(slots=True)
class DirectBinding:
    quantity: str
    kind: DirectBindingKind
    steps: list[int] = field(default_factory=list)
    source: InitialStateSource | None = None

    def to_dict(self) -> dict[str, object]:
        payload: dict[str, object] = {"quantity": self.quantity, "kind": self.kind}
        if self.steps:
            payload["steps"] = list(self.steps)
        if self.source is not None:
            payload["source"] = self.source
        return payload

    @classmethod
    def from_dict(cls, payload: dict[str, object]) -> "DirectBinding":
        return cls(
            quantity=str(payload["quantity"]),
            kind=payload["kind"],  # type: ignore[arg-type]
            steps=[int(step) for step in payload.get("steps", [])],
            source=payload.get("source"),  # type: ignore[arg-type]
        )


@dataclass(slots=True)
class SlotBinding:
    slot: str
    kind: SlotBindingKind

    def to_dict(self) -> dict[str, object]:
        return {"slot": self.slot, "kind": self.kind}

    @classmethod
    def from_dict(cls, payload: dict[str, object]) -> "SlotBinding":
        return cls(
            slot=str(payload["slot"]),
            kind=payload["kind"],  # type: ignore[arg-type]
        )


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

    @classmethod
    def from_dict(cls, payload: dict[str, object]) -> "Observation":
        return cls(
            quantity=str(payload["quantity"]),
            loss=payload.get("loss", "mse"),  # type: ignore[arg-type]
            schedule=payload.get("schedule", "dense_per_step"),  # type: ignore[arg-type]
            steps=[int(step) for step in payload.get("steps", [])],
        )


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

    @classmethod
    def from_dict(cls, payload: dict[str, object]) -> "CompileSpec":
        return cls(
            mode=payload["mode"],  # type: ignore[arg-type]
            horizon_steps=int(payload["horizon_steps"]),
            direct_bindings=[
                DirectBinding.from_dict(item)
                for item in payload.get("direct_bindings", [])
            ],
            slot_bindings=[
                SlotBinding.from_dict(item)
                for item in payload.get("slot_bindings", [])
            ],
            observations=[
                Observation.from_dict(item)
                for item in payload.get("observations", [])
            ],
        )

    def to_json(self, *, indent: int = 2) -> str:
        return json.dumps(self.to_dict(), indent=indent)

    @classmethod
    def from_json(cls, payload: str) -> "CompileSpec":
        raw = json.loads(payload)
        if not isinstance(raw, dict):
            raise ValueError("compile spec JSON must decode to an object")
        return cls.from_dict(raw)

    def write_json(self, path: str | Path, *, indent: int = 2) -> Path:
        output = Path(path)
        output.write_text(self.to_json(indent=indent))
        return output

    @classmethod
    def read_json(cls, path: str | Path) -> "CompileSpec":
        return cls.from_json(Path(path).read_text())


@dataclass(frozen=True, slots=True)
class Artifact:
    model_name: str
    backend: Backend
    suggested_filename: str
    source: str

    @classmethod
    def from_payload(cls, payload: dict[str, object]) -> "Artifact":
        return cls(
            model_name=str(payload["model_name"]),
            backend=str(payload["backend"]),
            suggested_filename=str(payload["suggested_filename"]),
            source=str(payload["source"]),
        )

    def write(self, path: str | Path | None = None) -> Path:
        output = Path(path) if path is not None else Path(self.suggested_filename)
        output.write_text(self.source)
        return output


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


def load_spec(path: str | Path) -> CompileSpec:
    return CompileSpec.read_json(path)
