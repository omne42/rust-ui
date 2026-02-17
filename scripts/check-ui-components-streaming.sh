#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[streaming] contract: button remains snapshot-only (no stream markers)"
cargo test -p ui-components --test button_semantics button_stays_snapshot_only_and_does_not_mount_stream_contract_fields

echo "[streaming] contract: streaming/snapshot definition stays LLM-only"
cargo test -p ui-components --test button_semantics button_streaming_definition_is_llm_output_only_with_two_modes

echo "[streaming] OK"
