#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import re
import sys
from pathlib import Path
from collections import defaultdict

ROOT = Path.cwd()
PAGES_RS = ROOT / "apps/docs-app/src/pages/components/pages.rs"
PAGES_DIR = ROOT / "apps/docs-app/src/pages/components/pages"

MATRIX_TITLE_RE = re.compile(r"(state\s*matrix|comparison|对比|状态矩阵)", re.IGNORECASE)
SHOWCASE_TITLE_RE = re.compile(r"(hello|default|quick start|基础|默认)", re.IGNORECASE)
IDENT_RE = re.compile(r"^[a-z_][a-z0-9_]*$")

# Components that are docs-only concepts rather than a render component fn.
SOURCE_CHECK_NA = {"ThemeVisualBaseline", "AiSpace"}


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError as err:
        print(f"[playground-standard] failed to read {path}: {err}", file=sys.stderr)
        sys.exit(2)


def extract_module_aliases(source: str) -> dict[str, str]:
    aliases: dict[str, str] = {}
    m = re.search(r"use\s+self::\s*\{(?P<body>.*?)\};", source, re.S)
    if not m:
        return aliases
    body = m.group("body")
    for raw in body.split(","):
        item = raw.strip()
        if not item:
            continue
        if " as " in item:
            mod, alias = [x.strip() for x in item.split(" as ", 1)]
            aliases[alias] = mod
        else:
            aliases[item] = item
    return aliases


def resolve_module_name(module: str, aliases: dict[str, str]) -> str:
    return aliases.get(module, module)


def split_macro_args(inner: str) -> list[str]:
    args: list[str] = []
    buf: list[str] = []
    in_quotes = False
    for ch in inner:
        if ch == '"':
            in_quotes = not in_quotes
            buf.append(ch)
            continue
        if ch == "," and not in_quotes:
            args.append("".join(buf).strip())
            buf.clear()
            continue
        buf.append(ch)
    tail = "".join(buf).strip()
    if tail:
        args.append(tail)
    return args


def extract_quoted_string(raw: str) -> str | None:
    m = re.search(r'"([^"]*)"', raw)
    if not m:
        return None
    return m.group(1)


def parse_catalog_entries(source: str) -> list[dict[str, str]]:
    aliases = extract_module_aliases(source)
    entries: list[dict[str, str]] = []

    macro_calls = re.finditer(r"component_doc!\((?P<body>.*?)\)", source, re.S)
    for m in macro_calls:
        body = m.group("body")
        args = split_macro_args(body)
        if len(args) != 4:
            continue
        name = extract_quoted_string(args[0])
        slug = extract_quoted_string(args[1])
        if not name:
            continue
        if not slug:
            continue
        page = args[3].strip()
        if "::" not in page:
            continue
        module, func = [x.strip() for x in page.split("::", 1)]
        entries.append(
            {
                "name": name,
                "slug": slug,
                "module": resolve_module_name(module, aliases),
                "func": func,
            }
        )
    return entries


def slice_fn_block(source: str, fn_name: str) -> str | None:
    patterns = [f"pub(super) fn {fn_name}(", f"pub fn {fn_name}("]
    start = -1
    for p in patterns:
        start = source.find(p)
        if start != -1:
            break
    if start == -1:
        return None

    after = source[start + 1 :]
    next_pub_super = after.find("\npub(super) fn ")
    next_pub = after.find("\npub fn ")
    ends = [len(source)]
    if next_pub_super != -1:
        ends.append(start + 1 + next_pub_super)
    if next_pub != -1:
        ends.append(start + 1 + next_pub)
    end = min(ends)
    return source[start:end]


def delegate_target(block: str) -> tuple[str, str] | None:
    # wrapper-style delegation: `module::func()`
    # Support plain wrappers and wrappers with local marker bindings before delegation.
    for line in block.splitlines():
        trimmed = line.strip().rstrip(";")
        m = re.search(r"\b([A-Za-z0-9_]+)::([A-Za-z0-9_]+)\(\)$", trimmed)
        if m:
            return m.group(1), m.group(2)
    return None


def resolve_component_fn_block(module: str, func: str, module_cache: dict[str, str]) -> str:
    cur_module = module
    cur_func = func
    block: str | None = None

    for _ in range(6):
        if cur_module not in module_cache:
            module_cache[cur_module] = read_text(PAGES_DIR / f"{cur_module}.rs")
        source = module_cache[cur_module]
        block = slice_fn_block(source, cur_func)
        if block is None:
            raise RuntimeError(f"missing function {cur_module}::{cur_func}")
        if iter_playground_tags(block):
            return block
        target = delegate_target(block)
        if not target:
            return block
        cur_module, cur_func = target

    return block or ""


def iter_playground_tags(block: str) -> list[str]:
    tags: list[str] = []
    i = 0
    while True:
        j = block.find("<Playground", i)
        if j == -1:
            break
        line_start = block.rfind("\n", 0, j) + 1
        line_prefix = block[line_start:j]
        if line_prefix.lstrip().startswith("//"):
            i = j + len("<Playground")
            continue
        if j > 0 and block[j - 1] in {'"', "'"}:
            i = j + len("<Playground")
            continue
        k = block.find(">", j)
        if k == -1:
            break
        tags.append(block[j:k])
        i = k + 1
    return tags


def extract_title(tag: str) -> str:
    m = re.search(r'title="([^"]+)"', tag)
    return m.group(1) if m else ""


def extract_test_config_idents(tag: str) -> list[str]:
    out: list[str] = []
    for m in re.finditer(r"test_config_signal=([A-Za-z_][A-Za-z0-9_]*)", tag):
        out.append(m.group(1))
    return out


def extract_code_signal_ident(tag: str) -> str | None:
    m = re.search(r"code_signal=([A-Za-z_][A-Za-z0-9_]*)", tag)
    if not m:
        return None
    return m.group(1)


def source_candidates() -> dict[str, list[tuple[Path, str]]]:
    mapping: dict[str, list[tuple[Path, str]]] = defaultdict(list)
    search_roots = [ROOT / "components", ROOT / "crates"]
    for base in search_roots:
        if not base.exists():
            continue
        for path in base.rglob("*.rs"):
            path_text = path.as_posix()
            if "target/" in path_text:
                continue
            if "/tests/" in path_text or "/test/" in path_text or "/benches/" in path_text:
                continue
            if "/src/" not in path_text:
                continue
            text = read_text(path)
            for m in re.finditer(r"\bpub\s+fn\s+([A-Z][A-Za-z0-9_]*)\b", text):
                mapping[m.group(1)].append((path, text))
    return mapping


def skip_ws(source: str, pos: int) -> int:
    while pos < len(source) and source[pos].isspace():
        pos += 1
    return pos


def skip_angle_group(source: str, pos: int) -> int | None:
    if pos >= len(source) or source[pos] != "<":
        return pos
    depth = 0
    i = pos
    while i < len(source):
        ch = source[i]
        if ch == "<":
            depth += 1
        elif ch == ">":
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    return None


def split_top_level_commas(source: str) -> list[str]:
    out: list[str] = []
    buf: list[str] = []
    stack: list[str] = []
    in_string = False
    is_escaped = False
    pairs = {")": "(", "]": "[", "}": "{", ">": "<"}

    for ch in source:
        if in_string:
            buf.append(ch)
            if is_escaped:
                is_escaped = False
                continue
            if ch == "\\":
                is_escaped = True
                continue
            if ch == '"':
                in_string = False
            continue

        if ch == '"':
            in_string = True
            buf.append(ch)
            continue

        if ch in "([{<":
            stack.append(ch)
            buf.append(ch)
            continue
        if ch in ")]}>":
            if stack and stack[-1] == pairs[ch]:
                stack.pop()
            buf.append(ch)
            continue
        if ch == "," and not stack:
            out.append("".join(buf).strip())
            buf.clear()
            continue
        buf.append(ch)

    tail = "".join(buf).strip()
    if tail:
        out.append(tail)
    return out


def strip_leading_attrs(param: str) -> str:
    s = param.lstrip()
    while s.startswith("#["):
        i = 2
        depth = 1
        while i < len(s):
            ch = s[i]
            if ch == "[":
                depth += 1
            elif ch == "]":
                depth -= 1
                if depth == 0:
                    i += 1
                    break
            i += 1
        s = s[i:].lstrip()
    return s


def extract_fn_params(source: str, fn_name: str) -> str:
    m = re.search(rf"\bpub\s+fn\s+{re.escape(fn_name)}\b", source)
    if not m:
        return ""
    pos = skip_ws(source, m.end())
    pos_after_generics = skip_angle_group(source, pos)
    if pos_after_generics is None:
        return ""
    pos = skip_ws(source, pos_after_generics)
    if pos >= len(source) or source[pos] != "(":
        return ""

    depth = 0
    params_chars: list[str] = []
    for ch in source[pos + 1 :]:
        if ch == "(":
            depth += 1
            params_chars.append(ch)
            continue
        if ch == ")":
            if depth == 0:
                break
            depth -= 1
            params_chars.append(ch)
            continue
        params_chars.append(ch)

    return "".join(params_chars)


def extract_props_from_signature(source: str, fn_name: str) -> list[str]:
    params = extract_fn_params(source, fn_name)
    if not params:
        return []

    out: list[str] = []
    for chunk in split_top_level_commas(params):
        cleaned = strip_leading_attrs(chunk).strip()
        if not cleaned:
            continue
        if ":" not in cleaned:
            continue
        name = cleaned.split(":", 1)[0].strip()
        if name.startswith("mut "):
            name = name[len("mut ") :].strip()
        if name.startswith("ref "):
            name = name[len("ref ") :].strip()
        if IDENT_RE.match(name):
            out.append(name)
    return out


def extract_signal_snippet(block: str, ident: str) -> str:
    m = re.search(rf"\blet\s+{re.escape(ident)}\s*=\s*Signal::derive", block)
    if not m:
        return ""
    start = m.start()
    open_brace = block.find("{", m.end())
    if open_brace == -1:
        return ""

    depth = 0
    in_string = False
    is_escaped = False
    end = -1
    for i, ch in enumerate(block[open_brace:], start=open_brace):
        if in_string:
            if is_escaped:
                is_escaped = False
            elif ch == "\\":
                is_escaped = True
            elif ch == '"':
                in_string = False
            continue
        if ch == '"':
            in_string = True
            continue
        if ch == "{":
            depth += 1
            continue
        if ch == "}":
            depth -= 1
            if depth == 0:
                end = i + 1
                break
    if end == -1:
        return ""
    return block[start:end]


def camel_to_kebab(name: str) -> str:
    step1 = re.sub(r"(.)([A-Z][a-z]+)", r"\1-\2", name)
    step2 = re.sub(r"([a-z0-9])([A-Z])", r"\1-\2", step1)
    return step2.lower()


def main() -> int:
    pages_source = read_text(PAGES_RS)
    entries = parse_catalog_entries(pages_source)
    if not entries:
        print("[playground-standard] no catalog entries found", file=sys.stderr)
        return 2

    module_cache: dict[str, str] = {}
    sources = source_candidates()
    errors: list[str] = []

    for entry in entries:
        name = entry["name"]
        slug = entry["slug"]
        module = entry["module"]
        func = entry["func"]
        label = f"{name} ({module}::{func})"
        try:
            block = resolve_component_fn_block(module, func, module_cache)
        except RuntimeError as err:
            errors.append(f"{label}: {err}")
            continue

        tags = iter_playground_tags(block)
        if not tags:
            errors.append(f"{label}: no <Playground> found")
            continue

        titles = [extract_title(tag) for tag in tags]
        has_showcase = any("controls=" not in tag for tag in tags) and any(
            SHOWCASE_TITLE_RE.search(title) for title in titles
        )
        has_workbench = any(
            ("controls=" in tag) and ("test_config_signal=" in tag) for tag in tags
        )
        has_matrix = any(MATRIX_TITLE_RE.search(title) for title in titles)

        first_tag = tags[0]
        first_title = titles[0] if titles else ""
        first_is_showcase = ("controls=" not in first_tag) and bool(
            SHOWCASE_TITLE_RE.search(first_title)
        )
        if not first_is_showcase:
            errors.append(
                f"{label}: first playground must be showcase (no controls + Hello/Default-style title)"
            )

        if len(tags) < 2:
            errors.append(
                f"{label}: second playground missing (expected config workbench with controls + test_config_signal)"
            )
        else:
            second_tag = tags[1]
            second_is_workbench = ("controls=" in second_tag) and (
                "test_config_signal=" in second_tag
            )
            if not second_is_workbench:
                errors.append(
                    f"{label}: second playground must be config workbench (controls + test_config_signal)"
                )

        if not has_showcase:
            errors.append(
                f"{label}: missing simple showcase playground (expected Hello/Default-style title + no controls)"
            )
        if not has_workbench:
            errors.append(
                f"{label}: missing config workbench (expected controls + test_config_signal)"
            )
        if not has_matrix:
            errors.append(
                f"{label}: missing multi-parameter comparison playground (State Matrix / Comparison)"
            )
        if has_showcase and has_workbench and has_matrix:
            showcase_idx = next(
                i
                for i, tag in enumerate(tags)
                if ("controls=" not in tag)
                and SHOWCASE_TITLE_RE.search(extract_title(tag))
            )
            workbench_idx = next(
                i
                for i, tag in enumerate(tags)
                if ("controls=" in tag) and ("test_config_signal=" in tag)
            )
            matrix_idx = next(
                i
                for i, title in enumerate(titles)
                if MATRIX_TITLE_RE.search(title)
            )
            if not (showcase_idx < workbench_idx < matrix_idx):
                errors.append(
                    f"{label}: invalid playground order (expected Showcase -> Workbench -> Matrix)"
                )

        for tag in tags:
            if "controls=" not in tag or "test_config_signal=" not in tag:
                continue
            code_ident = extract_code_signal_ident(tag)
            if not code_ident:
                errors.append(f"{label}: workbench missing code_signal")
                continue
            code_snippet = extract_signal_snippet(block, code_ident)
            if not code_snippet:
                errors.append(
                    f"{label}: cannot resolve code_signal body `{code_ident}` for workbench"
                )
                continue
            if ".get(" not in code_snippet and ".get()" not in code_snippet:
                errors.append(
                    f"{label}: workbench code_signal `{code_ident}` is not reactive to config (missing .get())"
                )

        if name in SOURCE_CHECK_NA:
            continue

        config_text_parts: list[str] = []
        for tag in tags:
            if "controls=" not in tag or "test_config_signal=" not in tag:
                continue
            for ident in extract_test_config_idents(tag):
                snippet = extract_signal_snippet(block, ident)
                if snippet:
                    config_text_parts.append(snippet)
        config_text = "\n".join(config_text_parts)

        candidates = sources.get(name, [])
        if not candidates:
            errors.append(f"{label}: cannot locate source `pub fn {name}(...)` for API coverage")
            continue

        callback_props_union: set[str] = set()
        for _, cand_text in candidates:
            for prop in extract_props_from_signature(cand_text, name):
                if prop != "children" and prop.startswith("on_"):
                    callback_props_union.add(prop)

        fallback_slug = camel_to_kebab(name)
        slug_snake = slug.replace("-", "_")
        fallback_slug_snake = fallback_slug.replace("-", "_")

        def candidate_rank(item: tuple[Path, str]) -> tuple[int, int, int, int, int, str]:
            path = item[0].as_posix()
            exact_root = (
                f"/components/{slug}/" in path
                or f"/crates/{slug}/" in path
                or f"/components/{fallback_slug}/" in path
                or f"/crates/{fallback_slug}/" in path
            )
            has_slug_segment = (
                f"/{slug}/" in path
                or f"/{slug_snake}/" in path
                or f"/{fallback_slug}/" in path
                or f"/{fallback_slug_snake}/" in path
            )
            if path.endswith("/src/view.rs"):
                view_bucket = 0
            elif path.endswith("/src/mod.rs"):
                view_bucket = 1
            else:
                view_bucket = 2
            return (
                0 if exact_root else 1,
                0 if has_slug_segment else 1,
                view_bucket,
                len(path),
                len(path.split("/")),
                path,
            )

        source_path: Path | None = None
        props: list[str] = []
        best_key: tuple[float, float, int, int, int] | None = None
        for rank_index, (cand_path, cand_text) in enumerate(
            sorted(candidates, key=candidate_rank)
        ):
            cand_props = extract_props_from_signature(cand_text, name)
            if not cand_props:
                continue
            required = [p for p in cand_props if p != "children"]
            required_count = max(len(required), 1)
            page_hit_count = sum(
                1 for prop in required if ((f"{prop}=" in block) or (f"{prop}:" in block))
            )
            config_hit_count = sum(1 for prop in required if f"{prop}:" in config_text)
            score_key = (
                page_hit_count / required_count,
                config_hit_count / required_count,
                page_hit_count,
                config_hit_count,
                -rank_index,
            )
            if best_key is None or score_key > best_key:
                best_key = score_key
                source_path = cand_path
                props = cand_props

        if source_path is None:
            source_path = sorted(candidates, key=candidate_rank)[0][0]

        if not props:
            errors.append(
                f"{label}: source found at {source_path} but failed to parse props for API coverage"
            )
            continue

        ignored = {"children"}
        required_props = [p for p in props if p not in ignored]

        for prop in required_props:
            prop_used = (f"{prop}=" in block) or (f"{prop}:" in block)
            if not prop_used:
                errors.append(f"{label}: missing API usage for `{prop}` in page block")
                continue

            if not config_text:
                errors.append(
                    f"{label}: missing config signal body for API coverage (prop `{prop}`)"
                )
                continue
            if f"{prop}:" not in config_text:
                errors.append(
                    f"{label}: config preview missing API key `{prop}:` in test_config_signal path"
                )

        for callback_prop in sorted(callback_props_union):
            if f"{callback_prop}:" not in config_text:
                errors.append(
                    f"{label}: callback config missing `{callback_prop}:` in test_config_signal path"
                )

    print("[playground-standard] scope: docs-app catalog components")
    print(f"[playground-standard] checked components: {len(entries)}")
    if not errors:
        print("[playground-standard] PASS")
        return 0

    print(f"[playground-standard] FAIL ({len(errors)} violations)")
    for err in errors:
        print(f" - {err}")
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
PY
