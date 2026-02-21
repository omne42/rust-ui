#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[e2e-list] contract: checklist e2e-selector/stable-wait governance"
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_documents_e2e_selector_and_stable_wait_rules

echo "[e2e-list] contract: semantic selectors + settled waits"
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_e2e_selector_contract_uses_semantic_markers_and_settled_waits

echo "[e2e-list] contract: ready/settled semantic breakpoints for list paths"
cargo test -p ui-components --test list_module_semantics --no-default-features --features component-list,inject-css list_e2e_contract_covers_ready_and_settled_conditions_for_list_paths

echo "[e2e-list] OK"
