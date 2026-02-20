use super::*;

#[test]
fn variant_class_names_are_stable() {
    assert_eq!(
        CheckboxVariant::Default.class_name(),
        "ui-checkbox--variant-default"
    );
    assert_eq!(
        CheckboxVariant::Accent.class_name(),
        "ui-checkbox--variant-accent"
    );
}

#[test]
fn size_class_names_are_stable() {
    assert_eq!(
        CheckboxSize::Default.class_name(),
        "ui-checkbox--size-default"
    );
    assert_eq!(CheckboxSize::Sm.class_name(), "ui-checkbox--size-sm");
    assert_eq!(CheckboxSize::Lg.class_name(), "ui-checkbox--size-lg");
}

#[test]
fn resolve_state_tracks_checked_enabled_interactions() {
    let state = resolve_state(true, false, true, true, true, true);

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
    let state = resolve_state(false, true, true, true, true, true);

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
