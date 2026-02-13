use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn clear_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/clear_button/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ClearButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn clear_button_uses_logic_state_model() {
    let logic_source = load_source("src/clear_button/logic.rs");
    let view_source = load_source("src/clear_button/view.rs");

    for needle in [
        "pub enum ClearButtonVariant",
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
            "ClearButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(ClearButtonStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "ClearButton view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn clear_button_uses_headless_hooks() {
    let source = load_source("src/clear_button/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "ClearButton should use headless `{needle}` hooks for consistent modality semantics."
        );
    }
}

#[test]
fn clear_button_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/clear_button/view.rs");

    for attr in [
        "data-slot=\"clear-button\"",
        "data-state=state.data_state_attr",
        "data-variant=state.variant_attr",
        "data-inset=state.is_inset.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-prevent-focus=state.prevent_focus.then_some(\"true\")",
        "data-exclude-tab=state.exclude_from_tab_order.then_some(\"true\")",
        "data-hovered=move || hover.is_hovered.get().then_some(\"true\")",
        "data-pressed=move || aria.is_pressed.get().then_some(\"true\")",
        "data-has-handler=state.has_custom_press_handler.then_some(\"true\")",
        "data-focus-mode=state.focus_mode_attr",
        "data-aria-source=state.aria_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "ClearButton should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn clear_button_styles_include_variant_and_state_markers() {
    let source = load_source("src/clear_button/styles.rs");

    for selector in [
        ".ui-clear-button--variant-default",
        ".ui-clear-button[data-variant=\"over-background\"]",
        ".ui-clear-button--inset",
        ".ui-clear-button[data-inset=\"true\"]",
        ".ui-clear-button.is-hovered",
        ".ui-clear-button[data-hovered=\"true\"]",
        ".ui-clear-button.is-active",
        ".ui-clear-button[data-pressed=\"true\"]",
        ".ui-clear-button--disabled",
        ".ui-clear-button[data-disabled=\"true\"]",
        ".ui-clear-button--focus-visible",
        ".ui-clear-button--custom-class",
        ".ui-clear-button[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ClearButton styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn clear_button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "pub(super) fn clear_button() -> AnyView",
        "title=\"ClearButton\"",
        "slug=\"clear-button\"",
        "title=\"Default + OverBackground\"",
        "title=\"Inset + Focus Mode + Disabled\"",
    ] {
        assert!(
            source.contains(needle),
            "clear-button docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn clear_button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "<Playground title=\"Default + OverBackground\" code_signal=basic_code>",
        "aria_label=\"Clear query\".to_string()",
        "variant=ui_components::ClearButtonVariant::OverBackground",
        "aria_label=\"Dismiss overlay\".to_string()",
        "<Playground title=\"Inset + Focus Mode + Disabled\" code_signal=state_code>",
        "inset=true",
        "prevent_focus=true",
        "aria_label=\"Clear token\".to_string()",
        "class_name=\"docs-clear-button-custom\".to_string()",
        "disabled=true",
        "exclude_from_tab_order=true",
        "aria_label=\"Disabled clear\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "clear-button docs playground should contain `{needle}`.",
        );
    }
}
