#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[entrypoints] contract: lib.rs public surface + feature gating"
cargo test -p ui-components --test button_semantics ui_components_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks

echo "[entrypoints] contract: css registry feature-gated aggregation"
cargo test -p ui-components --test button_semantics ui_components_css_registry_remains_feature_gated_and_non_global

echo "[entrypoints] contract: UiRoot centralized theme + i18n"
cargo test -p ui-components --test button_semantics ui_root_centralizes_theme_injection_and_i18n_context

echo "[entrypoints] contract: active_highlight shared primitive boundary"
cargo test -p ui-components --test button_semantics active_highlight_stays_shared_motion_primitive_without_component_semantics

echo "[entrypoints] contract: forbidden entrypoint files absent / headless canonical files present"
cargo test -p ui-components --test button_semantics ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present

echo "[entrypoints] contract: well entrypoint boundaries and forbidden file guards"
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_ui_layout_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present

echo "[entrypoints] contract: tabs entrypoint boundaries and forbidden file guards"
cargo test -p ui-components --test tabs_semantics tabs_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present

echo "[entrypoints] contract: tag entrypoint boundaries and forbidden file guards"
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present

echo "[entrypoints] contract: tag-group entrypoint boundaries and forbidden file guards"
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present

echo "[entrypoints] contract: swatch entrypoint boundaries and forbidden file guards"
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present

echo "[entrypoints] contract: textarea entrypoint boundaries and forbidden file guards"
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present

echo "[entrypoints] contract: time-field entrypoint boundaries and forbidden file guards"
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present

echo "[entrypoints] contract: command-dialog fixed entry files and forbidden file guards"
cargo test -p ui-components --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: dialog fixed entry files and forbidden file guards"
cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: error-view fixed entry files and forbidden file guards"
cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: combo-box fixed entry files and forbidden file guards"
cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: drop-zone fixed entry files and forbidden file guards"
cargo test -p ui-components --test drop_zone_semantics --no-default-features --features component-drop_zone,inject-css drop_zone_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: modal fixed entry files and forbidden file guards"
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: drawer fixed entry files and forbidden file guards"
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: checkbox-field fixed entry files and forbidden file guards"
cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: checkbox-group fixed entry files and forbidden file guards"
cargo test -p ui-components --test checkbox_group_semantics --no-default-features --features component-checkbox_group,inject-css checkbox_group_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: form-field fixed entry files and forbidden file guards"
cargo test -p ui-components --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: date-input-group fixed entry files and forbidden file guards"
cargo test -p ui-date-input-group date_input_group_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: checkbox fixed entry files and forbidden file guards"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_ui_components_fixed_entry_files_follow_layered_boundaries_locally

echo "[entrypoints] contract: alert-dialog fixed entry files and forbidden file guards"
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: list fixed entry files and forbidden file guards"
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: menu fixed entry files and forbidden file guards"
cargo test -p ui-menu menu_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: command fixed entry files and forbidden file guards"
cargo test -p ui-command --lib command_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: color-editor fixed entry files and forbidden file guards"
cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: color-slider fixed entry files and forbidden file guards"
cargo test -p ui-components --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: color-thumb fixed entry files and forbidden file guards"
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: color-swatch fixed entry files and forbidden file guards"
cargo test -p ui-components --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: color-swatch-picker fixed entry files and forbidden file guards"
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: color-picker fixed entry files and forbidden file guards"
cargo test -p ui-color-picker color_picker_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: flip-card fixed entry files and forbidden file guards"
cargo test -p ui-flip-card flip_card_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: fieldset fixed entry files and forbidden file guards"
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: hover-card fixed entry files and forbidden file guards"
cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: slider lib.rs public surface + feature gating"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_ui_components_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks

echo "[entrypoints] contract: slider css registry feature-gated aggregation"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_ui_components_css_registry_remains_feature_gated_and_non_global

echo "[entrypoints] contract: slider UiRoot centralized theme + i18n"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_ui_root_centralizes_theme_injection_and_i18n_context

echo "[entrypoints] contract: slider active_highlight shared primitive boundary"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_active_highlight_stays_shared_motion_primitive_without_component_semantics

echo "[entrypoints] contract: slider forbidden entrypoint files absent / headless canonical files present"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present

echo "[entrypoints] contract: circular-progress fixed entry files and forbidden file guards"
cargo test -p ui-components --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: coachmark fixed entry files and forbidden file guards"
cargo test -p ui-components --lib --no-default-features --features component-coachmark,inject-css coachmark_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: chart fixed entry files and forbidden file guards"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: carousel fixed entry files and forbidden file guards"
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: scroll-area fixed entry files and forbidden file guards"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_ui_layout_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] contract: overlays fixed entry files and forbidden file guards"
cargo test -p ui-overlays overlays_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] OK"
