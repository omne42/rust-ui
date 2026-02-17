#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[inner-html] contract: button runtime paths reject raw html injection"
cargo test -p ui-components --test button_semantics button_inner_html_is_disallowed_in_button_runtime_paths

echo "[inner-html] contract: docs inner_html stays trusted and whitelisted"
cargo test -p ui-components --test button_semantics docs_inner_html_is_restricted_to_trusted_whitelisted_markdown_sources

echo "[inner-html] OK"
