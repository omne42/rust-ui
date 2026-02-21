use super::*;
use crate::command_dialog::CommandDialogSlot;

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-dialog ".to_string())),
        Some("docs-dialog".to_string())
    );

    assert_eq!(normalize_id_base(None), DEFAULT_ID_BASE);
    assert_eq!(
        normalize_id_base(Some(" docs-command-dialog ".to_string())),
        "docs-command-dialog"
    );

    assert_eq!(normalize_title(None), DEFAULT_TITLE);
    assert_eq!(
        normalize_title(Some(" Quick Actions ".to_string())),
        "Quick Actions"
    );
    assert_eq!(
        resolve_text_with_empty_default(Some(" quick actions ")),
        " quick actions ".to_string()
    );
    assert_eq!(resolve_text_with_empty_default(None), String::new());

    assert!(!normalize_is_disabled(None, false));
    assert!(normalize_is_disabled(None, true));
    assert!(normalize_is_disabled(Some(true), false));
    assert!(!normalize_is_disabled(Some(false), true));
}

#[test]
fn normalize_props_centralizes_state_inputs_and_sources() {
    let normalized = normalize_props(CommandDialogNormalizationInput {
        open_input: Some(true),
        default_open: Some(false),
        has_open_prop: true,
        has_on_action: true,
        has_on_open_change: true,
        close_on_action: false,
        id_base: Some(" docs-command-dialog ".to_string()),
        title: Some(" Quick Actions ".to_string()),
        description: Some(" Search quickly ".to_string()),
        is_disabled: Some(true),
        disabled: false,
        placeholder: Some(" Search command ".to_string()),
        empty_label: Some(" Empty state ".to_string()),
        aria_label: Some(" Open command dialog ".to_string()),
        class_name: Some(" docs-class ".to_string()),
        has_custom_command_motion: true,
        has_custom_overlay_motion: true,
    });

    assert_eq!(normalized.id_base, "docs-command-dialog");
    assert_eq!(normalized.title, "Quick Actions");
    assert_eq!(normalized.description_text, "Search quickly".to_string());
    assert_eq!(normalized.placeholder_text, "Search command".to_string());
    assert_eq!(normalized.empty_label_text, "Empty state".to_string());
    assert_eq!(
        normalized.aria_label_text,
        "Open command dialog".to_string()
    );
    assert_eq!(normalized.class_name, Some("docs-class".to_string()));
    assert_eq!(normalized.open_input, Some(true));
    assert_eq!(normalized.default_open, Some(false));
    assert!(normalized.is_controlled);
    assert!(normalized.has_description);
    assert!(!normalized.close_on_action);
    assert!(normalized.disabled);
    assert!(normalized.has_custom_id_base);
    assert!(normalized.has_custom_title);
    assert!(normalized.has_custom_description);
    assert!(normalized.has_custom_placeholder);
    assert!(normalized.has_custom_empty_label);
    assert!(normalized.has_custom_aria_label);
    assert!(normalized.has_custom_class_name);
    assert!(normalized.has_custom_on_action);
    assert!(normalized.has_custom_on_open_change);
    assert!(normalized.has_custom_default_open);
    assert!(normalized.has_custom_close_on_action);
    assert!(normalized.has_custom_disabled);
    assert!(normalized.has_custom_command_motion);
    assert!(normalized.has_custom_overlay_motion);
}

#[test]
fn resolve_part_state_uses_normalized_contract_and_slot_rules() {
    let normalized = normalize_props(CommandDialogNormalizationInput {
        open_input: None,
        default_open: None,
        has_open_prop: false,
        has_on_action: false,
        has_on_open_change: false,
        close_on_action: true,
        id_base: Some(" docs-command-dialog ".to_string()),
        title: Some(" Quick Actions ".to_string()),
        description: Some(" Search quickly ".to_string()),
        is_disabled: None,
        disabled: false,
        placeholder: Some(" Search command ".to_string()),
        empty_label: Some(" Empty state ".to_string()),
        aria_label: Some(" Open command dialog ".to_string()),
        class_name: Some(" docs-class ".to_string()),
        has_custom_command_motion: false,
        has_custom_overlay_motion: false,
    });

    let root_state = resolve_part_state(&normalized, CommandDialogSlot::Root, true);
    assert!(root_state.has_custom_class_name);
    assert_eq!(root_state.state_attr, "open");

    let modal_state = resolve_part_state(&normalized, CommandDialogSlot::Modal, false);
    assert!(
        !modal_state.has_custom_class_name,
        "non-root slots should not inherit root-only custom class marker"
    );
    assert_eq!(modal_state.state_attr, "closed");
}

#[test]
fn resolve_state_tracks_flags_and_sources() {
    let state = resolve_state(CommandDialogPartStateInput {
        slot: CommandDialogSlot::Root,
        is_open: true,
        has_description: true,
        close_on_action: false,
        disabled: true,
        is_controlled: true,
        has_custom_id_base: true,
        has_custom_title: true,
        has_custom_description: true,
        has_custom_placeholder: true,
        has_custom_empty_label: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        has_custom_on_action: true,
        has_custom_on_open_change: true,
        has_custom_default_open: true,
        has_custom_close_on_action: true,
        has_custom_disabled: true,
        has_custom_command_motion: true,
        has_custom_overlay_motion: true,
    });

    assert_eq!(state.state_attr, "open");
    assert_eq!(state.description_attr, "present");
    assert_eq!(state.close_on_action_attr, "false");
    assert_eq!(state.disabled_attr, "true");
    assert_eq!(state.open_mode_attr, "controlled");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.action_source_attr, "custom");
    assert_eq!(state.command_motion_source_attr, "custom");
    assert_eq!(state.overlay_motion_source_attr, "custom");
}

#[test]
fn compose_class_name_contains_state_markers() {
    let class_name = compose_class_name(
        Some("docs-command-dialog".to_string()),
        resolve_state(CommandDialogPartStateInput {
            slot: CommandDialogSlot::Root,
            is_open: false,
            has_description: true,
            close_on_action: true,
            disabled: false,
            is_controlled: false,
            has_custom_id_base: true,
            has_custom_title: true,
            has_custom_description: true,
            has_custom_placeholder: true,
            has_custom_empty_label: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            has_custom_on_action: true,
            has_custom_on_open_change: true,
            has_custom_default_open: true,
            has_custom_close_on_action: true,
            has_custom_disabled: true,
            has_custom_command_motion: true,
            has_custom_overlay_motion: true,
        }),
    );

    for token in [
        "ui-command-dialog",
        "ui-command-dialog--closed",
        "ui-command-dialog--with-description",
        "ui-command-dialog--close-on-action",
        "ui-command-dialog--uncontrolled",
        "ui-command-dialog--custom-id",
        "ui-command-dialog--custom-title",
        "ui-command-dialog--custom-description",
        "ui-command-dialog--custom-placeholder",
        "ui-command-dialog--custom-empty-label",
        "ui-command-dialog--custom-aria-label",
        "ui-command-dialog--custom-action",
        "ui-command-dialog--custom-open-change",
        "ui-command-dialog--custom-default-open",
        "ui-command-dialog--custom-close-on-action",
        "ui-command-dialog--custom-disabled",
        "ui-command-dialog--custom-command-motion",
        "ui-command-dialog--custom-overlay-motion",
        "ui-command-dialog--custom-class",
        "docs-command-dialog",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}

#[test]
fn normalize_open_state_options_wires_controlled_and_default_inputs() {
    let options = normalize_open_state_options(Some(true), Some(false));
    assert_eq!(options.is_open, Some(true));
    assert_eq!(options.default_open, Some(false));
    assert!(options.on_open_change.is_none());
}

#[test]
fn apply_open_change_uses_overlay_trigger_primitive_rules() {
    let mut uncontrolled =
        use_overlay_trigger_state(normalize_open_state_options(None, Some(false)));
    apply_open_change(&mut uncontrolled, None, true);
    assert!(uncontrolled.is_open());

    let mut controlled = use_overlay_trigger_state(normalize_open_state_options(Some(false), None));
    apply_open_change(&mut controlled, Some(false), true);
    assert!(
        !controlled.is_open(),
        "controlled state should stay in sync with external input until next sync"
    );
}

#[test]
fn open_state_control_mode_switch_stays_explicit_and_stable() {
    let mut state = use_overlay_trigger_state(normalize_open_state_options(None, Some(false)));
    apply_open_change(&mut state, None, true);
    assert!(
        state.is_open(),
        "uncontrolled mode should allow internal state updates"
    );

    state.sync_controlled(Some(false));
    assert!(
        !state.is_open(),
        "switching to controlled mode should sync to external source of truth"
    );

    apply_open_change(&mut state, Some(false), true);
    assert!(
        !state.is_open(),
        "controlled mode should ignore internal writes until controlled value changes"
    );

    state.sync_controlled(Some(true));
    assert!(
        state.is_open(),
        "controlled updates should be reflected after the external value changes"
    );
}
