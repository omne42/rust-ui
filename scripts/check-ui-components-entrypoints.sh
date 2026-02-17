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

echo "[entrypoints] OK"
