use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn empty_does_not_expose_view_module() {
    let source = load_source("src/empty/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Empty internals should stay private; found `pub mod view`."
    );
}

#[test]
fn empty_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/empty/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::{"),
        "empty module should export Empty component family."
    );
    assert!(
        crate_source.contains("pub use empty::{"),
        "crate root should re-export Empty component contracts."
    );
}

#[test]
fn empty_wraps_slot_contracts() {
    let source = load_source("src/empty/view.rs");

    for needle in [
        "pub fn Empty(",
        "pub fn EmptyHeader(",
        "pub fn EmptyTitle(",
        "pub fn EmptyDescription(",
        "pub fn EmptyContent(",
        "pub fn EmptyMedia(",
        "data-slot=\"empty\"",
        "data-slot=\"empty-header\"",
        "data-slot=\"empty-title\"",
        "data-slot=\"empty-description\"",
        "data-slot=\"empty-content\"",
        "data-slot=\"empty-icon\"",
    ] {
        assert!(
            source.contains(needle),
            "Empty wrapper should preserve slot contract marker `{needle}`."
        );
    }
}

#[test]
fn empty_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_empty.rs");

    for needle in [
        "pub(super) fn empty() -> AnyView",
        "title=\"Empty\"",
        "slug=\"empty\"",
        "<Empty",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_empty docs page should contain `{needle}`."
        );
    }
}
