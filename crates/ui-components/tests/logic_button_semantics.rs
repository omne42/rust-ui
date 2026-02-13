use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn logic_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/logic_button/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "LogicButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn logic_button_uses_logic_state_model() {
    let logic_source = load_source("src/logic_button/logic.rs");
    let view_source = load_source("src/logic_button/view.rs");

    for needle in [
        "pub enum LogicButtonVariant",
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
            "LogicButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(LogicButtonStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "LogicButton view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn logic_button_uses_headless_hooks() {
    let source = load_source("src/logic_button/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "LogicButton should use headless `{needle}` hooks for consistent modality semantics."
        );
    }
}

#[test]
fn logic_button_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/logic_button/view.rs");

    for attr in [
        "data-slot=\"logic-button\"",
        "data-state=state.data_state_attr",
        "data-variant=state.variant_attr",
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
            "LogicButton should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn logic_button_styles_include_variant_and_state_markers() {
    let source = load_source("src/logic_button/styles.rs");

    for selector in [
        ".ui-logic-button--variant-and",
        ".ui-logic-button[data-variant=\"or\"]",
        ".ui-logic-button.is-hovered",
        ".ui-logic-button[data-hovered=\"true\"]",
        ".ui-logic-button.is-active",
        ".ui-logic-button[data-pressed=\"true\"]",
        ".ui-logic-button--disabled",
        ".ui-logic-button[data-disabled=\"true\"]",
        ".ui-logic-button--focus-visible",
        ".ui-logic-button--custom-class",
        ".ui-logic-button[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "LogicButton styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn logic_button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "pub(super) fn logic_button() -> AnyView",
        "title=\"LogicButton\"",
        "slug=\"logic-button\"",
        "description=\"Spectrum-style boolean operator button with centralized variant normalization, headless press/hover/focus behavior, and stable state/source data contracts.\"",
        "<Playground title=\"AND + OR variants\" code_signal=basic_code>",
        "<Playground title=\"Custom class + Disabled\" code_signal=state_code>",
        "<LogicButton",
    ] {
        assert!(
            source.contains(needle),
            "actions_extra docs should include `{needle}` for logic-button primary playground coverage.",
        );
    }
}

#[test]
fn logic_button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "title=\"AND + OR variants\"",
        "variant=LogicButtonVariant::And",
        "\"AND\"",
        "variant=LogicButtonVariant::Or",
        "\"OR\"",
        "title=\"Custom class + Disabled\"",
        "class_name=\"docs-logic-button-custom\".to_string()",
        "\"Custom\"",
        "disabled=true",
        "\"Disabled\"",
    ] {
        assert!(
            source.contains(needle),
            "logic-button docs playgrounds should contain `{needle}`.",
        );
    }
}
