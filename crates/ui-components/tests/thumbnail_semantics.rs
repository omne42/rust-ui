use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn thumbnail_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/thumbnail/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Thumbnail internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn thumbnail_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/thumbnail/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Thumbnail;"),
        "thumbnail module should export `Thumbnail`."
    );
    assert!(
        crate_source.contains("pub use thumbnail::{Thumbnail, ThumbnailMotion, ThumbnailSize};"),
        "crate root should re-export Thumbnail contract."
    );
}

#[test]
fn thumbnail_attaches_motion_driver() {
    let source = load_source("src/thumbnail/view.rs");

    assert!(
        source.contains("attach_motion"),
        "Thumbnail should attach its motion driver for focus/selection feedback."
    );
}

#[test]
fn thumbnail_emits_motion_marker_attributes() {
    let source = load_source("src/thumbnail/view.rs");

    for attr in [
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
    ] {
        assert!(
            source.contains(attr),
            "Thumbnail should expose `{attr}` for stable motion-source inspection."
        );
    }
}

#[test]
fn thumbnail_styles_include_motion_marker_contracts() {
    let source = load_source("src/thumbnail/styles.rs");

    for selector in [
        ".ui-thumbnail[data-motion-source=\"custom\"]",
        ".ui-thumbnail[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Thumbnail styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn thumbnail_motion_uses_spring_animator() {
    let source = load_source("src/thumbnail/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Thumbnail motion should animate via springs to match the repo motion spec."
    );
}

#[test]
fn thumbnail_styles_use_css_variables_for_motion() {
    let source = load_source("src/thumbnail/styles.rs");

    for name in ["--ui-thumbnail-scale", "--ui-thumbnail-ring-opacity"] {
        assert!(
            source.contains(name),
            "Thumbnail styles should define `{name}` so motion updates only touch CSS variables."
        );
    }
}

#[test]
fn thumbnail_docs_page_exists_in_display_extra_thumbnail() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");

    for needle in [
        "pub(super) fn thumbnail() -> AnyView",
        "title=\"Thumbnail\"",
        "slug=\"thumbnail\"",
        "<Thumbnail",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_thumbnail docs page should contain `{needle}`."
        );
    }
}
