#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

if [[ $# -lt 1 ]]; then
  echo "usage: $0 <app-dir> [trunk build args...]" >&2
  echo "example: $0 apps/web-demo" >&2
  exit 1
fi

APP_DIR="$1"
shift

if [[ ! -d "$ROOT_DIR/$APP_DIR" ]]; then
  echo "build-static-release: app dir not found: $APP_DIR" >&2
  exit 1
fi

if [[ ! -f "$ROOT_DIR/$APP_DIR/index.html" ]]; then
  echo "build-static-release: missing index.html in $APP_DIR" >&2
  exit 1
fi

if ! command -v trunk >/dev/null 2>&1; then
  echo "build-static-release: missing trunk; install: cargo install trunk" >&2
  exit 1
fi

cd "$ROOT_DIR/$APP_DIR"

# Trunk treats NO_COLOR as a boolean true/false. Some shells export NO_COLOR=1.
NO_COLOR=false trunk build --release "$@"

DIST_DIR="$ROOT_DIR/$APP_DIR/dist"
if [[ ! -d "$DIST_DIR" ]]; then
  echo "build-static-release: dist output missing: $DIST_DIR" >&2
  exit 1
fi

while IFS= read -r -d '' file; do
  gzip -fk -9 "$file"
done < <(find "$DIST_DIR" -type f \( -name '*.wasm' -o -name '*.js' -o -name '*.css' \) -print0)

if command -v brotli >/dev/null 2>&1; then
  while IFS= read -r -d '' file; do
    brotli -f -q 11 "$file"
  done < <(find "$DIST_DIR" -type f \( -name '*.wasm' -o -name '*.js' -o -name '*.css' \) -print0)
else
  echo "build-static-release: brotli not found, skipped .br generation" >&2
fi

cat > "$DIST_DIR/_headers" <<'HEADERS'
/index.html
  Cache-Control: no-cache

/*.html
  Cache-Control: no-cache

/*.wasm
  Cache-Control: public, max-age=31536000, immutable
  Content-Type: application/wasm

/*.js
  Cache-Control: public, max-age=31536000, immutable

/*.css
  Cache-Control: public, max-age=31536000, immutable
HEADERS

echo "build-static-release: done"
echo "  app:   $APP_DIR"
echo "  dist:  $DIST_DIR"
echo "  note:  .gz/.br sidecars generated for wasm/js/css (brotli optional)"
