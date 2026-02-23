#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[platform] compile-only: default native path"
cargo check -p ui-layout

echo "[platform] compile-only: minimal native path"
cargo check -p ui-layout --no-default-features --features component-button,inject-css

echo "[platform] compile-only: action-bar native path"
cargo check -p ui-layout --no-default-features --features component-action_bar,inject-css

echo "[platform] compile-only: tag native path"
cargo check -p ui-layout --no-default-features --features component-tag,inject-css

echo "[platform] compile-only: tag-group native path"
cargo check -p ui-layout --no-default-features --features component-tag_group,inject-css

echo "[platform] compile-only: well native path"
cargo check -p ui-layout --no-default-features --features component-well,inject-css

echo "[platform] compile-only: textarea native path"
cargo check -p ui-layout --no-default-features --features component-textarea,inject-css

echo "[platform] compile-only: time-field native path"
cargo check -p ui-layout --no-default-features --features component-time_field,inject-css

echo "[platform] compile-only: scroll-area native path"
cargo check -p ui-layout --no-default-features --features component-scroll_area,inject-css

echo "[platform] compile-only: ui-motion native path"
cargo check -p ui-motion

echo "[platform] compile-only: ssr native path"
cargo check -p ui-headless --no-default-features --features ssr

echo "[platform] compile-only: web wasm path (ui-headless)"
cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web

echo "[platform] compile-only: ui-motion wasm path"
cargo check -p ui-motion --target wasm32-unknown-unknown

echo "[platform] compile-only: web wasm path"
cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css

echo "[platform] compile-only: action-bar wasm path"
cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-action_bar,inject-css

echo "[platform] compile-only: tag wasm path"
cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-tag,inject-css

echo "[platform] compile-only: tag-group wasm path"
cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-tag_group,inject-css

echo "[platform] compile-only: well wasm path"
cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-well,inject-css

echo "[platform] compile-only: textarea wasm path"
cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-textarea,inject-css

echo "[platform] compile-only: time-field wasm path"
cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-time_field,inject-css

echo "[platform] compile-only: scroll-area wasm path"
cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-scroll_area,inject-css

echo "[platform] compile guard: ui-headless web+ssr must fail"
MUTEX_LOG="$(mktemp)"
if cargo check -p ui-headless --no-default-features --features web,ssr >"$MUTEX_LOG" 2>&1; then
  echo "[platform] expected ui-headless web+ssr to fail, but command succeeded" >&2
  cat "$MUTEX_LOG" >&2
  rm -f "$MUTEX_LOG"
  exit 1
fi
if ! rg -n "mutually exclusive" "$MUTEX_LOG" >/dev/null; then
  echo "[platform] ui-headless web+ssr failed for an unexpected reason" >&2
  cat "$MUTEX_LOG" >&2
  rm -f "$MUTEX_LOG"
  exit 1
fi
rm -f "$MUTEX_LOG"

echo "[platform] ui-motion non-wasm stub tests"
cargo test -p ui-motion --test non_wasm_stub

echo "[platform] button-copy reduced-motion/ssr/wasm inheritance contract"
cargo test -p ui-layout --test button_copy_semantics button_copy_reduced_motion_ssr_wasm_branches_are_covered_via_button_contract

echo "[platform] time-field reduced-motion/ssr/wasm contract"
cargo test -p ui-layout --test time_field_semantics --no-default-features --features component-time_field time_field_reduced_motion_ssr_wasm_branches_keep_semantics_consistent

echo "[platform] tag motion contractualization (N/A runtime attach + reduced-motion + non-wasm no-op)"
cargo test -p ui-layout --test tag_semantics --no-default-features --features component-tag,inject-css tag_motion_contract_uses_ui_motion_non_wasm_stub_and_keeps_component_safe_without_motion
cargo test -p ui-layout --test tag_semantics --no-default-features --features component-tag,inject-css tag_reduced_motion_ssr_wasm_contract_is_n_a_but_semantics_stay_platform_stable

echo "[platform] tag-group motion contractualization (N/A runtime attach + reduced-motion + non-wasm no-op)"
cargo test -p ui-layout --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_motion_contract_uses_ui_motion_non_wasm_stub_and_keeps_component_safe_without_motion
cargo test -p ui-layout --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_reduced_motion_ssr_wasm_contract_is_n_a_but_semantics_stay_platform_stable

echo "[platform] scroll-area reduced-motion/ssr/wasm contract"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area scroll_area_reduced_motion_ssr_wasm_contract_is_consistent

echo "[platform] source guard: non-wasm button files must not reference web_sys"
for file in \
  crates/ui-layout/src/button/mod.rs \
  crates/ui-layout/src/button/logic.rs \
  crates/ui-layout/src/button/spec.rs \
  crates/ui-layout/src/button/styles.rs \
  crates/ui-layout/src/button/view.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm action-bar files must not reference web_sys"
for file in \
  crates/ui-layout/src/action_bar/mod.rs \
  crates/ui-layout/src/action_bar/i18n.rs \
  crates/ui-layout/src/action_bar/logic.rs \
  crates/ui-layout/src/action_bar/styles.rs \
  crates/ui-layout/src/action_bar/view.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm tag files must not reference web_sys"
for file in \
  crates/ui-layout/src/tag/mod.rs \
  crates/ui-layout/src/tag/logic.rs \
  crates/ui-layout/src/tag/styles.rs \
  crates/ui-layout/src/tag/view.rs \
  crates/ui-layout/src/tag/group/mod.rs \
  crates/ui-layout/src/tag/group/logic.rs \
  crates/ui-layout/src/tag/group/styles.rs \
  crates/ui-layout/src/tag/group/view.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm well files must not reference web_sys"
for file in \
  crates/ui-layout/src/well/mod.rs \
  crates/ui-layout/src/well/i18n.rs \
  crates/ui-layout/src/well/logic.rs \
  crates/ui-layout/src/well/styles.rs \
  crates/ui-layout/src/well/view.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm textarea files must not reference web_sys"
for file in \
  crates/ui-layout/src/text_input/textarea/mod.rs \
  crates/ui-layout/src/text_input/textarea/logic.rs \
  crates/ui-layout/src/text_input/textarea/styles.rs \
  crates/ui-layout/src/text_input/textarea/view.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm time-field files must not reference web_sys"
for file in \
  crates/ui-layout/src/text_input/time_field/mod.rs \
  crates/ui-layout/src/text_input/time_field/i18n.rs \
  crates/ui-layout/src/text_input/time_field/logic.rs \
  crates/ui-layout/src/text_input/time_field/styles.rs \
  crates/ui-layout/src/text_input/time_field/view.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm scroll-area files must not reference web_sys"
for file in \
  crates/ui-layout/src/scroll_area/mod.rs \
  crates/ui-layout/src/scroll_area/logic.rs \
  crates/ui-layout/src/scroll_area/styles.rs \
  crates/ui-layout/src/scroll_area/motion.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: button motion must keep explicit wasm/non-wasm branches"
if ! rg -n -F '#[cfg(target_arch = "wasm32")]' crates/ui-layout/src/button/motion.rs >/dev/null; then
  echo "[platform] missing wasm cfg branch in button motion" >&2
  exit 1
fi

if ! rg -n -F '#[cfg(not(target_arch = "wasm32"))]' crates/ui-layout/src/button/motion.rs >/dev/null; then
  echo "[platform] missing non-wasm cfg branch in button motion" >&2
  exit 1
fi

echo "[platform] source guard: action-bar motion must keep explicit wasm/non-wasm branches"
if ! rg -n -F '#[cfg(target_arch = "wasm32")]' crates/ui-layout/src/action_bar/motion.rs >/dev/null; then
  echo "[platform] missing wasm cfg branch in action-bar motion" >&2
  exit 1
fi

if ! rg -n -F '#[cfg(not(target_arch = "wasm32"))]' crates/ui-layout/src/action_bar/motion.rs >/dev/null; then
  echo "[platform] missing non-wasm cfg branch in action-bar motion" >&2
  exit 1
fi

echo "[platform] source guard: textarea motion must keep explicit wasm/non-wasm branches"
if ! rg -n -F '#[cfg(target_arch = "wasm32")]' crates/ui-layout/src/text_input/textarea/motion.rs >/dev/null; then
  echo "[platform] missing wasm cfg branch in textarea motion" >&2
  exit 1
fi

if ! rg -n -F '#[cfg(not(target_arch = "wasm32"))]' crates/ui-layout/src/text_input/textarea/motion.rs >/dev/null; then
  echo "[platform] missing non-wasm cfg branch in textarea motion" >&2
  exit 1
fi

echo "[platform] source guard: time-field motion must keep explicit wasm/non-wasm branches"
if ! rg -n -F '#[cfg(target_arch = "wasm32")]' crates/ui-layout/src/text_input/time_field/motion.rs >/dev/null; then
  echo "[platform] missing wasm cfg branch in time-field motion" >&2
  exit 1
fi

if ! rg -n -F '#[cfg(not(target_arch = "wasm32"))]' crates/ui-layout/src/text_input/time_field/motion.rs >/dev/null; then
  echo "[platform] missing non-wasm cfg branch in time-field motion" >&2
  exit 1
fi

echo "[platform] OK"
