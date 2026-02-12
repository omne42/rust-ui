use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

fn docs_pages_rs() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages/components/pages.rs")
}

fn docs_page_module_path(module: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/pages/components/pages")
        .join(format!("{module}.rs"))
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
        format!("pub(super) fn {fn_name}("),
        format!("pub fn {fn_name}("),
    ];
    let start = patterns.iter().find_map(|pattern| source.find(pattern))?;

    let after_start = &source[start + 1..];
    let next_pub_super = after_start
        .find("\npub(super) fn ")
        .map(|pos| start + 1 + pos);
    let next_pub = after_start.find("\npub fn ").map(|pos| start + 1 + pos);

    let end = match (next_pub_super, next_pub) {
        (Some(a), Some(b)) => a.min(b),
        (Some(a), None) => a,
        (None, Some(b)) => b,
        (None, None) => source.len(),
    };

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
        fs::read_to_string(&path).unwrap_or_else(|err| {
            panic!("failed to read {path:?}: {err}");
        })
    });

    slice_fn_block(source, func).map(|block| block.to_string())
}

#[test]
fn all_component_pages_have_at_least_one_playground() {
    let pages_rs_path = docs_pages_rs();
    let pages_rs_source = fs::read_to_string(&pages_rs_path).unwrap_or_else(|err| {
        panic!("failed to read {pages_rs_path:?}: {err}");
    });

    let entries = extract_catalog_entries(&pages_rs_source);
    assert!(
        !entries.is_empty(),
        "no component doc targets found in {pages_rs_path:?}"
    );

    let mut module_sources: BTreeMap<String, String> = BTreeMap::new();

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

        let component_needle = format!("<{name}");
        assert!(
            block.contains(&component_needle),
            "`{module}::{func}` must include at least one `{component_needle} ...` usage (playground should demo the component itself)"
        );

        let title_needle = format!("title=\"{name}\"");
        assert!(
            block.contains(&title_needle),
            "`{module}::{func}` should set ComponentPage title to match catalog name (`{title_needle}`)"
        );
    }
}
