use super::*;
use crate::dialog::DialogSlot;

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
fn resolve_state_tracks_size_description_and_sources() {
    let state = resolve_state(DialogPartStateInput {
        slot: DialogSlot::Root,
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
fn normalize_open_state_supports_controlled_and_uncontrolled_modes() {
    let controlled = normalize_open_state(DialogOpenStateInput {
        is_open: Some(Signal::derive(|| true)),
        open: Some(Signal::derive(|| false)),
        default_open: Some(false),
        on_open_change: None,
    });

    assert!(matches!(controlled.mode, DialogOpenMode::Controlled));
    assert!(controlled.open.is_some());
    assert!(!controlled.default_open);
    assert!(controlled.has_default_open);
    assert_eq!(controlled.open_prop_source_attr, "is_open");
    assert_eq!(controlled.open_mode_attr, "controlled");
    assert_eq!(controlled.open_source_attr, "controlled");
    assert_eq!(controlled.open_change_source_attr, "none");

    let uncontrolled = normalize_open_state(DialogOpenStateInput {
        is_open: None,
        open: None,
        default_open: Some(true),
        on_open_change: Some(Callback::new(|_: bool| {})),
    });

    assert!(matches!(uncontrolled.mode, DialogOpenMode::Uncontrolled));
    assert!(uncontrolled.open.is_none());
    assert!(uncontrolled.default_open);
    assert!(uncontrolled.has_default_open);
    assert!(uncontrolled.has_open_change_handler);
    assert_eq!(uncontrolled.open_prop_source_attr, "none");
    assert_eq!(uncontrolled.open_mode_attr, "uncontrolled");
    assert_eq!(uncontrolled.open_source_attr, "default");
    assert_eq!(uncontrolled.open_change_source_attr, "custom");
}

#[test]
fn normalize_open_state_uses_implicit_default_when_missing() {
    let normalized = normalize_open_state(DialogOpenStateInput {
        is_open: None,
        open: None,
        default_open: None,
        on_open_change: None,
    });

    assert!(matches!(normalized.mode, DialogOpenMode::Uncontrolled));
    assert!(!normalized.default_open);
    assert!(!normalized.has_default_open);
    assert!(!normalized.has_open_change_handler);
    assert_eq!(normalized.open_prop_source_attr, "none");
    assert_eq!(normalized.open_mode_attr, "uncontrolled");
    assert_eq!(normalized.open_source_attr, "implicit-default");
    assert_eq!(normalized.open_change_source_attr, "none");
}

#[test]
fn normalize_close_config_owns_close_defaults_and_priority() {
    let normalized = normalize_close_config(DialogCloseConfigInput {
        is_close_button_visible: true,
        show_close_button: Some(false),
        close_label: "   ",
    });

    assert_eq!(
        normalized.close_button_visibility,
        DialogCloseButtonVisibility::Hidden
    );
    assert!(!normalized.show_close_button());
    assert_eq!(normalized.close_label, DEFAULT_CLOSE_LABEL);
    assert!(!normalized.has_custom_close_label);
    assert_eq!(
        normalized.close_button_prop_source_attr(),
        "show_close_button"
    );

    let defaulted = normalize_close_config(DialogCloseConfigInput {
        is_close_button_visible: true,
        show_close_button: None,
        close_label: "Dismiss dialog",
    });

    assert_eq!(
        defaulted.close_button_visibility,
        DialogCloseButtonVisibility::Visible
    );
    assert!(defaulted.show_close_button());
    assert_eq!(defaulted.close_label, "Dismiss dialog");
    assert!(defaulted.has_custom_close_label);
    assert_eq!(
        defaulted.close_button_prop_source_attr(),
        "is_close_button_visible"
    );
}

#[test]
fn normalize_exit_config_owns_noop_default() {
    let custom_callback = Callback::new(|_| {});
    let custom = normalize_exit_config(Some(custom_callback));
    assert!(custom.has_custom_on_exit_complete);
    custom.on_exit_complete.run(());

    let defaulted = normalize_exit_config(None);
    assert!(!defaulted.has_custom_on_exit_complete);
    defaulted.on_exit_complete.run(());
}

#[test]
fn can_request_close_follows_mode_and_open_change_handler() {
    assert!(can_request_close(DialogOpenMode::Uncontrolled, false));
    assert!(can_request_close(DialogOpenMode::Controlled, true));
    assert!(!can_request_close(DialogOpenMode::Controlled, false));
}

#[test]
fn resolve_part_states_concentrates_slot_state_derivation() {
    let states = resolve_part_states(DialogPartStatesInput {
        size: DialogSize::Lg,
        has_description: true,
        has_footer: true,
        close_button_visibility: DialogCloseButtonVisibility::Hidden,
        has_custom_id_base: true,
        has_custom_title: true,
        has_custom_description: true,
        has_custom_close_label: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_on_exit_complete: true,
    });

    assert_eq!(states.root.slot, DialogSlot::Root);
    assert_eq!(states.header.slot, DialogSlot::Header);
    assert_eq!(states.title.slot, DialogSlot::Title);
    assert_eq!(states.description.slot, DialogSlot::Description);
    assert_eq!(states.body.slot, DialogSlot::Body);
    assert_eq!(states.footer.slot, DialogSlot::Footer);
    assert_eq!(states.close.slot, DialogSlot::Close);
    assert!(states.root.has_custom_class_name);
    assert!(!states.header.has_custom_class_name);
    assert!(states.root.has_custom_motion);
    assert!(!states.header.has_custom_motion);
}

#[test]
fn compose_class_name_adds_state_and_custom_markers() {
    let class_name = compose_class_name(
        Some("docs-dialog-custom".to_string()),
        resolve_state(DialogPartStateInput {
            slot: DialogSlot::Root,
            size: DialogSize::Lg,
            has_description: true,
            has_footer: true,
            show_close_button: true,
            has_custom_id_base: true,
            has_custom_title: true,
            has_custom_description: true,
            has_custom_close_label: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_on_exit_complete: true,
        }),
    );

    for token in [
        "ui-dialog",
        "ui-dialog--size-lg",
        "ui-dialog--with-description",
        "ui-dialog--with-footer",
        "ui-dialog--close-shown",
        "ui-dialog--custom-size",
        "ui-dialog--custom-id",
        "ui-dialog--custom-title",
        "ui-dialog--custom-description",
        "ui-dialog--custom-close",
        "ui-dialog--custom-motion",
        "ui-dialog--custom-exit",
        "ui-dialog--custom-class",
        "docs-dialog-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}
