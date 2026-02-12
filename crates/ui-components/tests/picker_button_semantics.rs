use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn picker_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/picker_button/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "PickerButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn picker_button_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/picker_button/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::PickerButton;"),
        "picker_button module should export `PickerButton`."
    );
    assert!(
        crate_source.contains("pub use picker_button::PickerButton;"),
        "crate root should re-export `PickerButton`."
    );
}

#[test]
fn picker_button_logic_exposes_state_helpers() {
    let source = load_source("src/picker_button/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(input: PickerButtonStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: PickerButtonState)",
        "DEFAULT_ARIA_LABEL",
    ] {
        assert!(
            source.contains(needle),
            "PickerButton logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn picker_button_view_uses_logic_state_contracts() {
    let source = load_source("src/picker_button/view.rs");

    for needle in [
        "pub fn PickerButton(",
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(PickerButtonStateInput {",
        "logic::compose_class_name(class_name, state)",
        "<FieldButton",
        "on_press: Option<OnPress>",
        "data-slot=\"picker-button\"",
        "data-state=state.data_state_attr",
        "data-quiet=state.is_quiet.then_some(\"true\")",
        "data-invalid=state.is_invalid.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-active=state.is_forced_active.then_some(\"true\")",
        "data-has-handler=state.has_custom_press_handler.then_some(\"true\")",
        "data-active-mode=state.active_mode_attr",
        "data-quiet-mode=state.quiet_attr",
        "data-invalid-mode=state.invalid_attr",
        "data-disabled-mode=state.disabled_attr",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
        "data-handler-source=state.handler_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "PickerButton view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn picker_button_styles_include_state_and_source_markers() {
    let source = load_source("src/picker_button/styles.rs");

    for selector in [
        ".ui-picker-button {",
        ".ui-picker-button[data-state=\"disabled\"]",
        ".ui-picker-button[data-state=\"invalid\"]",
        ".ui-picker-button[data-state=\"active\"]",
        ".ui-picker-button[data-quiet=\"true\"]",
        ".ui-picker-button[data-invalid=\"true\"]",
        ".ui-picker-button[data-disabled=\"true\"]",
        ".ui-picker-button[data-active=\"true\"]",
        ".ui-picker-button[data-has-handler=\"true\"]",
        ".ui-picker-button[data-aria-source=\"custom\"]",
        ".ui-picker-button[data-class-source=\"custom\"]",
        ".ui-picker-button[data-handler-source=\"custom\"]",
        ".ui-picker-button--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "PickerButton styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn picker_button_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::picker_button::styles::CSS);"),
        "ui-components css aggregator should include picker_button styles."
    );
}

#[test]
fn picker_button_docs_page_contains_state_source_playground() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/actions_extra_picker_button.rs",
    );

    for needle in [
        "pub(super) fn picker_button() -> AnyView",
        "title=\"PickerButton\"",
        "slug=\"picker-button\"",
        "State + Source Markers",
        "data-handler-source",
        "<PickerButton",
    ] {
        assert!(
            source.contains(needle),
            "actions_extra_picker_button docs page should contain `{needle}`."
        );
    }
}

#[test]
fn picker_button_docs_state_matrix_playground_locks_contract_values() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/actions_extra_picker_button.rs",
    );

    for needle in [
        "title=\"State Matrix\"",
        "<PickerButton quiet=true>",
        "\"Filter\"",
        "<PickerButton invalid=true>",
        "\"Required\"",
        "<PickerButton disabled=true>",
        "\"Disabled\"",
    ] {
        assert!(
            source.contains(needle),
            "PickerButton docs state-matrix playground should contain `{needle}`.",
        );
    }
}

#[test]
fn picker_button_docs_state_source_playground_locks_contract_values() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/actions_extra_picker_button.rs",
    );

    for needle in [
        "title=\"State + Source Markers\"",
        "quiet=true",
        "invalid=true",
        "is_active=true",
        "aria_label=\"Inspect picker trigger\".to_string()",
        "class_name=\"docs-picker-button-state\".to_string()",
        "on_press=marker_press",
        "\"Inspect markers\"",
        "Inspect wrapper markers like `data-state`, `data-quiet`, `data-invalid`, `data-disabled`, `data-active`, `data-has-handler`, `data-aria-source`, `data-class-source`, and `data-handler-source`.",
    ] {
        assert!(
            source.contains(needle),
            "PickerButton docs state/source playground should contain `{needle}`.",
        );
    }
}
