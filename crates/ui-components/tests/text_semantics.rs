use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn text_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/text/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Text internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn text_uses_logic_state_model() {
    let logic_source = load_source("src/text/logic.rs");
    let view_source = load_source("src/text/view.rs");

    for needle in [
        "pub enum TextTone",
        "pub enum TextAlign",
        "pub enum TextWeight",
        "pub enum TextElement",
        "pub fn normalize_optional_text(",
        "pub fn normalize_content(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Text logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_content(Some(text))",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(TextStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Text view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn text_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/text/view.rs");

    for attr in [
        "data-slot=\"text\"",
        "data-tone=move || state.get().tone_attr",
        "data-align=move || state.get().align_attr",
        "data-weight=move || state.get().weight_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-truncate=move || state.get().is_truncated.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Text should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn text_styles_include_tone_align_weight_and_markers() {
    let source = load_source("src/text/styles.rs");

    for selector in [
        ".ui-text--tone-default",
        ".ui-text[data-tone=\"default\"]",
        ".ui-text--tone-subtle",
        ".ui-text--tone-strong",
        ".ui-text--align-start",
        ".ui-text[data-align=\"start\"]",
        ".ui-text--align-center",
        ".ui-text--align-end",
        ".ui-text--align-justify",
        ".ui-text--weight-regular",
        ".ui-text[data-weight=\"regular\"]",
        ".ui-text--weight-medium",
        ".ui-text--weight-semibold",
        ".ui-text--weight-bold",
        ".ui-text--disabled",
        ".ui-text[data-disabled=\"true\"]",
        ".ui-text--truncate",
        ".ui-text[data-truncate=\"true\"]",
        ".ui-text--custom-class",
        ".ui-text[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Text styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
