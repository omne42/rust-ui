#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[inner-html] contract: button runtime paths reject raw html injection"
cargo test -p ui-components --test button_semantics button_inner_html_is_disallowed_in_button_runtime_paths

echo "[inner-html] contract: checkbox component/docs reject raw html injection"
cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_inner_html_usage_is_forbidden_in_component_and_docs_examples_locally

echo "[inner-html] contract: docs inner_html stays trusted and whitelisted"
cargo test -p ui-components --test button_semantics docs_inner_html_is_restricted_to_trusted_whitelisted_markdown_sources

echo "[inner-html] contract: tag component/docs reject raw html injection"
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: tag-group component/docs reject raw html injection"
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: tabs component rejects raw html injection"
cargo test -p ui-components --test tabs_semantics tabs_inner_html_usage_is_explicitly_na_and_guarded

echo "[inner-html] contract: swatch component/docs reject raw html injection"
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: textarea component/docs reject raw html injection"
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: circular-progress component/docs reject raw html injection"
cargo test -p ui-components --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: time-field component/docs reject raw html injection"
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: scroll-area component/docs reject raw html injection"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_inner_html_usage_is_explicitly_na_and_guarded

echo "[inner-html] contract: slider component/docs reject raw html injection"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_inner_html_usage_is_forbidden_in_component_and_docs

echo "[inner-html] contract: alert-dialog component/docs reject raw html injection"
cargo test -p ui-components --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: dialog component/docs inner_html is explicitly N/A and guarded"
cargo test -p ui-components --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_inner_html_usage_is_explicitly_na_and_guarded

echo "[inner-html] contract: chart component/docs inner_html is explicitly N/A and guarded"
cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css chart_inner_html_usage_is_explicitly_na_and_guarded

echo "[inner-html] contract: carousel component/docs inner_html is explicitly N/A and guarded"
cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_inner_html_usage_is_explicitly_na_and_guarded

echo "[inner-html] contract: collapsible component/docs reject raw html injection"
cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: autocomplete component/docs reject raw html injection"
cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: combo-box component/docs reject raw html injection"
cargo test -p ui-components --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: flip-card component/docs inner_html surface is absent and guarded"
cargo test -p ui-flip-card flip_card_inner_html_usage_is_absent_and_untrusted_injection_paths_are_blocked

echo "[inner-html] contract: modal component/docs reject raw html injection"
cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: overlays family/docs reject raw html injection and keep docs-shell whitelist"
cargo test -p ui-overlays overlays_inner_html_usage_is_forbidden_and_docs_shell_path_is_whitelisted

echo "[inner-html] contract: drawer component/docs reject raw html injection"
cargo test -p ui-components --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: error-view component/docs reject raw html injection"
cargo test -p ui-components --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: fieldset component/docs reject raw html injection"
cargo test -p ui-components --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: color-editor component/docs reject raw html injection"
cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: color-swatch component/docs reject raw html injection"
cargo test -p ui-components --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: color-thumb component/docs reject raw html injection"
cargo test -p ui-components --test color_thumb_semantics --no-default-features --features component-color_thumb,inject-css color_thumb_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: color-swatch-picker component/docs reject raw html injection"
cargo test -p ui-components --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: color-picker component/docs reject raw html injection"
cargo test -p ui-color-picker color_picker_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: color-slider component/docs reject raw html injection"
cargo test -p ui-color-slider color_slider_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: form-field component/docs inner_html surface is absent and guarded"
cargo test -p ui-components --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: hover-card component/docs inner_html is explicitly N/A and guarded"
cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_inner_html_usage_is_explicitly_na_and_guarded

echo "[inner-html] contract: list component/docs inner_html is explicitly N/A and guarded"
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_inner_html_usage_is_explicitly_na_and_guarded

echo "[inner-html] contract: menu component/docs reject raw html injection and keep docs-shell whitelist boundary"
cargo test -p ui-menu menu_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] OK"
