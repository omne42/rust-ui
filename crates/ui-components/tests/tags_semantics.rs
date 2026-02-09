use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn tags_does_not_expose_view_module() {
    let source = load_source("src/tags/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Tags internals should stay private; found `pub mod view`."
    );
}

#[test]
fn tags_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/tags/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Tags;"),
        "tags module should export `Tags`."
    );
    assert!(
        crate_source.contains("pub use tags::Tags;"),
        "crate root should re-export `Tags`."
    );
}

#[test]
fn tags_wraps_tag_group_contract() {
    let source = load_source("src/tags/view.rs");

    for needle in [
        "pub fn Tags(",
        "<TagGroup",
        "on_remove: Option<Callback<Tag>>",
    ] {
        assert!(
            source.contains(needle),
            "Tags wrapper should preserve TagGroup contract marker `{needle}`."
        );
    }
}

#[test]
fn tags_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra_tags.rs");

    for needle in [
        "pub(super) fn tags() -> AnyView",
        "title=\"Tags\"",
        "slug=\"tags\"",
        "<Tags",
    ] {
        assert!(
            source.contains(needle),
            "collections_extra_tags docs page should contain `{needle}`."
        );
    }
}
