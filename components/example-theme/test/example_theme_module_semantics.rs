use std::fs;
use std::path::Path;

fn resolve_source_path(rel_path: &str) -> Option<std::path::PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let mut candidates = vec![manifest_dir.join(rel_path)];

    if let Some(component_rel_path) = rel_path.strip_prefix("../../components/") {
        let direct = workspace_dir.join("components").join(component_rel_path);
        candidates.push(direct.clone());

        let parts: Vec<&str> = component_rel_path.split('/').collect();
        if parts.len() > 3 && parts.get(1) == Some(&"src") && parts.get(2) == parts.first() {
            let collapsed = workspace_dir
                .join("components")
                .join(parts[0])
                .join("src")
                .join(parts[3..].join("/"));
            candidates.push(collapsed);
        }
    }

    if let Some(src_rel_path) = rel_path.strip_prefix("src/") {
        let segments: Vec<&str> = src_rel_path.split('/').collect();
        let components_root = workspace_dir.join("components");

        if let Ok(entries) = fs::read_dir(&components_root) {
            let component_dirs: Vec<String> = entries
                .flatten()
                .filter_map(|entry| {
                    let path = entry.path();
                    path.is_dir()
                        .then(|| entry.file_name().to_string_lossy().to_string())
                })
                .collect();

            for component_dir in component_dirs {
                for start in 0..segments.len() {
                    for end in start..segments.len() {
                        let name = segments[start..=end]
                            .iter()
                            .map(|segment| segment.replace('_', "-"))
                            .collect::<Vec<_>>()
                            .join("-");

                        if name != component_dir {
                            continue;
                        }

                        if end + 1 >= segments.len() {
                            candidates
                                .push(components_root.join(&component_dir).join("src/mod.rs"));
                            candidates
                                .push(components_root.join(&component_dir).join("src/check2.md"));
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                            continue;
                        }

                        let suffix = segments[end + 1..].join("/");
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("src")
                                .join(&suffix),
                        );
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("test")
                                .join(&suffix),
                        );

                        if suffix == "check2.md" {
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                        }
                    }
                }
            }
        }
    }

    candidates.into_iter().find(|path| path.exists())
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_source_path(rel_path)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path));

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}
#[test]
fn example_theme_module_exports_compatibility_contract() {
    let source = load_source("../../components/example-theme/src/mod.rs");

    for needle in [
        "pub use ui_theme::Theme;",
        "pub fn example_theme() -> Theme {",
        "Theme::light()",
    ] {
        assert!(
            source.contains(needle),
            "example_theme module should include `{needle}` for @a11y-baseline/example-theme compatibility."
        );
    }
}

#[test]
fn crate_root_registers_example_theme_module() {
    let lib_source = load_source("src/lib.rs");
    let cargo_source = load_source("Cargo.toml");

    assert!(
        lib_source.contains("pub use ui_example_theme as example_theme;"),
        "crate root should re-export ui-example-theme as `example_theme` for @a11y-baseline/example-theme compatibility."
    );
    assert!(
        cargo_source.contains("component-example_theme = [\"dep:ui-example-theme\"]"),
        "component-example_theme feature should depend on dep:ui-example-theme after extraction."
    );
    assert!(
        cargo_source.contains(
            "ui-example-theme = { path = \"../../components/example-theme\", optional = true }"
        ),
        "ui Cargo.toml should include the optional ui-example-theme dependency."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn example_theme_compatibility_reuses_ui_root_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    assert!(
        source.contains("\"example-theme\" => &[\"ui-root\"],"),
        "component docs mapping should route example-theme coverage to the existing ui-root playground."
    );

    let ui_root_source = load_source("../../apps/docs-app/src/pages/components/pages/ui_root.rs");

    for needle in [
        "pub(super) fn ui_root() -> AnyView",
        "title=\"UiRoot\"",
        "slug=\"ui-root\"",
        "<UiRoot",
        "Theme::dark()",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "ui_root docs should contain `{needle}` for example-theme compatibility coverage."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn example_theme_module_docs_page_covers_primary_playgrounds() {
    let component_map_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");
    let ui_root_source = load_source("../../apps/docs-app/src/pages/components/pages/ui_root.rs");

    for needle in [
        "\"example-theme\" => &[\"ui-root\"],",
        "\"theme-dark\" => &[\"ui-root\"],",
    ] {
        assert!(
            component_map_source.contains(needle),
            "component docs mapping should include `{needle}` for example_theme_module primary coverage.",
        );
    }

    for needle in [
        "pub(super) fn ui_root() -> AnyView",
        "title=\"UiRoot\"",
        "slug=\"ui-root\"",
        "<Playground title=\"Usage\" code_signal=usage_code>",
        "<Playground title=\"State Contract\" code_signal=contract_code>",
        "<UiRoot",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "ui_root docs page should include `{needle}` for example_theme_module ui_root coverage.",
        );
    }
}

#[test]
fn example_theme_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let ui_root_source = load_source("../../apps/docs-app/src/pages/components/pages/ui_root.rs");

    for needle in [
        "let usage_code = Signal::derive(move || {",
        "let theme = Signal::derive(|| Theme::dark());",
        "<UiRoot theme=theme safe_area=true inject_components_css=true>",
        "title=\"State Contract\"",
        "let contract_code = Signal::derive(move || {",
        "data-theme-scheme=\"light|dark\"",
        "data-state=\"default|safe-area\"",
        "data-safe-area=\"true\" (optional)",
        "\"UiRoot injects BASE_CSS + theme CSS variables + component CSS in one place.\"",
        "\"`data-theme-scheme` mirrors the CSS `color-scheme` value (`light`/`dark`).\"",
        "\"`data-state` + `data-safe-area` describe safe-area mode.\"",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "example_theme_module docs playgrounds should contain `{needle}`.",
        );
    }
}
