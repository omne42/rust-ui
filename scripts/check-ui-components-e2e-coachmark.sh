#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-coachmark] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-components --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-coachmark] contract: semantic selectors + settled waits"
cargo test -p ui-components --lib --no-default-features --features component-coachmark,inject-css coachmark_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-coachmark] contract: ready/settled semantic breakpoints for overlay paths"
cargo test -p ui-components --lib --no-default-features --features component-coachmark,inject-css coachmark_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths

echo "[e2e-coachmark] contract: checklist repeatable key-flow governance"
cargo test -p ui-components --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_repeatable_key_flow_e2e_regression_rules

echo "[e2e-coachmark] contract: repeatable key-flow covers overlay/focus/keyboard semantic breakpoints"
cargo test -p ui-components --lib --no-default-features --features component-coachmark,inject-css coachmark_e2e_key_flow_regression_covers_overlay_focus_keyboard_with_semantic_breakpoints

echo "[e2e-coachmark] OK"
