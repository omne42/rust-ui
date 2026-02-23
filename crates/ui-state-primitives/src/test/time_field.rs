use super::*;

#[test]
fn normalize_disabled_state_prefers_is_disabled_alias() {
    assert!(normalize_disabled_state(TimeFieldDisabledStateInput {
        is_disabled: Some(true),
        disabled: false,
    }));
    assert!(!normalize_disabled_state(TimeFieldDisabledStateInput {
        is_disabled: None,
        disabled: false,
    }));
    assert!(normalize_disabled_state(TimeFieldDisabledStateInput {
        is_disabled: None,
        disabled: true,
    }));
}

#[test]
fn resolve_value_axis_tracks_mode_and_source_markers() {
    let controlled = resolve_value_axis_state(TimeFieldValueAxisInput {
        is_controlled: true,
        has_default_value: true,
        has_value_change_handler: false,
    });
    assert_eq!(controlled.control_mode_attr, "controlled");
    assert_eq!(controlled.default_value_source_attr, "custom");
    assert_eq!(controlled.value_change_source_attr, "none");
    assert!(!controlled.has_value_change_handler);

    let uncontrolled = resolve_value_axis_state(TimeFieldValueAxisInput {
        is_controlled: false,
        has_default_value: false,
        has_value_change_handler: true,
    });
    assert_eq!(uncontrolled.control_mode_attr, "uncontrolled");
    assert_eq!(uncontrolled.default_value_source_attr, "default");
    assert_eq!(uncontrolled.value_change_source_attr, "on_value_change");
    assert!(uncontrolled.has_value_change_handler);
}
