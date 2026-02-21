use super::*;

#[test]
fn variant_and_autofocus_attrs_follow_contract() {
    assert_eq!(
        AlertDialogVariant::Default.class_name(),
        "ui-alert-dialog--variant-default"
    );
    assert_eq!(
        AlertDialogVariant::Destructive.class_name(),
        "ui-alert-dialog--variant-destructive"
    );
    assert_eq!(AlertDialogVariant::Warning.data_attr(), "warning");
    assert_eq!(AlertDialogVariant::Error.data_attr(), "error");

    assert_eq!(AlertDialogAutoFocusButton::None.as_attr(), "none");
    assert_eq!(AlertDialogAutoFocusButton::Cancel.as_attr(), "cancel");
    assert_eq!(AlertDialogAutoFocusButton::Secondary.as_attr(), "secondary");
    assert_eq!(AlertDialogAutoFocusButton::Confirm.as_attr(), "confirm");
}

#[test]
fn normalize_helpers_trim_and_default_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  alert-note  ".to_string())),
        Some("alert-note".to_string())
    );

    assert_eq!(
        normalize_required_text("  Delete item  ".to_string(), DEFAULT_TITLE),
        "Delete item"
    );
    assert_eq!(
        normalize_required_text("\n\t".to_string(), DEFAULT_TITLE),
        DEFAULT_TITLE
    );

    assert_eq!(
        normalize_id_base("  docs-alert  ".to_string()),
        "docs-alert"
    );
    assert_eq!(normalize_id_base("\n\t".to_string()), DEFAULT_ID_BASE);

    assert_eq!(normalize_cancel_label(None), DEFAULT_CANCEL_LABEL);
    assert_eq!(
        normalize_cancel_label(Some("  Cancel now  ".to_string())),
        "Cancel now"
    );
    assert_eq!(
        normalize_secondary_label(Some("  Save draft  ".to_string())),
        Some("Save draft".to_string())
    );
}

#[test]
fn resolve_disabled_flag_prefers_is_prefix_and_keeps_legacy_alias() {
    assert!(resolve_disabled_flag(Some(true), Some(false), false));
    assert!(!resolve_disabled_flag(None, Some(false), true));
    assert!(resolve_disabled_flag(None, None, true));
}

#[test]
fn resolve_state_core_tracks_sources_and_variant_contracts() {
    let state = resolve_state_core(AlertDialogStateCoreInput {
        is_open: true,
        variant: AlertDialogVariant::Error,
        auto_focus_button: AlertDialogAutoFocusButton::Secondary,
        show_description: true,
        show_cancel: true,
        show_secondary: true,
        confirm_disabled: true,
        secondary_disabled: true,
        has_custom_id_base: true,
        has_custom_title: true,
        has_custom_description: true,
        has_custom_confirm_label: true,
        has_custom_cancel_label: true,
        has_custom_secondary_label: true,
        has_custom_on_cancel: true,
        has_custom_on_secondary: true,
        has_custom_auto_focus_button: true,
        has_custom_motion: true,
        has_on_exit_complete: true,
    });

    assert_eq!(state.state_attr, "open");
    assert_eq!(state.variant_attr, "error");
    assert_eq!(state.description_attr, "present");
    assert_eq!(state.cancel_attr, "shown");
    assert_eq!(state.secondary_attr, "shown");
    assert_eq!(state.confirm_disabled_attr, "true");
    assert_eq!(state.secondary_disabled_attr, "true");
    assert_eq!(state.auto_focus_attr, "secondary");
    assert_eq!(state.variant_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.exit_source_attr, "custom");
}
