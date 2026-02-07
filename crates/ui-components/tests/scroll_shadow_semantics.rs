use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn scroll_shadow_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/scroll_shadow/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ScrollShadow internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_uses_logic_state_model() {
    let view_source = load_source("src/scroll_shadow/view.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");

    for needle in [
        "pub struct ScrollShadowStateInput",
        "pub struct ScrollShadowState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_max_height(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn resolve_edge_state(",
        "pub fn edge_state_attr(",
        "pub fn is_scrollable(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ScrollShadow logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(ScrollShadowStateInput {",
        "logic::compose_class_name(class_name, state)",
        "logic::edge_state_attr(shadow_top.get(), shadow_bottom.get())",
        "logic::is_scrollable(shadow_top.get(), shadow_bottom.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollShadow view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/scroll_shadow/view.rs");

    for attr in [
        "data-slot=\"scroll-shadow\"",
        "data-max-height=state.max_height_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-state=move || logic::edge_state_attr(shadow_top.get(), shadow_bottom.get())",
        "data-scrollable=move || {",
        "data-shadow-top=move || shadow_top.get().then_some(\"true\")",
        "data-shadow-bottom=move || shadow_bottom.get().then_some(\"true\")",
        "class:ui-scroll-shadow--shadow-top=move || shadow_top.get()",
        "class:ui-scroll-shadow--shadow-bottom=move || shadow_bottom.get()",
        "class:ui-scroll-shadow--scrollable=move || {",
    ] {
        assert!(
            source.contains(attr),
            "ScrollShadow should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn scroll_shadow_styles_include_state_marker_contracts() {
    let source = load_source("src/scroll_shadow/styles.rs");

    for selector in [
        ".ui-scroll-shadow--scrollable",
        ".ui-scroll-shadow[data-scrollable=\"true\"]",
        ".ui-scroll-shadow--max-height-custom .ui-scroll-shadow__viewport",
        ".ui-scroll-shadow[data-max-height=\"custom\"] .ui-scroll-shadow__viewport",
        ".ui-scroll-shadow--shadow-top::before",
        ".ui-scroll-shadow[data-shadow-top=\"true\"]::before",
        ".ui-scroll-shadow[data-state=\"both\"]::before",
        ".ui-scroll-shadow--shadow-bottom::after",
        ".ui-scroll-shadow[data-shadow-bottom=\"true\"]::after",
        ".ui-scroll-shadow[data-state=\"both\"]::after",
        "--ui-scroll-shadow-max-h",
    ] {
        assert!(
            source.contains(selector),
            "ScrollShadow styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
