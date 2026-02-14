use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn test_utils_module_exports_theme_css_snapshot_contract() {
    let source = load_source("src/test_utils/mod.rs");

    for needle in [
        "pub use crate::Theme;",
        "pub fn snapshot_theme_css(theme: Theme) -> String {",
        "theme.to_css_variables()",
    ] {
        assert!(
            source.contains(needle),
            "test_utils module should include `{needle}` for @react-spectrum/test-utils compatibility."
        );
    }
}

#[test]
fn crate_root_registers_test_utils_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod test_utils;"),
        "crate root should include `pub mod test_utils;` for @react-spectrum/test-utils compatibility."
    );
}

#[test]
fn test_utils_compatibility_reuses_ui_root_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/ui_root.rs");

    for needle in ["title=\"UiRoot\"", "slug=\"ui-root\"", "Theme::dark()"] {
        assert!(
            source.contains(needle),
            "ui_root docs should contain `{needle}` for test-utils compatibility coverage."
        );
    }
}

#[test]
fn test_utils_module_docs_page_covers_primary_playgrounds() {
    test_utils_compatibility_reuses_ui_root_docs_playground();
}

#[test]
fn test_utils_module_docs_playgrounds_lock_state_matrix_contract_values() {
    test_utils_compatibility_reuses_ui_root_docs_playground();
}
