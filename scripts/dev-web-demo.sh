#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

# Ensure rustup-installed tools (trunk/wasm-bindgen) are discoverable even if the
# caller shell doesn't source ~/.cargo/env.
if [[ -d "$HOME/.cargo/bin" ]]; then
  export PATH="$HOME/.cargo/bin:$PATH"
fi

# trunk treats NO_COLOR as a boolean env var (true/false). Some environments set
# NO_COLOR=1, which breaks trunk argument parsing.
unset NO_COLOR

require_cmd() {
  local name="$1"
  if ! command -v "$name" >/dev/null 2>&1; then
    return 1
  fi
}

if ! require_cmd cargo; then
  echo "dev-web-demo: missing cargo; install Rust (recommended: rustup)" >&2
  exit 1
fi

if ! require_cmd trunk; then
  echo "dev-web-demo: missing trunk; install: cargo install trunk" >&2
  exit 1
fi

if ! require_cmd wasm-bindgen; then
  echo "dev-web-demo: missing wasm-bindgen; install: cargo install wasm-bindgen-cli --version 0.2.108" >&2
  exit 1
fi

if require_cmd rustup; then
  if ! rustup target list --installed | grep -qx "wasm32-unknown-unknown"; then
    echo "dev-web-demo: missing wasm32 target; install: rustup target add wasm32-unknown-unknown" >&2
    exit 1
  fi
else
  sysroot="$(rustc --print sysroot)"
  if [[ ! -d "$sysroot/lib/rustlib/wasm32-unknown-unknown" ]]; then
    cat >&2 <<'EOF'
dev-web-demo: wasm32-unknown-unknown target not installed.

Recommended:
  1) install rustup
  2) rustup target add wasm32-unknown-unknown
EOF
    exit 1
  fi
fi

cd "$ROOT_DIR/apps/web-demo"
exec trunk serve --open true "$@"
