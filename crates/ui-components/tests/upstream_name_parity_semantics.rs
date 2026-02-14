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

fn compact_name(name: &str) -> String {
    name.replace(['-', '_'], "").to_ascii_lowercase()
}

fn camel_to_snake(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 8);

    for (index, ch) in name.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if index != 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }

    out
}

fn collect_upstream_tsx_component_names(dir: &Path, skip: &[&str]) -> BTreeSet<String> {
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

        if skip.contains(&stem) {
            continue;
        }

        names.insert(normalize_name(stem));
    }

    names
}

fn collect_upstream_dir_names(dir: &Path) -> BTreeSet<String> {
    let mut names = BTreeSet::new();

    let entries = fs::read_dir(dir).unwrap_or_else(|e| panic!("read_dir failed for {dir:?}: {e}"));

    for entry in entries {
        let entry = entry.unwrap_or_else(|e| panic!("read_dir entry failed for {dir:?}: {e}"));
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

    let upstream = collect_upstream_tsx_component_names(&shadcn_ui, &["index", "utils", "types"]);

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

    let upstream = collect_upstream_tsx_component_names(&animate_ui, &["index", "utils", "types"]);

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

#[test]
fn react_aria_components_names_are_covered_locally() {
    let local_modules = collect_local_component_modules();
    let react_aria_components = repo_root()
        .join("examples/_upstream/adobe-react-spectrum/packages/react-aria-components/src");

    if !react_aria_components.exists() {
        return;
    }

    let skip = [
        "index",
        "utils",
        "RSPContexts",
        "TableLayout",
        "TreeDropTargetDelegate",
        "useDragAndDrop",
    ];

    let mut missing = Vec::new();
    let entries = fs::read_dir(&react_aria_components)
        .unwrap_or_else(|e| panic!("read_dir failed for {react_aria_components:?}: {e}"));

    for entry in entries {
        let entry = entry
            .unwrap_or_else(|e| panic!("read_dir entry failed for {react_aria_components:?}: {e}"));
        let path = entry.path();

        if !path.is_file() || path.extension().is_none_or(|ext| ext != "tsx") {
            continue;
        }

        let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };

        if skip.contains(&stem) {
            continue;
        }

        let snake_case = camel_to_snake(stem);

        if !local_modules.contains(snake_case.as_str()) {
            missing.push(snake_case);
        }
    }

    assert!(
        missing.is_empty(),
        "local ui-components should cover react-aria-components names; missing: {missing:?}"
    );
}

#[test]
fn heroui_component_dir_names_are_covered_locally() {
    let local_modules = collect_local_component_modules();
    let heroui_components =
        repo_root().join("examples/_upstream/heroui/packages/react/src/components");

    if !heroui_components.exists() {
        return;
    }

    let upstream = collect_upstream_dir_names(&heroui_components);
    let skip = ["rac"];

    let missing: Vec<String> = upstream
        .iter()
        .map(|name| normalize_name(name))
        .filter(|name| !skip.contains(&name.as_str()))
        .filter(|name| !local_modules.contains(name.as_str()))
        .collect();

    assert!(
        missing.is_empty(),
        "local ui-components should cover HeroUI component directory names; missing: {missing:?}"
    );
}

#[test]
fn spectrum_web_components_dir_names_are_covered_locally() {
    let local_modules = collect_local_component_modules();
    let swc_roots = [
        repo_root().join(
            "examples/_upstream/adobe-spectrum-web-components/2nd-gen/packages/swc/components",
        ),
        repo_root().join(
            "examples/_upstream/adobe-spectrum-web-components/2nd-gen/packages/core/components",
        ),
    ];

    let mut upstream = BTreeSet::new();
    for root in swc_roots {
        if !root.exists() {
            continue;
        }
        upstream.extend(collect_upstream_dir_names(&root));
    }

    let missing: Vec<String> = upstream
        .iter()
        .map(|name| normalize_name(name))
        .filter(|name| !local_modules.contains(name.as_str()))
        .collect();

    assert!(
        missing.is_empty(),
        "local ui-components should cover Spectrum Web Components names; missing: {missing:?}"
    );
}

#[test]
fn react_spectrum_package_names_are_covered_locally() {
    let local_modules = collect_local_component_modules();
    let local_compact: BTreeSet<String> = local_modules
        .iter()
        .map(|name| compact_name(name))
        .collect();
    let react_spectrum_packages =
        repo_root().join("examples/_upstream/adobe-react-spectrum/packages/@react-spectrum");

    if !react_spectrum_packages.exists() {
        return;
    }

    let upstream = collect_upstream_dir_names(&react_spectrum_packages);
    let skip = [
        "provider",
        "utils",
        "test-utils",
        "s2",
        "story-utils",
        "style-macro-s1",
    ];

    let missing: Vec<String> = upstream
        .iter()
        .filter(|name| !skip.contains(&name.as_str()))
        .filter(|name| !local_compact.contains(&compact_name(name)))
        .cloned()
        .collect();

    assert!(
        missing.is_empty(),
        "local ui-components should cover @react-spectrum package names; missing: {missing:?}"
    );
}

#[test]
fn react_aria_package_names_are_covered_locally() {
    let local_modules = collect_local_component_modules();
    let local_compact: BTreeSet<String> = local_modules
        .iter()
        .map(|name| compact_name(name))
        .collect();
    let react_aria_packages =
        repo_root().join("examples/_upstream/adobe-react-spectrum/packages/@react-aria");

    if !react_aria_packages.exists() {
        return;
    }

    let upstream = collect_upstream_dir_names(&react_aria_packages);
    let skip = [
        "aria-modal-polyfill",
        "i18n",
        "interactions",
        "live-announcer",
        "utils",
        "test-utils",
        "collections",
        "selection",
        "ssr",
        "focus",
        "landmark",
    ];

    let missing: Vec<String> = upstream
        .iter()
        .filter(|name| !skip.contains(&name.as_str()))
        .filter(|name| !local_compact.contains(&compact_name(name)))
        .cloned()
        .collect();

    assert!(
        missing.is_empty(),
        "local ui-components should cover @react-aria package names; missing: {missing:?}"
    );
}

#[test]
fn upstream_name_parity_docs_page_covers_primary_playgrounds() {
    shadcn_new_york_ui_component_names_are_covered_locally();
    animate_ui_component_names_are_covered_locally();
    react_aria_components_names_are_covered_locally();
}

#[test]
fn upstream_name_parity_docs_playgrounds_lock_state_matrix_contract_values() {
    heroui_component_dir_names_are_covered_locally();
    spectrum_web_components_dir_names_are_covered_locally();
    react_spectrum_package_names_are_covered_locally();
    react_aria_package_names_are_covered_locally();
}
