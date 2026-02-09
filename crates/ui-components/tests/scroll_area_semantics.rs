use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn scroll_area_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/scroll_area/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ScrollArea internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn scroll_area_uses_logic_state_model() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");

    for needle in [
        "pub enum ScrollAreaOrientation",
        "pub struct ScrollAreaStateInput",
        "pub struct ScrollAreaState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_max_height(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ScrollArea logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(ScrollAreaStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollArea view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn scroll_area_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/scroll_area/view.rs");

    for attr in [
        "data-slot=\"scroll-area\"",
        "data-orientation=state.orientation_attr",
        "data-disabled=state.disabled.then_some(\"true\")",
        "data-max-height=state.max_height_attr",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "role=\"region\"",
        "aria-label=aria_label",
        "data-slot=\"scroll-area-viewport\"",
        "aria-disabled=state.disabled.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "ScrollArea should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn scroll_area_styles_include_state_marker_contracts() {
    let source = load_source("src/scroll_area/styles.rs");

    for selector in [
        ".ui-scroll-area--vertical .ui-scroll-area__viewport",
        ".ui-scroll-area[data-orientation=\"vertical\"] .ui-scroll-area__viewport",
        ".ui-scroll-area--horizontal .ui-scroll-area__viewport",
        ".ui-scroll-area[data-orientation=\"horizontal\"] .ui-scroll-area__viewport",
        ".ui-scroll-area--both .ui-scroll-area__viewport",
        ".ui-scroll-area[data-orientation=\"both\"] .ui-scroll-area__viewport",
        ".ui-scroll-area--max-height-custom .ui-scroll-area__viewport",
        ".ui-scroll-area[data-max-height=\"custom\"] .ui-scroll-area__viewport",
        ".ui-scroll-area--disabled",
        ".ui-scroll-area[data-disabled=\"true\"]",
        "--ui-scroll-area-max-h",
    ] {
        assert!(
            source.contains(selector),
            "ScrollArea styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn scroll_area_docs_page_exists_in_layout_extra() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn scroll_area() -> AnyView",
        "title=\"ScrollArea\"",
        "slug=\"scroll-area\"",
        "<ScrollArea",
    ] {
        assert!(
            docs.contains(needle),
            "ScrollArea docs page should contain `{needle}`."
        );
    }
}
