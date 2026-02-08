use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn heading_does_not_expose_logic_or_render_modules() {
    let source = load_source("src/heading/mod.rs");

    for needle in ["pub mod logic", "pub mod render"] {
        assert!(
            !source.contains(needle),
            "Heading internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn heading_uses_logic_state_model() {
    let logic_source = load_source("src/heading/logic.rs");
    let render_source = load_source("src/heading/render.rs");

    for needle in [
        "pub enum HeadingLevel",
        "pub enum HeadingTone",
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
            "Heading logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(HeadingStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            render_source.contains(needle),
            "Heading render should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn heading_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/heading/render.rs");

    for attr in [
        "data-slot=\"heading\"",
        "data-level=move || state.get().level_attr",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-truncate=move || state.get().is_truncated.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Heading should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn heading_styles_include_level_tone_and_custom_markers() {
    let source = load_source("src/heading/styles.rs");

    for selector in [
        ".ui-heading--tone-default",
        ".ui-heading[data-tone=\"strong\"]",
        ".ui-heading--tone-muted",
        ".ui-heading--level-1",
        ".ui-heading[data-level=\"6\"]",
        ".ui-heading--truncate",
        ".ui-heading[data-truncate=\"true\"]",
        ".ui-heading--custom-class",
        ".ui-heading[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Heading styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
