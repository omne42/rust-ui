use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn iconset_does_not_expose_view_module() {
    let source = load_source("src/iconset/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Iconset internals should stay private; found `pub mod view`."
    );
}

#[test]
fn iconset_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/iconset/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::{Iconset, IconsetGlyph};"),
        "iconset module should export `Iconset` and `IconsetGlyph`."
    );
    assert!(
        crate_source
            .contains("pub use iconset::{Iconset, IconsetGlyph, IconsetSize, IconsetTone};"),
        "crate root should re-export Iconset contracts."
    );
}

#[test]
fn iconset_wraps_icon_contract() {
    let source = load_source("src/iconset/view.rs");

    for needle in [
        "pub fn Iconset(",
        "#[prop(into)] icon: String",
        "glyphs: Vec<IconsetGlyph>",
        "<Icon",
        "data-slot=\"iconset\"",
        "data-icon-source=icon_source",
    ] {
        assert!(
            source.contains(needle),
            "Iconset wrapper should preserve Icon contract marker `{needle}`."
        );
    }
}

#[test]
fn iconset_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_iconset.rs");

    for needle in [
        "pub(super) fn iconset() -> AnyView",
        "title=\"Iconset\"",
        "slug=\"iconset\"",
        "<Iconset",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_iconset docs page should contain `{needle}`."
        );
    }
}
