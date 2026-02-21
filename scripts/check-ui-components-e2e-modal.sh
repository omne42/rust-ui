#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-modal] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-modal] contract: semantic selectors + settled waits"
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_e2e_contract_uses_semantic_selectors_and_stable_waits

echo "[e2e-modal] contract: motion-ready/settled semantic breakpoints"
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_e2e_contract_covers_ready_and_settled_conditions_for_overlay_dismissal

echo "[e2e-modal] contract: replayable critical flow with overlay/focus/keyboard checkpoints"
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_e2e_regression_flow_is_replayable_and_maps_failures_to_semantic_breakpoints

echo "[e2e-modal] OK"
