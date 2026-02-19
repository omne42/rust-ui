#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-color-field] contract: semantic selectors + settled waits"
cargo test -p ui-layout --test color_field_semantics --no-default-features --features component-color_field,inject-css color_field_e2e_contract_uses_semantic_selectors_and_settled_waits

echo "[e2e-color-field] contract: repeatable key/pointer flow + copy-ready source"
cargo test -p ui-layout --test color_field_semantics --no-default-features --features component-color_field,inject-css color_field_e2e_contract_covers_repeatable_flow_and_copy_ready_source

echo "[e2e-color-field] contract: check2 governance complete"
cargo test -p ui-layout --test color_field_semantics --no-default-features --features component-color_field,inject-css color_field_check2_marks_component_governance_complete

echo "[e2e-color-field] OK"
