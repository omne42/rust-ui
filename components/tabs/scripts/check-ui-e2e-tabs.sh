#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-tabs] contract: semantic selectors + settled waits"
# cargo test -p ui --test tabs_semantics tabs_e2e_selector_contract_uses_semantic_markers_and_settled_waits
CARGO_TARGET_DIR=target-tabs-check2 cargo test -p ui --test tabs_semantics --no-default-features --features component-tabs,inject-css tabs_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-tabs] contract: repeatable keyboard focus flow"
# cargo test -p ui --test tabs_semantics tabs_e2e_key_flow_covers_keyboard_focus_and_semantic_state_sync
CARGO_TARGET_DIR=target-tabs-check2 cargo test -p ui --test tabs_semantics --no-default-features --features component-tabs,inject-css tabs_e2e_key_flow_covers_keyboard_focus_and_semantic_state_sync

echo "[e2e-tabs] OK"
