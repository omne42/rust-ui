use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn example_theme_module_exports_compatibility_contract() {
    let source = load_source("src/example_theme/mod.rs");

    for needle in [
        "pub use ui_theme::Theme;",
        "pub fn example_theme() -> Theme {",
        "Theme::light()",
    ] {
        assert!(
            source.contains(needle),
            "example_theme module should include `{needle}` for @react-aria/example-theme compatibility."
        );
    }
}

#[test]
fn crate_root_registers_example_theme_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod example_theme;"),
        "crate root should include `pub mod example_theme;` for @react-aria/example-theme compatibility."
    );
}

#[test]
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
        "\"`data-theme-scheme` mirrors `Theme::scheme` (`light`/`dark`).\"",
        "\"`data-state` + `data-safe-area` describe safe-area mode.\"",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "example_theme_module docs playgrounds should contain `{needle}`.",
        );
    }
}
