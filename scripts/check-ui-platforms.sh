#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[platform] compile-only: default native path"
cargo check -p ui

echo "[platform] compile-only: minimal native path"
cargo check -p ui --no-default-features --features component-button,inject-css

echo "[platform] compile-only: breadcrumb native path"
cargo check -p ui --no-default-features --features component-breadcrumb,inject-css

echo "[platform] compile-only: action-bar native path"
cargo check -p ui --no-default-features --features component-action_bar,inject-css

echo "[platform] compile-only: alert-dialog native path"
cargo check -p ui --no-default-features --features component-alert_dialog,inject-css

echo "[platform] compile-only: coachmark native path"
cargo check -p ui --no-default-features --features component-coachmark,inject-css

echo "[platform] compile-only: tag native path"
cargo check -p ui --no-default-features --features component-tag,inject-css

echo "[platform] compile-only: tag-group native path"
cargo check -p ui --no-default-features --features component-tag_group,inject-css

echo "[platform] compile-only: circular-progress native path"
cargo check -p ui --no-default-features --features component-circular_progress,inject-css

echo "[platform] compile-only: avatar-group native path"
cargo check -p ui --no-default-features --features component-avatar_group,inject-css

echo "[platform] compile-only: chart native path"
cargo check -p ui --no-default-features --features component-chart,inject-css

echo "[platform] compile-only: drawer native path"
cargo check -p ui --no-default-features --features component-drawer,inject-css

echo "[platform] compile-only: autocomplete native path"
cargo check -p ui --no-default-features --features component-autocomplete,inject-css

echo "[platform] compile-only: hover-card native path"
cargo check -p ui --no-default-features --features component-hover_card,inject-css

echo "[platform] compile-only: error-view native path"
cargo check -p ui --no-default-features --features component-error_view,inject-css

echo "[platform] compile-only: fieldset native path"
cargo check -p ui --no-default-features --features component-fieldset,inject-css

echo "[platform] compile-only: well native path"
cargo check -p ui-layout --no-default-features --features component-well,inject-css

echo "[platform] compile-only: textarea native path"
cargo check -p ui --no-default-features --features component-textarea,inject-css

echo "[platform] compile-only: time-field native path"
cargo check -p ui --no-default-features --features component-time_field,inject-css

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
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css

echo "[platform] compile-only: breadcrumb wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-breadcrumb,inject-css

echo "[platform] compile-only: action-bar wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-action_bar,inject-css

echo "[platform] compile-only: alert-dialog wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-alert_dialog,inject-css

echo "[platform] compile-only: coachmark wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-coachmark,inject-css

echo "[platform] compile-only: tag wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-tag,inject-css

echo "[platform] compile-only: tag-group wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-tag_group,inject-css

echo "[platform] compile-only: circular-progress wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-circular_progress,inject-css

echo "[platform] compile-only: avatar-group wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-avatar_group,inject-css

echo "[platform] compile-only: chart wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-chart,inject-css

echo "[platform] compile-only: drawer wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-drawer,inject-css

echo "[platform] compile-only: autocomplete wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-autocomplete,inject-css

echo "[platform] compile-only: hover-card wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-hover_card,inject-css

echo "[platform] compile-only: error-view wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-error_view,inject-css

echo "[platform] compile-only: fieldset wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-fieldset,inject-css

echo "[platform] compile-only: well wasm path"
cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-well,inject-css

echo "[platform] compile-only: textarea wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-textarea,inject-css

echo "[platform] compile-only: time-field wasm path"
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-time_field,inject-css

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

echo "[platform] ui-motion reduced-motion spring contract"
cargo test -p ui-motion --test spring

echo "[platform] overlays motion contractualization (component contract + reduced-motion + non-wasm no-op)"
cargo test -p ui-overlays overlays_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] circular-progress reduced-motion/ssr/wasm contract"
cargo test -p ui --test circular_progress_semantics circular_progress_reduced_motion_ssr_wasm_branches_keep_semantics_consistent

echo "[platform] circular-progress motion contractualization (N/A runtime attach + no-op/reduced guards)"
cargo test -p ui --test circular_progress_semantics circular_progress_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards

echo "[platform] form-field motion contractualization (N/A runtime attach + reduced-motion + non-wasm no-op)"
cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards

echo "[platform] date-input-group motion contractualization (spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui-date-input-group date_input_group_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] bottom-sheet motion contractualization (spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] alert-dialog motion contractualization (overlay attach + reduced-motion + non-wasm no-op)"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] command-dialog motion contractualization (component contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] dialog motion contractualization (component contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] command motion contractualization (component contract + reduced-motion + non-wasm no-op)"
cargo test -p ui-command --lib command_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] combo-box motion contractualization (spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test combo_box_semantics --no-default-features --features component-combo_box,inject-css combo_box_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] alert-dialog reduced-motion/ssr/wasm contract"
cargo test -p ui --test alert_dialog_semantics --no-default-features --features component-alert_dialog,inject-css alert_dialog_reduced_motion_ssr_wasm_branches_keep_semantics_consistent

echo "[platform] button-copy reduced-motion/ssr/wasm inheritance contract"
cargo test -p ui --test button_copy_semantics button_copy_reduced_motion_ssr_wasm_branches_are_covered_via_button_contract

echo "[platform] button motion contractualization (spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test button_semantics button_motion_sanitizes_custom_contract_values
cargo test -p ui --test button_semantics button_reduced_motion_and_ssr_wasm_semantics_contract_is_enforced

echo "[platform] time-field reduced-motion/ssr/wasm contract"
cargo test -p ui --test time_field_semantics --no-default-features --features component-time_field time_field_reduced_motion_ssr_wasm_branches_keep_semantics_consistent

echo "[platform] tag motion contractualization (N/A runtime attach + reduced-motion + non-wasm no-op)"
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_motion_contract_uses_ui_motion_non_wasm_stub_and_keeps_component_safe_without_motion
cargo test -p ui --test tag_semantics --no-default-features --features component-tag,inject-css tag_reduced_motion_ssr_wasm_contract_is_n_a_but_semantics_stay_platform_stable

echo "[platform] tag-group motion contractualization (N/A runtime attach + reduced-motion + non-wasm no-op)"
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_motion_contract_uses_ui_motion_non_wasm_stub_and_keeps_component_safe_without_motion
cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_reduced_motion_ssr_wasm_contract_is_n_a_but_semantics_stay_platform_stable

echo "[platform] checkbox-field reduced-motion/ssr/wasm contract"
cargo test -p ui --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] checkbox motion contractualization (spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe_locally

echo "[platform] list motion contractualization (spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] color-editor motion contractualization (composed spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] color-slider motion contractualization (spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] color-swatch motion contractualization (spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] color-swatch-picker motion contractualization (spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test color_swatch_picker_semantics --no-default-features --features component-color_swatch_picker,inject-css color_swatch_picker_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] color-picker motion contractualization (composed spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui-color-picker color_picker_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] scroll-area reduced-motion/ssr/wasm contract"
cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area scroll_area_reduced_motion_ssr_wasm_contract_is_consistent

echo "[platform] coachmark reduced-motion/ssr/wasm contract"
cargo test -p ui --lib coachmark_reduced_motion_ssr_wasm_branches_keep_semantics_consistent

echo "[platform] coachmark motion contractualization (spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --lib --no-default-features --features component-coachmark,inject-css coachmark_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] chart reduced-motion/ssr/wasm contract"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_reduced_motion_ssr_wasm_branches_keep_semantics_consistent

echo "[platform] chart motion contractualization (spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] carousel motion contractualization (spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] autocomplete reduced-motion/ssr/wasm contract"
cargo test -p ui --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_reduced_motion_ssr_wasm_branches_keep_semantics_consistent

echo "[platform] hover-card reduced-motion/ssr/wasm contract"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_reduced_motion_ssr_wasm_branches_keep_semantics_consistent

echo "[platform] hover-card motion contractualization (spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] error-view reduced-motion/ssr/wasm contract"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_reduced_motion_ssr_wasm_branches_keep_semantics_consistent

echo "[platform] error-view motion contractualization (spring contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test error_view_semantics --no-default-features --features component-error_view,inject-css error_view_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] fieldset reduced-motion/ssr/wasm contract"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_reduced_motion_ssr_wasm_branches_keep_semantics_consistent

echo "[platform] fieldset motion contractualization (component contract + reduced-motion + non-wasm no-op)"
cargo test -p ui --test fieldset_semantics --no-default-features --features component-fieldset,inject-css fieldset_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe

echo "[platform] source guard: non-wasm button files must not reference web_sys"
for file in \
  crates/ui/src/button/mod.rs \
  crates/ui/src/button/logic.rs \
  crates/ui/src/button/spec.rs \
  crates/ui/src/button/styles.rs \
  crates/ui/src/button/view.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm action-bar files must not reference web_sys"
for file in \
  components/action-bar/src/mod.rs \
  components/action-bar/src/i18n.rs \
  components/action-bar/src/logic.rs \
  components/action-bar/src/styles.rs \
  components/action-bar/src/view.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm breadcrumb files must not reference web_sys"
for file in \
  components/breadcrumb/src/mod.rs \
  components/breadcrumb/src/logic.rs \
  components/breadcrumb/src/styles.rs \
  components/breadcrumb/src/view.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm alert-dialog files (except wasm-gated view) must not reference web_sys"
for file in \
  components/alert-dialog/src/mod.rs \
  components/alert-dialog/src/logic.rs \
  components/alert-dialog/src/styles.rs \
  components/alert-dialog/src/motion.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm coachmark files must not reference web_sys"
for file in \
  components/coachmark/src/mod.rs \
  components/coachmark/src/logic.rs \
  components/coachmark/src/motion.rs \
  components/coachmark/src/styles.rs \
  components/coachmark/src/view.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: alert-dialog view must keep explicit wasm/non-wasm cfg gates"
if ! rg -n -F '#[cfg(target_arch = "wasm32")]' components/alert-dialog/src/view.rs >/dev/null; then
  echo "[platform] missing wasm cfg branch in alert-dialog view" >&2
  exit 1
fi
if ! rg -n -F '#[cfg(not(target_arch = "wasm32"))]' components/alert-dialog/src/view.rs >/dev/null; then
  echo "[platform] missing non-wasm cfg branch in alert-dialog view" >&2
  exit 1
fi

echo "[platform] source guard: non-wasm tag files must not reference web_sys"
for file in \
  components/tag/src/mod.rs \
  components/tag/src/logic.rs \
  components/tag/src/styles.rs \
  components/tag/src/view.rs \
  components/tag/src/group/mod.rs \
  components/tag/src/group/logic.rs \
  components/tag/src/group/styles.rs \
  components/tag/src/group/view.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm circular-progress files must not reference web_sys"
for file in \
  components/circular-progress/src/mod.rs \
  components/circular-progress/src/logic.rs \
  components/circular-progress/src/styles.rs \
  components/circular-progress/src/view.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm avatar-group files must not reference web_sys"
for file in \
  components/avatar-group/src/mod.rs \
  components/avatar-group/src/logic.rs \
  components/avatar-group/src/styles.rs \
  components/avatar-group/src/view.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm chart files must not reference web_sys"
for file in \
  components/chart/src/mod.rs \
  components/chart/src/logic.rs \
  components/chart/src/styles.rs \
  components/chart/src/view.rs \
  components/chart/src/motion.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm autocomplete files must not reference web_sys"
for file in \
  components/autocomplete/src/mod.rs \
  components/autocomplete/src/logic.rs \
  components/autocomplete/src/styles.rs \
  components/autocomplete/src/view.rs \
  components/autocomplete/src/motion.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm error-view files must not reference web_sys"
for file in \
  components/error-view/src/mod.rs \
  components/error-view/src/logic.rs \
  components/error-view/src/styles.rs \
  components/error-view/src/view.rs \
  components/error-view/src/protocol.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm fieldset files must not reference web_sys"
for file in \
  components/fieldset/src/mod.rs \
  components/fieldset/src/logic.rs \
  components/fieldset/src/styles.rs \
  components/fieldset/src/view.rs \
  components/fieldset/src/motion.rs \
  components/fieldset/src/protocol.rs
do
  if rg -n "web_sys|web-sys|js_sys|wasm_bindgen|window\\(|document\\(" "$file" >/dev/null; then
    echo "[platform] forbidden browser reference in non-wasm path file: $file" >&2
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
  components/text-input/src/textarea/mod.rs \
  components/text-input/src/textarea/logic.rs \
  components/text-input/src/textarea/styles.rs \
  components/text-input/src/textarea/view.rs
do
  if rg -n "web_sys" "$file" >/dev/null; then
    echo "[platform] forbidden web_sys reference in non-wasm path file: $file" >&2
    exit 1
  fi
done

echo "[platform] source guard: non-wasm time-field files must not reference web_sys"
for file in \
  components/text-input/src/time_field/mod.rs \
  components/text-input/src/time_field/i18n.rs \
  components/text-input/src/time_field/logic.rs \
  components/text-input/src/time_field/styles.rs \
  components/text-input/src/time_field/view.rs
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
if ! rg -n -F '#[cfg(target_arch = "wasm32")]' crates/ui/src/button/motion.rs >/dev/null; then
  echo "[platform] missing wasm cfg branch in button motion" >&2
  exit 1
fi

if ! rg -n -F '#[cfg(not(target_arch = "wasm32"))]' crates/ui/src/button/motion.rs >/dev/null; then
  echo "[platform] missing non-wasm cfg branch in button motion" >&2
  exit 1
fi

echo "[platform] source guard: action-bar motion must keep explicit wasm/non-wasm branches"
if ! rg -n -F '#[cfg(target_arch = "wasm32")]' components/action-bar/src/motion.rs >/dev/null; then
  echo "[platform] missing wasm cfg branch in action-bar motion" >&2
  exit 1
fi

if ! rg -n -F '#[cfg(not(target_arch = "wasm32"))]' components/action-bar/src/motion.rs >/dev/null; then
  echo "[platform] missing non-wasm cfg branch in action-bar motion" >&2
  exit 1
fi

echo "[platform] source guard: textarea motion must keep explicit wasm/non-wasm branches"
if ! rg -n -F '#[cfg(target_arch = "wasm32")]' components/text-input/src/textarea/motion.rs >/dev/null; then
  echo "[platform] missing wasm cfg branch in textarea motion" >&2
  exit 1
fi

if ! rg -n -F '#[cfg(not(target_arch = "wasm32"))]' components/text-input/src/textarea/motion.rs >/dev/null; then
  echo "[platform] missing non-wasm cfg branch in textarea motion" >&2
  exit 1
fi

echo "[platform] source guard: time-field motion must keep explicit wasm/non-wasm branches"
if ! rg -n -F '#[cfg(target_arch = "wasm32")]' components/text-input/src/time_field/motion.rs >/dev/null; then
  echo "[platform] missing wasm cfg branch in time-field motion" >&2
  exit 1
fi

if ! rg -n -F '#[cfg(not(target_arch = "wasm32"))]' components/text-input/src/time_field/motion.rs >/dev/null; then
  echo "[platform] missing non-wasm cfg branch in time-field motion" >&2
  exit 1
fi

echo "[platform] source guard: chart motion must keep explicit wasm/non-wasm branches"
if ! rg -n -F '#[cfg(target_arch = "wasm32")]' components/chart/src/motion.rs >/dev/null; then
  echo "[platform] missing wasm cfg branch in chart motion" >&2
  exit 1
fi

if ! rg -n -F '#[cfg(not(target_arch = "wasm32"))]' components/chart/src/motion.rs >/dev/null; then
  echo "[platform] missing non-wasm cfg branch in chart motion" >&2
  exit 1
fi

echo "[platform] source guard: autocomplete motion must keep explicit wasm/non-wasm branches"
if ! rg -n -F '#[cfg(target_arch = "wasm32")]' components/autocomplete/src/motion.rs >/dev/null; then
  echo "[platform] missing wasm cfg branch in autocomplete motion" >&2
  exit 1
fi
if ! rg -n -F '#[cfg(not(target_arch = "wasm32"))]' components/autocomplete/src/motion.rs >/dev/null; then
  echo "[platform] missing non-wasm cfg branch in autocomplete motion" >&2
  exit 1
fi

echo "[platform] source guard: error-view motion must keep explicit wasm/non-wasm branches"
if ! rg -n -F '#[cfg(target_arch = "wasm32")]' components/error-view/src/motion.rs >/dev/null; then
  echo "[platform] missing wasm cfg branch in error-view motion" >&2
  exit 1
fi
if ! rg -n -F '#[cfg(not(target_arch = "wasm32"))]' components/error-view/src/motion.rs >/dev/null; then
  echo "[platform] missing non-wasm cfg branch in error-view motion" >&2
  exit 1
fi

echo "[platform] source guard: autocomplete non-wasm motion fallback must remain predictable"
if ! rg -n -F 'if !is_open.get() {' components/autocomplete/src/motion.rs >/dev/null; then
  echo "[platform] missing non-wasm close fallback branch in autocomplete motion" >&2
  exit 1
fi
if ! rg -n -F 'on_exit_complete.run(())' components/autocomplete/src/motion.rs >/dev/null; then
  echo "[platform] missing non-wasm on_exit_complete callback in autocomplete motion" >&2
  exit 1
fi

echo "[platform] OK"
