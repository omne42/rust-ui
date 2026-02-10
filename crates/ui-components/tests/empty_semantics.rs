use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn empty_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/empty/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Empty internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn empty_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/empty/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use logic::EmptyMediaVariant;"),
        "empty module should export `EmptyMediaVariant` from logic."
    );
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
fn empty_logic_exposes_state_helpers() {
    let source = load_source("src/empty/logic.rs");

    for needle in [
        "pub enum EmptyMediaVariant",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(input: EmptyPartStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: EmptyPartState)",
    ] {
        assert!(
            source.contains(needle),
            "Empty logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn empty_view_uses_logic_state_contracts() {
    let source = load_source("src/empty/view.rs");

    for needle in [
        "pub fn Empty(",
        "pub fn EmptyHeader(",
        "pub fn EmptyTitle(",
        "pub fn EmptyDescription(",
        "pub fn EmptyContent(",
        "pub fn EmptyMedia(",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(EmptyPartStateInput {",
        "logic::compose_class_name(class_name, state)",
        "data-slot=state.slot_attr",
        "data-state=state.state_attr",
        "data-class-source=state.class_source_attr",
        "data-variant-source=state.variant_source_attr",
        "data-variant=state.media_variant_attr",
    ] {
        assert!(
            source.contains(needle),
            "Empty view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn empty_styles_include_state_and_source_markers() {
    let source = load_source("src/empty/styles.rs");

    for selector in [
        ".ui-empty {",
        ".ui-empty[data-state=\"root\"]",
        ".ui-empty__header[data-state=\"header\"]",
        ".ui-empty__title[data-state=\"title\"]",
        ".ui-empty__description[data-state=\"description\"]",
        ".ui-empty__content[data-state=\"content\"]",
        ".ui-empty__media[data-state=\"media\"]",
        ".ui-empty__media[data-variant=\"icon\"]",
        ".ui-empty__media[data-variant-source=\"custom\"]",
        ".ui-empty--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "Empty styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn empty_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::empty::styles::CSS);"),
        "ui-components css aggregator should include empty styles."
    );
}

#[test]
fn empty_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_empty.rs");

    for needle in [
        "pub(super) fn empty() -> AnyView",
        "title=\"Empty\"",
        "slug=\"empty\"",
        "State + Source Markers",
        "data-variant-source",
        "<Empty",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_empty docs page should contain `{needle}`."
        );
    }
}
