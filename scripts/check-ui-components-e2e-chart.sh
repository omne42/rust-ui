#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-chart] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-chart] contract: semantic selectors + stable waits"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_e2e_selector_contract_uses_semantic_markers_and_stable_waits

echo "[e2e-chart] contract: ready/settled semantic breakpoints"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_e2e_contract_covers_ready_and_settled_conditions_for_chart_interaction

echo "[e2e-chart] contract: repeatable key-flow governance in check2"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_e2e_repeatable_key_flow_rules

echo "[e2e-chart] contract: repeatable key-flow semantic breakpoints"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_e2e_key_flow_is_repeatable_and_failure_points_are_semantic

echo "[e2e-chart] OK"
