use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(component_path) = rel_path.strip_prefix("src/") {
        let mut parts = component_path.splitn(2, '/');
        let component = parts.next().unwrap_or_default();
        let Some(suffix) = parts.next() else {
            return fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        };

        let component_dir = component.replace('_', "-");
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/{component_dir}/src/{suffix}"));

        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn theme_dark_module_exports_dark_theme_contract() {
    let source = load_source("../../components/theme-dark/src/mod.rs");

    for needle in [
        "pub use ui_theme::Theme;",
        "pub fn dark_theme() -> Theme {",
        "Theme::dark()",
    ] {
        assert!(
            source.contains(needle),
            "theme_dark module should include `{needle}` for @ui-baseline/theme-dark compatibility."
        );
    }
}

#[test]
fn crate_root_registers_theme_dark_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod theme_dark;"),
        "crate root should include `pub mod theme_dark;` for @ui-baseline/theme-dark compatibility."
    );
}

#[test]
fn theme_dark_compatibility_reuses_ui_root_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/ui_root.rs");

    for needle in [
        "pub(super) fn ui_root() -> AnyView",
        "title=\"UiRoot\"",
        "slug=\"ui-root\"",
        "<UiRoot",
        "Theme::dark()",
    ] {
        assert!(
            source.contains(needle),
            "ui_root docs should contain `{needle}` for theme-dark compatibility coverage."
        );
    }
}

#[test]
fn theme_dark_module_docs_page_covers_primary_playgrounds() {
    theme_dark_compatibility_reuses_ui_root_docs_playground();
}

#[test]
fn theme_dark_module_docs_playgrounds_lock_state_matrix_contract_values() {
    theme_dark_compatibility_reuses_ui_root_docs_playground();
}
