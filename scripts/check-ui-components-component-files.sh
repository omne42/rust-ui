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

echo "[component-files] OK"
