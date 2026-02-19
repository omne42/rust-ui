#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-tag] contract: semantic selectors + settled waits"
cargo test -p ui-layout --test tag_semantics --no-default-features --features component-tag,inject-css tag_e2e_selector_contract_uses_semantic_markers_and_settled_waits
cargo test -p ui-layout --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-tag] contract: repeatable key flow"
cargo test -p ui-layout --test tag_semantics --no-default-features --features component-tag,inject-css tag_e2e_key_flow_is_repeatable_and_failure_points_are_semantic
cargo test -p ui-layout --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_e2e_key_flow_is_repeatable_and_failure_points_are_semantic

echo "[e2e-tag] OK"
