use super::*;

fn resolve(input: FieldButtonResolveInput) -> FieldButtonResolved {
    resolve_props(input)
}

#[test]
fn resolve_props_defaults_to_field_button_aria_label() {
    let state = resolve(FieldButtonResolveInput {
        is_quiet: false,
        is_invalid: false,
        is_disabled: false,
        is_active: false,
        aria_label: None,
        class_name: None,
        button_type: ButtonType::Button,
        on_press: None,
    });

    assert_eq!(state.aria_label, DEFAULT_ARIA_LABEL);
}

#[test]
fn resolve_props_maps_quiet_and_invalid_to_button_tokens() {
    let state = resolve(FieldButtonResolveInput {
        is_quiet: true,
        is_invalid: true,
        is_disabled: false,
        is_active: false,
        aria_label: None,
        class_name: None,
        button_type: ButtonType::Button,
        on_press: None,
    });

    assert_eq!(state.variant, ButtonVariant::Ghost);
    assert_eq!(state.color, ButtonColor::Danger);
}

#[test]
fn resolve_state_centralizes_quiet_invalid_and_active_markers() {
    let state = resolve_state(FieldButtonStateInput {
        tone: FieldButtonTone::Quiet,
        validation: FieldButtonValidation::Invalid,
        is_disabled: false,
        is_active: true,
        has_custom_aria_label: false,
        has_custom_class_name: false,
        has_custom_press_handler: false,
    });

    assert_eq!(state.variant, ButtonVariant::Ghost);
    assert_eq!(state.color, ButtonColor::Danger);
    assert!(state.is_active);
}

#[test]
fn resolve_state_consumes_button_state_primitive_for_disabled_contract() {
    let state = resolve_state(FieldButtonStateInput {
        tone: FieldButtonTone::Default,
        validation: FieldButtonValidation::Default,
        is_disabled: true,
        is_active: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
        has_custom_press_handler: false,
    });

    assert!(state.is_disabled);
}

#[test]
fn resolve_props_marks_custom_class_and_handler_sources() {
    let state = resolve(FieldButtonResolveInput {
        is_quiet: false,
        is_invalid: false,
        is_disabled: false,
        is_active: false,
        aria_label: Some("Open".to_string()),
        class_name: Some("docs-field-button-custom".to_string()),
        button_type: ButtonType::Button,
        on_press: Some(Callback::new(|_| {})),
    });

    assert!(state.class_name.contains("ui-field-button--custom-class"));
    assert!(state.class_name.contains("ui-field-button--custom-handler"));
    assert!(
        state
            .class_name
            .contains("ui-field-button--custom-aria-label")
    );
    assert!(state.class_name.contains("docs-field-button-custom"));
}

#[test]
fn resolve_props_adds_active_marker_when_forced_active() {
    let state = resolve(FieldButtonResolveInput {
        is_quiet: false,
        is_invalid: false,
        is_disabled: false,
        is_active: true,
        aria_label: None,
        class_name: None,
        button_type: ButtonType::Button,
        on_press: None,
    });

    assert!(state.class_name.contains("ui-field-button--active"));
    assert!(state.class_name.contains("is-active"));
}

#[test]
fn compose_class_name_uses_state_only_and_appends_custom_class() {
    let class_name = compose_class_name(
        FieldButtonState {
            variant: ButtonVariant::Default,
            color: ButtonColor::Default,
            tone: FieldButtonTone::Quiet,
            validation: FieldButtonValidation::Default,
            is_disabled: true,
            is_active: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_press_handler: false,
        },
        Some("docs-field-button-custom"),
    );

    assert!(class_name.contains("ui-field-button--quiet"));
    assert!(class_name.contains("ui-field-button--disabled"));
    assert!(class_name.contains("ui-field-button--custom-class"));
    assert!(class_name.contains("docs-field-button-custom"));
}
