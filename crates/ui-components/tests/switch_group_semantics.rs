use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn switch_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/switch_group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SwitchGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn switch_group_uses_logic_state_model() {
    let logic_source = load_source("src/switch_group/logic.rs");
    let view_source = load_source("src/switch_group/view.rs");

    for needle in [
        "pub enum SwitchGroupOrientation",
        "pub enum SwitchGroupTone",
        "pub fn resolve_ids(",
        "pub fn normalize_optional_text(",
        "pub fn normalize_label(",
        "pub fn normalize_description(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_error_message(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "label_source_attr",
        "aria_source_attr",
        "error_source_attr",
        "class_source_attr",
        "message_kind_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "SwitchGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::resolve_ids(id_base)",
        "logic::normalize_label(label)",
        "logic::normalize_description(description)",
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_error_message(error_message, invalid)",
        "logic::resolve_state(SwitchGroupStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "SwitchGroup view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn switch_group_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/switch_group/view.rs");

    for attr in [
        "data-slot=\"switch-group\"",
        "data-slot=\"switch-group-label\"",
        "data-slot=\"switch-group-group\"",
        "data-slot=\"switch-group-description\"",
        "data-slot=\"switch-group-error\"",
        "data-orientation=move || state.get().orientation_attr",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-message-kind=move || state.get().message_kind_attr",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-has-label=move || state.get().has_label.then_some(\"true\")",
        "data-has-description=move || state.get().has_description.then_some(\"true\")",
        "data-has-error=move || state.get().has_error_message.then_some(\"true\")",
        "data-shows-error=move || state.get().shows_error.then_some(\"true\")",
        "data-has-messages=move || state.get().has_messages.then_some(\"true\")",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "SwitchGroup should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn switch_group_styles_include_state_markers() {
    let source = load_source("src/switch_group/styles.rs");

    for selector in [
        ".ui-switch-group--orientation-vertical",
        ".ui-switch-group[data-orientation=\"horizontal\"]",
        ".ui-switch-group--tone-default",
        ".ui-switch-group[data-tone=\"muted\"]",
        ".ui-switch-group--required .ui-switch-group__label::after",
        ".ui-switch-group[data-required=\"true\"] .ui-switch-group__label::after",
        ".ui-switch-group--invalid .ui-switch-group__group",
        ".ui-switch-group[data-invalid=\"true\"] .ui-switch-group__group",
        ".ui-switch-group--disabled",
        ".ui-switch-group[data-disabled=\"true\"]",
        ".ui-switch-group--custom-class",
        ".ui-switch-group[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "SwitchGroup styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn switch_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "pub(super) fn switch_group() -> AnyView",
        "title=\"SwitchGroup\"",
        "slug=\"switch-group\"",
        "description=\"Spectrum/HeroUI-style switch grouping primitive with centralized orientation/tone/validation/message-state contracts and stable data markers.\"",
        "<Playground title=\"Required + Description\" code_signal=base_code>",
        "<Playground title=\"Horizontal + Invalid + Disabled + Custom Class\" code_signal=states_code>",
        "<SwitchGroup",
    ] {
        assert!(
            source.contains(needle),
            "forms_groups switch_group docs should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn switch_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "title=\"Required + Description\"",
        "required=true",
        "aria_label=\"Notification switches\".to_string()",
        "title=\"Horizontal + Invalid + Disabled + Custom Class\"",
        "orientation=SwitchGroupOrientation::Horizontal",
        "tone=SwitchGroupTone::Muted",
        "invalid=true",
        "disabled=true",
        "error_message=\"At least one critical channel must stay enabled.\".to_string()",
        "class_name=\"docs-switch-group-custom\".to_string()",
        "<Switch checked=critical_alerts set_checked=set_critical_alerts disabled=true>",
        "<Switch checked=maintenance_mode set_checked=set_maintenance_mode disabled=true>",
    ] {
        assert!(
            source.contains(needle),
            "forms_groups switch_group docs playgrounds should contain `{needle}` for state-matrix contracts.",
        );
    }
}
