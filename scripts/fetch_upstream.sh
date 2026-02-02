#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
DEST_DIR="$ROOT_DIR/examples/_upstream"

mkdir -p "$DEST_DIR"
export GIT_LFS_SKIP_SMUDGE=1

clone_depth1() {
  local url="$1"
  local name="$2"
  local dest="$DEST_DIR/$name"

  if [[ -d "$dest/.git" ]]; then
    printf 'skip: %s (already cloned)\n' "$name"
    return 0
  fi

  printf 'clone: %s\n' "$name"
  git clone --depth 1 "$url" "$dest"
}

clone_depth1 https://github.com/adobe/react-spectrum.git adobe-react-spectrum
clone_depth1 https://github.com/adobe/spectrum-css.git adobe-spectrum-css
clone_depth1 https://github.com/adobe/spectrum-web-components.git adobe-spectrum-web-components

clone_depth1 https://github.com/facebook/react.git facebook-react
clone_depth1 https://github.com/vuejs/core.git vue-core

clone_depth1 https://github.com/leptos-rs/leptos.git leptos
clone_depth1 https://github.com/tauri-apps/tauri.git tauri
clone_depth1 https://github.com/Synphonyte/leptos-use.git leptos-use

clone_depth1 https://github.com/w3c/aria-practices.git w3c-aria-practices

