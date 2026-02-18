#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[component-files] contract: required file layout"
cargo test -p ui-components --test button_semantics button_component_directory_has_standard_file_layout

echo "[component-files] contract: mod.rs minimal stable exports"
cargo test -p ui-components --test button_semantics button_mod_rs_keeps_minimal_stable_exports

echo "[component-files] contract: logic/styles/view/motion/spec responsibilities"
cargo test -p ui-components --test button_semantics button_component_file_responsibilities_remain_scoped

echo "[component-files] contract: well required file layout + export boundary + scoped responsibilities"
cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_component_directory_has_standard_file_layout
cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_mod_rs_keeps_minimal_stable_exports
cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_component_file_responsibilities_remain_scoped

echo "[component-files] contract: tabs required file layout + export boundary + scoped responsibilities"
cargo test -p ui-components --test tabs_semantics tabs_component_directory_has_standard_file_layout
cargo test -p ui-components --test tabs_semantics tabs_mod_rs_keeps_minimal_stable_exports
cargo test -p ui-components --test tabs_semantics tabs_component_file_responsibilities_remain_scoped

echo "[component-files] contract: tag required file layout + export boundary + scoped responsibilities"
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_component_directory_has_standard_file_layout
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_mod_rs_keeps_minimal_stable_exports
cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_component_file_responsibilities_remain_scoped

echo "[component-files] contract: tag-group required file layout + export boundary + scoped responsibilities"
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_component_directory_has_standard_file_layout
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_mod_rs_keeps_minimal_stable_exports
cargo test -p ui-components --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_component_file_responsibilities_remain_scoped

echo "[component-files] contract: swatch required file layout + export boundary + scoped responsibilities"
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_component_directory_has_standard_file_layout
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_mod_rs_keeps_minimal_stable_exports
cargo test -p ui-components --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_component_file_responsibilities_remain_scoped

echo "[component-files] contract: textarea required file layout + export boundary + scoped responsibilities"
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_component_directory_has_standard_file_layout
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_mod_rs_keeps_minimal_stable_exports
cargo test -p ui-components --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_component_file_responsibilities_remain_scoped

echo "[component-files] contract: time-field required file layout + export boundary + scoped responsibilities"
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_component_directory_has_standard_file_layout
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_mod_rs_keeps_minimal_stable_exports
cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_component_file_responsibilities_remain_scoped

echo "[component-files] contract: slider required file layout + export boundary + scoped responsibilities"
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_component_directory_has_standard_file_layout_and_no_spec_file
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_mod_rs_keeps_minimal_stable_exports
cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_component_file_responsibilities_remain_scoped

echo "[component-files] contract: scroll-area required file layout + export boundary + scoped responsibilities"
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_component_directory_has_standard_file_layout
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_mod_rs_keeps_minimal_stable_exports
cargo test -p ui-components --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_component_file_responsibilities_remain_scoped

echo "[component-files] OK"
