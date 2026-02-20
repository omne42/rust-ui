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
