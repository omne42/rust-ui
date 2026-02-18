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
        "pub use ui_state_primitives::breadcrumbs::{",
        "pub fn resolve_state(items: &[BreadcrumbItem])",
        "breadcrumbs_primitives::resolve_state(BreadcrumbsStateInput {",
        "items: &item_inputs",
        "pub fn resolve_root_state(",
        "-> BreadcrumbsRootState",
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
    assert!(
        view_source.contains("} = logic::resolve_root_state(aria_label, class_name);"),
        "Breadcrumbs view should consume normalized root state from logic."
    );
}

#[test]
fn breadcrumbs_emits_baseline_style_slots_and_root_attrs() {
    let source = load_source("src/breadcrumbs/view.rs");

    for needle in [
        "data-slot=\"breadcrumbs\"",
        "data-slot=\"breadcrumbs-list\"",
        "data-slot=\"breadcrumbs-item\"",
        "data-slot=\"breadcrumbs-link\"",
        "data-slot=\"breadcrumbs-current\"",
        "data-slot=\"breadcrumbs-label\"",
        "data-slot=\"breadcrumbs-separator\"",
        "data-aria-source=aria_source_attr",
        "data-class-source=class_source_attr",
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
            "Breadcrumbs should expose `{needle}` for baseline-style styling and regression checks."
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

#[test]
fn breadcrumbs_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "pub(super) fn breadcrumbs() -> AnyView",
        "title=\"Breadcrumbs\"",
        "slug=\"breadcrumbs\"",
        "description=\"Breadcrumb nav with current-page semantics and baseline-style root state attrs.\"",
        "<Playground title=\"Trail\" code_signal=code>",
        "<Playground title=\"Label-Only + Empty\" code_signal=states_code>",
        "<Breadcrumbs items=items />",
        "aria_label=\"Label-only trail\".to_string()",
        "aria_label=\"Empty trail\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "collections docs page should include `{needle}` for breadcrumbs coverage.",
        );
    }
}

#[test]
fn breadcrumbs_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "label: \"Home\".to_string()",
        "href: Some(\"#/docs/welcome\".to_string())",
        "label: \"Components\".to_string()",
        "href: Some(\"#/components\".to_string())",
        "label: \"Breadcrumbs\".to_string()",
        "href: None",
        "label: \"Library\".to_string()",
        "label: \"UI\".to_string()",
        "label: \"Current\".to_string()",
        "let empty_items = Vec::<BreadcrumbItem>::new();",
        "\"all labels (no links)\"",
        "\"empty trail (0 items)\"",
    ] {
        assert!(
            source.contains(needle),
            "breadcrumbs docs playgrounds should contain `{needle}`.",
        );
    }
}
