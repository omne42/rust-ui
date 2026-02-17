use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn field_button_has_no_compat_module_and_is_reexported_from_button_field() {
    let source = load_source("src/lib.rs");

    for needle in ["pub use button::field::FieldButton;", "FieldButton"] {
        assert!(
            source.contains(needle),
            "crate re-exports should include `{needle}` from button/field."
        );
    }

    assert!(
        !source.contains("pub mod field_button;"),
        "compat module `src/field_button.rs` should not be reintroduced."
    );
}

#[test]
fn field_button_implementation_lives_under_button_field_module() {
    let source = load_source("src/button/field/mod.rs");

    for needle in [
        "pub mod styles;",
        "pub const DEFAULT_ARIA_LABEL: &str = \"FieldButton\";",
        "pub fn FieldButton(",
    ] {
        assert!(
            source.contains(needle),
            "button/field module should define `{needle}` as canonical FieldButton contract."
        );
    }
}

#[test]
fn field_button_api_naming_uses_is_prefix_only() {
    let source = load_source("src/button/field/mod.rs");

    for needle in [
        "#[prop(optional)] is_quiet: bool",
        "#[prop(optional)] is_invalid: bool",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_active: bool",
    ] {
        assert!(
            source.contains(needle),
            "FieldButton API naming should include `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] quiet: bool",
        "#[prop(optional)] invalid: bool",
        "#[prop(optional)] disabled: bool",
    ] {
        assert!(
            !source.contains(forbidden),
            "FieldButton should not expose legacy boolean alias `{forbidden}`."
        );
    }
}

#[test]
fn field_button_uses_button_state_machine_and_headless_hooks() {
    let source = load_source("src/button/field/mod.rs");

    for needle in [
        "let state = logic::resolve_state(logic::ButtonStateInput {",
        "let aria = use_button(ButtonOptions {",
        "let focus_ring = use_focus_ring(FocusRingOptions {",
        "let hover = use_hover(HoverOptions {",
        "motion::attach_motion(",
    ] {
        assert!(
            source.contains(needle),
            "FieldButton should be wired through shared button logic/headless hooks via `{needle}`."
        );
    }
}

#[test]
fn field_button_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/button/field/mod.rs");

    for attr in [
        "data-slot=\"field-button\"",
        "data-state=data_state_attr",
        "data-quiet=is_quiet.then_some(\"true\")",
        "data-invalid=is_invalid.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-active=move || (is_active || aria.is_pressed.get()).then_some(\"true\")",
        "data-hovered=move || hover.is_hovered.get().then_some(\"true\")",
        "data-pressed=move || aria.is_pressed.get().then_some(\"true\")",
        "data-has-handler=has_custom_press_handler.then_some(\"true\")",
        "data-active-mode=if is_active { \"forced\" } else { \"interactive\" }",
        "data-quiet-mode=if is_quiet { \"true\" } else { \"false\" }",
        "data-invalid-mode=if is_invalid { \"true\" } else { \"false\" }",
        "data-disabled-mode=if state.is_disabled { \"true\" } else { \"false\" }",
        "data-aria-source=if has_custom_aria_label { \"custom\" } else { \"default\" }",
        "data-custom-class=has_custom_class_name.then_some(\"true\")",
        "data-class-source=if has_custom_class_name { \"custom\" } else { \"default\" }",
    ] {
        assert!(
            source.contains(attr),
            "FieldButton should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn field_button_styles_include_quiet_invalid_and_active_markers() {
    let source = load_source("src/button/field/styles.rs");

    for selector in [
        ".ui-field-button--quiet",
        ".ui-field-button[data-quiet=\"true\"]",
        ".ui-field-button--invalid",
        ".ui-field-button[data-invalid=\"true\"]",
        ".ui-field-button.is-hovered",
        ".ui-field-button[data-hovered=\"true\"]",
        ".ui-field-button.is-active",
        ".ui-field-button[data-active=\"true\"]",
        ".ui-field-button[data-pressed=\"true\"]",
        ".ui-field-button--disabled",
        ".ui-field-button[data-disabled=\"true\"]",
        ".ui-field-button--focus-visible",
        ".ui-field-button--custom-class",
        ".ui-field-button[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "FieldButton styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn field_button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "pub(super) fn field_button() -> AnyView",
        "title=\"FieldButton\"",
        "slug=\"field-button\"",
        "description=\"baseline-style field trigger button with centralized quiet/invalid/active/disabled state contracts and headless press/hover/focus behavior.\"",
        "<Playground title=\"Default + Quiet\" code_signal=default_code>",
        "<Playground title=\"Invalid + Active + Disabled\" code_signal=state_code>",
        "<FieldButton",
    ] {
        assert!(
            source.contains(needle),
            "actions_extra docs page should include `{needle}` for field_button primary playground coverage.",
        );
    }
}

#[test]
fn field_button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "title=\"Default + Quiet\"",
        "aria_label=\"Open options\".to_string()",
        "\"Options\"",
        "is_quiet=true",
        "aria_label=\"Open calendar\".to_string()",
        "\"📅\"",
        "title=\"Invalid + Active + Disabled\"",
        "is_invalid=true",
        "is_active=true",
        "aria_label=\"Invalid trigger\".to_string()",
        "class_name=\"docs-field-button-custom\".to_string()",
        "\"Needs fix\"",
        "is_disabled=true",
        "aria_label=\"Disabled trigger\".to_string()",
        "\"Disabled\"",
    ] {
        assert!(
            source.contains(needle),
            "field_button docs playgrounds should contain `{needle}`.",
        );
    }
}
