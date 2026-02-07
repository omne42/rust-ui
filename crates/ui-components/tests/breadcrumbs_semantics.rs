use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn breadcrumbs_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/breadcrumbs/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Breadcrumbs internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn breadcrumbs_uses_logic_state_model() {
    let view_source = load_source("src/breadcrumbs/view.rs");
    let logic_source = load_source("src/breadcrumbs/logic.rs");

    for needle in [
        "pub struct BreadcrumbsState",
        "pub fn resolve_state(items: &[BreadcrumbItem])",
        "pub item_count: usize",
        "pub has_links: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "Breadcrumbs logic should include `{needle}` for centralized state derivation."
        );
    }

    assert!(
        view_source.contains("let state = logic::resolve_state(&items);"),
        "Breadcrumbs view should derive root state through resolve_state."
    );
}

#[test]
fn breadcrumbs_emits_spectrum_style_slots_and_root_attrs() {
    let source = load_source("src/breadcrumbs/view.rs");

    for needle in [
        "data-slot=\"breadcrumbs\"",
        "data-slot=\"breadcrumbs-list\"",
        "data-slot=\"breadcrumbs-item\"",
        "data-slot=\"breadcrumbs-link\"",
        "data-slot=\"breadcrumbs-current\"",
        "data-slot=\"breadcrumbs-label\"",
        "data-slot=\"breadcrumbs-separator\"",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-has-links=state.has_links.then_some(\"true\")",
        "data-has-current-page=state.has_current_page.then_some(\"true\")",
        "data-count=state.item_count.to_string()",
        "data-index=index",
        "data-last=is_last.then_some(\"true\")",
        "data-href=href_for_attr",
    ] {
        assert!(
            source.contains(needle),
            "Breadcrumbs should expose `{needle}` for Spectrum-style styling and regression checks."
        );
    }
}

#[test]
fn breadcrumbs_wires_nav_and_current_page_semantics() {
    let source = load_source("src/breadcrumbs/view.rs");

    for needle in ["aria-label=aria_label", "aria-current=\"page\""] {
        assert!(
            source.contains(needle),
            "Breadcrumbs should wire `{needle}` for accessible navigation semantics."
        );
    }
}
