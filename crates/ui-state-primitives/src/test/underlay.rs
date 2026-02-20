use super::*;

#[test]
fn resolve_state_tracks_state_tone_and_source_markers() {
    let state = resolve_state(UnderlayPartStateInput {
        slot: UnderlaySlot::Root,
        open: true,
        transparent: true,
        disabled: false,
        has_on_close: true,
        has_custom_transparent: true,
        has_custom_disabled: false,
        has_custom_close_handler: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.slot_attr, "underlay");
    assert_eq!(state.base_class, "ui-underlay");
    assert_eq!(state.state_attr, "open");
    assert_eq!(state.tone_attr, "transparent");
    assert_eq!(state.close_mode_attr, "interactive");
    assert_eq!(state.open_attr, Some("true"));
    assert_eq!(state.transparent_attr, Some("true"));
    assert_eq!(state.disabled_attr, None);
    assert_eq!(state.interactive_attr, Some("true"));
    assert_eq!(state.transparent_source_attr, "custom");
    assert_eq!(state.disabled_source_attr, "default");
    assert_eq!(state.close_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn resolve_state_disables_interaction_when_disabled() {
    let state = resolve_state(UnderlayPartStateInput {
        slot: UnderlaySlot::Root,
        open: true,
        transparent: false,
        disabled: true,
        has_on_close: true,
        has_custom_transparent: false,
        has_custom_disabled: true,
        has_custom_close_handler: false,
        has_custom_class_name: false,
    });

    assert!(!state.is_open);
    assert!(!state.is_interactive);
    assert_eq!(state.state_attr, "disabled");
    assert_eq!(state.close_mode_attr, "static");
    assert_eq!(state.open_attr, None);
    assert_eq!(state.interactive_attr, None);
    assert_eq!(state.disabled_attr, Some("true"));
}

#[test]
fn helper_attrs_cover_all_paths() {
    assert_eq!(state_attr(false, false), "closed");
    assert_eq!(state_attr(true, false), "open");
    assert_eq!(state_attr(true, true), "disabled");

    assert_eq!(tone_attr(false), "scrim");
    assert_eq!(tone_attr(true), "transparent");

    assert_eq!(close_mode_attr(false), "static");
    assert_eq!(close_mode_attr(true), "interactive");
}
