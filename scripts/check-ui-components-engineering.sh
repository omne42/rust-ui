#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[engineering] contract: serde schema + structured migration errors"
cargo test -p ui-components --test button_semantics button_engineering_contract_uses_serde_schema_and_structured_migration_errors

echo "[engineering] contract: tracing target semantics"
cargo test -p ui-components --test button_semantics button_engineering_contract_uses_consistent_tracing_targets

echo "[engineering] contract: runtime boundary leakage"
cargo test -p ui-components --test button_semantics button_engineering_contract_avoids_runtime_leaks_in_public_api

echo "[engineering] OK"
