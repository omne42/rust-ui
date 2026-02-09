use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn split_view_does_not_expose_view_module() {
    let source = load_source("src/split_view/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "SplitView internals should stay private; found `pub mod view`."
    );
}

#[test]
fn split_view_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/split_view/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::SplitView;"),
        "split_view module should export `SplitView`."
    );
    assert!(
        crate_source.contains("pub use split_view::SplitView;"),
        "crate root should re-export `SplitView`."
    );
}

#[test]
fn split_view_wraps_resizable_contract() {
    let source = load_source("src/split_view/view.rs");

    for needle in [
        "pub fn SplitView(",
        "<Resizable",
        "split_percent: Option<Signal<f64>>",
    ] {
        assert!(
            source.contains(needle),
            "SplitView wrapper should preserve Resizable contract marker `{needle}`."
        );
    }
}

#[test]
fn split_view_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_split_view.rs");

    for needle in [
        "pub(super) fn split_view() -> AnyView",
        "title=\"SplitView\"",
        "slug=\"split-view\"",
        "<SplitView",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_split_view docs page should contain `{needle}`."
        );
    }
}
