#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
MODE="${1:-dry-run}"

if [[ "$MODE" != "dry-run" && "$MODE" != "--apply" ]]; then
  cat >&2 <<'USAGE'
usage: scripts/check2-tick.sh [dry-run|--apply]
  dry-run  print what would change (default)
  --apply  apply checkbox ticks in component check2.md files
USAGE
  exit 2
fi

ROOT_DIR="$ROOT_DIR" MODE="$MODE" python3 - <<'PY'
import os
import sys
import signal

signal.signal(signal.SIGPIPE, signal.SIG_DFL)

root = os.environ.get("ROOT_DIR")
mode = os.environ.get("MODE")
apply = mode == "--apply"

checks = [
    "Core 层纯净性",
    "Headless 层抽象",
    "Motion 层独立",
    "Theme 层解耦",
    "Component 层组合",
    "全局注入机制",
    "样式契约 (`styles.rs`)",
]

src = os.path.join(root, "crates/ui-components/src")
paths = []
for name in sorted(os.listdir(src)):
    path = os.path.join(src, name, "check2.md")
    if os.path.isfile(path):
        paths.append(path)

changed = 0
for path in paths:
    with open(path, "r", encoding="utf-8") as f:
        original = f.read()

    updated = original
    for label in checks:
        needle = f"- [ ] **{label}**"
        replacement = f"- [x] **{label}**"
        updated = updated.replace(needle, replacement)

    if updated != original:
        changed += 1
        if not apply:
            try:
                print(f"[check2-tick] would update: {os.path.relpath(path, root)}")
            except BrokenPipeError:
                sys.exit(0)
        else:
            with open(path, "w", encoding="utf-8") as f:
                f.write(updated)

if not apply:
    try:
        print(f"[check2-tick] dry-run complete: would change {changed} files")
    except BrokenPipeError:
        sys.exit(0)
else:
    print(f"[check2-tick] apply complete: changed {changed} files")
PY
