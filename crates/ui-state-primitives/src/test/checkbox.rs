use super::*;

#[test]
fn resolve_checked_axis_prefers_primary_controlled_value_then_alias() {
    let primary = resolve_checked_axis(CheckboxCheckedAxisInput {
        is_checked: Some(true),
        checked: Some(false),
        default_checked: Some(false),
    });
    assert_eq!(primary.mode, CheckboxControlMode::Controlled);
    assert_eq!(primary.source, CheckboxCheckedValueSource::IsChecked);
    assert!(primary.checked);

    let alias = resolve_checked_axis(CheckboxCheckedAxisInput {
        is_checked: None,
        checked: Some(true),
        default_checked: Some(false),
    });
    assert_eq!(alias.mode, CheckboxControlMode::Controlled);
    assert_eq!(alias.source, CheckboxCheckedValueSource::CheckedAlias);
    assert!(alias.checked);
}

#[test]
fn resolve_checked_axis_uses_default_for_uncontrolled_and_falls_back_to_false() {
    let with_default = resolve_checked_axis(CheckboxCheckedAxisInput {
        is_checked: None,
        checked: None,
        default_checked: Some(true),
    });
    assert_eq!(with_default.mode, CheckboxControlMode::Uncontrolled);
    assert_eq!(
        with_default.source,
        CheckboxCheckedValueSource::DefaultChecked
    );
    assert!(with_default.checked);

    let implicit = resolve_checked_axis(CheckboxCheckedAxisInput {
        is_checked: None,
        checked: None,
        default_checked: None,
    });
    assert_eq!(implicit.mode, CheckboxControlMode::Uncontrolled);
    assert_eq!(implicit.source, CheckboxCheckedValueSource::ImplicitDefault);
    assert!(!implicit.checked);
}

#[test]
fn resolve_checked_change_handler_source_prefers_primary_name() {
    assert_eq!(
        resolve_checked_change_handler_source(true, true),
        CheckboxChangeHandlerSource::OnCheckedChange
    );
    assert_eq!(
        resolve_checked_change_handler_source(false, true),
        CheckboxChangeHandlerSource::SetCheckedAlias
    );
    assert_eq!(
        resolve_checked_change_handler_source(false, false),
        CheckboxChangeHandlerSource::Missing
    );
}

#[test]
fn checked_source_and_handler_source_attrs_are_closed_enumerations() {
    assert_eq!(
        CheckboxCheckedValueSource::IsChecked.source_attr(),
        "is-checked"
    );
    assert_eq!(
        CheckboxCheckedValueSource::CheckedAlias.source_attr(),
        "checked-alias"
    );
    assert_eq!(
        CheckboxCheckedValueSource::DefaultChecked.source_attr(),
        "default-checked"
    );
    assert_eq!(
        CheckboxCheckedValueSource::ImplicitDefault.source_attr(),
        "implicit-default"
    );

    assert_eq!(
        CheckboxChangeHandlerSource::OnCheckedChange.source_attr(),
        "on-checked-change"
    );
    assert_eq!(
        CheckboxChangeHandlerSource::SetCheckedAlias.source_attr(),
        "set-checked-alias"
    );
    assert_eq!(
        CheckboxChangeHandlerSource::Missing.source_attr(),
        "missing"
    );
}

#[test]
fn resolve_state_tracks_checked_enabled_interactions() {
    let state = resolve_state(CheckboxStateInput {
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
    let state = resolve_state(CheckboxStateInput {
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
