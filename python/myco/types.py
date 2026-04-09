from __future__ import annotations

import json
import types
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterable, Literal

Mode = Literal["simulate", "fit", "train"]
ConsistencyPolicy = Literal["off", "equation_only", "all"]
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


@dataclass(frozen=True, slots=True)
class PlanSourceSpan:
    start_line: int
    start_column: int
    end_line: int
    end_column: int

    @classmethod
    def from_payload(cls, payload: dict[str, object]) -> "PlanSourceSpan":
        return cls(
            start_line=int(payload["start_line"]),
            start_column=int(payload["start_column"]),
            end_line=int(payload["end_line"]),
            end_column=int(payload["end_column"]),
        )


@dataclass(frozen=True, slots=True)
class PathExplanation:
    output: str
    source: str
    direction: str
    cost: int
    dependencies: tuple[str, ...]
    expression: str | None
    provenance_label: str | None
    source_span: PlanSourceSpan | None

    @classmethod
    def from_payload(cls, payload: dict[str, object]) -> "PathExplanation":
        source_span = payload.get("source_span")
        return cls(
            output=str(payload["output"]),
            source=str(payload["source"]),
            direction=str(payload["direction"]),
            cost=int(payload["cost"]),
            dependencies=tuple(payload["dependencies"]),
            expression=payload.get("expression"),
            provenance_label=payload.get("provenance_label"),
            source_span=(
                PlanSourceSpan.from_payload(source_span)
                if isinstance(source_span, dict)
                else None
            ),
        )


@dataclass(frozen=True, slots=True)
class AlternativeExplanation:
    output: str
    source: str
    direction: str
    cost: int
    expression: str | None
    provenance_label: str | None
    source_span: PlanSourceSpan | None

    @classmethod
    def from_payload(cls, payload: dict[str, object]) -> "AlternativeExplanation":
        source_span = payload.get("source_span")
        return cls(
            output=str(payload["output"]),
            source=str(payload["source"]),
            direction=str(payload["direction"]),
            cost=int(payload["cost"]),
            expression=payload.get("expression"),
            provenance_label=payload.get("provenance_label"),
            source_span=(
                PlanSourceSpan.from_payload(source_span)
                if isinstance(source_span, dict)
                else None
            ),
        )


@dataclass(frozen=True, slots=True)
class BlockedCandidateExplanation:
    output: str
    source: str
    direction: str
    cost: int
    missing_dependencies: tuple[str, ...]
    expression: str | None
    provenance_label: str | None
    source_span: PlanSourceSpan | None

    @classmethod
    def from_payload(cls, payload: dict[str, object]) -> "BlockedCandidateExplanation":
        source_span = payload.get("source_span")
        return cls(
            output=str(payload["output"]),
            source=str(payload["source"]),
            direction=str(payload["direction"]),
            cost=int(payload["cost"]),
            missing_dependencies=tuple(payload["missing_dependencies"]),
            expression=payload.get("expression"),
            provenance_label=payload.get("provenance_label"),
            source_span=(
                PlanSourceSpan.from_payload(source_span)
                if isinstance(source_span, dict)
                else None
            ),
        )


@dataclass(frozen=True, slots=True)
class UnresolvedQuantityExplanation:
    quantity: str
    blocked_candidates: tuple[BlockedCandidateExplanation, ...]

    @classmethod
    def from_payload(cls, payload: dict[str, object]) -> "UnresolvedQuantityExplanation":
        return cls(
            quantity=str(payload["quantity"]),
            blocked_candidates=tuple(
                BlockedCandidateExplanation.from_payload(item)
                for item in payload["blocked_candidates"]
            ),
        )


@dataclass(frozen=True, slots=True)
class PlanExplanation:
    available_current: tuple[str, ...]
    chosen_current: tuple[PathExplanation, ...]
    chosen_temporal: tuple[PathExplanation, ...]
    alternatives: tuple[AlternativeExplanation, ...]
    unresolved: tuple[UnresolvedQuantityExplanation, ...]

    @classmethod
    def from_payload(cls, payload: dict[str, object]) -> "PlanExplanation":
        return cls(
            available_current=tuple(payload["available_current"]),
            chosen_current=tuple(
                PathExplanation.from_payload(item) for item in payload["chosen_current"]
            ),
            chosen_temporal=tuple(
                PathExplanation.from_payload(item) for item in payload["chosen_temporal"]
            ),
            alternatives=tuple(
                AlternativeExplanation.from_payload(item)
                for item in payload["alternatives"]
            ),
            unresolved=tuple(
                UnresolvedQuantityExplanation.from_payload(item)
                for item in payload["unresolved"]
            ),
        )


@dataclass(frozen=True, slots=True)
class QuantityExplanation:
    quantity: str
    direct_binding: str | None
    slot_provider: str | None
    observed: bool
    chosen_current: PathExplanation | None
    chosen_temporal: PathExplanation | None
    alternatives: tuple[AlternativeExplanation, ...]
    blocked_candidates: tuple[BlockedCandidateExplanation, ...]
    unresolved: bool

    @classmethod
    def from_payload(cls, payload: dict[str, object]) -> "QuantityExplanation":
        current = payload.get("chosen_current")
        temporal = payload.get("chosen_temporal")
        return cls(
            quantity=str(payload["quantity"]),
            direct_binding=payload.get("direct_binding"),
            slot_provider=payload.get("slot_provider"),
            observed=bool(payload["observed"]),
            chosen_current=(
                PathExplanation.from_payload(current)
                if isinstance(current, dict)
                else None
            ),
            chosen_temporal=(
                PathExplanation.from_payload(temporal)
                if isinstance(temporal, dict)
                else None
            ),
            alternatives=tuple(
                AlternativeExplanation.from_payload(item)
                for item in payload["alternatives"]
            ),
            blocked_candidates=tuple(
                BlockedCandidateExplanation.from_payload(item)
                for item in payload["blocked_candidates"]
            ),
            unresolved=bool(payload["unresolved"]),
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
    consistency_policy: ConsistencyPolicy = "equation_only"
    direct_bindings: list[DirectBinding] = field(default_factory=list)
    slot_bindings: list[SlotBinding] = field(default_factory=list)
    observations: list[Observation] = field(default_factory=list)

    def to_dict(self) -> dict[str, object]:
        return {
            "mode": self.mode,
            "horizon_steps": self.horizon_steps,
            "consistency_policy": self.consistency_policy,
            "direct_bindings": [binding.to_dict() for binding in self.direct_bindings],
            "slot_bindings": [binding.to_dict() for binding in self.slot_bindings],
            "observations": [observation.to_dict() for observation in self.observations],
        }

    @classmethod
    def from_dict(cls, payload: dict[str, object]) -> "CompileSpec":
        return cls(
            mode=payload["mode"],  # type: ignore[arg-type]
            horizon_steps=int(payload["horizon_steps"]),
            consistency_policy=payload.get("consistency_policy", "equation_only"),  # type: ignore[arg-type]
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
class SlotInterface:
    slot: str
    kind: SlotBindingKind
    inputs: tuple[str, ...]
    outputs: tuple[str, ...]
    input_arity: int
    output_arity: int

    @classmethod
    def from_payload(cls, payload: dict[str, object]) -> "SlotInterface":
        return cls(
            slot=str(payload["slot"]),
            kind=payload["kind"],  # type: ignore[arg-type]
            inputs=tuple(payload["inputs"]),
            outputs=tuple(payload["outputs"]),
            input_arity=int(payload["input_arity"]),
            output_arity=int(payload["output_arity"]),
        )


@dataclass(frozen=True, slots=True)
class ArtifactMetadata:
    compile_mode: Mode
    consistency_policy: ConsistencyPolicy
    loss_helpers_enabled: bool
    learned_initial_state: tuple[str, ...]
    learned_slots: tuple[str, ...]
    slot_interfaces: tuple[SlotInterface, ...]

    @classmethod
    def from_payload(cls, payload: dict[str, object]) -> "ArtifactMetadata":
        return cls(
            compile_mode=payload["compile_mode"],  # type: ignore[arg-type]
            consistency_policy=payload["consistency_policy"],  # type: ignore[arg-type]
            loss_helpers_enabled=bool(payload["loss_helpers_enabled"]),
            learned_initial_state=tuple(payload["learned_initial_state"]),
            learned_slots=tuple(payload["learned_slots"]),
            slot_interfaces=tuple(
                SlotInterface.from_payload(item) for item in payload["slot_interfaces"]
            ),
        )


@dataclass(frozen=True, slots=True)
class Artifact:
    model_name: str
    backend: Backend
    suggested_filename: str
    metadata: ArtifactMetadata
    source: str

    @classmethod
    def from_payload(cls, payload: dict[str, object]) -> "Artifact":
        return cls(
            model_name=str(payload["model_name"]),
            backend=str(payload["backend"]),
            suggested_filename=str(payload["suggested_filename"]),
            metadata=ArtifactMetadata.from_payload(payload["metadata"]),
            source=str(payload["source"]),
        )

    def write(self, path: str | Path | None = None) -> Path:
        output = Path(path) if path is not None else Path(self.suggested_filename)
        output.write_text(self.source)
        return output

    def to_module(self, module_name: str | None = None) -> types.ModuleType:
        name = module_name or _sanitize_module_name(self.model_name)
        module = types.ModuleType(name)
        module.__file__ = f"<generated:{name}>"
        exec(compile(self.source, module.__file__, "exec"), module.__dict__)
        return module

    def slot_interface(self, slot_name: str) -> SlotInterface | None:
        for slot in self.metadata.slot_interfaces:
            if slot.slot == slot_name:
                return slot
        return None


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


def _sanitize_module_name(name: str) -> str:
    lowered = "".join(ch.lower() if ch.isalnum() else "_" for ch in name).strip("_")
    return lowered or "myco_artifact"
