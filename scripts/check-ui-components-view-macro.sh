#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[view-macro] contract: button view split"
cargo test -p ui-components --test button_semantics button_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] contract: button function-first split"
cargo test -p ui-components --test button_semantics button_view_functional_split_prefers_plain_functions_over_local_components

echo "[view-macro] contract: button static fragment constantization"
cargo test -p ui-components --test button_semantics button_static_fragments_are_constantized_with_stable_a11y_semantics

echo "[view-macro] contract: share-button view macro split"
cargo test -p ui-components --test share_button_semantics share_button_view_macro_complexity_is_split_into_semantic_subrenders

echo "[view-macro] OK"
