use super::*;
use crate::tray::TrayPartStateInput;

#[test]
fn compose_class_name_includes_state_markers() {
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

    let class_name = compose_class_name(Some("docs-tray".to_string()), state);

    for token in [
        "ui-tray",
        "ui-tray--with-description",
        "ui-tray--with-footer",
        "ui-tray--close-hidden",
        "ui-tray--fixed-height",
        "ui-tray--custom-id",
        "ui-tray--custom-title",
        "ui-tray--custom-description",
        "ui-tray--custom-footer",
        "ui-tray--custom-close",
        "ui-tray--custom-size",
        "ui-tray--custom-motion",
        "ui-tray--custom-exit",
        "ui-tray--custom-dismiss",
        "ui-tray--custom-keyboard-dismiss",
        "ui-tray--custom-class",
        "docs-tray",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
