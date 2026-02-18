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
cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present

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

echo "[entrypoints] contract: scroll-area fixed entry files and forbidden file guards"
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_ui_components_fixed_entry_files_follow_layered_boundaries

echo "[entrypoints] OK"
