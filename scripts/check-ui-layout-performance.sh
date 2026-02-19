#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[perf] contract: button performance governance"
cargo test -p ui-layout --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: input performance governance"
cargo test -p ui-layout --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: swatch performance governance"
cargo test -p ui-layout --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: button-copy performance governance"
cargo test -p ui-layout --test button_copy_semantics button_copy_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: action-button performance governance"
cargo test -p ui-layout --test action_button_semantics action_button_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: share-button performance governance"
cargo test -p ui-layout --test share_button_semantics share_button_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: action-bar performance governance"
cargo test -p ui-layout --test action_bar_semantics --no-default-features --features component-action_bar,inject-css action_bar_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: tag performance governance"
cargo test -p ui-layout --test tag_semantics --no-default-features --features component-tag,inject-css tag_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: tag-group performance governance"
cargo test -p ui-layout --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: textarea performance governance"
cargo test -p ui-layout --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_performance_governance_contract_is_budgeted_traceable_and_blocking

echo "[perf] contract: time-field performance governance"
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: slider performance governance"
cargo test -p ui-layout --test slider_semantics --no-default-features --features component-slider,inject-css slider_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: scroll-area performance governance"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_performance_governance_budget_is_defined_and_blocking

echo "[perf] contract: docs perf probe budgets"
cargo test -p ui-layout --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages

echo "[perf] contract: render_count follow-up tracking"
cargo test -p ui-layout --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan

echo "[perf] OK"
