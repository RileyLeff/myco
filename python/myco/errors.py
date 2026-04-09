from __future__ import annotations

import json
from dataclasses import dataclass

from ._myco_py import MycoError as _BridgeMycoError


@dataclass(frozen=True, slots=True)
class SourcePosition:
    line: int
    column: int


@dataclass(frozen=True, slots=True)
class SourceSpan:
    start: SourcePosition
    end: SourcePosition


@dataclass(frozen=True, slots=True)
class Diagnostic:
    severity: str
    message: str
    span: SourceSpan | None = None


class MycoError(Exception):
    def __init__(self, diagnostics: list[Diagnostic]):
        self.diagnostics = tuple(diagnostics)
        super().__init__(self._format_message())

    def _format_message(self) -> str:
        if not self.diagnostics:
            return "Myco compilation failed"
        return "\n".join(_render_diagnostic(diagnostic) for diagnostic in self.diagnostics)


def bridge_call(fn, *args):
    try:
        return fn(*args)
    except _BridgeMycoError as exc:  # pragma: no cover - exercised by wrapper tests
        raise MycoError(_parse_diagnostics(str(exc))) from None


def _parse_diagnostics(payload: str) -> list[Diagnostic]:
    try:
        raw_items = json.loads(payload)
    except json.JSONDecodeError:
        return [Diagnostic(severity="error", message=payload)]

    diagnostics: list[Diagnostic] = []
    for item in raw_items:
        span_payload = item.get("span")
        span = None
        if span_payload is not None:
            span = SourceSpan(
                start=SourcePosition(**span_payload["start"]),
                end=SourcePosition(**span_payload["end"]),
            )
        diagnostics.append(
            Diagnostic(
                severity=str(item["severity"]),
                message=str(item["message"]),
                span=span,
            )
        )
    return diagnostics


def _render_diagnostic(diagnostic: Diagnostic) -> str:
    if diagnostic.span is None:
        return f"{diagnostic.severity}: {diagnostic.message}"
    start = diagnostic.span.start
    end = diagnostic.span.end
    return (
        f"{diagnostic.severity} at {start.line}:{start.column}-"
        f"{end.line}:{end.column}: {diagnostic.message}"
    )
