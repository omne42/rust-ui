use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn collect_local_component_modules() -> BTreeSet<String> {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut names = BTreeSet::new();

    let entries =
        fs::read_dir(&src_root).unwrap_or_else(|e| panic!("read_dir failed for {src_root:?}: {e}"));

    for entry in entries {
        let entry = entry.unwrap_or_else(|e| panic!("read_dir entry failed for {src_root:?}: {e}"));
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        names.insert(name.to_string());
    }

    names
}

fn normalize_name(name: &str) -> String {
    name.replace('-', "_")
}

fn collect_upstream_component_names(dir: &Path) -> BTreeSet<String> {
    let mut names = BTreeSet::new();

    let entries = fs::read_dir(dir).unwrap_or_else(|e| panic!("read_dir failed for {dir:?}: {e}"));

    for entry in entries {
        let entry = entry.unwrap_or_else(|e| panic!("read_dir entry failed for {dir:?}: {e}"));
        let path = entry.path();

        if !path.is_file() || path.extension().is_none_or(|ext| ext != "tsx") {
            continue;
        }

        let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };

        if matches!(stem, "index" | "utils" | "types") {
            continue;
        }

        names.insert(normalize_name(stem));
    }

    names
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn shadcn_new_york_ui_component_names_are_covered_locally() {
    let local_modules = collect_local_component_modules();
    let shadcn_ui =
        repo_root().join("examples/_upstream/shadcn-ui/apps/v4/registry/new-york-v4/ui");

    if !shadcn_ui.exists() {
        return;
    }

    let upstream = collect_upstream_component_names(&shadcn_ui);

    let missing: Vec<String> = upstream
        .iter()
        .filter(|name| !local_modules.contains(name.as_str()))
        .cloned()
        .collect();

    assert!(
        missing.is_empty(),
        "local ui-components should cover shadcn new-york-v4/ui names; missing: {missing:?}"
    );
}

#[test]
fn animate_ui_component_names_are_covered_locally() {
    let local_modules = collect_local_component_modules();
    let animate_ui =
        repo_root().join("examples/_upstream/animate-ui/packages/ui/src/components/ui");

    if !animate_ui.exists() {
        return;
    }

    let upstream = collect_upstream_component_names(&animate_ui);

    let missing: Vec<String> = upstream
        .iter()
        .filter(|name| !local_modules.contains(name.as_str()))
        .cloned()
        .collect();

    assert!(
        missing.is_empty(),
        "local ui-components should cover animate-ui component names; missing: {missing:?}"
    );
}
