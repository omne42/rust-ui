use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn close_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/close_button/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "CloseButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn close_button_uses_logic_state_model() {
    let logic_source = load_source("src/close_button/logic.rs");
    let view_source = load_source("src/close_button/view.rs");

    for needle in [
        "pub enum CloseButtonVariant",
        "pub enum CloseButtonSize",
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
            "CloseButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(CloseButtonStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "CloseButton view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn close_button_uses_headless_hooks() {
    let source = load_source("src/close_button/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "CloseButton should use headless `{needle}` hooks for consistent modality semantics."
        );
    }
}

#[test]
fn close_button_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/close_button/view.rs");

    for attr in [
        "data-slot=\"close-button\"",
        "data-state=state.data_state_attr",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-hovered=move || hover.is_hovered.get().then_some(\"true\")",
        "data-pressed=move || aria.is_pressed.get().then_some(\"true\")",
        "data-has-handler=state.has_custom_press_handler.then_some(\"true\")",
        "data-aria-source=state.aria_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "CloseButton should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn close_button_styles_include_variant_size_and_state_markers() {
    let source = load_source("src/close_button/styles.rs");

    for selector in [
        ".ui-close-button--variant-default",
        ".ui-close-button[data-variant=\"over-background\"]",
        ".ui-close-button--size-sm",
        ".ui-close-button[data-size=\"md\"]",
        ".ui-close-button--size-lg",
        ".ui-close-button[data-size=\"xl\"]",
        ".ui-close-button.is-hovered",
        ".ui-close-button[data-hovered=\"true\"]",
        ".ui-close-button.is-active",
        ".ui-close-button[data-pressed=\"true\"]",
        ".ui-close-button--disabled",
        ".ui-close-button[data-disabled=\"true\"]",
        ".ui-close-button--focus-visible",
        ".ui-close-button--custom-class",
        ".ui-close-button[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "CloseButton styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
