#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

APP_DIR="${1:-}"
APP_KIND="${2:-}"

if [[ -z "$APP_DIR" || -z "$APP_KIND" ]]; then
  cat >&2 <<'USAGE'
usage: scripts/e2e-csr.sh <app-dir> <app-kind>

  app-kind:
    docs-app   run the docs-app Playwright suite (recommended)
USAGE
  exit 2
fi

require_cmd() {
  local name="$1"
  command -v "$name" >/dev/null 2>&1
}

if ! require_cmd cargo; then
  echo "e2e-csr: missing cargo; install Rust (recommended: rustup)" >&2
  exit 1
fi

if ! require_cmd trunk; then
  echo "e2e-csr: missing trunk; install: cargo install trunk" >&2
  exit 1
fi

if ! require_cmd wasm-bindgen; then
  echo "e2e-csr: missing wasm-bindgen; install: cargo install wasm-bindgen-cli --version 0.2.108" >&2
  exit 1
fi

if ! require_cmd curl; then
  echo "e2e-csr: missing curl" >&2
  exit 1
fi

if ! require_cmd node || ! require_cmd npm || ! require_cmd npx; then
  echo "e2e-csr: missing node/npm/npx (required for Playwright E2E)" >&2
  exit 1
fi

python_bin=""
if require_cmd python3; then
  python_bin="python3"
elif require_cmd python; then
  python_bin="python"
else
  echo "e2e-csr: missing python (needed to pick a free port)" >&2
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

log_file="$(mktemp -t rust-ui-e2e-trunk.XXXXXX.log)"

cleanup_keep_artifacts=1
cleanup() {
  if [[ -n "${TRUNK_PID:-}" ]]; then
    kill "${TRUNK_PID}" >/dev/null 2>&1 || true
  fi
  if [[ "$cleanup_keep_artifacts" == "0" ]]; then
    rm -f "$log_file" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

unset NO_COLOR

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
    echo "e2e-csr: trunk exited early (log: $log_file)" >&2
    tail -n 200 "$log_file" >&2 || true
    exit 1
  fi
done

case "$APP_KIND" in
  docs-app)
    (
      cd "$ROOT_DIR/e2e"
      if [[ "${E2E_SKIP_NPM_INSTALL:-}" != "1" ]]; then
        npm install --no-fund --no-audit
      fi
      if [[ "${E2E_SKIP_PLAYWRIGHT_INSTALL:-}" != "1" ]]; then
        npx playwright install chromium
      fi
      E2E_BASE_URL="$url" npx playwright test --config playwright.config.mjs
    )
    ;;
  *)
    echo "e2e-csr: unknown app-kind: $APP_KIND" >&2
    exit 2
    ;;
esac

cleanup_keep_artifacts=0
echo "e2e-csr: ok ($APP_DIR)"

