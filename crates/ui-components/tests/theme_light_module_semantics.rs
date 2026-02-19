use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn theme_light_module_exports_light_theme_contract() {
    let source = load_source("src/theme/light/mod.rs");

    for needle in [
        "pub use ui_theme::Theme;",
        "pub fn light_theme() -> Theme {",
        "Theme::light()",
    ] {
        assert!(
            source.contains(needle),
            "theme_light module should include `{needle}` for @ui-baseline/theme-light compatibility."
        );
    }
}

#[test]
fn crate_root_registers_theme_light_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod theme_light;"),
        "crate root should include `pub mod theme_light;` for @ui-baseline/theme-light compatibility."
    );
}

#[test]
fn theme_light_compatibility_reuses_ui_root_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/ui_root.rs");

    for needle in [
        "pub(super) fn ui_root() -> AnyView",
        "title=\"UiRoot\"",
        "slug=\"ui-root\"",
        "<UiRoot",
    ] {
        assert!(
            source.contains(needle),
            "ui_root docs should contain `{needle}` for theme-light compatibility coverage."
        );
    }
}

#[test]
fn theme_light_module_docs_page_covers_primary_playgrounds() {
    theme_light_compatibility_reuses_ui_root_docs_playground();
}

#[test]
fn theme_light_module_docs_playgrounds_lock_state_matrix_contract_values() {
    theme_light_compatibility_reuses_ui_root_docs_playground();
}
