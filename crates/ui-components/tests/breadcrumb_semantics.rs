use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn breadcrumb_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/breadcrumb/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Breadcrumb internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn breadcrumb_is_exported_from_module_and_registered_in_crate() {
    let module_source = load_source("src/breadcrumb/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::{"),
        "breadcrumb module should export breadcrumb primitive family.",
    );
    assert!(
        crate_source.contains("pub mod breadcrumb;"),
        "crate root should register breadcrumb module.",
    );
}

#[test]
fn breadcrumb_logic_exposes_state_helpers() {
    let source = load_source("src/breadcrumb/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_href(",
        "pub fn resolve_root_state(",
        "pub fn resolve_slot_state(",
        "pub fn resolve_link_state(",
        "pub fn resolve_separator_state(",
        "pub fn compose_class_name(",
        "pub fn compose_link_class_name(",
        "pub fn compose_separator_class_name(",
        "DEFAULT_ARIA_LABEL",
    ] {
        assert!(
            source.contains(needle),
            "Breadcrumb logic should include `{needle}` for centralized state/source contracts.",
        );
    }
}

#[test]
fn breadcrumb_view_uses_logic_state_contracts() {
    let source = load_source("src/breadcrumb/view.rs");

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_root_state(BreadcrumbRootStateInput {",
        "logic::resolve_link_state(BreadcrumbLinkStateInput {",
        "logic::resolve_separator_state(BreadcrumbSeparatorStateInput {",
        "data-slot=\"breadcrumb\"",
        "data-state=state.state_attr",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
        "data-href-state=state.href_state_attr",
        "data-content-source=state.content_source_attr",
        "data-label-source=\"default\"",
    ] {
        assert!(
            source.contains(needle),
            "Breadcrumb view should expose stable marker contract `{needle}`.",
        );
    }
}

#[test]
fn breadcrumb_styles_include_state_and_accessibility_markers() {
    let source = load_source("src/breadcrumb/styles.rs");

    for selector in [
        ".ui-breadcrumb {",
        ".ui-breadcrumb[data-aria-source=\"custom\"]",
        ".ui-breadcrumb__list {",
        ".ui-breadcrumb__link--placeholder",
        ".ui-breadcrumb__link[data-href-state=\"absent\"]",
        ".ui-breadcrumb__separator--custom-content",
        ".ui-breadcrumb__separator[data-content-source=\"custom\"]",
        ".ui-breadcrumb__ellipsis-label",
        ".ui-breadcrumb--custom-class",
        "@media (forced-colors: active)",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(
            source.contains(selector),
            "Breadcrumb styles should include `{selector}` as stable style markers.",
        );
    }
}

#[test]
fn breadcrumb_css_is_aggregated_in_component_layer() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::breadcrumb::styles::CSS);"),
        "ui-components css aggregator should include breadcrumb styles.",
    );
}

#[test]
fn breadcrumb_docs_page_contains_state_source_playground() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb_shadcn.rs",
    );

    for needle in [
        "pub(super) fn breadcrumb_primitives() -> AnyView",
        "title=\"BreadcrumbList\"",
        "slug=\"breadcrumb-list\"",
        "State + Source Markers",
        "data-aria-source",
    ] {
        assert!(
            source.contains(needle),
            "collections_breadcrumb_shadcn docs page should contain `{needle}`.",
        );
    }
}
