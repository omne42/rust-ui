use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn input_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/input/group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "InputGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn input_group_uses_logic_state_model() {
    let logic_source = load_source("src/input/group/logic.rs");
    let view_source = load_source("src/input/group/view.rs");

    for needle in [
        "pub enum InputGroupPhase",
        "pub enum InputGroupAttachment",
        "pub struct InputGroupStateInput",
        "pub struct InputGroupState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "label_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "InputGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(InputGroupStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "InputGroup view should derive root state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn input_group_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/input/group/view.rs");

    for attr in [
        "data-slot=\"input-group\"",
        "data-state=move || state.get().phase_attr",
        "data-attachment=move || state.get().attachment_attr",
        "data-enabled=move || state.get().is_enabled.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-attached=move || state.get().is_attached.then_some(\"true\")",
        "data-detached=move || state.get().is_detached.then_some(\"true\")",
        "data-has-start=move || state.get().has_start_content.then_some(\"true\")",
        "data-has-end=move || state.get().has_end_content.then_some(\"true\")",
        "data-label-source=move || state.get().label_source_attr",
        "data-custom-label=move || state.get().has_custom_label.then_some(\"true\")",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "role=\"group\"",
    ] {
        assert!(
            source.contains(attr),
            "InputGroup should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn input_group_styles_define_attachment_and_state_contracts() {
    let source = load_source("src/input/group/styles.rs");

    for selector in [
        ".ui-input-group--invalid .ui-input-group__control",
        ".ui-input-group[data-invalid=\"true\"] .ui-input-group__control",
        ".ui-input-group--state-disabled",
        ".ui-input-group[data-state=\"disabled\"]",
        ".ui-input-group[data-disabled=\"true\"]",
        ".ui-input-group--detached .ui-input-group__control",
        ".ui-input-group[data-attachment=\"detached\"] .ui-input-group__control",
        ".ui-input-group--label-custom",
        ".ui-input-group[data-label-source=\"custom\"]",
        ".ui-input-group--custom-class",
        ".ui-input-group[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "InputGroup styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn input_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn input_group() -> AnyView",
        "title=\"InputGroup\"",
        "slug=\"input-group\"",
        "description=\"Composes one or more inputs with shared prefix/suffix addons and baseline-style state contracts.\"",
        "<Playground title=\"Attached Addons\" code_signal=code>",
        "<Playground title=\"Detached + Disabled\" code_signal=states_code>",
        "<InputGroup",
    ] {
        assert!(
            source.contains(needle),
            "forms docs should include `{needle}` for input_group primary playground coverage.",
        );
    }
}

#[test]
fn input_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Attached Addons\"",
        "aria_label=\"Email input group\".to_string()",
        "start_content=move || view! { <span>\"@\"</span> }",
        "end_content=move || view! { <span>\".com\"</span> }",
        "id=\"docs-input-group-email\".to_string()",
        "title=\"Detached + Disabled\"",
        "attached=false",
        "aria_label=\"Search controls\".to_string()",
        "id=\"docs-input-group-search\".to_string()",
        "disabled=true",
        "aria_label=\"Disabled controls\".to_string()",
        "id=\"docs-input-group-disabled\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "input_group docs playgrounds should contain `{needle}`.",
        );
    }
}
