use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn search_does_not_expose_view_module() {
    let source = load_source("src/search/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Search internals should stay private; found `pub mod view`."
    );
}

#[test]
fn search_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/search/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Search;"),
        "search module should export `Search`."
    );
    assert!(
        crate_source.contains("pub use search::Search;"),
        "crate root should re-export `Search`."
    );
}

#[test]
fn search_wraps_search_field_contract() {
    let source = load_source("src/search/view.rs");

    for needle in [
        "pub fn Search(",
        "<SearchField",
        "motion: SearchFieldMotion",
    ] {
        assert!(
            source.contains(needle),
            "Search wrapper should preserve SearchField contract marker `{needle}`."
        );
    }
}

#[test]
fn search_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_search.rs");

    for needle in [
        "pub(super) fn search() -> AnyView",
        "title=\"Search\"",
        "slug=\"search\"",
        "<Search",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra_search docs page should contain `{needle}`."
        );
    }
}
