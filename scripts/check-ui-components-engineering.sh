#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[engineering] contract: serde schema + structured migration errors"
cargo test -p ui-components --test button_semantics button_engineering_contract_uses_serde_schema_and_structured_migration_errors

echo "[engineering] contract: tracing target semantics"
cargo test -p ui-components --test button_semantics button_engineering_contract_uses_consistent_tracing_targets

echo "[engineering] contract: runtime boundary leakage"
cargo test -p ui-components --test button_semantics button_engineering_contract_avoids_runtime_leaks_in_public_api

echo "[engineering] contract: button-copy tracing + runtime boundary leakage"
cargo test -p ui-components --test button_copy_semantics button_copy_engineering_contract_reuses_button_tracing_and_avoids_runtime_leaks

echo "[engineering] contract: action-button tracing + runtime boundary leakage"
cargo test -p ui-components --test action_button_semantics action_button_engineering_contract_reuses_button_tracing_and_avoids_runtime_leaks

echo "[engineering] contract: well serde/spec NA + tracing semantics + runtime boundary leakage"
# cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
# cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
# cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_avoids_runtime_leaks_in_public_api_surface
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: tabs serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test tabs_semantics tabs_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test tabs_semantics tabs_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test tabs_semantics tabs_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: tag serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: tag-group serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: swatch serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: textarea serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: time-field serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: slider serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] contract: scroll-area serde/spec NA + tracing semantics + runtime boundary leakage"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_engineering_contract_avoids_runtime_leaks_in_public_api_surface

echo "[engineering] OK"
