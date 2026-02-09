use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn infield_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/infield_button/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "InfieldButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn infield_button_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/infield_button/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::InfieldButton;"),
        "infield_button module should export `InfieldButton`."
    );
    assert!(
        crate_source.contains("pub use infield_button::InfieldButton;"),
        "crate root should re-export InfieldButton contract."
    );
}

#[test]
fn infield_button_uses_logic_state_model() {
    let logic_source = load_source("src/infield_button/logic.rs");
    let view_source = load_source("src/infield_button/view.rs");

    for needle in [
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
            "InfieldButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(InfieldButtonStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "InfieldButton view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn infield_button_uses_headless_hooks() {
    let source = load_source("src/infield_button/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "InfieldButton should use headless `{needle}` hooks for consistent modality semantics."
        );
    }
}

#[test]
fn infield_button_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/infield_button/view.rs");

    for attr in [
        "data-slot=\"infield-button\"",
        "data-state=state.data_state_attr",
        "data-quiet=state.is_quiet.then_some(\"true\")",
        "data-invalid=state.is_invalid.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-active=move || (is_active || aria.is_pressed.get()).then_some(\"true\")",
        "data-hovered=move || hover.is_hovered.get().then_some(\"true\")",
        "data-pressed=move || aria.is_pressed.get().then_some(\"true\")",
        "data-has-handler=state.has_custom_press_handler.then_some(\"true\")",
        "data-active-mode=state.active_mode_attr",
        "data-quiet-mode=state.quiet_attr",
        "data-invalid-mode=state.invalid_attr",
        "data-disabled-mode=state.disabled_attr",
        "data-aria-source=state.aria_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "InfieldButton should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn infield_button_styles_include_quiet_invalid_and_active_markers() {
    let source = load_source("src/infield_button/styles.rs");

    for selector in [
        ".ui-infield-button--quiet",
        ".ui-infield-button[data-quiet=\"true\"]",
        ".ui-infield-button--invalid",
        ".ui-infield-button[data-invalid=\"true\"]",
        ".ui-infield-button.is-hovered",
        ".ui-infield-button[data-hovered=\"true\"]",
        ".ui-infield-button.is-active",
        ".ui-infield-button[data-active=\"true\"]",
        ".ui-infield-button[data-pressed=\"true\"]",
        ".ui-infield-button--disabled",
        ".ui-infield-button[data-disabled=\"true\"]",
        ".ui-infield-button--focus-visible",
        ".ui-infield-button--custom-class",
        ".ui-infield-button[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "InfieldButton styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn infield_button_docs_page_exists_in_actions_extra() {
    let actions_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "pub(super) fn infield_button() -> AnyView",
        "title=\"InfieldButton\"",
        "slug=\"infield-button\"",
        "<InfieldButton",
    ] {
        assert!(
            actions_extra.contains(needle),
            "actions_extra docs page should contain `{needle}`."
        );
    }
}
