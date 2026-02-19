#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[inner-html] contract: button runtime paths reject raw html injection"
cargo test -p ui-layout --test button_semantics button_inner_html_is_disallowed_in_button_runtime_paths

echo "[inner-html] contract: docs inner_html stays trusted and whitelisted"
cargo test -p ui-layout --test button_semantics docs_inner_html_is_restricted_to_trusted_whitelisted_markdown_sources

echo "[inner-html] contract: tag component/docs reject raw html injection"
cargo test -p ui-layout --test tag_semantics --no-default-features --features component-tag,inject-css tag_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: tag-group component/docs reject raw html injection"
cargo test -p ui-layout --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: tabs component rejects raw html injection"
cargo test -p ui-layout --test tabs_semantics tabs_inner_html_usage_is_explicitly_na_and_guarded

echo "[inner-html] contract: swatch component/docs reject raw html injection"
cargo test -p ui-layout --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: textarea component/docs reject raw html injection"
cargo test -p ui-layout --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: time-field component/docs reject raw html injection"
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_inner_html_usage_is_forbidden_in_component_and_docs_examples

echo "[inner-html] contract: scroll-area component/docs reject raw html injection"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_inner_html_usage_is_explicitly_na_and_guarded

echo "[inner-html] contract: slider component/docs reject raw html injection"
cargo test -p ui-layout --test slider_semantics --no-default-features --features component-slider,inject-css slider_inner_html_usage_is_forbidden_in_component_and_docs

echo "[inner-html] OK"
