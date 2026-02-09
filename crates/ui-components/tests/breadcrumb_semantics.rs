use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn breadcrumb_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/breadcrumbs/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::{Breadcrumb, Breadcrumbs};"),
        "breadcrumbs module should export `Breadcrumb` and `Breadcrumbs`."
    );
    assert!(
        crate_source.contains("pub use breadcrumbs::{Breadcrumb, BreadcrumbItem, Breadcrumbs};"),
        "crate root should re-export breadcrumb contracts."
    );
}

#[test]
fn breadcrumb_wraps_breadcrumbs_with_identical_contract() {
    let source = load_source("src/breadcrumbs/view.rs");

    for needle in [
        "pub fn Breadcrumb(",
        "items: Vec<BreadcrumbItem>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "<Breadcrumbs items=items aria_label=aria_label class_name=class_name />",
    ] {
        assert!(
            source.contains(needle),
            "Breadcrumb wrapper should include `{needle}` to preserve Breadcrumbs behavior."
        );
    }
}

#[test]
fn breadcrumb_uses_existing_breadcrumbs_state_data_contracts() {
    let source = load_source("src/breadcrumbs/view.rs");

    for needle in [
        "data-slot=\"breadcrumbs\"",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-has-links=state.has_links.then_some(\"true\")",
        "data-has-current-page=state.has_current_page.then_some(\"true\")",
        "data-count=state.item_count.to_string()",
    ] {
        assert!(
            source.contains(needle),
            "Breadcrumb should rely on existing Breadcrumbs data contracts; missing `{needle}`."
        );
    }
}
