#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[streaming] contract: button remains snapshot-only (no stream markers)"
cargo test -p ui-layout --test button_semantics button_stays_snapshot_only_and_does_not_mount_stream_contract_fields

echo "[streaming] contract: streaming/snapshot definition stays LLM-only"
cargo test -p ui-layout --test button_semantics button_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: well stays snapshot-compatible without streaming protocol fields"
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_streaming_semantics_are_not_required_for_snapshot_container_scope

echo "[streaming] contract: well checklist pins two-mode streaming definition"
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: well checklist pins snapshot as baseline capability"
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_check2_documents_snapshot_as_default_baseline_capability

echo "[streaming] contract: well consumes complete snapshot result stably"
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: well checklist pins streaming required/optional classification"
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_check2_documents_streaming_required_optional_classification_rules

echo "[streaming] contract: well optional streaming scope keeps role/aria/data markers continuous"
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous

echo "[streaming] contract: well keeps validation/retry/resilience policy outside component layer"
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: tabs remains snapshot-only (no stream markers)"
cargo test -p ui-layout --test tabs_semantics tabs_stays_snapshot_only_and_does_not_mount_stream_contract_fields

echo "[streaming] contract: tabs checklist pins two-mode streaming definition + fallback"
cargo test -p ui-layout --test tabs_semantics tabs_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: tag keeps snapshot baseline with explicit fallback markers"
cargo test -p ui-layout --test tag_semantics --no-default-features --features component-tag,inject-css tag_snapshot_baseline_and_streaming_fallback_contract_are_explicit
cargo test -p ui-layout --test tag_semantics --no-default-features --features component-tag,inject-css tag_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui-layout --test tag_semantics --no-default-features --features component-tag,inject-css tag_streaming_optional_scope_keeps_aria_and_data_markers_continuous
cargo test -p ui-layout --test tag_semantics --no-default-features --features component-tag,inject-css tag_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: tag-group keeps snapshot baseline with explicit fallback markers"
cargo test -p ui-layout --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_snapshot_baseline_and_streaming_fallback_contract_are_explicit
cargo test -p ui-layout --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui-layout --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui-layout --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: swatch keeps snapshot baseline with explicit fallback markers"
cargo test -p ui-layout --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_snapshot_baseline_and_streaming_fallback_contract_are_explicit
cargo test -p ui-layout --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui-layout --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui-layout --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: textarea checklist pins two-mode streaming definition"
cargo test -p ui-layout --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: textarea snapshot baseline stays default capability"
cargo test -p ui-layout --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui-layout --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: textarea streaming required/optional classification stays scoped and explicit"
cargo test -p ui-layout --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui-layout --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui-layout --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: time-field checklist pins two-mode streaming definition + snapshot baseline"
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_streaming_definition_is_llm_output_only_with_two_modes
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_snapshot_baseline_and_streaming_fallback_contract_are_explicit
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_snapshot_baseline_consumes_complete_result_and_renders_stably
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: slider checklist pins two-mode streaming definition"
cargo test -p ui-layout --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: scroll-area checklist pins two-mode streaming definition"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: scroll-area snapshot baseline stays default capability"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: scroll-area streaming required/optional classification stays scoped and explicit"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: slider snapshot baseline stays default capability"
cargo test -p ui-layout --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui-layout --test slider_semantics --no-default-features --features component-slider,inject-css slider_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: slider streaming required/optional classification stays scoped and explicit"
cargo test -p ui-layout --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui-layout --test slider_semantics --no-default-features --features component-slider,inject-css slider_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui-layout --test slider_semantics --no-default-features --features component-slider,inject-css slider_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] OK"
