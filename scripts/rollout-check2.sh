#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
SRC_DIR="$ROOT_DIR/crates/ui-components/src"
TEMPLATE_PATH="/root/code/personal/omne/check2.md"
REPORT_PATH="$ROOT_DIR/crates/ui-components/CHECK2_AUDIT_REPORT.md"

MODE="${1:-copy}"

if [[ "$MODE" != "copy" && "$MODE" != "audit" ]]; then
  cat >&2 <<'USAGE'
usage: scripts/rollout-check2.sh [copy|audit]
  copy  copy /root/code/personal/omne/check2.md into each component directory
  audit generate per-component file-structure report (no file mutation)
USAGE
  exit 2
fi

if [[ ! -f "$TEMPLATE_PATH" ]]; then
  echo "template not found: $TEMPLATE_PATH" >&2
  exit 1
fi

if [[ ! -d "$SRC_DIR" ]]; then
  echo "components dir not found: $SRC_DIR" >&2
  exit 1
fi

updated_readme=0
mapfile -t component_dirs < <(find "$SRC_DIR" -mindepth 1 -maxdepth 1 -type d | sort)

if [[ ${#component_dirs[@]} -eq 0 ]]; then
  echo "no component directories found under $SRC_DIR" >&2
  exit 1
fi

run_ts="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
processed=0
updated_check2=0

if [[ "$MODE" == "copy" ]]; then
  for dir in "${component_dirs[@]}"; do
    cp "$TEMPLATE_PATH" "$dir/check2.md"
    processed=$((processed + 1))
    updated_check2=$((updated_check2 + 1))
  done
  echo "copy complete"
  echo "processed=$processed check2=$updated_check2"
  exit 0
fi

missing_logic=0
missing_styles=0
missing_view=0
missing_motion=0
unchecked_count=0

{
  echo "# Check2 Audit Report"
  echo
  echo "- Timestamp: $run_ts"
  echo
  echo "| Component | logic.rs | styles.rs | view.rs | motion.rs | check2 unchecked |"
  echo "|---|---|---|---|---|---|"
} > "$REPORT_PATH"

for dir in "${component_dirs[@]}"; do
  component="$(basename "$dir")"
  processed=$((processed + 1))

  has_logic="yes"
  has_styles="yes"
  has_view="yes"
  has_motion="yes"

  [[ -f "$dir/logic.rs" ]] || { has_logic="no"; missing_logic=$((missing_logic + 1)); }
  [[ -f "$dir/styles.rs" ]] || { has_styles="no"; missing_styles=$((missing_styles + 1)); }
  [[ -f "$dir/view.rs" ]] || { has_view="no"; missing_view=$((missing_view + 1)); }
  [[ -f "$dir/motion.rs" ]] || { has_motion="no"; missing_motion=$((missing_motion + 1)); }

  unchecked="n/a"
  if [[ -f "$dir/check2.md" ]]; then
    unchecked="$(rg -n "\\[ \\]" "$dir/check2.md" | wc -l)"
    unchecked_count=$((unchecked_count + unchecked))
  fi

  echo "| $component | $has_logic | $has_styles | $has_view | $has_motion | $unchecked |" >> "$REPORT_PATH"
done

{
  echo
  echo "- Processed component directories: $processed"
  echo "- Missing logic.rs: $missing_logic"
  echo "- Missing styles.rs: $missing_styles"
  echo "- Missing view.rs: $missing_view"
  echo "- Missing motion.rs: $missing_motion"
  echo "- Total unchecked checklist boxes: $unchecked_count"
} >> "$REPORT_PATH"

echo "audit complete"
echo "processed=$processed missing_logic=$missing_logic missing_styles=$missing_styles missing_view=$missing_view missing_motion=$missing_motion unchecked_total=$unchecked_count"
echo "report=$REPORT_PATH"
