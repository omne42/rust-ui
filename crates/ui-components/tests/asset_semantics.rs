use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn asset_does_not_expose_view_module() {
    let source = load_source("src/asset/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Asset internals should stay private; found `pub mod view`."
    );
}

#[test]
fn asset_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/asset/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Asset;"),
        "asset module should export `Asset`."
    );
    assert!(
        crate_source.contains("pub use asset::{Asset, AssetMotion, AssetSize, AssetVariant};"),
        "crate root should re-export Asset contract."
    );
}

#[test]
fn asset_wraps_thumbnail_contract() {
    let source = load_source("src/asset/view.rs");

    for needle in [
        "pub fn Asset(",
        "variant: AssetVariant",
        "<Thumbnail",
        "data-slot=\"asset\"",
    ] {
        assert!(
            source.contains(needle),
            "Asset wrapper should preserve Thumbnail contract marker `{needle}`."
        );
    }
}

#[test]
fn asset_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_asset.rs");

    for needle in [
        "pub(super) fn asset() -> AnyView",
        "title=\"Asset\"",
        "slug=\"asset\"",
        "<Asset",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_asset docs page should contain `{needle}`."
        );
    }
}
