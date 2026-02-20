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
