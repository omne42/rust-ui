use super::*;
use leptos::prelude::Callable;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

#[test]
fn state_dismiss_and_keyboard_attrs_follow_contract() {
    assert_eq!(state_attr_for_open(true), "open");
    assert_eq!(state_attr_for_open(false), "closed");
    assert_eq!(dismiss_attr(true), "dismissable");
    assert_eq!(dismiss_attr(false), "locked");
    assert_eq!(keyboard_dismiss_attr(false), "enabled");
    assert_eq!(keyboard_dismiss_attr(true), "disabled");
}

#[test]
fn agent_contract_is_schema_typed_and_snapshot_based() {
    let contract = agent_contract();

    assert_eq!(contract.schema_attr, "sheet.v1");
    assert_eq!(contract.intent_attr, "overlay");
    assert_eq!(contract.action_attr, "dismiss");
    assert_eq!(contract.render_mode_attr, "snapshot");
    assert_eq!(contract.streaming_attr, "optional");
    assert_eq!(contract.fallback_attr, "snapshot");
    assert_eq!(contract.output_status_attr, "verified");
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-sheet ".to_string())),
        Some("docs-sheet".to_string())
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
    let resolved = resolve_states(SheetStateInputs {
        open: true,
        placement: SheetPlacement::Right,
        dismiss_mode: SheetDismissMode::Locked,
        keyboard_dismiss_mode: SheetKeyboardDismissMode::Disabled,
        has_custom_motion: true,
        has_custom_aria_labelledby: true,
        has_custom_aria_describedby: true,
        has_on_exit_complete: true,
    });

    assert_eq!(resolved.root_state.slot_attr, "sheet");
    assert_eq!(resolved.root_state.state_attr, "open");
    assert_eq!(resolved.backdrop_state.slot_attr, "sheet-backdrop");
    assert_eq!(resolved.backdrop_state.state_attr, "backdrop");
    assert_eq!(resolved.panel_state.slot_attr, "sheet-panel");
    assert_eq!(resolved.panel_state.state_attr, "panel");
}

#[test]
fn sheet_mode_enums_map_bool_inputs_to_closed_set() {
    assert_eq!(
        SheetDismissMode::from_is_dismissable(true),
        SheetDismissMode::Dismissable
    );
    assert_eq!(
        SheetDismissMode::from_is_dismissable(false),
        SheetDismissMode::Locked
    );
    assert_eq!(
        SheetKeyboardDismissMode::from_is_disabled(true),
        SheetKeyboardDismissMode::Disabled
    );
    assert_eq!(
        SheetKeyboardDismissMode::from_is_disabled(false),
        SheetKeyboardDismissMode::Enabled
    );
}

#[test]
fn resolve_state_tracks_source_markers() {
    let state = resolve_state(SheetPartStateInput {
        slot: SheetSlot::Root,
        open: true,
        placement: SheetPlacement::Right,
        is_dismissable: false,
        is_keyboard_dismiss_disabled: true,
        has_custom_motion: true,
        has_custom_aria_labelledby: true,
        has_custom_aria_describedby: true,
        has_on_exit_complete: true,
    });

    assert_eq!(state.slot_attr, "sheet");
    assert_eq!(state.base_class, "ui-sheet");
    assert_eq!(state.state_attr, "open");
    assert_eq!(state.placement_attr, "right");
    assert_eq!(state.dismiss_attr, "locked");
    assert_eq!(state.keyboard_dismiss_attr, "disabled");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.placement_source_attr, "custom");
    assert_eq!(state.dismiss_source_attr, "custom");
    assert_eq!(state.keyboard_dismiss_source_attr, "custom");
    assert_eq!(state.aria_labelledby_source_attr, "custom");
    assert_eq!(state.aria_describedby_source_attr, "custom");
    assert_eq!(state.exit_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_custom_markers() {
    let class_name = compose_class_name(resolve_state(SheetPartStateInput {
        slot: SheetSlot::Root,
        open: false,
        placement: SheetPlacement::Right,
        is_dismissable: false,
        is_keyboard_dismiss_disabled: true,
        has_custom_motion: true,
        has_custom_aria_labelledby: true,
        has_custom_aria_describedby: true,
        has_on_exit_complete: true,
    }));

    for token in [
        "ui-sheet",
        "ui-sheet--placement-right",
        "ui-sheet--custom-motion",
        "ui-sheet--custom-placement",
        "ui-sheet--custom-dismiss",
        "ui-sheet--custom-keyboard-dismiss",
        "ui-sheet--custom-aria-labelledby",
        "ui-sheet--custom-aria-describedby",
        "ui-sheet--custom-exit",
    ] {
        assert!(
            class_name.contains(token),
            "sheet class name should include `{token}`"
        );
    }
}

#[test]
fn should_close_on_escape_requires_topmost_non_composing_non_prevented_escape() {
    assert!(should_close_on_escape("Escape", true, false, false, false));
    assert!(!should_close_on_escape("Enter", true, false, false, false));
    assert!(!should_close_on_escape(
        "Escape", false, false, false, false
    ));
    assert!(!should_close_on_escape("Escape", true, true, false, false));
    assert!(!should_close_on_escape("Escape", true, false, true, false));
    assert!(!should_close_on_escape("Escape", true, false, false, true));
}
