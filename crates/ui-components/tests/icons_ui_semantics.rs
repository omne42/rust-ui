use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn icons_ui_does_not_expose_view_module() {
    let source = load_source("src/icons_ui/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "IconsUi internals should stay private; found `pub mod view`."
    );
}

#[test]
fn icons_ui_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/icons_ui/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::IconsUi;"),
        "icons_ui module should export `IconsUi`."
    );
    assert!(
        crate_source.contains("pub use icons_ui::{IconsUi, IconsUiSize, IconsUiTone};"),
        "crate root should re-export `IconsUi` contracts."
    );
}

#[test]
fn icons_ui_wraps_iconset_contract() {
    let source = load_source("src/icons_ui/view.rs");

    for needle in [
        "pub fn IconsUi(",
        "fn default_ui_glyphs()",
        "<Iconset",
        "iconset=\"ui\".to_string()",
        "data-slot=\"icons-ui\"",
    ] {
        assert!(
            source.contains(needle),
            "IconsUi wrapper should preserve Iconset contract marker `{needle}`."
        );
    }
}

#[test]
fn icons_ui_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_icons_ui.rs");

    for needle in [
        "pub(super) fn icons_ui() -> AnyView",
        "title=\"IconsUi\"",
        "slug=\"icons-ui\"",
        "<IconsUi",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_icons_ui docs page should contain `{needle}`."
        );
    }
}
