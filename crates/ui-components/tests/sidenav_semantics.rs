use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sidenav_does_not_expose_view_module() {
    let source = load_source("src/sidenav/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Sidenav internals should stay private; found `pub mod view`."
    );
}

#[test]
fn sidenav_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/sidenav/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Sidenav;"),
        "sidenav module should export `Sidenav`."
    );
    assert!(
        crate_source.contains("pub use sidenav::Sidenav;"),
        "crate root should re-export `Sidenav`."
    );
}

#[test]
fn sidenav_wraps_sidebar_contract() {
    let source = load_source("src/sidenav/view.rs");

    for needle in [
        "pub fn Sidenav(",
        "<Sidebar",
        "on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            source.contains(needle),
            "Sidenav wrapper should preserve Sidebar contract marker `{needle}`."
        );
    }
}

#[test]
fn sidenav_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_sidenav.rs");

    for needle in [
        "pub(super) fn sidenav() -> AnyView",
        "title=\"Sidenav\"",
        "slug=\"sidenav\"",
        "<Sidenav",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_sidenav docs page should contain `{needle}`."
        );
    }
}
