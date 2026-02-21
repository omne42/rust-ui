#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-command-dialog] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-components --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-command-dialog] contract: semantic selectors + settled waits"
cargo test -p ui-components --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-command-dialog] contract: repeatable key flow with semantic breakpoints"
cargo test -p ui-components --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic

echo "[e2e-command-dialog] OK"
