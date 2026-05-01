#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""Print the spec section index.

Usage:
    spec_index.py           # full tree with subsections
    spec_index.py --flat    # one-line-per-section, no indent
    spec_index.py --top     # top-level only (Parts and numbered sections)
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

SPEC = Path(__file__).resolve().parent.parent / "planning" / "v2" / "spec.md"

HEADER_RE = re.compile(
    r"^(#+)\s+"
    r"(?:(Part\s+[IVXLCDM]+)|(Appendix\s+[A-Z])|(\d+(?:\.\d+)?))"
    r"\s*[—.\-–]*\s*(.*)$"
)


def main() -> int:
    ap = argparse.ArgumentParser(description="Print spec section index.")
    ap.add_argument("--flat", action="store_true", help="no indentation")
    ap.add_argument("--top", action="store_true", help="top-level only")
    args = ap.parse_args()

    lines = SPEC.read_text().splitlines()
    for line in lines:
        m = HEADER_RE.match(line)
        if not m:
            continue
        level = len(m.group(1))
        part, appendix, numeric, title = m.group(2), m.group(3), m.group(4), m.group(5)
        token = part or appendix or numeric
        if args.top and level > 3:
            continue
        if args.top and numeric and "." in numeric:
            continue
        indent = "" if args.flat else "  " * max(0, level - 2)
        label = f"{token}" + (f" — {title}" if title else "")
        print(f"{indent}{label}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
