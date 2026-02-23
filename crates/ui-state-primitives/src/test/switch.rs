use super::*;

#[test]
fn resolve_checked_axis_marks_control_and_sources() {
    let controlled = resolve_checked_axis(SwitchCheckedAxisInput {
        has_checked: true,
        has_default_checked: true,
        has_on_checked_change: true,
        has_set_checked: false,
    });
    assert_eq!(
        controlled.control_mode,
        SwitchCheckedControlMode::Controlled
    );
    assert!(controlled.is_controlled);
    assert_eq!(controlled.checked_source_attr, "checked");
    assert_eq!(controlled.default_checked_source_attr, "provided");
    assert_eq!(controlled.checked_change_source_attr, "on_checked_change");

    let uncontrolled = resolve_checked_axis(SwitchCheckedAxisInput {
        has_checked: false,
        has_default_checked: false,
        has_on_checked_change: false,
        has_set_checked: true,
    });
    assert_eq!(
        uncontrolled.control_mode,
        SwitchCheckedControlMode::Uncontrolled
    );
    assert!(!uncontrolled.is_controlled);
    assert_eq!(uncontrolled.checked_source_attr, "default");
    assert_eq!(uncontrolled.default_checked_source_attr, "default");
    assert_eq!(uncontrolled.checked_change_source_attr, "set_checked");
}

#[test]
fn resolve_checked_axis_marks_merged_handlers_source() {
    let merged = resolve_checked_axis(SwitchCheckedAxisInput {
        has_checked: true,
        has_default_checked: true,
        has_on_checked_change: true,
        has_set_checked: true,
    });

    assert_eq!(
        merged.checked_change_source_attr,
        "on_checked_change+set_checked"
    );
}

#[test]
fn resolve_state_tracks_checked_enabled_interactions() {
    let state = resolve_state(SwitchStateInput {
        is_checked: true,
        is_disabled: false,
        is_pressed: true,
        is_hovered: true,
        is_focused: true,
        is_focus_visible: true,
    });

    assert!(state.is_checked);
    assert!(!state.is_unchecked);
    assert!(!state.is_disabled);
    assert!(state.is_enabled);
    assert!(state.is_pressed);
    assert!(state.is_hovered);
    assert!(state.is_focused);
    assert!(state.is_focus_visible);
    assert_eq!(state.data_state(), "checked");
}

#[test]
fn resolve_state_clears_interaction_flags_when_disabled() {
    let state = resolve_state(SwitchStateInput {
        is_checked: false,
        is_disabled: true,
        is_pressed: true,
        is_hovered: true,
        is_focused: true,
        is_focus_visible: true,
    });

    assert!(!state.is_checked);
    assert!(state.is_unchecked);
    assert!(state.is_disabled);
    assert!(!state.is_enabled);
    assert!(!state.is_pressed);
    assert!(!state.is_hovered);
    assert!(!state.is_focused);
    assert!(!state.is_focus_visible);
    assert_eq!(state.data_state(), "unchecked");
}
