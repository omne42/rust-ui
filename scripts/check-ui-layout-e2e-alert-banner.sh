#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/codex-alert-banner-target}"

echo "[e2e-alert-banner] contract: semantic selectors + stable wait"
cargo test -p ui-layout --test alert_banner_semantics --no-default-features --features component-alert_banner,inject-css alert_banner_e2e_contract_uses_semantic_selectors_and_stable_waits

echo "[e2e-alert-banner] OK"
