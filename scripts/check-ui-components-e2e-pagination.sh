#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/codex-pagination-target}"

echo "[e2e-pagination] contract: semantic selectors + stable wait"
cargo test -p ui-components --test pagination_semantics --no-default-features --features component-pagination,inject-css pagination_e2e_contract_uses_semantic_selectors_and_stable_waits

echo "[e2e-pagination] OK"
