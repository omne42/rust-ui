use super::*;

#[test]
fn slot_attrs_and_classes_follow_contract() {
    assert_eq!(TraySlot::Root.as_attr(), "tray");
    assert_eq!(TraySlot::Header.as_attr(), "tray-header");
    assert_eq!(TraySlot::Title.as_attr(), "tray-title");
    assert_eq!(TraySlot::Description.as_attr(), "tray-description");
    assert_eq!(TraySlot::Body.as_attr(), "tray-body");
    assert_eq!(TraySlot::Footer.as_attr(), "tray-footer");
    assert_eq!(TraySlot::Close.as_attr(), "tray-close");

    assert_eq!(TraySlot::Root.base_class(), "ui-tray");
    assert_eq!(TraySlot::Header.base_class(), "ui-tray__header");
    assert_eq!(TraySlot::Title.base_class(), "ui-tray__title");
    assert_eq!(TraySlot::Description.base_class(), "ui-tray__description");
    assert_eq!(TraySlot::Body.base_class(), "ui-tray__body");
    assert_eq!(TraySlot::Footer.base_class(), "ui-tray__footer");
    assert_eq!(TraySlot::Close.base_class(), "ui-tray__close");
}

#[test]
fn state_attrs_follow_contract() {
    assert_eq!(state_attr(true), "with-description");
    assert_eq!(state_attr(false), "title-only");
    assert_eq!(description_attr(true), "present");
    assert_eq!(description_attr(false), "absent");
    assert_eq!(footer_attr(true), "present");
    assert_eq!(footer_attr(false), "absent");
    assert_eq!(close_button_attr(true), "shown");
    assert_eq!(close_button_attr(false), "hidden");
    assert_eq!(size_attr(true), "fixed");
    assert_eq!(size_attr(false), "auto");
    assert_eq!(dismiss_attr(true), "dismissable");
    assert_eq!(dismiss_attr(false), "locked");
    assert_eq!(keyboard_dismiss_attr(false), "enabled");
    assert_eq!(keyboard_dismiss_attr(true), "disabled");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-tray ".to_string())),
        Some("docs-tray".to_string())
    );

    assert_eq!(
        normalize_required_text(" Tray ".to_string(), DEFAULT_TITLE),
        "Tray"
    );
    assert_eq!(
        normalize_required_text(" ".to_string(), DEFAULT_TITLE),
        DEFAULT_TITLE
    );

    assert_eq!(normalize_id_base(" docs-tray ".to_string()), "docs-tray");
    assert_eq!(normalize_id_base(" ".to_string()), DEFAULT_ID_BASE);
}

#[test]
fn resolve_state_tracks_source_markers() {
    let state = resolve_state(TrayPartStateInput {
        slot: TraySlot::Root,
        has_description: true,
        has_footer: true,
        show_close_button: false,
        is_fixed_height: true,
        is_dismissable: false,
        is_keyboard_dismiss_disabled: true,
        has_custom_id_base: true,
        has_custom_title: true,
        has_custom_description: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_on_exit_complete: true,
    });

    assert_eq!(state.slot_attr, "tray");
    assert_eq!(state.base_class, "ui-tray");
    assert_eq!(state.state_attr, "with-description");
    assert_eq!(state.description_attr, "present");
    assert_eq!(state.footer_attr, "present");
    assert_eq!(state.close_button_attr, "hidden");
    assert_eq!(state.size_attr, "fixed");
    assert_eq!(state.dismiss_attr, "locked");
    assert_eq!(state.keyboard_dismiss_attr, "disabled");
    assert_eq!(state.description_source_attr, "custom");
    assert_eq!(state.footer_source_attr, "custom");
    assert_eq!(state.close_source_attr, "custom");
    assert_eq!(state.size_source_attr, "custom");
    assert_eq!(state.dismiss_source_attr, "custom");
    assert_eq!(state.keyboard_dismiss_source_attr, "custom");
    assert_eq!(state.id_source_attr, "custom");
    assert_eq!(state.title_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.exit_source_attr, "custom");
}
