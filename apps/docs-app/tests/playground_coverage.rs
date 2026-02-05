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

fn extract_page_targets(source: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("page:") else {
            continue;
        };
        let symbol = rest.trim().trim_end_matches(',');
        let Some((module, func)) = symbol.split_once("::") else {
            continue;
        };
        if module.trim().is_empty() || func.trim().is_empty() {
            continue;
        }
        out.push((module.trim().to_string(), func.trim().to_string()));
    }

    out
}

fn slice_fn_block<'a>(source: &'a str, fn_name: &str) -> Option<&'a str> {
    let patterns = [
        format!("pub(super) fn {fn_name}"),
        format!("pub fn {fn_name}"),
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

    let targets = extract_page_targets(&pages_rs_source);
    assert!(
        !targets.is_empty(),
        "no component doc targets found in {pages_rs_path:?}"
    );

    let mut module_sources: BTreeMap<String, String> = BTreeMap::new();

    for (module, func) in targets {
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
    }
}
