use super::*;

#[test]
fn orientation_and_tone_contracts_are_stable() {
    assert_eq!(
        SwitchGroupOrientation::Vertical.class_name(),
        "ui-switch-group--orientation-vertical"
    );
    assert_eq!(
        SwitchGroupOrientation::Horizontal.class_name(),
        "ui-switch-group--orientation-horizontal"
    );

    assert_eq!(SwitchGroupOrientation::Vertical.as_attr(), "vertical");
    assert_eq!(SwitchGroupOrientation::Horizontal.as_attr(), "horizontal");

    assert_eq!(
        SwitchGroupTone::Default.class_name(),
        "ui-switch-group--tone-default"
    );
    assert_eq!(
        SwitchGroupTone::Muted.class_name(),
        "ui-switch-group--tone-muted"
    );

    assert_eq!(SwitchGroupTone::Default.as_attr(), "default");
    assert_eq!(SwitchGroupTone::Muted.as_attr(), "muted");
}

#[test]
fn resolve_state_tracks_markers_and_sources() {
    let state = resolve_state(SwitchGroupStateInput {
        orientation: SwitchGroupOrientation::Horizontal,
        tone: SwitchGroupTone::Muted,
        required: true,
        disabled: false,
        invalid: true,
        has_label: true,
        has_description: true,
        has_error_message: true,
        has_custom_label: true,
        has_custom_aria_label: false,
        has_custom_error_message: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.orientation_attr, "horizontal");
    assert_eq!(state.tone_attr, "muted");
    assert!(state.is_required);
    assert!(state.is_invalid);
    assert!(state.has_messages);
    assert!(state.shows_error);
    assert_eq!(state.message_kind_attr, "error");
    assert_eq!(state.data_state_attr, "invalid");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.error_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn resolve_state_prioritizes_invalid_disabled_marker() {
    let state = resolve_state(SwitchGroupStateInput {
        orientation: SwitchGroupOrientation::Vertical,
        tone: SwitchGroupTone::Default,
        required: false,
        disabled: true,
        invalid: true,
        has_label: true,
        has_description: false,
        has_error_message: true,
        has_custom_label: false,
        has_custom_aria_label: false,
        has_custom_error_message: false,
        has_custom_class_name: false,
    });

    assert_eq!(state.data_state_attr, "invalid-disabled");
    assert!(state.shows_error);
    assert_eq!(state.error_source_attr, "default");
}
