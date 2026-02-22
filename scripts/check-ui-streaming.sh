#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[streaming] contract: button remains snapshot-only (no stream markers)"
cargo test -p ui --test button_semantics button_stays_snapshot_only_and_does_not_mount_stream_contract_fields

echo "[streaming] contract: streaming/snapshot definition stays LLM-only"
cargo test -p ui --test button_semantics button_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: button snapshot baseline stays default capability"
cargo test -p ui --test button_semantics button_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test button_semantics button_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: button checklist pins streaming required/optional classification"
cargo test -p ui --test button_semantics button_check2_documents_streaming_required_optional_classification_rules

echo "[streaming] contract: button optional-streaming scope keeps role/aria/data markers continuous"
cargo test -p ui --test button_semantics button_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous

echo "[streaming] contract: button keeps validation/retry/resilience policy outside component layer"
cargo test -p ui --test button_semantics button_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

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
cargo test -p ui --test tabs_semantics tabs_stays_snapshot_only_and_does_not_mount_stream_contract_fields

echo "[streaming] contract: tabs checklist pins two-mode streaming definition + fallback"
cargo test -p ui --test tabs_semantics tabs_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: checkbox-field remains snapshot-only (no stream markers)"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_stays_snapshot_only_and_does_not_mount_stream_contract_fields

echo "[streaming] contract: checkbox-field checklist pins two-mode streaming definition + fallback"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: checkbox-field snapshot baseline stays default capability"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: checkbox-field streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: checkbox-group checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: checkbox-group snapshot baseline stays default capability"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: checkbox-group streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_check2_marks_streaming_scope_as_optional_with_snapshot_fallback

echo "[streaming] contract: checkbox checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: checkbox snapshot baseline stays default capability"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: checkbox streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_marks_streaming_scope_as_optional_with_snapshot_fallback

echo "[streaming] contract: chart checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: chart snapshot baseline stays default capability"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: chart streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: carousel remains snapshot-only for non-LLM rendering scope"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_stays_snapshot_only_and_does_not_mount_stream_contract_fields

echo "[streaming] contract: carousel checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: carousel snapshot baseline stays default capability"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: carousel streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: list checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: list snapshot baseline stays default capability"
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: list streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_marks_streaming_scope_as_optional_with_snapshot_fallback

echo "[streaming] contract: command checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui-command --lib command_check2_documents_streaming_definition_is_llm_output_only_with_two_modes
echo "[streaming] contract: command snapshot baseline stays default capability"
cargo test -p ui-command --lib command_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui-command --lib command_snapshot_baseline_consumes_complete_result_and_renders_stably
echo "[streaming] contract: command streaming required/optional classification stays scoped and explicit"
cargo test -p ui-command --lib command_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui-command --lib command_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui-command --lib command_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: error-view checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: error-view snapshot baseline stays default capability"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: error-view streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: flip-card checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui-flip-card flip_card_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: flip-card snapshot baseline stays default capability"
cargo test -p ui-flip-card flip_card_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: flip-card streaming required/optional classification stays scoped and explicit"
cargo test -p ui-flip-card flip_card_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui-flip-card flip_card_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui-flip-card flip_card_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: command-dialog checklist pins two-mode streaming definition"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_streaming_display_modes_are_limited_to_streaming_and_snapshot
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_snapshot_baseline_consumes_complete_result_and_renders_stably
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: dialog checklist pins two-mode streaming definition"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_streaming_display_modes_are_limited_to_streaming_and_snapshot
echo "[streaming] contract: dialog snapshot baseline stays default capability"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_snapshot_baseline_consumes_complete_result_and_renders_stably
echo "[streaming] contract: dialog streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: modal checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: alert-dialog checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: alert-dialog snapshot baseline stays default capability"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: alert-dialog streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: overlays checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui-overlays overlays_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: overlays snapshot baseline stays default capability"
cargo test -p ui-overlays overlays_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: overlays streaming required/optional classification stays scoped and explicit"
cargo test -p ui-overlays overlays_streaming_required_optional_classification_rules_are_scope_driven_and_boundary_safe

echo "[streaming] contract: hover-card checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: coachmark checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: coachmark snapshot baseline stays default capability"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: coachmark streaming required/optional classification stays scoped and explicit"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: hover-card snapshot baseline stays default capability"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: hover-card streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: drawer checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: drawer snapshot baseline stays default capability"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_snapshot_baseline_consumes_complete_result_and_renders_stably
echo "[streaming] contract: drawer streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: bottom-sheet checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_streaming_definition_is_llm_output_only_with_two_modes
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_streaming_display_modes_are_limited_to_streaming_and_snapshot

echo "[streaming] contract: bottom-sheet snapshot baseline stays default capability"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: bottom-sheet streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: modal snapshot baseline stays default capability"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_snapshot_baseline_consumes_complete_result_and_renders_stably
echo "[streaming] contract: modal streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: fieldset checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: fieldset snapshot baseline stays default capability"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_snapshot_baseline_consumes_complete_result_and_renders_stably
echo "[streaming] contract: fieldset streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: form-field checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: form-field snapshot baseline stays default capability"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_snapshot_as_default_baseline_capability

echo "[streaming] contract: form-field streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: circular-progress checklist pins two-mode streaming definition"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: circular-progress snapshot baseline stays default capability"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: circular-progress streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: color-editor checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: color-thumb checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: color-thumb snapshot baseline stays default capability"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: color-thumb streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: color-swatch checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: color-swatch-picker checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: color-swatch-picker snapshot baseline stays default capability"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_snapshot_baseline_consumes_complete_result_and_renders_stably
echo "[streaming] contract: color-swatch-picker streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: color-swatch snapshot baseline stays default capability"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: color-swatch streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: color-slider checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: color-wheel checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: color-wheel snapshot baseline stays default capability"
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: color-wheel streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: color-picker checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui-color-picker color_picker_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: color-picker snapshot baseline stays default capability"
cargo test -p ui-color-picker color_picker_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: color-picker streaming required/optional classification stays scoped and explicit"
cargo test -p ui-color-picker color_picker_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui-color-picker color_picker_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui-color-picker color_picker_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: color-slider snapshot baseline stays default capability"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: color-slider streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: color-editor snapshot baseline stays default capability"
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_snapshot_baseline_consumes_complete_result_and_renders_stably
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: combo-box checklist pins two-mode streaming definition"
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: drop-zone checklist pins two-mode streaming definition (LLM-only scope)"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: drop-zone snapshot baseline stays default capability"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: drop-zone streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: combo-box snapshot baseline stays default capability"
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: combo-box streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: tag keeps snapshot baseline with explicit fallback markers"
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_snapshot_baseline_and_streaming_fallback_contract_are_explicit
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_streaming_optional_scope_keeps_aria_and_data_markers_continuous
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: tag-group keeps snapshot baseline with explicit fallback markers"
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_snapshot_baseline_and_streaming_fallback_contract_are_explicit
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: swatch keeps snapshot baseline with explicit fallback markers"
cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_snapshot_baseline_and_streaming_fallback_contract_are_explicit
cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: textarea checklist pins two-mode streaming definition"
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] contract: textarea snapshot baseline stays default capability"
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: textarea streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: time-field checklist pins two-mode streaming definition + snapshot baseline"
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_streaming_definition_is_llm_output_only_with_two_modes
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_snapshot_baseline_and_streaming_fallback_contract_are_explicit
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_snapshot_baseline_consumes_complete_result_and_renders_stably
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] contract: slider checklist pins two-mode streaming definition"
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_streaming_definition_is_llm_output_only_with_two_modes

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
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_snapshot_as_default_baseline_capability
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_snapshot_baseline_consumes_complete_result_and_renders_stably

echo "[streaming] contract: slider streaming required/optional classification stays scoped and explicit"
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_streaming_required_optional_classification_rules
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous
cargo test -p ui --test slider_semantics --no-default-features --features component-slider,inject-css slider_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer

echo "[streaming] OK"
