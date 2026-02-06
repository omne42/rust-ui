#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

APP_DIR="${1:-}"
READY_SELECTOR="${2:-}"

if [[ -z "$APP_DIR" || -z "$READY_SELECTOR" ]]; then
  echo "usage: smoke-csr.sh <app-dir> <ready-selector>" >&2
  exit 2
fi

require_cmd() {
  local name="$1"
  if ! command -v "$name" >/dev/null 2>&1; then
    return 1
  fi
}

if ! require_cmd cargo; then
  echo "smoke-csr: missing cargo; install Rust (recommended: rustup)" >&2
  exit 1
fi

if ! require_cmd trunk; then
  echo "smoke-csr: missing trunk; install: cargo install trunk" >&2
  exit 1
fi

if ! require_cmd wasm-bindgen; then
  echo "smoke-csr: missing wasm-bindgen; install: cargo install wasm-bindgen-cli --version 0.2.108" >&2
  exit 1
fi

if ! require_cmd curl; then
  echo "smoke-csr: missing curl" >&2
  exit 1
fi

if ! require_cmd node || ! require_cmd npx; then
  echo "smoke-csr: missing node/npx (required for Playwright smoke)" >&2
  exit 1
fi

python_bin=""
if require_cmd python3; then
  python_bin="python3"
elif require_cmd python; then
  python_bin="python"
else
  echo "smoke-csr: missing python (needed to pick a free port)" >&2
  exit 1
fi

port="$("$python_bin" - <<'PY'
import socket

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.bind(("127.0.0.1", 0))
print(s.getsockname()[1])
s.close()
PY
)"

log_file="$(mktemp -t rust-ui-smoke-trunk.XXXXXX.log)"
png_file="$(mktemp -t rust-ui-smoke.XXXXXX.png)"

cleanup_keep_artifacts=1
cleanup() {
  if [[ -n "${TRUNK_PID:-}" ]]; then
    kill "${TRUNK_PID}" >/dev/null 2>&1 || true
  fi
  if [[ "$cleanup_keep_artifacts" == "0" ]]; then
    rm -f "$log_file" "$png_file" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

# trunk treats NO_COLOR as a boolean env var (true/false). Some environments set
# NO_COLOR=1, which breaks trunk argument parsing.
unset NO_COLOR

# WASM CSR builds can hit Tachys' attribute tuple limits without this cfg.
# This is also set in `.cargo/config.toml`, but keeping it here makes `trunk`
# invocations more robust across shells/CI.
if [[ "${RUSTFLAGS:-}" != *"--cfg erase_components"* ]]; then
  export RUSTFLAGS="--cfg erase_components ${RUSTFLAGS:-}"
fi

(
  cd "$ROOT_DIR/$APP_DIR"
  exec trunk serve --address 127.0.0.1 --port "$port" --open false
) >"$log_file" 2>&1 &
TRUNK_PID="$!"

url="http://127.0.0.1:${port}/"

for _ in {1..80}; do
  if curl -fsS "$url" >/dev/null 2>&1; then
    break
  fi
  sleep 0.25
  if ! kill -0 "$TRUNK_PID" >/dev/null 2>&1; then
    echo "smoke-csr: trunk exited early (log: $log_file)" >&2
    tail -n 200 "$log_file" >&2 || true
    exit 1
  fi
done

if ! npx --yes playwright screenshot \
  --block-service-workers \
  --wait-for-selector "$READY_SELECTOR" \
  --timeout 30000 \
  "$url" \
  "$png_file"; then
  echo "smoke-csr: playwright failed (log: $log_file, screenshot: $png_file)" >&2
  exit 1
fi

cleanup_keep_artifacts=0
echo "smoke-csr: ok ($APP_DIR)"
