#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""Fetch a section, subsection, part, or appendix from the spec by id.

Usage:
    spec_section.py 3              # full §3
    spec_section.py 3.9            # full §3.9
    spec_section.py 3 --summary    # just the Summary paragraph under §3
    spec_section.py "Part II"      # full Part II
    spec_section.py "Appendix C"   # full Appendix C
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


def parse_id(arg: str) -> str:
    s = arg.strip()
    if re.fullmatch(r"\d+(\.\d+)?", s):
        return s
    m = re.fullmatch(r"(?i)appendix\s+([a-z])", s)
    if m:
        return "Appendix " + m.group(1).upper()
    m = re.fullmatch(r"(?i)part\s+([ivxlcdm]+)", s)
    if m:
        return "Part " + m.group(1).upper()
    raise ValueError(f"cannot parse section id: {arg!r}")


def section_token(match: re.Match) -> str:
    _, part, appendix, numeric, _ = match.groups()
    return part or appendix or numeric


def find_section(lines: list[str], target: str) -> tuple[int, int] | None:
    start: int | None = None
    start_level: int = 0
    for i, line in enumerate(lines):
        m = HEADER_RE.match(line)
        if not m:
            continue
        level = len(m.group(1))
        token = section_token(m)
        if start is None:
            if token == target:
                start = i
                start_level = level
            continue
        if level <= start_level:
            return (start, i)
    if start is not None:
        return (start, len(lines))
    return None


def extract_summary(body: list[str]) -> list[str]:
    found = False
    out: list[str] = []
    for line in body:
        if not found:
            if line.lstrip().startswith("**Summary.**"):
                found = True
                out.append(line)
            continue
        if line.strip() == "":
            break
        out.append(line)
    return out


def main() -> int:
    doc = __doc__ or ""
    ap = argparse.ArgumentParser(description=doc.splitlines()[0])
    ap.add_argument("section")
    ap.add_argument("--summary", action="store_true")
    args = ap.parse_args()

    try:
        target = parse_id(args.section)
    except ValueError as e:
        print(f"error: {e}", file=sys.stderr)
        return 2

    lines = SPEC.read_text().splitlines()
    span = find_section(lines, target)
    if span is None:
        print(f"error: {target!r} not found in {SPEC}", file=sys.stderr)
        return 1

    start, end = span
    body = lines[start:end]

    if args.summary:
        summary = extract_summary(body[1:])
        if not summary:
            print(
                f"error: {target!r} has no **Summary.** paragraph",
                file=sys.stderr,
            )
            return 1
        print(body[0])
        print()
        print("\n".join(summary))
        return 0

    print("\n".join(body))
    return 0


if __name__ == "__main__":
    sys.exit(main())
