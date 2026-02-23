use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use ui_test_support::source_contract;

fn docs_pages_rs() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages/components/pages.rs")
}

fn docs_page_module_path(module: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/pages/components/pages")
        .join(format!("{module}.rs"))
}

fn docs_page_submodule_path(module: &str, submodule: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/pages/components/pages")
        .join(module)
        .join(format!("{submodule}.rs"))
}

fn read_actions_source() -> String {
    let root_path = docs_page_module_path("actions");
    let mut source = source_contract::source_from_path(&root_path);

    let mut files = Vec::new();
    walk_rs_files(&component_pages_root().join("actions"), &mut files);
    files.sort();
    for file in files {
        source.push('\n');
        source.push_str(&source_contract::source_from_path(&file));
    }

    source
}

fn component_pages_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages/components/pages")
}

fn docs_only_page_names() -> HashSet<&'static str> {
    HashSet::from(["ThemeVisualBaseline"])
}

fn shared_page_title_names() -> HashSet<&'static str> {
    HashSet::from(["ThemeVisualBaseline", "AccordionItem", "AiSpace"])
}

fn walk_rs_files(root: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(root).unwrap_or_else(|err| {
        panic!("failed to read {root:?}: {err}");
    });

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            walk_rs_files(&path, out);
            continue;
        }

        if path.extension().and_then(|ext| ext.to_str()) == Some("rs")
            && !path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("part_"))
        {
            out.push(path);
        }
    }
}

#[derive(Clone, Debug)]
struct CatalogEntry {
    name: String,
    module: String,
    func: String,
}

fn extract_quoted_string(input: &str) -> Option<String> {
    let start = input.find('"')?;
    let end = input[start + 1..].find('"')?;
    Some(input[start + 1..start + 1 + end].to_string())
}

fn extract_module_aliases(source: &str) -> HashMap<String, String> {
    let mut aliases = HashMap::new();

    let Some(use_start) = source.find("use self::{") else {
        return aliases;
    };
    let after_use = &source[use_start + "use self::{".len()..];
    let Some(use_end) = after_use.find("};") else {
        return aliases;
    };

    let use_body = &after_use[..use_end];
    for item in use_body.split(',') {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }

        if let Some((module, alias)) = item.split_once(" as ") {
            let module = module.trim();
            let alias = alias.trim();
            if !module.is_empty() && !alias.is_empty() {
                aliases.insert(alias.to_string(), module.to_string());
            }
            continue;
        }

        aliases.insert(item.to_string(), item.to_string());
    }

    aliases
}

fn resolve_module_name(module: &str, aliases: &HashMap<String, String>) -> String {
    aliases
        .get(module)
        .cloned()
        .unwrap_or_else(|| module.to_string())
}

fn split_macro_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in input.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                current.push(ch);
            }
            ',' if !in_quotes => {
                args.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }

    if !current.trim().is_empty() {
        args.push(current.trim().to_string());
    }

    args
}

fn parse_component_doc_macro(
    call: &str,
    aliases: &HashMap<String, String>,
) -> Option<CatalogEntry> {
    let trimmed = call.trim();
    let inner = trimmed
        .strip_prefix("component_doc!(")?
        .strip_suffix(")")?
        .trim();

    let args = split_macro_args(inner);
    if args.len() != 4 {
        return None;
    }

    let name = extract_quoted_string(&args[0])?;
    let page = args[3].trim();
    let (module, func) = page.split_once("::")?;

    let module = resolve_module_name(module.trim(), aliases);

    if module.trim().is_empty() || func.trim().is_empty() || name.trim().is_empty() {
        return None;
    }

    Some(CatalogEntry {
        name,
        module,
        func: func.trim().to_string(),
    })
}

fn extract_catalog_entries(source: &str) -> Vec<CatalogEntry> {
    let mut out = Vec::new();
    let aliases = extract_module_aliases(source);

    let mut in_entry = false;
    let mut name: Option<String> = None;
    let mut page: Option<(String, String)> = None;

    let mut in_macro = false;
    let mut macro_buffer = String::new();

    for line in source.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("component_doc!(") {
            in_macro = true;
            macro_buffer.clear();
            macro_buffer.push_str(trimmed);

            if trimmed.ends_with(")") || trimmed.ends_with("),") {
                let macro_call = macro_buffer.trim_end_matches(',');
                if let Some(entry) = parse_component_doc_macro(macro_call, &aliases) {
                    out.push(entry);
                }
                in_macro = false;
            }
            continue;
        }

        if in_macro {
            macro_buffer.push(' ');
            macro_buffer.push_str(trimmed);

            if trimmed.ends_with("),") {
                let macro_call = macro_buffer.trim_end_matches(',');
                if let Some(entry) = parse_component_doc_macro(macro_call, &aliases) {
                    out.push(entry);
                }
                in_macro = false;
            }
            continue;
        }

        if trimmed.starts_with("ComponentDoc {") {
            in_entry = true;
            name = None;
            page = None;
            continue;
        }

        if !in_entry {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("name:") {
            name = extract_quoted_string(rest.trim());
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("page:") {
            let symbol = rest.trim().trim_end_matches(',');
            if let Some((module, func)) = symbol.split_once("::") {
                let module = resolve_module_name(module.trim(), &aliases);
                page = Some((module, func.trim().to_string()));
            }
            continue;
        }

        if trimmed.starts_with("},") {
            in_entry = false;
            let Some(name) = name.take() else {
                continue;
            };
            let Some((module, func)) = page.take() else {
                continue;
            };

            if module.is_empty() || func.is_empty() || name.trim().is_empty() {
                continue;
            }

            out.push(CatalogEntry { name, module, func });
        }
    }

    out
}

fn slice_fn_block<'a>(source: &'a str, fn_name: &str) -> Option<&'a str> {
    let patterns = [
        format!("pub(crate) fn {fn_name}("),
        format!("pub(super) fn {fn_name}("),
        format!("pub fn {fn_name}("),
    ];
    let start = patterns.iter().find_map(|pattern| source.find(pattern))?;

    let after_start = &source[start + 1..];
    let next_pub_crate = after_start
        .find("\npub(crate) fn ")
        .map(|pos| start + 1 + pos);
    let next_pub_super = after_start
        .find("\npub(super) fn ")
        .map(|pos| start + 1 + pos);
    let next_pub = after_start.find("\npub fn ").map(|pos| start + 1 + pos);

    let end = [next_pub_crate, next_pub_super, next_pub]
        .into_iter()
        .flatten()
        .min()
        .unwrap_or(source.len());

    Some(&source[start..end])
}

fn delegate_target(block: &str) -> Option<(String, String)> {
    for line in block.lines() {
        let trimmed = line.trim();
        if !trimmed.contains("::") || !trimmed.ends_with("()") {
            continue;
        }

        let symbol = trimmed.trim_end_matches("()").trim_end_matches(';').trim();
        if let Some((module, func)) = symbol.split_once("::") {
            let module = module.trim();
            let func = func.trim();
            if !module.is_empty() && !func.is_empty() {
                return Some((module.to_string(), func.to_string()));
            }
        }
    }

    None
}

fn load_fn_block(
    module_sources: &mut BTreeMap<String, String>,
    module: &str,
    func: &str,
) -> Option<String> {
    let source = module_sources.entry(module.to_string()).or_insert_with(|| {
        let path = docs_page_module_path(module);
        source_contract::source_from_path(&path)
    });

    if let Some(block) = slice_fn_block(source, func) {
        return Some(block.to_string());
    }

    let submodule = extract_reexport_submodule(source, func)?;
    let sub_key = format!("{module}/{submodule}");
    let sub_source = module_sources.entry(sub_key).or_insert_with(|| {
        let path = docs_page_submodule_path(module, &submodule);
        source_contract::source_from_path(&path)
    });

    slice_fn_block(sub_source, func).map(|block| block.to_string())
}

fn extract_reexport_submodule(source: &str, func: &str) -> Option<String> {
    for line in source.lines() {
        let trimmed = line.trim().trim_end_matches(';');
        let Some(rest) = trimmed.strip_prefix("pub(super) use ") else {
            continue;
        };
        let Some((submodule, target)) = rest.split_once("::") else {
            continue;
        };
        if target.trim() == func {
            return Some(submodule.trim().to_string());
        }
    }

    None
}

#[test]
fn all_component_pages_have_at_least_one_playground() {
    let pages_rs_path = docs_pages_rs();
    let pages_rs_source = source_contract::source_from_path(&pages_rs_path);

    let entries = extract_catalog_entries(&pages_rs_source);
    assert!(
        !entries.is_empty(),
        "no component doc targets found in {pages_rs_path:?}"
    );

    let mut module_sources: BTreeMap<String, String> = BTreeMap::new();

    let docs_only_names = docs_only_page_names();
    let shared_title_names = shared_page_title_names();

    for entry in entries {
        let CatalogEntry { name, module, func } = entry;

        let mut target_module = module.clone();
        let mut target_func = func.clone();
        let mut block = load_fn_block(&mut module_sources, &target_module, &target_func)
            .unwrap_or_else(|| {
                let path = docs_page_module_path(&target_module);
                panic!("missing function `{target_module}::{target_func}` in {path:?}");
            });

        for _ in 0..4 {
            if block.contains("<Playground") {
                break;
            }

            let Some((next_module, next_func)) = delegate_target(&block) else {
                break;
            };

            target_module = next_module;
            target_func = next_func;
            block = load_fn_block(&mut module_sources, &target_module, &target_func)
                .unwrap_or_else(|| {
                    let path = docs_page_module_path(&target_module);
                    panic!("missing function `{target_module}::{target_func}` in {path:?}");
                });
        }

        assert!(
            block.contains("<Playground"),
            "`{module}::{func}` must include at least one `<Playground ...>` section"
        );

        if !docs_only_names.contains(name.as_str()) {
            let component_needle = format!("<{name}");
            assert!(
                block.contains(&component_needle),
                "`{module}::{func}` must include at least one `{component_needle} ...` usage (playground should demo the component itself)"
            );
        }

        if !shared_title_names.contains(name.as_str()) {
            let title_needle = format!("title=\"{name}\"");
            assert!(
                block.contains(&title_needle),
                "`{module}::{func}` should set ComponentPage title to match catalog name (`{title_needle}`)"
            );
        }
    }
}

#[test]
fn playgrounds_with_controls_define_code_signal() {
    let mut files = Vec::new();
    walk_rs_files(&component_pages_root(), &mut files);

    for file in files {
        let source = source_contract::source_from_path(&file);

        let mut scan_from = 0;

        while let Some(rel_start) = source[scan_from..].find("<Playground") {
            let tag_start = scan_from + rel_start;
            let tag_source = &source[tag_start..];
            let Some(rel_end) = tag_source.find('>') else {
                panic!("unterminated <Playground ...> tag in {file:?}");
            };

            let tag = &tag_source[..rel_end];
            if tag.contains("controls=") {
                assert!(
                    tag.contains("code_signal="),
                    "{file:?}: playground with controls must define code_signal for copy-ready dynamic code",
                );
            }

            scan_from = tag_start + rel_end + 1;
        }
    }
}

#[test]
fn actions_dynamic_snippets_inline_props_without_intermediate_let_bindings() {
    let source = read_actions_source();
    let forbidden = [
        "\"let variant = ButtonVariant::",
        "\"let size = ButtonSize::",
        "\"let variant = ToggleButtonVariant::",
        "\"let size = ToggleButtonSize::",
        "\"let disabled = {disabled};",
    ];

    for needle in forbidden {
        assert!(
            !source.contains(needle),
            "actions dynamic playground snippet should not contain `{needle}`",
        );
    }
}

#[test]
fn actions_size_controls_use_baseline_xs_to_xl_tokens() {
    let source = read_actions_source();
    let mut blocks = 0usize;
    let mut scan_from = 0usize;

    while let Some(rel_start) = source[scan_from..].find("let size_options = vec![") {
        let block_start = scan_from + rel_start;
        let block_source = &source[block_start..];
        let Some(rel_end) = block_source.find("];") else {
            panic!("unterminated size_options block in actions.rs");
        };

        let block = &block_source[..rel_end];
        for size_token in ["\"xs\"", "\"s\"", "\"m\"", "\"l\"", "\"xl\""] {
            assert!(
                block.contains(size_token),
                "size_options block must include {size_token}",
            );
        }

        blocks += 1;
        scan_from = block_start + rel_end + 2;
    }

    assert!(blocks > 0, "no size_options block found in actions.rs");
    assert!(
        source.contains("let (open_in_new_tab, set_open_in_new_tab) = signal(false);")
            && source.contains("let (sponsored_rel, set_sponsored_rel) = signal(false);")
            && source.contains("let (attached, set_attached) = signal(false);")
            && source.contains("let (meta_key_index, set_meta_key_index) = signal(Some(0_usize));")
            && source
                .contains("let (key_label_index, set_key_label_index) = signal(Some(0_usize));"),
        "dynamic playground controls must default to component default parameters",
    );
}

fn is_ident_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn snippet_declares_ident(snippet: &str, ident: &str) -> bool {
    snippet.lines().any(|line| {
        let trimmed = line.trim();
        if !trimmed.starts_with("let ") {
            return false;
        }

        trimmed.contains(&format!("let {ident}"))
            || trimmed.contains(&format!("{ident},"))
            || trimmed.contains(&format!("{ident})"))
            || trimmed.contains(&format!("{ident}:"))
            || trimmed.contains(&format!("{ident} ="))
    })
}

fn extract_code_snippets(source: &str) -> Vec<(usize, String)> {
    let mut snippets = Vec::new();
    let mut scan_from = 0usize;

    while let Some(rel_start) = source[scan_from..].find("Signal::derive(move || {") {
        let derive_start = scan_from + rel_start;
        let derive_tail = &source[derive_start..];
        let Some(rel_end) = derive_tail.find("\n    });") else {
            break;
        };
        let derive_end = derive_start + rel_end;
        let block = &source[derive_start..derive_end];

        let mut block_scan = 0usize;
        while let Some(raw_rel_start) = block[block_scan..].find("r#\"") {
            let raw_start = block_scan + raw_rel_start + 3;
            let Some(raw_rel_end) = block[raw_start..].find("\"#") else {
                break;
            };
            let raw_end = raw_start + raw_rel_end;
            let snippet = block[raw_start..raw_end].to_string();
            let line = source[..(derive_start + raw_start)].matches('\n').count() + 1;
            snippets.push((line, snippet));
            block_scan = raw_end + 2;
        }

        scan_from = derive_end + "\n    });".len();
    }

    snippets
}

fn collect_setter_idents(snippet: &str) -> HashSet<String> {
    let mut out = HashSet::new();

    for method in [".set(", ".update("] {
        let mut scan_from = 0usize;
        while let Some(rel) = snippet[scan_from..].find(method) {
            let method_start = scan_from + rel;
            let bytes = snippet.as_bytes();
            if method_start == 0 {
                scan_from = method_start + method.len();
                continue;
            }

            let ident_end = method_start;
            let mut ident_start = ident_end;
            while ident_start > 0 && is_ident_char(bytes[ident_start - 1]) {
                ident_start -= 1;
            }

            if ident_start < ident_end {
                let ident = &snippet[ident_start..ident_end];
                if ident.starts_with("set_") {
                    out.insert(ident.to_string());
                }
            }

            scan_from = method_start + method.len();
        }
    }

    out
}

#[test]
fn all_playground_tags_use_code_signal() {
    let mut files = Vec::new();
    walk_rs_files(&component_pages_root(), &mut files);

    for file in files {
        let source = source_contract::source_from_path(&file);

        let mut scan_from = 0usize;

        while let Some(rel_start) = source[scan_from..].find("<Playground") {
            let tag_start = scan_from + rel_start;
            if tag_start > 0 {
                let prev = source.as_bytes()[tag_start - 1] as char;
                if prev == '"' || prev == '\'' {
                    scan_from = tag_start + "<Playground".len();
                    continue;
                }
            }

            let tag_source = &source[tag_start..];
            let Some(rel_end) = tag_source.find('>') else {
                panic!("unterminated <Playground ...> tag in {file:?}");
            };

            let tag = &tag_source[..rel_end];
            assert!(
                tag.contains("code_signal="),
                "{file:?}: every <Playground ...> must define code_signal",
            );
            assert!(
                !tag.contains(" code="),
                "{file:?}: legacy code= prop is not allowed on <Playground ...>",
            );

            scan_from = tag_start + rel_end + 1;
        }
    }
}

#[test]
fn snippets_are_self_contained_without_external_bindings() {
    let mut files = Vec::new();
    walk_rs_files(&component_pages_root(), &mut files);

    let common_idents = [
        "items",
        "groups",
        "menus",
        "points",
        "labels",
        "nodes",
        "columns",
        "tags",
        "swatches",
        "store",
        "page",
        "mode",
        "anchor_ref",
        "on_action",
        "on_open_change",
        "on_submit",
        "on_clear",
        "on_remove",
        "on_files",
        "on_drop_files",
        "on_exit_complete",
        "finish_exit",
        "toggle",
        "close",
    ];

    for file in files {
        let source = source_contract::source_from_path(&file);

        for (line, snippet) in extract_code_snippets(&source) {
            for placeholder in ["{content}", "{rows}", "{chips}", "{grid}"] {
                assert!(
                    !snippet.contains(placeholder),
                    "{file:?}:{line}: snippet contains unresolved placeholder `{placeholder}`",
                );
            }

            for ident in common_idents {
                let needle = format!("{ident}={ident}");
                if snippet.contains(&needle) {
                    assert!(
                        snippet_declares_ident(&snippet, ident),
                        "{file:?}:{line}: snippet references external `{needle}` without local declaration",
                    );
                }
            }

            for setter in collect_setter_idents(&snippet) {
                assert!(
                    snippet_declares_ident(&snippet, &setter),
                    "{file:?}:{line}: snippet calls `{setter}.set/.update` without local declaration",
                );
            }

            if snippet.contains("open=open_signal") {
                assert!(
                    snippet_declares_ident(&snippet, "open_signal"),
                    "{file:?}:{line}: snippet references external `open_signal`",
                );
            }

            for snippet_line in snippet.lines() {
                let trimmed = snippet_line.trim();
                assert!(
                    !(trimmed.starts_with("let ") && trimmed.contains("_signal: Signal<")),
                    "{file:?}:{line}: snippet should inline `Signal::derive(...)` instead of `{trimmed}`",
                );
            }
        }
    }
}
