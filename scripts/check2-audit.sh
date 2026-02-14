#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
export ROOT_DIR

fail() {
  echo "[check2] FAIL: $*" >&2
  return 1
}

pass() {
  echo "[check2] ok: $*"
}

must_have_rg() {
  local what="$1"
  local file="$2"
  local pattern="$3"
  if has_rg_matches "$pattern" "$file"; then
    pass "$what"
  else
    ok=0
    fail "$what (missing pattern: $pattern in $file)" || true
  fi
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
slow=0
e2e=0
e2e_coverage="sample"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --slow)
      slow=1
      shift
      ;;
    --e2e)
      e2e=1
      e2e_coverage="sample"
      shift
      ;;
    --e2e-all)
      e2e=1
      e2e_coverage="all"
      shift
      ;;
    -h|--help)
      cat <<'USAGE'
usage: scripts/check2-audit.sh [--slow] [--e2e|--e2e-all]

  --slow  additionally runs docs coverage checks (invokes cargo tests)
  --e2e      runs Playwright E2E suite against docs-app (sample coverage)
  --e2e-all  runs Playwright E2E suite against docs-app (full component coverage)
USAGE
      exit 0
      ;;
    *)
      echo "[check2] FAIL: unknown arg: $1" >&2
      exit 2
      ;;
  esac
done

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

# 5.1) Tree-shaking: component-level feature gates exist (package mode)
components_toml="$ROOT_DIR/crates/ui-components/Cargo.toml"
if has_rg_matches "^all-components\\s*=\\s*\\[" "$components_toml" \
  && has_rg_matches "^component-[a-z0-9_]+\\s*=\\s*\\[\\]\\s*$" "$components_toml"; then
  pass "ui-components defines all-components + per-component features"
else
  ok=0
  fail "ui-components missing all-components/component-* feature definitions in $components_toml" || true
fi

components_lib="$components_src/lib.rs"
if has_rg_matches "\\#\\[cfg\\(feature = \\\"component-" "$components_lib"; then
  pass "ui-components gates component modules behind component-* features"
else
  ok=0
  fail "ui-components missing component-* cfg gates in $components_lib" || true
fi

css_rs="$components_src/css.rs"
if has_rg_matches "\\#\\[cfg\\(feature = \\\"component-" "$css_rs"; then
  pass "ui-components gates CSS aggregation behind component-* features"
else
  ok=0
  fail "ui-components css aggregation is not feature-gated in $css_rs" || true
fi

# 6) styles contract: no hardcoded hex colors; no HTML style="..."; styles aggregated
if has_rg_matches "#[0-9a-fA-F]{3,8}\\b" "$components_src" --glob "*/styles.rs"; then
  ok=0
  fail "ui-components styles.rs contains hex colors (disallowed by check2)" || true
else
  pass "ui-components styles.rs contains no hex colors"
fi

if has_rg_matches "style=\\\"" "$components_src" --glob "*/view.rs"; then
  ok=0
  fail "ui-components view.rs contains inline style=\\\"...\\\" (disallowed by check2)" || true
else
  pass "ui-components view.rs contains no style=\\\"...\\\""
fi

if has_rg_matches "(format!\\(|String::|to_string\\()" "$components_src" --glob "*/styles.rs"; then
  ok=0
  fail "ui-components styles.rs appears to build CSS dynamically (disallowed by check2)" || true
else
  pass "ui-components styles.rs is static (no obvious dynamic builders)"
fi

if rg -n --pcre2 --glob "*/styles.rs" "var\\(--(?!ui-)" "$components_src" >/dev/null 2>&1; then
  ok=0
  fail "ui-components styles.rs contains CSS vars not under --ui-* (disallowed by check2)" || true
else
  pass "ui-components styles.rs uses only --ui-* CSS vars (grep-based check)"
fi

if rg -n --glob "*/styles.rs" -- '--tw-|@apply|\btw-' "$components_src" >/dev/null 2>&1; then
  ok=0
  fail "ui-components styles.rs appears to use utility-first patterns (grep-based check)" || true
else
  pass "ui-components styles.rs does not use obvious utility-first patterns"
fi

if rg -n --pcre2 --glob "*/styles.rs" "\\b(rgb|hsl)a?\\(" "$components_src" >/dev/null 2>&1; then
  ok=0
  fail "ui-components styles.rs contains rgb()/hsl() colors (disallowed by check2)" || true
else
  pass "ui-components styles.rs contains no rgb()/hsl() colors"
fi

# Ensure every component-local styles.rs with embedded raw string CSS is aggregated in css.rs.
python3 - <<'PY'
import os
root = os.environ["ROOT_DIR"]
components_src = os.path.join(root, "crates/ui-components/src")
css_rs = os.path.join(components_src, "css.rs")
css = open(css_rs, "r", encoding="utf-8").read()

missing = []
for comp in sorted(os.listdir(components_src)):
    d = os.path.join(components_src, comp)
    if not os.path.isdir(d):
        continue
    styles = os.path.join(d, "styles.rs")
    if not os.path.exists(styles):
        continue
    txt = open(styles, "r", encoding="utf-8").read()
    if "pub const CSS" not in txt:
        continue
    # heuristic: only require aggregation for components that define embedded CSS text
    if 'r#"' not in txt and 'r"' not in txt:
        continue
    if f"crate::{comp}::styles::CSS" in css or f"crate::{comp}::CSS" in css:
        continue
    missing.append(comp)

if missing:
    print("[check2] FAIL: component styles.rs not aggregated in css.rs:", ", ".join(missing))
    raise SystemExit(1)
print("[check2] ok: component-local CSS aggregated in css.rs")
PY
if [[ $? -ne 0 ]]; then
  ok=0
fi

# 7) UiRoot injection: base CSS + theme vars + component CSS are injected
root_rs="$components_src/root.rs"
must_have_rg "UiRoot injects BASE_CSS" "$root_rs" "BASE_CSS"
must_have_rg "UiRoot injects theme CSS variables" "$root_rs" "to_css_variables\\(\\)"
must_have_rg "UiRoot injects component CSS" "$root_rs" "push_components_css\\("

# 8) semantics coverage: every check2.md component directory has a semantics test file
python3 - <<'PY'
import os
from pathlib import Path

root = Path(os.environ["ROOT_DIR"])
components_root = root / "crates/ui-components"
src = components_root / "src"
tests = components_root / "tests"

check2_dirs = [p.name for p in sorted(src.iterdir()) if p.is_dir() and (p / "check2.md").exists()]
test_names = {p.name for p in tests.glob("*.rs")}

missing = []
for name in check2_dirs:
    if f"{name}_semantics.rs" in test_names:
        continue
    if f"{name}_module_semantics.rs" in test_names:
        continue
    missing.append(name)

if missing:
    print("[check2] FAIL: missing semantics tests for:", ", ".join(missing))
    raise SystemExit(1)
print("[check2] ok: every check2.md module has a semantics test")
PY
if [[ $? -ne 0 ]]; then
  ok=0
fi

# 8.1) unit tests coverage: every component logic.rs has at least one #[test]
python3 - <<'PY'
import os
from pathlib import Path

root = Path(os.environ["ROOT_DIR"])
src = root / "crates/ui-components/src"

missing = []
for d in sorted(src.iterdir()):
    if not d.is_dir():
        continue
    logic = d / "logic.rs"
    if not logic.exists():
        continue
    txt = logic.read_text(encoding="utf-8")
    if "#[test]" not in txt:
        missing.append(d.name)

if missing:
    print("[check2] FAIL: logic.rs missing #[test] coverage in:", ", ".join(missing))
    raise SystemExit(1)
print("[check2] ok: every component logic.rs contains #[test] coverage (grep-based check)")
PY
if [[ $? -ne 0 ]]; then
  ok=0
fi

# 9) docs coverage (slow): docs-app catalog covers public ui-components modules + at least one Playground per page
if [[ "$slow" -eq 1 ]]; then
  if cargo test -p docs-app component_catalog_covers_public_component_modules >/dev/null \
    && cargo test -p docs-app every_component_doc_page_renders_at_least_one_playground >/dev/null; then
    pass "docs-app covers every public ui-components module (via tests)"
  else
    ok=0
    fail "docs-app coverage tests failed (run: cargo test -p docs-app component_catalog_covers_public_component_modules ; cargo test -p docs-app every_component_doc_page_renders_at_least_one_playground)" || true
  fi
fi

if [[ "$e2e" -eq 1 ]]; then
  if E2E_COVERAGE="$e2e_coverage" "$ROOT_DIR/scripts/e2e-docs-app.sh"; then
    pass "docs-app Playwright E2E suite passes (coverage: $e2e_coverage)"
  else
    ok=0
    fail "docs-app Playwright E2E suite failed (coverage: $e2e_coverage; run: E2E_COVERAGE=$e2e_coverage ./scripts/e2e-docs-app.sh)" || true
  fi
fi

if [[ "$ok" -ne 1 ]]; then
  echo "[check2] audit: FAILED" >&2
  exit 1
fi

echo "[check2] audit: PASSED"
