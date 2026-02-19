#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[view-macro] contract: button view split"
cargo test -p ui-layout --test button_semantics button_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: button function-first split"
cargo test -p ui-layout --test button_semantics button_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: button static fragment constantization"
cargo test -p ui-layout --test button_semantics button_static_fragments_are_constantized_with_stable_a11y_semantics

echo "[view-macro] contract: share-button view macro split"
cargo test -p ui-layout --test share_button_semantics share_button_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: tag view macro split"
cargo test -p ui-layout --test tag_semantics --no-default-features --features component-tag,inject-css tag_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: tag function-first split"
cargo test -p ui-layout --test tag_semantics --no-default-features --features component-tag,inject-css tag_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: tag static fragment constantization"
cargo test -p ui-layout --test tag_semantics --no-default-features --features component-tag,inject-css tag_static_fragments_are_constantized_with_stable_semantics

echo "[view-macro] contract: tag-group view macro split"
cargo test -p ui-layout --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: tag-group function-first split"
cargo test -p ui-layout --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: tag-group static fragment constantization"
cargo test -p ui-layout --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_static_fragments_are_constantized_with_stable_semantics

echo "[view-macro] contract: tabs view macro split"
cargo test -p ui-layout --test tabs_semantics tabs_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: tabs function-first split"
cargo test -p ui-layout --test tabs_semantics tabs_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: tabs static fragment constantization"
cargo test -p ui-layout --test tabs_semantics tabs_static_fragments_are_constantized_with_stable_semantics

echo "[view-macro] contract: well view macro complexity"
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders

echo "[view-macro] contract: well function-first simple split"
cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_view_functional_split_prefers_no_extra_local_components_for_simple_layout

echo "[view-macro] contract: swatch view macro complexity"
cargo test -p ui-layout --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders

echo "[view-macro] contract: swatch function-first simple split"
cargo test -p ui-layout --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_view_functional_split_prefers_no_extra_local_components_for_simple_layout

echo "[view-macro] contract: swatch static fragment scope"
cargo test -p ui-layout --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_static_fragments_are_constantized_or_absent_for_simple_indicator_layout

echo "[view-macro] contract: swatch functional split keeps stable semantic markers"
cargo test -p ui-layout --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_functional_split_keeps_semantic_markers_stable_for_test_selectors

echo "[view-macro] contract: textarea view macro complexity"
cargo test -p ui-layout --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_view_macro_complexity_is_bounded_with_semantic_subblocks

echo "[view-macro] contract: textarea function-first split"
cargo test -p ui-layout --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: textarea static fragment constantization"
cargo test -p ui-layout --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_static_fragments_are_constantized_or_absent_for_simple_input_layout

echo "[view-macro] contract: time-field view macro split"
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: time-field function-first split"
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: time-field static fragment constantization"
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_static_fragments_are_constantized_with_stable_semantics

echo "[view-macro] contract: scroll-area view macro complexity"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders

echo "[view-macro] contract: scroll-area function-first simple split"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_view_functional_split_prefers_no_extra_local_components_for_simple_layout

echo "[view-macro] contract: scroll-area static fragment scope"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_static_fragments_are_constantized_or_absent_for_simple_layout

echo "[view-macro] contract: slider view macro split"
cargo test -p ui-layout --test slider_semantics --no-default-features --features component-slider,inject-css slider_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: slider function-first split"
cargo test -p ui-layout --test slider_semantics --no-default-features --features component-slider,inject-css slider_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: slider static fragment constantization"
cargo test -p ui-layout --test slider_semantics --no-default-features --features component-slider,inject-css slider_static_fragments_are_constantized_with_stable_semantics

echo "[view-macro] OK"
