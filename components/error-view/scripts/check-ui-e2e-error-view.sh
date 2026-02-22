#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT_DIR"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/codex-error-view-target}"

echo "[e2e-error-view] contract: semantic selectors + stable wait"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_e2e_contract_uses_semantic_selectors_and_stable_waits

echo "[e2e-error-view] contract: selector stability via semantic markers"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_e2e_selector_contract_uses_semantic_markers_and_stable_waits

echo "[e2e-error-view] contract: motion path uses semantic ready/settled breakpoints"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_e2e_flow_covers_ready_and_settled_semantic_breakpoints

echo "[e2e-error-view] contract: repeatable flow with semantic failure breakpoints"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_e2e_flow_is_repeatable_and_failure_points_are_semantic

echo "[e2e-error-view] contract: high-risk path coverage (focus/keyboard + settled breakpoints)"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints

echo "[e2e-error-view] contract: interactive playground flow is repeatable with semantic breakpoints"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_interactive_playground_reuses_repeatable_semantic_e2e_flow

echo "[e2e-error-view] contract: e2e script coverage includes interactive playground contract"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_e2e_check_script_covers_interactive_playground_contract

echo "[e2e-error-view] contract: check2 evidence for repeatable e2e flow governance"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_e2e_repeatable_flow_rules

echo "[e2e-error-view] contract: check2 evidence for selector/wait governance"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-error-view] OK"
