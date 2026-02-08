use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn flex_does_not_expose_logic_or_render_modules() {
    let source = load_source("src/flex/mod.rs");

    for needle in ["pub mod logic", "pub mod render"] {
        assert!(
            !source.contains(needle),
            "Flex internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn flex_uses_logic_state_model() {
    let logic_source = load_source("src/flex/logic.rs");
    let render_source = load_source("src/flex/render.rs");

    for needle in [
        "pub enum FlexDirection",
        "pub enum FlexWrap",
        "pub enum FlexJustify",
        "pub enum FlexAlign",
        "pub enum FlexGap",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Flex logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(FlexStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            render_source.contains(needle),
            "Flex render should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn flex_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/flex/render.rs");

    for attr in [
        "data-slot=\"flex\"",
        "data-direction=move || state.get().direction_attr",
        "data-wrap=move || state.get().wrap_attr",
        "data-justify=move || state.get().justify_attr",
        "data-align=move || state.get().align_attr",
        "data-gap=move || state.get().gap_attr",
        "data-inline=move || state.get().is_inline.then_some(\"true\")",
        "data-state=move || state.get().data_state_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Flex should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn flex_styles_include_layout_state_markers() {
    let source = load_source("src/flex/styles.rs");

    for selector in [
        ".ui-flex--inline",
        ".ui-flex[data-inline=\"true\"]",
        ".ui-flex--direction-row",
        ".ui-flex[data-direction=\"column\"]",
        ".ui-flex--wrap-wrap",
        ".ui-flex[data-wrap=\"nowrap\"]",
        ".ui-flex--justify-space-between",
        ".ui-flex[data-justify=\"space-evenly\"]",
        ".ui-flex--align-baseline",
        ".ui-flex[data-align=\"stretch\"]",
        ".ui-flex--gap-md",
        ".ui-flex[data-gap=\"none\"]",
        ".ui-flex--custom-class",
        ".ui-flex[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Flex styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
