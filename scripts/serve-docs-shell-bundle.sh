#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

"$ROOT_DIR/scripts/build-docs-shell-bundle.sh"

cd "$ROOT_DIR/target/docs-shell-bundle/dist"
exec python3 -m http.server 8081 --bind 127.0.0.1
