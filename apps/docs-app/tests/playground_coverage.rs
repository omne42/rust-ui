use std::collections::BTreeMap;
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

fn extract_catalog_entries(source: &str) -> Vec<CatalogEntry> {
    let mut out = Vec::new();
    let mut in_entry = false;
    let mut name: Option<String> = None;
    let mut page: Option<(String, String)> = None;

    for line in source.lines() {
        let trimmed = line.trim();

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
                page = Some((module.trim().to_string(), func.trim().to_string()));
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
        let module_source = module_sources.entry(module.clone()).or_insert_with(|| {
            let path = docs_page_module_path(&module);
            fs::read_to_string(&path).unwrap_or_else(|err| {
                panic!("failed to read {path:?}: {err}");
            })
        });

        let Some(block) = slice_fn_block(module_source, &func) else {
            let path = docs_page_module_path(&module);
            panic!("missing function `{module}::{func}` in {path:?}");
        };

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
