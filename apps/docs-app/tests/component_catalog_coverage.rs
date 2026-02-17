use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn extract_component_fns(source: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut pending_component = false;

    for line in source.lines() {
        let trimmed = line.trim();

        if trimmed == "#[component]" {
            pending_component = true;
            continue;
        }

        if !pending_component {
            continue;
        }

        if trimmed.starts_with("#[") {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("pub fn ") {
            let name = rest
                .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
                .next()
                .unwrap_or_default()
                .trim();

            if !name.is_empty() {
                out.insert(name.to_string());
            }
        }

        pending_component = false;
    }

    out
}

fn walk_rs_files(root: &Path, out: &mut BTreeSet<String>) {
    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_rs_files(&path, out);
            continue;
        }

        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }

        let source = match fs::read_to_string(&path) {
            Ok(source) => source,
            Err(_) => continue,
        };

        out.extend(extract_component_fns(&source));
    }
}

fn ui_components_component_names() -> BTreeSet<String> {
    let root: PathBuf =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui-components/src");

    let mut out = BTreeSet::new();
    walk_rs_files(&root, &mut out);
    out
}

fn docs_only_component_entries() -> BTreeSet<String> {
    BTreeSet::from([
        "ListItem".to_string(),
        "ListSection".to_string(),
        "ThemeVisualBaseline".to_string(),
    ])
}

fn internal_only_component_entries() -> BTreeSet<String> {
    BTreeSet::from([
        "ListBox".to_string(),
        "ListBoxItem".to_string(),
        "ListBoxSection".to_string(),
    ])
}

#[test]
fn docs_catalog_covers_all_ui_components_components() {
    let mut expected = ui_components_component_names();
    for internal_only in internal_only_component_entries() {
        expected.remove(&internal_only);
    }
    expected.extend(docs_only_component_entries());

    let catalog = docs_app::pages::components::component_catalog();
    let actual: BTreeSet<String> = catalog.iter().map(|doc| doc.name.to_string()).collect();

    assert_eq!(
        expected, actual,
        "docs-app 组件目录需要覆盖 ui-components 全部 #[component] pub fn 组件（新增/删除组件必须同步更新 docs catalog）。"
    );
}

#[test]
fn docs_catalog_slugs_are_unique_and_non_empty() {
    let catalog = docs_app::pages::components::component_catalog();
    let mut slugs = BTreeSet::new();

    for doc in catalog {
        assert!(!doc.name.trim().is_empty(), "ComponentDoc.name 不能为空");
        assert!(
            !doc.slug.trim().is_empty(),
            "ComponentDoc.slug 不能为空 ({})",
            doc.name
        );
        assert!(
            !doc.group.trim().is_empty(),
            "ComponentDoc.group 不能为空 ({})",
            doc.name
        );
        assert!(
            slugs.insert(doc.slug),
            "ComponentDoc.slug 必须唯一，重复 slug: {}",
            doc.slug
        );
    }
}
