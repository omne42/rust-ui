use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn description_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/description/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Description internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn description_uses_logic_state_model() {
    let logic_source = load_source("src/description/logic.rs");
    let view_source = load_source("src/description/view.rs");

    for needle in [
        "pub enum DescriptionTone",
        "pub enum DescriptionElement",
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
            "Description logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_content(Some(text))",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(DescriptionStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Description view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn description_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/description/view.rs");

    for attr in [
        "data-slot=\"description\"",
        "slot=\"description\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-truncate=move || state.get().is_truncated.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Description should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn description_styles_include_tone_state_and_markers() {
    let source = load_source("src/description/styles.rs");

    for selector in [
        ".ui-description--tone-default",
        ".ui-description[data-tone=\"default\"]",
        ".ui-description--tone-muted",
        ".ui-description[data-tone=\"muted\"]",
        ".ui-description--tone-negative",
        ".ui-description[data-tone=\"negative\"]",
        ".ui-description--disabled",
        ".ui-description[data-disabled=\"true\"]",
        ".ui-description--truncate",
        ".ui-description[data-truncate=\"true\"]",
        ".ui-description--custom-class",
        ".ui-description[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Description styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
