use crate::popover::{
    DEFAULT_OPEN, PopoverOpenMode, PopoverOpenStateInput, PopoverPartStateInput, PopoverSlot,
    compose_class_name, modal_attr, normalize_optional_text, resolve_open_state, resolve_state,
    state_attr_for_open,
};

#[test]
fn state_and_modal_attrs_follow_contract() {
    assert_eq!(state_attr_for_open(true), "open");
    assert_eq!(state_attr_for_open(false), "closed");
    assert_eq!(modal_attr(true), "modal");
    assert_eq!(modal_attr(false), "non-modal");
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-popover  ".to_string())),
        Some("docs-popover".to_string())
    );
}

#[test]
fn resolve_open_state_supports_controlled_and_uncontrolled_modes() {
    let controlled = resolve_open_state(PopoverOpenStateInput {
        has_is_open_prop: true,
        has_open_prop: true,
        default_open: Some(true),
        has_on_open_change: true,
        has_on_close: true,
    });

    assert_eq!(controlled.mode, PopoverOpenMode::Controlled);
    assert_eq!(controlled.open_mode_attr, "controlled");
    assert_eq!(controlled.open_source_attr, "external");
    assert_eq!(controlled.open_prop_source_attr, "is_open");
    assert!(controlled.default_open);
    assert!(controlled.has_default_open);
    assert!(controlled.has_open_change_handler);
    assert!(controlled.has_on_close_handler);

    let uncontrolled = resolve_open_state(PopoverOpenStateInput {
        has_is_open_prop: false,
        has_open_prop: false,
        default_open: None,
        has_on_open_change: false,
        has_on_close: false,
    });

    assert_eq!(uncontrolled.mode, PopoverOpenMode::Uncontrolled);
    assert_eq!(uncontrolled.open_mode_attr, "uncontrolled");
    assert_eq!(uncontrolled.open_source_attr, "implicit-default");
    assert_eq!(uncontrolled.open_prop_source_attr, "none");
    assert_eq!(uncontrolled.default_open, DEFAULT_OPEN);
    assert!(!uncontrolled.has_default_open);
    assert!(!uncontrolled.has_open_change_handler);
    assert!(!uncontrolled.has_on_close_handler);
}

#[test]
fn resolve_open_state_tracks_open_change_source_priority() {
    let on_open_change = resolve_open_state(PopoverOpenStateInput {
        has_is_open_prop: false,
        has_open_prop: false,
        default_open: None,
        has_on_open_change: true,
        has_on_close: true,
    });
    assert_eq!(on_open_change.open_change_source_attr, "on_open_change");

    let on_close = resolve_open_state(PopoverOpenStateInput {
        has_is_open_prop: false,
        has_open_prop: false,
        default_open: None,
        has_on_open_change: false,
        has_on_close: true,
    });
    assert_eq!(on_close.open_change_source_attr, "on_close");

    let none = resolve_open_state(PopoverOpenStateInput {
        has_is_open_prop: false,
        has_open_prop: false,
        default_open: None,
        has_on_open_change: false,
        has_on_close: false,
    });
    assert_eq!(none.open_change_source_attr, "none");
}

#[test]
fn resolve_state_tracks_source_markers() {
    let state = resolve_state(PopoverPartStateInput {
        slot: PopoverSlot::Root,
        open: true,
        is_modal: false,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_custom_placement: true,
        has_on_exit_complete: true,
    });

    assert_eq!(state.slot_attr, "popover");
    assert_eq!(state.base_class, "ui-popover");
    assert_eq!(state.state_attr, "open");
    assert_eq!(state.modal_attr, "non-modal");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.placement_source_attr, "custom");
    assert_eq!(state.modal_source_attr, "custom");
    assert_eq!(state.exit_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_custom_markers() {
    let class_name = compose_class_name(
        Some("docs-popover-state".to_string()),
        resolve_state(PopoverPartStateInput {
            slot: PopoverSlot::Root,
            open: false,
            is_modal: false,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_placement: true,
            has_on_exit_complete: true,
        }),
    );

    for token in [
        "ui-popover",
        "ui-popover--custom-motion",
        "ui-popover--custom-placement",
        "ui-popover--non-modal",
        "ui-popover--custom-modal",
        "ui-popover--custom-exit",
        "ui-popover--custom-class",
        "docs-popover-state",
    ] {
        assert!(
            class_name.contains(token),
            "popover class name should include `{token}`"
        );
    }
}
