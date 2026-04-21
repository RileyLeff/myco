#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""Generate a role-specific scaffold from the spec (Phase 7).

Reads spec_new.md and concatenates:
    - Full text for sections relevant to the role.
    - **Summary.** paragraphs only for other sections.

Size target: 15-20k tokens per scaffold. Role maps live in ROLE_MAP below.

Usage:
    spec_roles.py compiler
    spec_roles.py workflow
    spec_roles.py backend

Status: stub. Role maps are placeholders; populate once the spec has stable
section numbering and fill density is high enough to carve meaningful roles.
"""
from __future__ import annotations

import argparse
import sys

ROLE_MAP: dict[str, list[str]] = {
    "compiler": [],
    "workflow": [],
    "backend": [],
}


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0] if __doc__ else "")
    ap.add_argument("role", choices=sorted(ROLE_MAP.keys()))
    args = ap.parse_args()
    print(
        f"spec_roles.py: role={args.role!r} — stub; "
        "role maps not populated yet (Phase 7).",
        file=sys.stderr,
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
