use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn breadcrumb_does_not_expose_view_module() {
    let source = load_source("src/breadcrumb/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Breadcrumb internals should stay private; found `pub mod view`."
    );
}

#[test]
fn breadcrumb_is_exported_from_module_and_registered_in_crate() {
    let module_source = load_source("src/breadcrumb/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::{"),
        "breadcrumb module should export breadcrumb primitive family."
    );
    assert!(
        crate_source.contains("pub mod breadcrumb;"),
        "crate root should register breadcrumb module."
    );
}

#[test]
fn breadcrumb_primitives_expose_slot_contracts() {
    let source = load_source("src/breadcrumb/view.rs");

    for needle in [
        "pub fn Breadcrumb(",
        "pub fn BreadcrumbList(",
        "pub fn BreadcrumbItem(",
        "pub fn BreadcrumbLink(",
        "pub fn BreadcrumbPage(",
        "pub fn BreadcrumbSeparator(",
        "pub fn BreadcrumbEllipsis(",
        "data-slot=\"breadcrumb\"",
        "data-slot=\"breadcrumb-list\"",
        "data-slot=\"breadcrumb-item\"",
        "data-slot=\"breadcrumb-link\"",
        "data-slot=\"breadcrumb-page\"",
        "data-slot=\"breadcrumb-separator\"",
        "data-slot=\"breadcrumb-ellipsis\"",
    ] {
        assert!(
            source.contains(needle),
            "Breadcrumb primitive family should include `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_docs_page_exists() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb_shadcn.rs",
    );

    for needle in [
        "pub(super) fn breadcrumb_primitives() -> AnyView",
        "title=\"BreadcrumbList\"",
        "slug=\"breadcrumb-list\"",
        "<BreadcrumbList",
    ] {
        assert!(
            source.contains(needle),
            "collections_breadcrumb_shadcn docs page should contain `{needle}`."
        );
    }
}
