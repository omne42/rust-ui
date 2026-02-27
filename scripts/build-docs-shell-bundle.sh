#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

if [[ -d "$HOME/.cargo/bin" ]]; then
  export PATH="$HOME/.cargo/bin:$PATH"
fi

unset NO_COLOR

if ! command -v trunk >/dev/null 2>&1; then
  echo "build-docs-shell-bundle: missing trunk; install: cargo install trunk" >&2
  exit 1
fi

if [[ "${RUSTFLAGS:-}" != *"--cfg erase_components"* ]]; then
  export RUSTFLAGS="--cfg erase_components ${RUSTFLAGS:-}"
fi

build_app() {
  local app="$1"
  local public_url="${2:-}"
  echo "[build] $app"
  (
    cd "$ROOT_DIR/apps/$app"
    if [[ -n "$public_url" ]]; then
      trunk build --release --public-url "$public_url"
    else
      trunk build --release
    fi
  )
}

build_app "docs-shell"
build_app "docs-pack-actions" "./"
build_app "docs-pack-forms" "./"
build_app "docs-pack-collections" "./"
build_app "docs-pack-layout" "./"

BUNDLE_DIR="$ROOT_DIR/target/docs-shell-bundle/dist"
rm -rf "$BUNDLE_DIR"
mkdir -p "$BUNDLE_DIR"

cp -a "$ROOT_DIR/apps/docs-shell/dist/." "$BUNDLE_DIR/"

mkdir -p "$BUNDLE_DIR/packs/actions"
cp -a "$ROOT_DIR/apps/docs-pack-actions/dist/." "$BUNDLE_DIR/packs/actions/"

mkdir -p "$BUNDLE_DIR/packs/forms"
cp -a "$ROOT_DIR/apps/docs-pack-forms/dist/." "$BUNDLE_DIR/packs/forms/"

mkdir -p "$BUNDLE_DIR/packs/collections"
cp -a "$ROOT_DIR/apps/docs-pack-collections/dist/." "$BUNDLE_DIR/packs/collections/"

mkdir -p "$BUNDLE_DIR/packs/layout"
cp -a "$ROOT_DIR/apps/docs-pack-layout/dist/." "$BUNDLE_DIR/packs/layout/"

echo

echo "[bundle] $BUNDLE_DIR"

echo "[sizes]"
ls -lh "$BUNDLE_DIR"/*_bg.wasm "$BUNDLE_DIR"/packs/*/*_bg.wasm
