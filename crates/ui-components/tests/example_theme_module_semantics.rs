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

    let layout_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn ui_root() -> AnyView",
        "title=\"UiRoot\"",
        "slug=\"ui-root\"",
        "<UiRoot",
        "Theme::dark()",
    ] {
        assert!(
            layout_source.contains(needle),
            "layout ui_root docs should contain `{needle}` for example-theme compatibility coverage."
        );
    }
}
