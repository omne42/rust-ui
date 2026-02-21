use super::*;

#[test]
fn normalize_helpers_trim_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-dialog  ".to_string())),
        Some("docs-dialog".to_string())
    );

    assert_eq!(
        normalize_required_text("  Confirm  ".to_string(), DEFAULT_TITLE),
        "Confirm"
    );
    assert_eq!(
        normalize_required_text("\n\t".to_string(), DEFAULT_TITLE),
        DEFAULT_TITLE
    );

    assert_eq!(
        normalize_id_base("  custom-dialog  ".to_string()),
        "custom-dialog"
    );
    assert_eq!(normalize_id_base("\n\t".to_string()), DEFAULT_ID_BASE);
}

#[test]
fn resolve_state_core_tracks_size_description_and_sources() {
    let state = resolve_state_core(DialogStateCoreInput {
        size: DialogSize::Lg,
        has_description: true,
        has_footer: true,
        show_close_button: false,
        has_custom_id_base: true,
        has_custom_title: true,
        has_custom_description: true,
        has_custom_close_label: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_on_exit_complete: true,
    });

    assert_eq!(state.size_attr, "lg");
    assert_eq!(state.state_attr, "with-description");
    assert_eq!(state.description_attr, "present");
    assert_eq!(state.footer_attr, "present");
    assert_eq!(state.close_button_attr, "hidden");
    assert_eq!(state.size_source_attr, "custom");
    assert_eq!(state.id_source_attr, "custom");
    assert_eq!(state.title_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.exit_source_attr, "custom");
}

#[test]
fn default_size_and_close_button_sources_stay_default_when_not_customized() {
    let state = resolve_state_core(DialogStateCoreInput {
        size: DEFAULT_SIZE,
        has_description: false,
        has_footer: false,
        show_close_button: DEFAULT_SHOW_CLOSE_BUTTON,
        has_custom_id_base: false,
        has_custom_title: false,
        has_custom_description: false,
        has_custom_close_label: false,
        has_custom_class_name: false,
        has_custom_motion: false,
        has_on_exit_complete: false,
    });

    assert_eq!(state.size_source_attr, "default");
    assert_eq!(state.close_source_attr, "default");
    assert_eq!(state.description_source_attr, "default");
    assert_eq!(state.motion_source_attr, "default");
    assert_eq!(state.exit_source_attr, "default");
}

#[test]
fn resolve_open_state_contract_tracks_sources_and_modes() {
    let controlled = resolve_open_state_contract(DialogOpenStateContractInput {
        has_is_open_prop: true,
        has_open_prop: true,
        has_default_open: true,
        has_open_change_handler: false,
    });
    assert_eq!(controlled.mode, DialogOpenMode::Controlled);
    assert_eq!(controlled.open_prop_source_attr, "is_open");
    assert_eq!(controlled.open_mode_attr, "controlled");
    assert_eq!(controlled.open_source_attr, "controlled");
    assert_eq!(controlled.open_change_source_attr, "none");

    let uncontrolled = resolve_open_state_contract(DialogOpenStateContractInput {
        has_is_open_prop: false,
        has_open_prop: false,
        has_default_open: true,
        has_open_change_handler: true,
    });
    assert_eq!(uncontrolled.mode, DialogOpenMode::Uncontrolled);
    assert_eq!(uncontrolled.open_prop_source_attr, "none");
    assert_eq!(uncontrolled.open_mode_attr, "uncontrolled");
    assert_eq!(uncontrolled.open_source_attr, "default");
    assert_eq!(uncontrolled.open_change_source_attr, "custom");
}

#[test]
fn can_request_close_follows_mode_and_handler() {
    assert!(can_request_close(DialogOpenMode::Uncontrolled, false));
    assert!(can_request_close(DialogOpenMode::Controlled, true));
    assert!(!can_request_close(DialogOpenMode::Controlled, false));
}

#[test]
fn resolve_close_button_contract_prefers_alias_when_present() {
    let alias = resolve_close_button_contract(DialogCloseButtonContractInput {
        is_close_button_visible: true,
        show_close_button: Some(false),
    });
    assert_eq!(alias.visibility, DialogCloseButtonVisibility::Hidden);
    assert_eq!(
        alias.prop_source,
        DialogCloseButtonPropSource::ShowCloseButton
    );

    let canonical = resolve_close_button_contract(DialogCloseButtonContractInput {
        is_close_button_visible: true,
        show_close_button: None,
    });
    assert_eq!(canonical.visibility, DialogCloseButtonVisibility::Visible);
    assert_eq!(
        canonical.prop_source,
        DialogCloseButtonPropSource::IsCloseButtonVisible
    );
}
