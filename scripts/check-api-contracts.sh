#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BASELINE_FILE="scripts/baseline/api_contract_violations.txt"

# Focus first on migrated/new source-first component surface.
TARGET_FILES=(
  "components/alert/src/view.rs"
  "components/breadcrumb/src/view.rs"
  "components/button/src/view.rs"
  "components/image/src/view.rs"
  "components/pagination/src/view.rs"
  "components/action-bar/src/view.rs"
  "components/item/src/view.rs"
  "components/snippet/src/view.rs"
  "components/underlay/src/view.rs"
  "components/text/src/view.rs"
  "components/icon/src/view.rs"
)

ADVANCED_FILES=(
  "components/accordion/src/view.rs"
  "components/accordion/src/logic.rs"
  "components/tabs/src/view.rs"
  "components/tabs/src/logic.rs"
  "components/menu/src/view.rs"
  "components/menu/src/logic.rs"
  "components/menu/src/dropdown/view.rs"
  "components/menu/src/dropdown/logic.rs"
  "components/menu/src/dropdown_menu/view.rs"
  "components/menu/src/dropdown_menu/logic.rs"
  "components/menu/src/trigger/view.rs"
  "components/menu/src/trigger/logic.rs"
  "components/menu/src/item/view.rs"
  "components/menu/src/item/logic.rs"
  "components/popover/src/view.rs"
  "components/popover/src/logic.rs"
  "components/tooltip/src/view.rs"
  "components/tooltip/src/logic.rs"
  "components/table/src/view.rs"
  "components/table/src/logic.rs"
  "crates/ui-layout/src/grid/view.rs"
  "crates/ui-layout/src/grid/logic.rs"
)

REGISTRATION_DIRS=(
  "components/accordion/src"
  "components/tabs/src"
  "components/menu/src"
)

SLOT_PROJECTION_DIRS=(
  "components/accordion/src"
  "components/tabs/src"
  "components/menu/src"
  "components/tooltip/src"
  "components/popover/src"
)

TARGET_COMPONENT_DIRS=(
  "components/alert/src"
  "components/breadcrumb/src"
  "components/button/src"
  "components/image/src"
  "components/pagination/src"
  "components/action-bar/src"
  "components/item/src"
  "components/snippet/src"
  "components/underlay/src"
  "components/text/src"
  "components/icon/src"
)

SPEC_ALLOWLIST=(
  "components/button/src/spec.rs"
)

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

emit() {
  local code="$1"
  local file="$2"
  local line="$3"
  local detail="$4"
  printf '%s|%s|%s|%s\n' "$code" "$file" "$line" "$detail" >>"$tmp"
}

has_prop_decl() {
  local file="$1"
  local pattern="$2"
  rg -n --pcre2 "$pattern" "$file" >/dev/null 2>&1
}

extract_prop_name() {
  local body="$1"
  sed -E 's/.*\][[:space:]]*([A-Za-z_][A-Za-z0-9_]*)[[:space:]]*:.*$/\1/' <<<"$body"
}

extract_prop_type() {
  local body="$1"
  sed -E 's/.*\][[:space:]]*[A-Za-z_][A-Za-z0-9_]*[[:space:]]*:[[:space:]]*([^,]+),?[[:space:]]*$/\1/' <<<"$body"
}

resolve_semantics_test_file() {
  local slug="$1"
  local slug_us="${slug//-/_}"
  local candidates=(
    "components/${slug}/test/${slug_us}_semantics.rs"
    "components/${slug}/src/test/${slug_us}_semantics.rs"
    "crates/ui-layout/tests/${slug_us}_semantics.rs"
    "components/${slug}/tests/${slug_us}_semantics.rs"
    "components/${slug}/test/${slug_us}.rs"
    "components/${slug}/test/logic.rs"
  )

  local candidate
  for candidate in "${candidates[@]}"; do
    if [[ -f "$candidate" ]]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done

  return 1
}

for file in "${TARGET_FILES[@]}"; do
  [[ -f "$file" ]] || continue

  # Rule 1a: callback props must use `on_*`.
  while IFS= read -r row; do
    [[ -z "$row" ]] && continue
    line="${row%%:*}"
    body="${row#*:}"
    name="$(extract_prop_name "$body")"
    [[ "$name" =~ ^on_ ]] || emit "callback_prefix" "$file" "$line" "$name"
  done < <(
    rg -n --pcre2 \
      '^\s*#\[prop[^\]]*\]\s*[A-Za-z_][A-Za-z0-9_]*\s*:\s*(Option<\s*)?(Callback<|OnPress\b)' \
      "$file" || true
  )

  # Rule 1b: bool props must use `is_*` or `default_*`.
  while IFS= read -r row; do
    [[ -z "$row" ]] && continue
    line="${row%%:*}"
    body="${row#*:}"
    name="$(extract_prop_name "$body")"
    if [[ ! "$name" =~ ^is_ ]] && [[ ! "$name" =~ ^default_ ]]; then
      emit "bool_prefix" "$file" "$line" "$name"
    fi
  done < <(
    rg -n --pcre2 \
      '^\s*#\[prop[^\]]*\]\s*[A-Za-z_][A-Za-z0-9_]*\s*:\s*(Option<\s*)?(Signal<\s*)?bool(\s*>\s*)?(\s*>\s*)?\s*,?\s*$' \
      "$file" || true
  )

  # Rule 2: controllable axis triad: value + on_*_change + default_*.
  mapfile -t default_axes < <(
    rg -n --pcre2 '^\s*#\[prop[^\]]*\]\s*default_([A-Za-z_][A-Za-z0-9_]*)\s*:' "$file" \
      | sed -E 's/.*default_([A-Za-z_][A-Za-z0-9_]*)[[:space:]]*:.*$/\1/' \
      | sort -u
  )
  mapfile -t change_axes < <(
    rg -n --pcre2 '^\s*#\[prop[^\]]*\]\s*on_([A-Za-z_][A-Za-z0-9_]*)_change\s*:' "$file" \
      | sed -E 's/.*on_([A-Za-z_][A-Za-z0-9_]*)_change[[:space:]]*:.*$/\1/' \
      | sort -u
  )

  axes="$(
    {
      printf '%s\n' "${default_axes[@]:-}"
      printf '%s\n' "${change_axes[@]:-}"
    } | sed '/^$/d' | sort -u
  )"

  while IFS= read -r axis; do
    [[ -z "$axis" ]] && continue

    has_prop_decl "$file" "^\\s*#\\[prop[^\\]]*\\]\\s*default_${axis}\\s*:" \
      || emit "triad_missing_default" "$file" "0" "$axis"
    has_prop_decl "$file" "^\\s*#\\[prop[^\\]]*\\]\\s*on_${axis}_change\\s*:" \
      || emit "triad_missing_change" "$file" "0" "$axis"
    if ! has_prop_decl "$file" "^\\s*#\\[prop[^\\]]*\\]\\s*(is_)?${axis}\\s*:"; then
      emit "triad_missing_value" "$file" "0" "$axis"
    fi
  done <<<"$axes"

  # Rule 3: view.rs must not perform default fallback normalization directly.
  while IFS= read -r row; do
    [[ -z "$row" ]] && continue
    line="${row%%:*}"
    emit "view_default_fallback" "$file" "$line" "unwrap_or"
  done < <(
    rg -n --pcre2 '\bunwrap_or(_else)?\s*\(' "$file" || true
  )

  # Rule 4: if logic.rs exists, view.rs must consume centralized normalization/state derivation.
  logic_file="$(dirname "$file")/logic.rs"
  if [[ -f "$logic_file" ]]; then
    if ! rg -n --pcre2 'logic::' "$file" >/dev/null 2>&1; then
      emit "view_missing_logic_normalization" "$file" "0" "logic::"
    fi
  fi

  # Rule 5a: discrete axis (`variant|size|mode|status`) must be typed (enum-like), not string/bool free-form.
  while IFS= read -r row; do
    [[ -z "$row" ]] && continue
    line="${row%%:*}"
    body="${row#*:}"
    name="$(extract_prop_name "$body")"
    type_expr="$(extract_prop_type "$body")"
    if [[ "$name" =~ (variant|size|mode|status) ]]; then
      if grep -Eq 'String|&str|Cow<|Vec<[[:space:]]*String[[:space:]]*>' <<<"$type_expr"; then
        emit "discrete_axis_must_be_enum" "$file" "$line" "${name}:${type_expr}"
      fi
      if grep -Eq 'bool|Signal<[[:space:]]*bool' <<<"$type_expr"; then
        emit "discrete_axis_bool_forbidden" "$file" "$line" "${name}:${type_expr}"
      fi
    fi
  done < <(
    rg -n --pcre2 \
      '^\s*#\[prop[^\]]*\]\s*[A-Za-z_][A-Za-z0-9_]*\s*:\s*[^,]+,?\s*$' \
      "$file" || true
  )

  # Rule 5b: bool explosion guard (many bool props usually indicates hidden state machine).
  bool_prop_count="$(
    {
      rg -n --pcre2 \
        '^\s*#\[prop[^\]]*\]\s*[A-Za-z_][A-Za-z0-9_]*\s*:\s*(Option<\s*)?(Signal<\s*)?bool(\s*>\s*)?(\s*>\s*)?\s*,?\s*$' \
        "$file" || true
    } \
      | wc -l \
      | tr -d ' '
  )"
  if [[ "$bool_prop_count" -gt 5 ]]; then
    emit "bool_explosion" "$file" "0" "$bool_prop_count"
  fi

  # Rule 6a: component view should not directly bind business/global stores.
  if rg -n --pcre2 '::store::|use .*store\b|\b[A-Za-z0-9_]*Store\b' "$file" >/dev/null 2>&1; then
    if ! rg -n --pcre2 '\bStoredValue\b' "$file" >/dev/null 2>&1; then
      emit "component_business_store_dependency" "$file" "0" "store"
    fi
  fi

  # Rule 6b: controllable primitive wiring should be kept out of `view.rs` default path.
  while IFS= read -r row; do
    [[ -z "$row" ]] && continue
    line="${row%%:*}"
    emit "view_direct_controllable_state" "$file" "$line" "use_controllable_*"
  done < <(
    rg -n --pcre2 'use_controllable_(open_)?state' "$file" || true
  )

  # Rule 6c: `logic.rs` should stay POJO-like (no reactive signal container types).
  if [[ -f "$logic_file" ]]; then
    while IFS= read -r row; do
      [[ -z "$row" ]] && continue
      line="${row%%:*}"
      emit "logic_framework_binding" "$logic_file" "$line" "Signal"
    done < <(
      rg -n --pcre2 '\b(ReadSignal|WriteSignal|RwSignal|Signal<|signal\s*\()' "$logic_file" || true
    )
  fi

  # Rule 7: async interaction contract.
  has_async_tokens=0
  if rg -n --pcre2 '\bis_loading\b|aria-busy|retry_|has_error|copy_error|is_copying' "$file" >/dev/null 2>&1; then
    has_async_tokens=1
  fi

  if [[ "$has_async_tokens" -eq 1 ]]; then
    rg -n --pcre2 'aria-busy' "$file" >/dev/null 2>&1 || emit "async_missing_aria_busy" "$file" "0" "aria-busy"
    rg -n --pcre2 '\bdisabled\s*=' "$file" >/dev/null 2>&1 || emit "async_missing_disabled_mapping" "$file" "0" "disabled"
    if rg -n --pcre2 'has_error|copy_error|on_copy_error|error_label' "$file" >/dev/null 2>&1; then
      rg -n --pcre2 'retry' "$file" >/dev/null 2>&1 || emit "async_missing_retry_path" "$file" "0" "retry"
    fi
  else
    check2_file="$(dirname "$file")/check2.md"
    if [[ -f "$check2_file" ]]; then
      if ! rg -n --pcre2 'N/A|无异步|无远程请求|无网络请求|不涉及异步' "$check2_file" >/dev/null 2>&1; then
        emit "async_na_reason_missing" "$check2_file" "0" "N/A"
      fi
    else
      emit "async_na_doc_missing" "$file" "0" "check2.md"
    fi
  fi

  # Rule 8a: DX - do not force internal state object as required prop.
  while IFS= read -r row; do
    [[ -z "$row" ]] && continue
    line="${row%%:*}"
    body="${row#*:}"
    name="$(extract_prop_name "$body")"
    if [[ "$name" =~ (^state$|_state$) ]]; then
      if [[ ! "$body" =~ optional ]]; then
        emit "required_internal_state_prop" "$file" "$line" "$name"
      fi
    fi
  done < <(
    rg -n --pcre2 \
      '^\s*#\[prop[^\]]*\]\s*[A-Za-z_][A-Za-z0-9_]*\s*:\s*[^,]+,?\s*$' \
      "$file" || true
  )

  # Rule 8b: docs-app should have minimal usage entry path for the component slug.
  component_dir="$(basename "$(dirname "$(dirname "$file")")")"
  if ! rg -n --fixed-strings "\"${component_dir}\"" apps/docs-app/src/pages/components >/dev/null 2>&1; then
    emit "docs_app_example_missing" "$file" "0" "$component_dir"
  fi

  # Rule 17a: A11y semantics and i18n/l10n hooks must be present.
  if rg -n --pcre2 'on:|on_press|<button|use_button|use_press|tabindex' "$file" >/dev/null 2>&1; then
    rg -n --pcre2 'role=|aria-' "$file" >/dev/null 2>&1 \
      || emit "a11y_semantics_missing" "$file" "0" "role/aria"
  fi

  if ! rg -n --pcre2 'lang=|dir=|locale_attrs\(|A11yDirection|use_ui_i18n' "$file" >/dev/null 2>&1; then
    emit "i18n_l10n_hook_missing" "$file" "0" "lang/dir/i18n"
  fi

  while IFS= read -r row; do
    [[ -z "$row" ]] && continue
    line="${row%%:*}"
    emit "hardcoded_visible_text" "$file" "$line" "literal"
  done < <(
    rg -n --pcre2 '^\s*"[^"]*[A-Za-z][^"]*"\s*$' "$file" || true
  )

  component_src_dir="$(dirname "$file")"
  while IFS= read -r row; do
    [[ -z "$row" ]] && continue
    line="${row%%:*}"
    emit "a11y_tool_redefinition" "$component_src_dir" "$line" "local-a11y-fn"
  done < <(
    rg -n --pcre2 \
      'fn\s+(locale_attrs|aria_expanded|aria_controls_when_open|overlay_dialog_attrs|popup_trigger_attrs|live_region_attrs)\s*\(' \
      "$component_src_dir" || true
  )

  # Rule 18: state observability should expose stable semantic markers.
  rg -n --pcre2 'data-slot=' "$file" >/dev/null 2>&1 \
    || emit "observability_data_slot_missing" "$file" "0" "data-slot"

  if rg -n --pcre2 'is_|default_|on_.*_change' "$file" >/dev/null 2>&1; then
    rg -n --pcre2 'data-[a-z0-9_-]*source=' "$file" >/dev/null 2>&1 \
      || emit "observability_source_marker_missing" "$file" "0" "data-*-source"
  fi

  if rg -n --pcre2 'data-[a-z0-9_-]+=\s*move\s*\|\|.*format!\(' "$file" >/dev/null 2>&1; then
    emit "observability_marker_free_text_risk" "$file" "0" "format!-marker"
  fi

  # Rule 19: style state dependency should be explicit/stable.
  styles_file="${component_src_dir}/styles.rs"
  if [[ -f "$styles_file" ]]; then
    while IFS= read -r row; do
      [[ -z "$row" ]] && continue
      line="${row%%:*}"
      emit "styles_fragile_selector_forbidden" "$styles_file" "$line" "nth-child"
    done < <(rg -n --pcre2 ':nth-child|:nth-of-type' "$styles_file" || true)
  fi

  while IFS= read -r row; do
    [[ -z "$row" ]] && continue
    line="${row%%:*}"
    if ! grep -q -- '--' <<<"$row"; then
      emit "inline_style_business_logic_forbidden" "$file" "$line" "style="
    fi
  done < <(rg -n --pcre2 'style\s*=' "$file" || true)

  # Rule 20: semantic contract tests are mandatory (not snapshot-only).
  if semantics_file="$(resolve_semantics_test_file "$component_dir")"; then
    rg -n --pcre2 'data-|aria-|role' "$semantics_file" >/dev/null 2>&1 \
      || emit "semantic_test_contract_missing" "$semantics_file" "0" "role/aria/data"

    if rg -n --pcre2 'snapshot|insta' "$semantics_file" >/dev/null 2>&1; then
      if ! rg -n --pcre2 'data-|aria-|role' "$semantics_file" >/dev/null 2>&1; then
        emit "semantic_test_snapshot_only" "$semantics_file" "0" "snapshot-only"
      fi
    fi

    if rg -n --pcre2 ':nth-child|:nth-of-type|locator\([^)]*>[^)]*>' "$semantics_file" >/dev/null 2>&1; then
      emit "semantic_test_fragile_selector" "$semantics_file" "0" "dom-depth-selector"
    fi

    if rg -n --pcre2 'default_|on_.*_change' "$file" >/dev/null 2>&1; then
      rg -n --pcre2 'controlled|uncontrolled|default_|on_.*_change' "$semantics_file" >/dev/null 2>&1 \
        || emit "semantic_test_matrix_controlled_missing" "$semantics_file" "0" "controlled/uncontrolled"
    fi

    if rg -n --pcre2 '\bis_disabled\b|disabled' "$file" >/dev/null 2>&1; then
      rg -n --pcre2 'disabled' "$semantics_file" >/dev/null 2>&1 \
        || emit "semantic_test_matrix_disabled_missing" "$semantics_file" "0" "disabled"
    fi

    if rg -n --pcre2 'on:keydown|key\(' "$file" >/dev/null 2>&1; then
      rg -n --pcre2 'keyboard|Arrow|Enter|Space|Tab|Escape' "$semantics_file" >/dev/null 2>&1 \
        || emit "semantic_test_matrix_keyboard_missing" "$semantics_file" "0" "keyboard"
    fi

    if rg -n --pcre2 'on:pointer|pointer|on:click' "$file" >/dev/null 2>&1; then
      rg -n --pcre2 'pointer|mouse|click' "$semantics_file" >/dev/null 2>&1 \
        || emit "semantic_test_matrix_pointer_missing" "$semantics_file" "0" "pointer"
    fi

    motion_file="${component_src_dir}/motion.rs"
    if [[ -f "$motion_file" ]] && rg -n --pcre2 'wasm32|ssr|web_sys|cfg\(target_arch' "$motion_file" >/dev/null 2>&1; then
      rg -n --pcre2 'ssr|wasm|target_arch' "$semantics_file" >/dev/null 2>&1 \
        || emit "semantic_test_matrix_platform_missing" "$semantics_file" "0" "ssr/wasm"
    fi
  else
    emit "semantic_test_missing" "$file" "0" "$component_dir"
  fi

  # Rule 9: explicit composition over parallel arrays.
  while IFS= read -r row; do
    [[ -z "$row" ]] && continue
    line="${row%%:*}"
    body="${row#*:}"
    name="$(extract_prop_name "$body")"
    type_expr="$(extract_prop_type "$body")"
    if [[ "$type_expr" =~ ^Vec\< ]] && [[ "$name" =~ ^(labels|titles|panels|descriptions)$ ]]; then
      emit "parallel_array_api_forbidden" "$file" "$line" "${name}:${type_expr}"
    fi
    if [[ "$name" == "items" ]] && grep -Eq 'Vec<[[:space:]]*(String|&str)' <<<"$type_expr"; then
      emit "itemspec_untyped_forbidden" "$file" "$line" "${name}:${type_expr}"
    fi
  done < <(
    rg -n --pcre2 \
      '^\s*#\[prop[^\]]*\]\s*[A-Za-z_][A-Za-z0-9_]*\s*:\s*Vec<[^,]+>,?\s*$' \
      "$file" || true
  )
done

# Advanced interaction / physics gate.
for file in "${ADVANCED_FILES[@]}"; do
  [[ -f "$file" ]] || continue

  # Rule 10: Macro/Micro duality for high-frequency drag loops.
  if rg -n --pcre2 'on:(pointermove|mousemove|touchmove)|drag(_|)move' "$file" >/dev/null 2>&1; then
    rg -n --pcre2 'DragEnd|on_drag_end|drag_end' "$file" >/dev/null 2>&1 \
      || emit "drag_missing_terminal_action" "$file" "0" "DragEnd"

    rg -n --pcre2 'motion::|attach_.*motion|request_animation_frame|raf|Spring|Effect::new' "$file" >/dev/null 2>&1 \
      || emit "drag_missing_local_loop" "$file" "0" "view/motion-local-loop"

    if rg -n --pcre2 'on:(pointermove|mousemove|touchmove)' "$file" >/dev/null 2>&1 \
      && rg -n --pcre2 'logic::(resolve_|normalize_)' "$file" >/dev/null 2>&1; then
      emit "drag_frame_cross_logic" "$file" "0" "pointermove->logic"
    fi
  fi

  # Rule 11: Two-pass rendering for geometry-dependent overlays.
  if [[ "$file" =~ /(tooltip|popover|menu)/src/view\.rs$ ]]; then
    rg -n --pcre2 'use_tooltip_position|use_popover_position|get_bounding_client_rect|measure|placement' "$file" >/dev/null 2>&1 \
      || emit "two_pass_missing_measure_stage" "$file" "0" "Intent->Measure"
    rg -n --pcre2 'logic::(resolve_|normalize_)' "$file" >/dev/null 2>&1 \
      || emit "two_pass_missing_rectification" "$file" "0" "Rectification(logic)"
    rg -n --pcre2 'get_untracked|!=.*set_|idempot|clamp|saturating_' "$file" >/dev/null 2>&1 \
      || emit "two_pass_missing_idempotence_guard" "$file" "0" "idempotence"
  fi

  # Rule 14: Env streams must be sampled/debounced and mapped to semantic actions.
  if rg -n --pcre2 'ResizeObserver|IntersectionObserver|on:resize|on:scroll|theme' "$file" >/dev/null 2>&1; then
    rg -n --pcre2 'debounce|throttle|sample|Memo::new|request_animation_frame' "$file" >/dev/null 2>&1 \
      || emit "env_stream_missing_sampling" "$file" "0" "sample/debounce"
    rg -n --pcre2 'Action::|request_.*change|on_.*change|logic::' "$file" >/dev/null 2>&1 \
      || emit "env_stream_missing_action_mapping" "$file" "0" "semantic-action"
  fi

  # Rule 15: Event light cone for large collections (table/grid).
  if [[ "$file" =~ /(table|grid)/src/view\.rs$ ]]; then
    if rg -n --pcre2 'map\(|for\s+[A-Za-z_]' "$file" >/dev/null 2>&1; then
      rg -n --pcre2 'Context|Selector|SelectionState::All|selection_state' "$file" >/dev/null 2>&1 \
        || emit "event_light_cone_missing_bus_selector" "$file" "0" "Context+Selector"
    fi
  fi

  # Rule 16: Causality bus should preserve TraceId across broadcasts.
  if rg -n --pcre2 'broadcast|publish|dispatch|bus' "$file" >/dev/null 2>&1; then
    rg -n --pcre2 'TraceId' "$file" >/dev/null 2>&1 \
      || emit "causality_bus_missing_trace_id" "$file" "0" "TraceId"
  fi
done

# Rule 12: Registration protocol and deterministic order for dynamic item sets.
for dir in "${REGISTRATION_DIRS[@]}"; do
  [[ -d "$dir" ]] || continue
  while IFS= read -r row; do
    [[ -z "$row" ]] && continue
    file="${row%%:*}"
    line="${row#*:}"
    line="${line%%:*}"
    emit "registration_hashset_forbidden" "$file" "$line" "HashSet-order"
  done < <(rg -n --pcre2 '\bHashSet\b' "$dir" || true)

  rg -n --pcre2 'RegistrationContext|register|unregister|items_order' "$dir" >/dev/null 2>&1 \
    || emit "registration_protocol_missing" "$dir" "0" "Register/Unregister/items_order"
done

# Rule 13: Slot projection strategy + KeepAlive hidden notification.
for dir in "${SLOT_PROJECTION_DIRS[@]}"; do
  [[ -d "$dir" ]] || continue
  rg -n --pcre2 '\b(Lazy|KeepAlive|Eager)\b' "$dir" >/dev/null 2>&1 \
    || emit "slot_projection_strategy_missing" "$dir" "0" "Lazy/KeepAlive/Eager"

  if rg -n --pcre2 '\bKeepAlive\b' "$dir" >/dev/null 2>&1; then
    rg -n --pcre2 'NotifyHidden|notify_hidden|on_hidden|pause' "$dir" >/dev/null 2>&1 \
      || emit "keepalive_notify_hidden_missing" "$dir" "0" "NotifyHidden"
  fi
done

# Rule 21: component file responsibilities (mod/logic/styles/view/motion).
for src_dir in "${TARGET_COMPONENT_DIRS[@]}"; do
  [[ -d "$src_dir" ]] || continue

  mod_file="${src_dir}/mod.rs"
  logic_file="${src_dir}/logic.rs"
  view_file="${src_dir}/view.rs"
  styles_file="${src_dir}/styles.rs"
  motion_file="${src_dir}/motion.rs"
  protocol_file="${src_dir}/protocol.rs"

  [[ -f "$mod_file" ]] || emit "component_mod_missing" "$src_dir" "0" "mod.rs"

  if [[ -f "$mod_file" ]]; then
    rg -n --pcre2 '^\s*pub (use|mod)\b' "$mod_file" >/dev/null 2>&1 \
      || emit "mod_exports_missing" "$mod_file" "0" "pub use/pub mod"

    while IFS= read -r row; do
      [[ -z "$row" ]] && continue
      line="${row%%:*}"
      emit "mod_impl_detail_forbidden" "$mod_file" "$line" "impl/type in mod.rs"
    done < <(
      rg -n --pcre2 '^\s*(pub\s+)?(fn|struct|enum|trait|impl)\b' "$mod_file" || true
    )

    while IFS= read -r row; do
      [[ -z "$row" ]] && continue
      line="${row%%:*}"
      emit "mod_rendering_forbidden" "$mod_file" "$line" "#[component]/view!"
    done < <(
      rg -n --pcre2 '#\[component\]|view!\s*\{' "$mod_file" || true
    )
  fi

  if [[ -f "$logic_file" ]]; then
    while IFS= read -r row; do
      [[ -z "$row" ]] && continue
      line="${row%%:*}"
      emit "logic_dom_view_binding_forbidden" "$logic_file" "$line" "DOM/view binding"
    done < <(
      rg -n --pcre2 '#\[component\]|view!\s*\{|\bNodeRef<|leptos::html|web_sys|wasm_bindgen' "$logic_file" || true
    )

    if rg -n --pcre2 '"ui-[a-z0-9_-]+' "$logic_file" >/dev/null 2>&1; then
      emit "logic_style_branch_forbidden" "$logic_file" "0" "ui-* class details"
    fi
  fi

  if [[ -f "$view_file" ]]; then
    while IFS= read -r row; do
      [[ -z "$row" ]] && continue
      line="${row%%:*}"
      emit "view_type_definition_forbidden" "$view_file" "$line" "struct/enum/trait/impl"
    done < <(
      rg -n --pcre2 '^\s*(pub\s+)?(struct|enum|trait|impl)\b' "$view_file" || true
    )

    if [[ -f "$protocol_file" ]]; then
      if ! rg -n --pcre2 '\bprotocol::|\buse crate::.*protocol' "$view_file" >/dev/null 2>&1; then
        emit "view_protocol_mount_missing" "$view_file" "0" "protocol::"
      fi
    fi
  fi

  if [[ -f "$styles_file" ]]; then
    if ! rg -n --pcre2 'var\(--ui-' "$styles_file" >/dev/null 2>&1; then
      emit "styles_ui_token_missing" "$styles_file" "0" "var(--ui-*)"
    fi

    while IFS= read -r row; do
      [[ -z "$row" ]] && continue
      line="${row%%:*}"
      emit "styles_private_token_forbidden" "$styles_file" "$line" "--(non-ui)-*"
    done < <(
      rg -n --pcre2 -- '--(?!ui-)[a-z0-9][a-z0-9-]*' "$styles_file" || true
    )

    while IFS= read -r row; do
      [[ -z "$row" ]] && continue
      line="${row%%:*}"
      emit "styles_runtime_logic_forbidden" "$styles_file" "$line" "Signal/Effect/render in styles"
    done < <(
      rg -n --pcre2 '#\[component\]|view!\s*\{|signal\s*\(|Effect::new|\b(ReadSignal|WriteSignal|RwSignal|Signal<)\b' "$styles_file" || true
    )
  fi

  if [[ -f "$motion_file" ]]; then
    while IFS= read -r row; do
      [[ -z "$row" ]] && continue
      line="${row%%:*}"
      emit "motion_rendering_forbidden" "$motion_file" "$line" "#[component]/view!"
    done < <(
      rg -n --pcre2 '#\[component\]|view!\s*\{|data-slot=|class=' "$motion_file" || true
    )

    if ! rg -n --pcre2 '\bui_motion::' "$motion_file" >/dev/null 2>&1; then
      emit "motion_contract_missing_ui_motion" "$motion_file" "0" "ui_motion"
    fi

    if rg -n --pcre2 'request_animation_frame|set_interval|set_timeout' "$motion_file" >/dev/null 2>&1; then
      if ! rg -n --pcre2 '\bui_motion::' "$motion_file" >/dev/null 2>&1; then
        emit "motion_engine_reimplementation_forbidden" "$motion_file" "0" "raf/timer outside ui_motion"
      fi
    fi
  fi
done

# Rule 22: spec.rs should stay scarce and versioned with contract tests.
mapfile -t spec_files < <(
  find components crates/ui-components -type f -path '*/src/spec.rs' 2>/dev/null | sort
)

for spec_file in "${spec_files[@]}"; do
  allowed=0
  for allow in "${SPEC_ALLOWLIST[@]}"; do
    if [[ "$spec_file" == "$allow" ]]; then
      allowed=1
      break
    fi
  done
  if [[ "$allowed" -ne 1 ]]; then
    emit "spec_sprawl_forbidden" "$spec_file" "0" "spec.rs allowlist"
  fi

  if ! rg -n --pcre2 'schema_version|VERSION|version' "$spec_file" >/dev/null 2>&1; then
    emit "spec_version_marker_missing" "$spec_file" "0" "version marker"
  fi

  component_slug="$(sed -E \
    -e 's#^components/([^/]+)/src/spec\.rs$#\1#' \
    -e 's#^crates/ui-components/src/([^/]+)/spec\.rs$#\1#' \
    <<<"$spec_file")"
  if [[ "$component_slug" == "$spec_file" ]]; then
    component_slug="$(basename "$(dirname "$spec_file")")"
  fi
  if [[ "$component_slug" == "src" ]]; then
    component_slug="$(basename "$(dirname "$(dirname "$spec_file")")")"
  fi

  if semantics_file="$(resolve_semantics_test_file "$component_slug")"; then
    rg -n --pcre2 'spec|schema' "$semantics_file" >/dev/null 2>&1 \
      || emit "spec_contract_test_missing" "$semantics_file" "0" "spec/schema assertion"
    rg -n --pcre2 'version|schema_version|evolution|演进' "$semantics_file" >/dev/null 2>&1 \
      || emit "spec_evolution_test_missing" "$semantics_file" "0" "version evolution assertion"
  else
    emit "spec_contract_test_missing" "$spec_file" "0" "semantics test not found"
  fi

  spec_mod_file="$(dirname "$spec_file")/mod.rs"
  if [[ -f "$spec_mod_file" ]]; then
    rg -n --pcre2 'pub mod spec;' "$spec_mod_file" >/dev/null 2>&1 \
      || emit "spec_mod_export_missing" "$spec_mod_file" "0" "pub mod spec;"
  fi
done

# Rule 23: token-first static style + centralized CSS registry.
css_registry_file="crates/ui-components/src/css.rs"
ui_root_file="crates/ui-components/src/root.rs"

if [[ ! -f "$css_registry_file" ]]; then
  emit "css_registry_missing" "$css_registry_file" "0" "crates/ui-components/src/css.rs"
else
  rg -n --pcre2 'push_components_css' "$css_registry_file" >/dev/null 2>&1 \
    || emit "css_registry_api_missing" "$css_registry_file" "0" "push_components_css"
  rg -n --fixed-strings '@layer ui' "$css_registry_file" >/dev/null 2>&1 \
    || emit "css_registry_layer_missing" "$css_registry_file" "0" "@layer ui"

  for src_dir in "${TARGET_COMPONENT_DIRS[@]}"; do
    component_slug="$(basename "$(dirname "$src_dir")")"
    component_feature_slug="${component_slug//-/_}"
    rg -n --fixed-strings "component-${component_feature_slug}" "$css_registry_file" >/dev/null 2>&1 \
      || emit "css_registry_component_missing" "$css_registry_file" "0" "component-${component_feature_slug}"
  done
fi

if [[ -f "$ui_root_file" ]]; then
  rg -n --pcre2 'crate::css::push_components_css' "$ui_root_file" >/dev/null 2>&1 \
    || emit "ui_root_css_injection_missing" "$ui_root_file" "0" "crate::css::push_components_css"
else
  emit "ui_root_missing" "$ui_root_file" "0" "crates/ui-components/src/root.rs"
fi

while IFS= read -r row; do
  [[ -z "$row" ]] && continue
  file="${row%%:*}"
  line="${row#*:}"
  line="${line%%:*}"
  emit "utility_or_css_in_rust_forbidden" "$file" "$line" "utility-first/CSS-in-Rust marker"
done < <(
  rg -n --pcre2 \
    'tailwind|class_variance_authority|cva!|stylist|stylex|emotion|tw_merge|tw!|css!\s*\(' \
    components crates/ui-components/src \
    --glob '!**/test/**' || true
)

# Rule 24: default theme visual baseline contract.
visual_docs_page="apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs"
visual_registry_page="apps/docs-app/src/pages/components/pages.rs"
visual_e2e_file="e2e/tests/docs_app_theme_visual_baseline.spec.mjs"

[[ -f "$visual_docs_page" ]] || emit "visual_baseline_docs_page_missing" "$visual_docs_page" "0" "theme_visual_baseline page"
[[ -f "$visual_registry_page" ]] || emit "visual_baseline_docs_registry_missing" "$visual_registry_page" "0" "components/pages.rs"
[[ -f "$visual_e2e_file" ]] || emit "visual_baseline_e2e_missing" "$visual_e2e_file" "0" "docs_app_theme_visual_baseline.spec.mjs"

if [[ -f "$visual_docs_page" ]]; then
  rg -n --pcre2 '<Button|ButtonVariant' "$visual_docs_page" >/dev/null 2>&1 \
    || emit "visual_baseline_button_missing" "$visual_docs_page" "0" "Button"
  rg -n --pcre2 '<Input\b' "$visual_docs_page" >/dev/null 2>&1 \
    || emit "visual_baseline_input_missing" "$visual_docs_page" "0" "Input"
  rg -n --pcre2 '<Overlay\b' "$visual_docs_page" >/dev/null 2>&1 \
    || emit "visual_baseline_overlay_missing" "$visual_docs_page" "0" "Overlay"
  rg -n --pcre2 'Default Theme Visual Baseline|Default theme visual baseline' "$visual_docs_page" >/dev/null 2>&1 \
    || emit "visual_baseline_quality_copy_missing" "$visual_docs_page" "0" "visual baseline quality copy"
fi

if [[ -f "$visual_registry_page" ]]; then
  rg -n --fixed-strings 'mod theme_visual_baseline;' "$visual_registry_page" >/dev/null 2>&1 \
    || emit "visual_baseline_registry_mod_missing" "$visual_registry_page" "0" "mod theme_visual_baseline;"
  rg -n --fixed-strings 'theme_visual_baseline::theme_visual_baseline' "$visual_registry_page" >/dev/null 2>&1 \
    || emit "visual_baseline_registry_route_missing" "$visual_registry_page" "0" "theme_visual_baseline::theme_visual_baseline"
fi

if [[ -f "$visual_e2e_file" ]]; then
  rg -n --fixed-strings 'theme-visual-baseline' "$visual_e2e_file" >/dev/null 2>&1 \
    || emit "visual_baseline_e2e_route_missing" "$visual_e2e_file" "0" "theme-visual-baseline"
  rg -n --pcre2 'toHaveScreenshot' "$visual_e2e_file" >/dev/null 2>&1 \
    || emit "visual_baseline_e2e_screenshot_missing" "$visual_e2e_file" "0" "toHaveScreenshot"
  rg -n --fixed-strings 'docs-app-theme-visual-baseline-button.png' "$visual_e2e_file" >/dev/null 2>&1 \
    || emit "visual_baseline_e2e_button_snapshot_missing" "$visual_e2e_file" "0" "button snapshot"
  rg -n --fixed-strings 'docs-app-theme-visual-baseline-input.png' "$visual_e2e_file" >/dev/null 2>&1 \
    || emit "visual_baseline_e2e_input_snapshot_missing" "$visual_e2e_file" "0" "input snapshot"
  rg -n --fixed-strings 'docs-app-theme-visual-baseline-overlay.png' "$visual_e2e_file" >/dev/null 2>&1 \
    || emit "visual_baseline_e2e_overlay_snapshot_missing" "$visual_e2e_file" "0" "overlay snapshot"
fi

sort -u "$tmp" -o "$tmp"

mkdir -p "$(dirname "$BASELINE_FILE")"

if [[ "${UPDATE_API_CONTRACT_BASELINE:-}" == "1" ]]; then
  cp "$tmp" "$BASELINE_FILE"
  echo "[api-contract] baseline refreshed: $BASELINE_FILE"
  exit 0
fi

if [[ ! -f "$BASELINE_FILE" ]]; then
  echo "[api-contract] baseline missing: $BASELINE_FILE" >&2
  echo "[api-contract] run: UPDATE_API_CONTRACT_BASELINE=1 ./scripts/check-api-contracts.sh" >&2
  exit 1
fi

if ! diff -u "$BASELINE_FILE" "$tmp" >/tmp/api-contract.diff; then
  echo "[api-contract] violation set changed (baseline drift)." >&2
  cat /tmp/api-contract.diff >&2
  rm -f /tmp/api-contract.diff
  exit 1
fi

rm -f /tmp/api-contract.diff
echo "[api-contract] OK"
