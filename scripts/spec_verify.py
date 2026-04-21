#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""Integrity checks on the canonical spec.

Checks:
    - Every top-level numbered section has a **Summary.** paragraph.
    - Every §X or §X.Y cross-reference resolves to a real section.
    - No em dashes (—) in prose (style rule).
    - No version strings (v2.1, v2.2, etc.) in prose.
    - No historical breadcrumbs ("was X", "absorbed into", "previously").

Usage:
    spec_verify.py                 # full spec
    spec_verify.py 3 3.9 "Part II" # scope to listed sections
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

SPEC = Path(__file__).resolve().parent.parent / "planning" / "v2" / "spec_new.md"

HEADER_RE = re.compile(
    r"^(#+)\s+"
    r"(?:(Part\s+[IVXLCDM]+)|(Appendix\s+[A-Z])|(\d+(?:\.\d+)?))"
    r"\s*[—.\-–]*\s*(.*)$"
)

XREF_RE = re.compile(r"§\s*(\d+(?:\.\d+)?)")
VERSION_RE = re.compile(r"\bv2\.\d\b")
EMDASH_RE = re.compile(r"—")
BREADCRUMB_PATTERNS = [
    re.compile(r"\bwas (?:previously |formerly |originally )"),
    re.compile(r"\babsorbed into\b", re.IGNORECASE),
    re.compile(r"\bmoved from (?:§|Part|Appendix)", re.IGNORECASE),
    re.compile(r"\bpreviously (?:§|Part|Appendix)", re.IGNORECASE),
    re.compile(r"\bformerly §", re.IGNORECASE),
]


def collect_sections(lines: list[str]) -> dict[str, tuple[int, int, int]]:
    """Return token -> (start_line, end_line, level)."""
    out: dict[str, tuple[int, int, int]] = {}
    opens: list[tuple[str, int, int]] = []
    for i, line in enumerate(lines):
        m = HEADER_RE.match(line)
        if not m:
            continue
        level = len(m.group(1))
        token = m.group(2) or m.group(3) or m.group(4)
        while opens and opens[-1][2] >= level:
            t, s, _ = opens.pop()
            if t not in out:
                out[t] = (s, i, _)
            else:
                out[t] = (out[t][0], i, out[t][2])
        opens.append((token, i, level))
    for t, s, lvl in opens:
        if t not in out:
            out[t] = (s, len(lines), lvl)
    return out


def has_summary(lines: list[str], start: int, end: int) -> bool:
    for line in lines[start:end]:
        if line.lstrip().startswith("**Summary.**"):
            return True
    return False


def in_scope(token: str, scope: set[str]) -> bool:
    if not scope:
        return True
    if token in scope:
        return True
    for s in scope:
        if token.startswith(s + "."):
            return True
    return False


def main() -> int:
    ap = argparse.ArgumentParser(description="Verify spec integrity.")
    ap.add_argument("scope", nargs="*", help="optional section tokens to scope")
    args = ap.parse_args()

    scope: set[str] = set()
    for s in args.scope:
        s = s.strip()
        if re.fullmatch(r"\d+(\.\d+)?", s):
            scope.add(s)
            continue
        m = re.fullmatch(r"(?i)part\s+([ivxlcdm]+)", s)
        if m:
            scope.add("Part " + m.group(1).upper())
            continue
        m = re.fullmatch(r"(?i)appendix\s+([a-z])", s)
        if m:
            scope.add("Appendix " + m.group(1).upper())
            continue
        print(f"warning: unrecognized scope token {s!r}", file=sys.stderr)

    lines = SPEC.read_text().splitlines()
    sections = collect_sections(lines)
    findings: list[str] = []

    numbered_top = [
        t for t, (_, _, lvl) in sections.items()
        if re.fullmatch(r"\d+", t) and lvl <= 3
    ]
    for t in numbered_top:
        if not in_scope(t, scope):
            continue
        s, e, _ = sections[t]
        if not has_summary(lines, s, e):
            findings.append(f"[summary] §{t} is missing **Summary.** paragraph")

    for i, line in enumerate(lines, 1):
        for m in XREF_RE.finditer(line):
            tgt = m.group(1)
            if tgt not in sections:
                findings.append(
                    f"[xref] {SPEC.name}:{i}: §{tgt} does not resolve"
                )

    for i, line in enumerate(lines, 1):
        if line.startswith("#"):
            continue
        if EMDASH_RE.search(line):
            findings.append(f"[style] {SPEC.name}:{i}: em dash (—)")
        if VERSION_RE.search(line):
            findings.append(f"[style] {SPEC.name}:{i}: version string")
        for pat in BREADCRUMB_PATTERNS:
            if pat.search(line):
                findings.append(
                    f"[breadcrumb] {SPEC.name}:{i}: historical phrasing"
                )
                break

    if not findings:
        print("ok: no issues found")
        return 0

    for f in findings:
        print(f)
    print(f"\n{len(findings)} issue(s)", file=sys.stderr)
    return 1


if __name__ == "__main__":
    sys.exit(main())
