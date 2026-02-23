use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use ui_test_support::source_contract;

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

fn component_pages_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages/components/pages")
}

fn workspace_component_view_path(component: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../components")
        .join(component)
        .join("src/view.rs")
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

fn read_file(path: &Path) -> String {
    source_contract::source_from_path(path)
}

fn slice_fn_block_opt<'a>(source: &'a str, fn_name: &str) -> Option<&'a str> {
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

fn extract_reexport_submodule(source: &str, fn_name: &str) -> Option<String> {
    for line in source.lines() {
        let trimmed = line.trim().trim_end_matches(';');
        let Some(rest) = trimmed.strip_prefix("pub(super) use ") else {
            continue;
        };
        let Some((submodule, target)) = rest.split_once("::") else {
            continue;
        };
        if target.trim() == fn_name {
            return Some(submodule.trim().to_string());
        }
    }

    None
}

fn load_fn_block(module: &str, fn_name: &str) -> String {
    let module_source = read_file(&docs_page_module_path(module));
    if let Some(block) = slice_fn_block_opt(&module_source, fn_name) {
        return block.to_string();
    }

    if let Some(submodule) = extract_reexport_submodule(&module_source, fn_name) {
        let sub_source = read_file(&docs_page_submodule_path(module, &submodule));
        if let Some(block) = slice_fn_block_opt(&sub_source, fn_name) {
            return block.to_string();
        }
    }

    panic!("missing function `{module}::{fn_name}`");
}

fn slice_between<'a>(source: &'a str, start_marker: &str, end_marker: &str) -> &'a str {
    let start = source
        .find(start_marker)
        .unwrap_or_else(|| panic!("missing marker `{start_marker}`"));
    let tail = &source[start..];
    let end = tail
        .find(end_marker)
        .unwrap_or_else(|| panic!("missing marker `{end_marker}` after `{start_marker}`"));
    &tail[..end]
}

fn extract_component_props(component_source: &str, fn_name: &str) -> BTreeSet<String> {
    let signature = format!("pub fn {fn_name}(");
    let start = component_source
        .find(&signature)
        .unwrap_or_else(|| panic!("missing signature `{signature}`"));
    let open_paren = start + signature.len() - 1;
    let mut depth = 0usize;
    let mut params = String::new();

    for ch in component_source[open_paren + 1..].chars() {
        match ch {
            '(' => {
                depth += 1;
                params.push(ch);
            }
            ')' if depth == 0 => break,
            ')' => {
                depth = depth.saturating_sub(1);
                params.push(ch);
            }
            _ => params.push(ch),
        }
    }

    params
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("#[") {
                return None;
            }
            let (name, _) = trimmed.split_once(':')?;
            let name = name.trim().trim_start_matches("mut ").trim();
            if name.is_empty()
                || !name
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
            {
                return None;
            }
            Some(name.to_string())
        })
        .collect()
}

fn assert_props_covered(
    target_block: &str,
    props: &BTreeSet<String>,
    ignored_props: &[&str],
    context: &str,
) {
    for prop in props {
        if ignored_props.contains(&prop.as_str()) {
            continue;
        }

        let as_attr = format!("{prop}=");
        let as_key = format!("{prop}:");
        assert!(
            target_block.contains(&as_attr) || target_block.contains(&as_key),
            "{context} missing API coverage for `{prop}`",
        );
    }
}

#[test]
fn accordion_playground_standard_is_enforced() {
    let accordion_block = load_fn_block("collections", "accordion");

    for marker in [
        "title=\"Hello World (Uncontrolled)\"",
        "title=\"State Matrix (Single + Disabled)\"",
        "title=\"Workbench (Isolated Canvas + Optional Persist)\"",
        "data-slot=\"accordion-workbench-controls\"",
        "data-slot=\"accordion-workbench-config-preview\"",
        "data-slot=\"accordion-item-api-controls\"",
        "data-slot=\"accordion-item-api-config-preview\"",
    ] {
        assert!(
            accordion_block.contains(marker),
            "accordion playground standard marker missing: `{marker}`",
        );
    }

    let workbench_config_block = slice_between(
        &accordion_block,
        "let workbench_actual_config = Signal::derive(move || {",
        "let item_api_code = Signal::derive(move || {",
    );
    let item_api_config_block = slice_between(
        &accordion_block,
        "let item_api_actual_config = Signal::derive(move || {",
        "view! {",
    );

    let accordion_view_source = read_file(&workspace_component_view_path("accordion"));
    let accordion_props = extract_component_props(&accordion_view_source, "Accordion");
    let accordion_item_props = extract_component_props(&accordion_view_source, "AccordionItem");

    assert_props_covered(
        workbench_config_block,
        &accordion_props,
        &["children"],
        "Accordion root workbench config",
    );
    assert_props_covered(
        item_api_config_block,
        &accordion_item_props,
        &["children"],
        "Accordion item API config",
    );
}

#[test]
fn all_playgrounds_with_controls_define_test_config_signal() {
    let mut files = Vec::new();
    walk_rs_files(&component_pages_root(), &mut files);

    for file in files {
        let source = read_file(&file);
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
            if tag.contains("controls=") {
                assert!(
                    tag.contains("test_config_signal="),
                    "{file:?}: playground with controls must define test_config_signal for live actual-config feedback",
                );
            }

            scan_from = tag_start + rel_end + 1;
        }
    }
}
