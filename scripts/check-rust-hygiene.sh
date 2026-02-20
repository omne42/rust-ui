#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

MAX_REPORT_LINES=80

./scripts/check-api-contracts.sh

unwrap_expect_count=0
let_underscore_count=0
string_clone_count=0

unwrap_expect_lines=()
let_underscore_lines=()
string_clone_lines=()

collect_non_test_source() {
  local file="$1"
  awk '
    BEGIN {
      in_cfg_test = 0;
      brace_depth = 0;
    }

    {
      line = $0;

      if (!in_cfg_test && line ~ /^[[:space:]]*#\[cfg\(test\)\]/) {
        pending_cfg_test = 1;
        next;
      }

      if (pending_cfg_test && line ~ /^[[:space:]]*mod[[:space:]]+[A-Za-z0-9_]+[[:space:]]*\{[[:space:]]*$/) {
        in_cfg_test = 1;
        pending_cfg_test = 0;
        brace_depth = 1;
        next;
      }

      if (pending_cfg_test) {
        pending_cfg_test = 0;
      }

      if (in_cfg_test) {
        open_count = gsub(/\{/, "{", line);
        close_count = gsub(/\}/, "}", line);
        brace_depth += open_count - close_count;
        if (brace_depth <= 0) {
          in_cfg_test = 0;
        }
        next;
      }

      print line;
    }
  ' "$file"
}

strip_rust_non_code() {
  perl -0777 -pe '
    s{//.*$}{}mg;
    s{/\*.*?\*/}{}gs;
    s{r(#+)?".*?"\1}{"__RAW_STR__"}gs;
    s{b"(?:\\.|[^"\\])*"}{"__STR__"}gs;
    s{"(?:\\.|[^"\\])*"}{"__STR__"}gs;
    s{'"'"'(?:\\.|[^'"'"'\\])+'"'"'}{'"'"'__CHR__'"'"'}gs;
  '
}

record_hits() {
  local kind="$1"
  local file="$2"
  local pattern="$3"
  local -n out_lines="$4"
  local -n out_count="$5"

  local sanitized
  sanitized="$(collect_non_test_source "$file")"
  local scan_input="$sanitized"
  if [[ "$kind" == "string_clone" ]]; then
    scan_input="$(printf '%s\n' "$sanitized" | strip_rust_non_code)"
  fi
  local matches
  matches="$(printf '%s\n' "$scan_input" | grep -nE "$pattern" || true)"

  if [[ -z "$matches" ]]; then
    return 0
  fi

  local line
  while IFS= read -r line; do
    out_count=$((out_count + 1))
    if [[ "${#out_lines[@]}" -lt "$MAX_REPORT_LINES" ]]; then
      out_lines+=("${file}:${line}")
    fi
  done <<<"$matches"
}

while IFS= read -r file; do
  record_hits \
    "unwrap/expect" \
    "$file" \
    '\.(unwrap|unwrap_err|expect)\s*\(' \
    unwrap_expect_lines \
    unwrap_expect_count

  record_hits \
    "let_underscore" \
    "$file" \
    '^[[:space:]]*let[[:space:]]+_[[:space:]]*=' \
    let_underscore_lines \
    let_underscore_count

  record_hits \
    "string_clone" \
    "$file" \
    '(\.to_owned\(\)|String::from\()' \
    string_clone_lines \
    string_clone_count

  record_hits \
    "string_clone" \
    "$file" \
    '([A-Za-z0-9_.()]*_(label|name|title|group|route|path|class|id|key|text|placeholder|src|role|description|icon|token|base|fallback|default|host|href|alt|aria|tooltip)\.to_string\(\)|\b(label|name|title|group|route|path|class|id|key|text|placeholder|src|role|description|icon|token|base|fallback|default|host|href|alt|aria|tooltip)\.to_string\(\)|\.(as_ref|as_str|trim|trim_start|trim_end|trim_start_matches|trim_end_matches)\([^)]*\)\.to_string\(\))' \
    string_clone_lines \
    string_clone_count
done < <(find crates apps -type f -name '*.rs' -path '*/src/*' | sort)

status=0

# Layer boundary guard: state primitives must stay framework/DOM-free.
if rg -n '\bleptos\b|web_sys|wasm_bindgen|gloo|js_sys' \
  crates/ui-state-primitives/src \
  --glob '!**/test/**' >/dev/null; then
  status=1
  echo "[rust-hygiene] ui-state-primitives must not depend on framework/DOM bindings" >&2
  rg -n '\bleptos\b|web_sys|wasm_bindgen|gloo|js_sys' \
    crates/ui-state-primitives/src \
    --glob '!**/test/**' >&2 || true
fi

# Layer boundary guard: ui-headless must not render components directly.
if rg -n '#\[component\]|view!\s*\{' crates/ui-headless/src --glob '!**/test/**' >/dev/null; then
  status=1
  echo "[rust-hygiene] ui-headless must not contain #[component] or view! rendering" >&2
  rg -n '#\[component\]|view!\s*\{' crates/ui-headless/src --glob '!**/test/**' >&2 || true
fi

if [[ "$unwrap_expect_count" -gt 0 ]]; then
  status=1
  echo "[rust-hygiene] forbidden unwrap/expect in non-test code: ${unwrap_expect_count}" >&2
  printf '%s\n' "${unwrap_expect_lines[@]}" >&2
  if [[ "$unwrap_expect_count" -gt "${#unwrap_expect_lines[@]}" ]]; then
    echo "[rust-hygiene] ... and $((unwrap_expect_count - ${#unwrap_expect_lines[@]})) more" >&2
  fi
fi

if [[ "$let_underscore_count" -gt 0 ]]; then
  status=1
  echo "[rust-hygiene] forbidden let _ = in non-test code: ${let_underscore_count}" >&2
  printf '%s\n' "${let_underscore_lines[@]}" >&2
  if [[ "$let_underscore_count" -gt "${#let_underscore_lines[@]}" ]]; then
    echo "[rust-hygiene] ... and $((let_underscore_count - ${#let_underscore_lines[@]})) more" >&2
  fi
fi

if [[ "$string_clone_count" -gt 0 ]]; then
  status=1
  echo "[rust-hygiene] string clone hotspots (prefer Cow<'static, str>): ${string_clone_count}" >&2
  printf '%s\n' "${string_clone_lines[@]}" >&2
  if [[ "$string_clone_count" -gt "${#string_clone_lines[@]}" ]]; then
    echo "[rust-hygiene] ... and $((string_clone_count - ${#string_clone_lines[@]})) more" >&2
  fi
fi

if [[ "$status" -ne 0 ]]; then
  echo "[rust-hygiene] failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn" >&2
  exit "$status"
fi

echo "[rust-hygiene] OK"
