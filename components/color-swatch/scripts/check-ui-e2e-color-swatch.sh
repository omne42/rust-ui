#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT_DIR"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/codex-color-swatch-target}"

echo "[e2e-color-swatch] contract: semantic selectors + stable wait"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_e2e_contract_uses_semantic_selectors_and_stable_waits

echo "[e2e-color-swatch] contract: selector stability via semantic markers"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_e2e_selector_contract_uses_semantic_markers_and_stable_waits

echo "[e2e-color-swatch] contract: motion path uses semantic ready/settled breakpoints"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_e2e_flow_covers_ready_and_settled_semantic_breakpoints

echo "[e2e-color-swatch] contract: repeatable key flow with keyboard/focus risk path"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_e2e_regression_suite_includes_repeatable_key_flow_and_keyboard_focus_risk_path

echo "[e2e-color-swatch] contract: interactive playground flow is repeatable with semantic breakpoints"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[e2e-color-swatch] contract: e2e script coverage includes interactive playground contract"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_e2e_check_script_covers_interactive_playground_contract

echo "[e2e-color-swatch] contract: check2 evidence for selector/wait governance"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-color-swatch] contract: check2 evidence for repeatable e2e regression collection"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_repeatable_e2e_regression_collection

echo "[e2e-color-swatch] OK"
