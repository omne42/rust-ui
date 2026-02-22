#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-textarea] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-textarea] contract: semantic selectors + settled waits"
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-textarea] contract: repeatable key flow with semantic breakpoints"
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_e2e_key_flow_is_repeatable_and_failure_points_are_semantic

echo "[e2e-textarea] OK"
