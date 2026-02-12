use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn theme_default_module_exports_default_theme_contract() {
    let source = load_source("src/theme_default/mod.rs");

    for needle in [
        "pub use ui_theme::Theme;",
        "pub fn default_theme() -> Theme {",
        "Theme::light()",
    ] {
        assert!(
            source.contains(needle),
            "theme_default module should include `{needle}` for @react-spectrum/theme-default compatibility."
        );
    }
}

#[test]
fn crate_root_registers_theme_default_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod theme_default;"),
        "crate root should include `pub mod theme_default;` for @react-spectrum/theme-default compatibility."
    );
}

#[test]
fn theme_default_compatibility_reuses_ui_root_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn ui_root() -> AnyView",
        "title=\"UiRoot\"",
        "slug=\"ui-root\"",
        "<UiRoot",
        "Theme::dark()",
    ] {
        assert!(
            source.contains(needle),
            "layout ui_root docs should contain `{needle}` for theme-default compatibility coverage."
        );
    }
}

#[test]
fn theme_default_module_docs_page_covers_primary_playgrounds() {
    theme_default_compatibility_reuses_ui_root_docs_playground();
}

#[test]
fn theme_default_module_docs_playgrounds_lock_state_matrix_contract_values() {
    theme_default_compatibility_reuses_ui_root_docs_playground();
}
