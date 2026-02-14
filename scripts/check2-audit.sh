#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

fail() {
  echo "[check2] FAIL: $*" >&2
  return 1
}

pass() {
  echo "[check2] ok: $*"
}

section_has_entries() {
  local toml="$1"
  local section="$2"
  awk -v section="[$section]" '
    $0 == section { in_section=1; next }
    in_section && $0 ~ /^\[/ { exit }
    in_section && $0 !~ /^[[:space:]]*(#|$)/ { found=1; exit }
    END { exit found ? 0 : 1 }
  ' "$toml"
}

has_rg_matches() {
  local pattern="$1"
  shift
  rg -n "$pattern" "$@" >/dev/null 2>&1
}

ok=1

# 1) ui-core purity (no deps; no leptos/dom/web-sys usage)
core_toml="$ROOT_DIR/crates/ui-core/Cargo.toml"
if section_has_entries "$core_toml" "dependencies"; then
  ok=0
  fail "ui-core has dependencies (expected none): $core_toml" || true
else
  pass "ui-core has no dependencies"
fi

if has_rg_matches "\\b(leptos|web_sys|wasm_bindgen)\\b" "$ROOT_DIR/crates/ui-core/src"; then
  ok=0
  fail "ui-core source references web-only crates/types (leptos/web_sys/wasm_bindgen)" || true
else
  pass "ui-core source is web-agnostic"
fi

# 2) ui-headless: web/ssr mutual exclusion guard + no theme/motion deps
headless_lib="$ROOT_DIR/crates/ui-headless/src/lib.rs"
headless_toml="$ROOT_DIR/crates/ui-headless/Cargo.toml"

if has_rg_matches "compile_error!\\(" "$headless_lib" && has_rg_matches "feature = \"web\".*feature = \"ssr\"" "$headless_lib"; then
  pass "ui-headless has web/ssr mutual exclusion guard"
else
  ok=0
  fail "ui-headless missing web/ssr mutual exclusion guard in $headless_lib" || true
fi

if has_rg_matches "^ui-(motion|theme)\\s*=" "$headless_toml"; then
  ok=0
  fail "ui-headless depends on ui-motion/ui-theme (should not): $headless_toml" || true
else
  pass "ui-headless does not depend on ui-motion/ui-theme"
fi

# 3) ui-motion: non-wasm no-op stubs exist (SSR/tooling safe)
motion_lib="$ROOT_DIR/crates/ui-motion/src/lib.rs"
if has_rg_matches "\\#\\[cfg\\(not\\(target_arch = \"wasm32\"\\)\\)\\]" "$motion_lib" \
  && has_rg_matches "^pub mod web \\{" "$motion_lib"; then
  pass "ui-motion provides non-wasm web no-op stubs"
else
  ok=0
  fail "ui-motion missing non-wasm web no-op stubs in $motion_lib" || true
fi

# 4) ui-theme: no web-only deps/types
theme_toml="$ROOT_DIR/crates/ui-theme/Cargo.toml"
if has_rg_matches "\\b(leptos|web-sys|web_sys|wasm-bindgen|wasm_bindgen)\\b" "$theme_toml"; then
  ok=0
  fail "ui-theme depends on web-only crates/types (should not): $theme_toml" || true
else
  pass "ui-theme has no web-only deps"
fi

# 5) ui-components: no obvious web-sys types in pub surface (best-effort grep)
components_src="$ROOT_DIR/crates/ui-components/src"
if has_rg_matches "\\bpub\\b[^\\n]*(leptos::web_sys|web_sys)::" "$components_src"; then
  ok=0
  fail "ui-components appears to expose web_sys types in pub items (grep-based check)" || true
else
  pass "ui-components pub items do not mention web_sys (grep-based check)"
fi

if [[ "$ok" -ne 1 ]]; then
  echo "[check2] audit: FAILED" >&2
  exit 1
fi

echo "[check2] audit: PASSED"
