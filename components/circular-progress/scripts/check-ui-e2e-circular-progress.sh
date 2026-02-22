#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-circular-progress] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-circular-progress] contract: semantic selectors + settled waits"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-circular-progress] contract: animation path ready/settled semantic breakpoints"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_e2e_contract_covers_ready_and_settled_semantic_breakpoints

echo "[e2e-circular-progress] contract: key flow regression is repeatable and semantic-breakpoint diagnosable"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_e2e_key_flow_regression_is_repeatable_and_breakpoint_diagnosable

echo "[e2e-circular-progress] OK"
