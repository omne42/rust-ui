use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn top_nav_does_not_expose_view_module() {
    let source = load_source("src/top_nav/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "TopNav internals should stay private; found `pub mod view`."
    );
}

#[test]
fn top_nav_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/top_nav/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::TopNav;"),
        "top_nav module should export `TopNav`."
    );
    assert!(
        crate_source.contains("pub use top_nav::{TopNav, TopNavItem, TopNavMotion};"),
        "crate root should re-export `TopNav`, `TopNavItem`, and `TopNavMotion`."
    );
}

#[test]
fn top_nav_wraps_navigation_menu_contract() {
    let source = load_source("src/top_nav/view.rs");

    for needle in [
        "pub fn TopNav(",
        "<NavigationMenu",
        "label: Option<String>",
        "data-slot=\"top-nav\"",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
    ] {
        assert!(
            source.contains(needle),
            "TopNav wrapper should preserve NavigationMenu contract marker `{needle}`."
        );
    }
}

#[test]
fn top_nav_styles_include_motion_marker_contracts() {
    let source = load_source("src/top_nav/styles.rs");

    for selector in [
        ".ui-top-nav[data-motion-source=\"custom\"]",
        ".ui-top-nav[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "TopNav styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn top_nav_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra_top_nav.rs");

    for needle in [
        "pub(super) fn top_nav() -> AnyView",
        "title=\"TopNav\"",
        "slug=\"top-nav\"",
        "<TopNav",
    ] {
        assert!(
            source.contains(needle),
            "collections_extra_top_nav docs page should contain `{needle}`."
        );
    }
}
