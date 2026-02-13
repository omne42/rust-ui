#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$repo_root"

mode="staged"
if [[ "${1:-}" == "--all" ]]; then
  mode="all"
elif [[ "${1:-}" == "--staged" || -z "${1:-}" ]]; then
  mode="staged"
else
  cat >&2 <<'__USAGE__'
usage: scripts/check-architecture.sh [--staged|--all]
  --staged  check staged snapshot only (default)
  --all     check working tree files
__USAGE__
  exit 2
fi

read_snapshot() {
  local path="$1"
  if [[ "$mode" == "staged" ]]; then
    if git cat-file -e ":$path" 2>/dev/null; then
      git show ":$path"
      return 0
    fi
    return 1
  fi

  [[ -f "$path" ]] || return 1
  cat "$path"
}

snapshot_exists() {
  local path="$1"
  if [[ "$mode" == "staged" ]]; then
    git cat-file -e ":$path" 2>/dev/null
    return $?
  fi
  [[ -f "$path" ]]
}

collect_files() {
  if [[ "$mode" == "staged" ]]; then
    git diff --cached --name-only --diff-filter=ACMR || true
  else
    rg --files -g 'crates/**/*.rs' -g 'apps/**/*.rs' -g 'crates/**/Cargo.toml' -g 'apps/**/Cargo.toml'
  fi
}

mapfile -t files < <(collect_files)
if [[ "${#files[@]}" -eq 0 ]]; then
  exit 0
fi

failures=0
report() {
  local title="$1"
  local detail="$2"
  failures=1
  printf 'architecture-check: %s\n  %s\n' "$title" "$detail" >&2
}

# 1) apps should consume ui-components, not lower layers directly.
for path in "${files[@]}"; do
  [[ "$path" == apps/*/src/* ]] || continue
  content="$(read_snapshot "$path" || true)"
  [[ -n "$content" ]] || continue
  if printf '%s\n' "$content" | rg -n "\bui_(core|headless|theme|motion)::" >/dev/null; then
    report "apps layering violation" "$path imports lower-layer crates (ui_core/ui_headless/ui_theme/ui_motion)."
  fi
done

# 2) ui-core must stay platform-agnostic and independent.
for path in "${files[@]}"; do
  [[ "$path" == crates/ui-core/src/* ]] || continue
  content="$(read_snapshot "$path" || true)"
  [[ -n "$content" ]] || continue

  if printf '%s\n' "$content" | rg -n "\b(web_sys|js_sys|wasm_bindgen)\b|leptos::web_sys|\bwindow\(|\bdocument\(" >/dev/null; then
    report "ui-core purity violation" "$path references platform/web APIs."
  fi

  if printf '%s\n' "$content" | rg -n "\bui_(headless|components|theme|motion)::" >/dev/null; then
    report "ui-core dependency direction violation" "$path references higher-layer internal crates."
  fi
done

# 3) headless/theme/motion dependency direction guardrails.
for path in "${files[@]}"; do
  [[ "$path" == crates/ui-headless/src/* ]] || continue
  content="$(read_snapshot "$path" || true)"
  [[ -n "$content" ]] || continue
  if printf '%s\n' "$content" | rg -n "\bui_(components|theme)::" >/dev/null; then
    report "ui-headless dependency direction violation" "$path references ui_components/ui_theme."
  fi
done

for path in "${files[@]}"; do
  [[ "$path" == crates/ui-theme/src/* ]] || continue
  content="$(read_snapshot "$path" || true)"
  [[ -n "$content" ]] || continue
  if printf '%s\n' "$content" | rg -n "\bui_components::" >/dev/null; then
    report "ui-theme dependency direction violation" "$path references ui_components."
  fi
done

for path in "${files[@]}"; do
  [[ "$path" == crates/ui-motion/src/* ]] || continue
  content="$(read_snapshot "$path" || true)"
  [[ -n "$content" ]] || continue
  if printf '%s\n' "$content" | rg -n "\bui_components::" >/dev/null; then
    report "ui-motion dependency direction violation" "$path references ui_components."
  fi
done

# 4) ui-components module structure: touched component dirs must keep logic/styles/view split.
#    Alias-only facade modules are exempt (mod.rs contains no module declarations).
component_dirs=()
for path in "${files[@]}"; do
  [[ "$path" == crates/ui-components/src/*/* ]] || continue
  comp="${path#crates/ui-components/src/}"
  comp="${comp%%/*}"
  [[ -n "$comp" ]] || continue
  case "$comp" in
    css|a11y|active_highlight|overlay_open|presence) continue ;;
  esac
  component_dirs+=("$comp")
done

if [[ "${#component_dirs[@]}" -gt 0 ]]; then
  mapfile -t uniq_component_dirs < <(printf '%s\n' "${component_dirs[@]}" | sort -u)
  for comp in "${uniq_component_dirs[@]}"; do
    mod_path="crates/ui-components/src/$comp/mod.rs"
    mod_content="$(read_snapshot "$mod_path" || true)"
    [[ -n "$mod_content" ]] || continue

    if printf '%s\n' "$mod_content" | rg -n "^mod render;" >/dev/null; then
      report "ui-components naming violation" "$mod_path uses render.rs; use view.rs."
    fi

    if ! printf '%s\n' "$mod_content" | rg -q "^(pub\s+mod|mod)\s+[A-Za-z_][A-Za-z0-9_]*\s*;"; then
      continue
    fi

    for req in logic.rs styles.rs view.rs; do
      req_path="crates/ui-components/src/$comp/$req"
      if ! snapshot_exists "$req_path"; then
        report "ui-components structure violation" "$comp is missing required file: $req"
      fi
    done
  done
fi

# 5) Block introducing render.rs files in component modules.
for path in "${files[@]}"; do
  [[ "$path" == crates/ui-components/src/*/render.rs ]] || continue
  report "ui-components forbidden file" "$path should be renamed to view.rs."
done

if [[ "$failures" -ne 0 ]]; then
  cat >&2 <<'__FAIL__'
architecture-check: failed.
Fix layering/module issues above before commit.
__FAIL__
  exit 1
fi

exit 0
