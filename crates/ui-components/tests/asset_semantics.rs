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

    assert!(
        !source.contains("pub mod logic"),
        "Asset `logic` module should stay private to avoid leaking internal state helpers."
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
        "logic::resolve_state(logic::AssetStateInput {",
        "logic::compose_class_name(class_name, state)",
        "<Thumbnail",
        "data-slot=\"asset\"",
        "data-size=state.size_attr",
        "data-state=state.data_state_attr",
        "data-label-source=state.label_source_attr",
        "data-content-source=state.content_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Asset wrapper should preserve Thumbnail contract marker `{needle}`."
        );
    }
}

#[test]
fn asset_styles_include_variant_state_and_accessibility_markers() {
    let source = load_source("src/asset/styles.rs");

    for selector in [
        ".ui-asset--variant-file",
        ".ui-asset[data-variant=\"file\"]",
        ".ui-asset--variant-folder",
        ".ui-asset--variant-custom",
        ".ui-asset--selected",
        ".ui-asset[data-selected=\"true\"]",
        ".ui-asset--focused",
        ".ui-asset[data-focused=\"true\"]",
        "@media (forced-colors: active)",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(
            source.contains(selector),
            "Asset styles should include `{selector}` for baseline-compatible state/accessibility contracts."
        );
    }
}

#[test]
fn asset_logic_tracks_label_content_and_class_sources() {
    let source = load_source("src/asset/logic.rs");

    for needle in [
        "pub const DEFAULT_FILE_LABEL: &str = \"File\";",
        "pub const DEFAULT_FOLDER_LABEL: &str = \"Folder\";",
        "pub const DEFAULT_CUSTOM_LABEL: &str = \"Asset\";",
        "pub struct AssetStateInput",
        "pub struct AssetState",
        "pub fn resolve_label(label: Option<String>, variant: AssetVariant) -> String",
        "pub fn resolve_state(input: AssetStateInput) -> AssetState",
        "label_source_attr",
        "content_source_attr",
        "class_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Asset logic should include `{needle}` for centralized state/source normalization."
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

#[test]
fn asset_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_asset.rs");

    for needle in [
        "pub(super) fn asset() -> AnyView",
        "title=\"Asset\"",
        "slug=\"asset\"",
        "title=\"File + Folder Variants\"",
        "title=\"Custom Image + Focused State\"",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            source.contains(needle),
            "display-extra-asset docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn asset_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_asset.rs");

    for needle in [
        "title=\"File + Folder Variants\"",
        "variant=AssetVariant::File",
        "variant=AssetVariant::Folder",
        "size=AssetSize::Size600",
        "label=\"Build Report\".to_string()",
        "label=\"Design Assets\".to_string()",
        "title=\"Custom Image + Focused State\"",
        "size=AssetSize::Size700",
        "selected=true",
        "focused=true",
        "title=\"State + Source Markers\"",
        "variant=AssetVariant::Custom",
        "size=AssetSize::Size800",
        "label=\"Featured Artwork\".to_string()",
        "class_name=\"docs-asset-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "asset docs playgrounds should contain `{needle}`.",
        );
    }
}
