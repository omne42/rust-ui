use super::*;
use leptos::prelude::Callable;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

#[test]
fn state_and_dismiss_attrs_follow_contract() {
    assert_eq!(state_attr_for_open(true), "open");
    assert_eq!(state_attr_for_open(false), "closed");
    assert_eq!(dismiss_attr(true), "dismissable");
    assert_eq!(dismiss_attr(false), "locked");
    assert_eq!(keyboard_dismiss_attr(false), "enabled");
    assert_eq!(keyboard_dismiss_attr(true), "disabled");
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-overlay  ".to_string())),
        Some("docs-overlay".to_string())
    );
}

#[test]
fn normalize_on_exit_complete_uses_noop_default_and_preserves_custom_handler() {
    normalize_on_exit_complete(None).run(());

    let called = Arc::new(AtomicBool::new(false));
    let called_for_callback = Arc::clone(&called);
    normalize_on_exit_complete(Some(leptos::prelude::Callback::new(move |_| {
        called_for_callback.store(true, Ordering::SeqCst);
    })))
    .run(());

    assert!(called.load(Ordering::SeqCst));
}

#[test]
fn resolve_states_centralizes_slot_state_derivation() {
    let resolved = resolve_states(OverlayStateInputs {
        open: true,
        dismiss_mode: OverlayDismissMode::Locked,
        keyboard_dismiss_mode: OverlayKeyboardDismissMode::Disabled,
        has_custom_role: true,
        has_custom_aria_labelledby: true,
        has_custom_aria_describedby: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_on_exit_complete: true,
    });

    assert_eq!(resolved.root_state.slot_attr, "overlay");
    assert_eq!(resolved.root_state.state_attr, "open");
    assert_eq!(resolved.backdrop_state.slot_attr, "overlay-backdrop");
    assert_eq!(resolved.backdrop_state.state_attr, "backdrop");
    assert_eq!(resolved.panel_state.slot_attr, "overlay-panel");
    assert_eq!(resolved.panel_state.state_attr, "panel");
}

#[test]
fn overlay_mode_enums_map_bool_inputs_to_closed_set() {
    assert_eq!(
        OverlayDismissMode::from_is_dismissable(true),
        OverlayDismissMode::Dismissable
    );
    assert_eq!(
        OverlayDismissMode::from_is_dismissable(false),
        OverlayDismissMode::Locked
    );
    assert_eq!(
        OverlayKeyboardDismissMode::from_is_disabled(true),
        OverlayKeyboardDismissMode::Disabled
    );
    assert_eq!(
        OverlayKeyboardDismissMode::from_is_disabled(false),
        OverlayKeyboardDismissMode::Enabled
    );
}

#[test]
fn resolve_state_tracks_source_markers() {
    let state = resolve_state(OverlayPartStateInput {
        slot: OverlaySlot::Root,
        open: true,
        is_dismissable: false,
        is_keyboard_dismiss_disabled: true,
        has_custom_role: true,
        has_custom_aria_labelledby: true,
        has_custom_aria_describedby: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_on_exit_complete: true,
    });

    assert_eq!(state.slot_attr, "overlay");
    assert_eq!(state.base_class, "ui-overlay");
    assert_eq!(state.state_attr, "open");
    assert_eq!(state.dismiss_attr, "locked");
    assert_eq!(state.keyboard_dismiss_attr, "disabled");
    assert_eq!(state.role_source_attr, "custom");
    assert_eq!(state.aria_labelledby_source_attr, "custom");
    assert_eq!(state.aria_describedby_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.dismiss_source_attr, "custom");
    assert_eq!(state.keyboard_dismiss_source_attr, "custom");
    assert_eq!(state.exit_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_custom_markers() {
    let class_name = compose_class_name(
        Some("docs-overlay-state".to_string()),
        resolve_state(OverlayPartStateInput {
            slot: OverlaySlot::Root,
            open: false,
            is_dismissable: false,
            is_keyboard_dismiss_disabled: true,
            has_custom_role: true,
            has_custom_aria_labelledby: true,
            has_custom_aria_describedby: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_on_exit_complete: true,
        }),
    );

    for token in [
        "ui-overlay",
        "ui-overlay--custom-motion",
        "ui-overlay--custom-role",
        "ui-overlay--custom-aria-labelledby",
        "ui-overlay--custom-aria-describedby",
        "ui-overlay--custom-dismiss",
        "ui-overlay--custom-keyboard-dismiss",
        "ui-overlay--custom-exit",
        "ui-overlay--custom-class",
        "docs-overlay-state",
    ] {
        assert!(
            class_name.contains(token),
            "overlay class name should include `{token}`"
        );
    }
}

#[test]
fn should_close_on_escape_requires_topmost_and_enabled_keyboard_dismiss() {
    assert!(should_close_on_escape("Escape", true, false, false, false));
    assert!(!should_close_on_escape("Enter", true, false, false, false));
    assert!(!should_close_on_escape(
        "Escape", false, false, false, false
    ));
    assert!(!should_close_on_escape("Escape", true, true, false, false));
    assert!(!should_close_on_escape("Escape", true, false, true, false));
    assert!(!should_close_on_escape("Escape", true, false, false, true));
}
