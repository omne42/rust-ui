#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[contract-hygiene] contract: no temporary patch markers in button paths"
cargo test -p ui-components --test button_semantics button_contract_consistency_has_no_temporary_patch_markers

echo "[contract-hygiene] contract: no unwrap/expect in non-test button+accordion code"
cargo test -p ui-components --test button_accordion_hygiene button_and_accordion_non_test_code_forbids_unwrap_and_expect

echo "[contract-hygiene] contract: no side-effect result swallowing in button+accordion code"
cargo test -p ui-components --test button_accordion_hygiene button_and_accordion_non_test_code_forbids_let_result_swallowing

echo "[contract-hygiene] OK"
