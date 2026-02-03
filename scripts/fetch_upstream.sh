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
  # Prevent interactive credential prompts in unattended environments.
  GIT_TERMINAL_PROMPT=0 git clone --depth 1 --single-branch --no-tags "$url" "$dest"
}

clone_sparse_depth1() {
  local url="$1"
  local name="$2"
  shift 2
  local dest="$DEST_DIR/$name"

  if [[ -d "$dest/.git" ]]; then
    printf 'skip: %s (already cloned)\n' "$name"
    return 0
  fi

  printf 'clone (sparse): %s\n' "$name"
  GIT_TERMINAL_PROMPT=0 git clone --depth 1 --single-branch --no-tags --filter=blob:none --no-checkout "$url" "$dest"

  (
    cd "$dest"
    git sparse-checkout init --cone
    git sparse-checkout set "$@"
    git checkout
  )
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

# UI / Motion references (React ecosystem)
clone_sparse_depth1 https://github.com/motiondivision/motion.git motion packages
clone_depth1 https://github.com/heroui-inc/heroui.git heroui
clone_depth1 https://github.com/shadcn-ui/ui.git shadcn-ui
clone_depth1 https://github.com/imskyleen/animate-ui.git animate-ui
