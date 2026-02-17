#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[perf] contract: button performance governance"
cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: docs perf probe budgets"
cargo test -p ui-components --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages

echo "[perf] contract: render_count follow-up tracking"
cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan

echo "[perf] OK"
