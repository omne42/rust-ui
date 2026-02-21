use super::*;
use crate::tray::TrayPartStateInput;
use leptos::prelude::Callable;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

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

#[test]
fn resolve_states_centralizes_slot_state_derivation() {
    let resolved = resolve_states(TrayStateInputs {
        description_mode: TrayDescriptionMode::WithDescription,
        footer_mode: TrayFooterMode::WithFooter,
        close_button_mode: TrayCloseButtonMode::Hidden,
        size_mode: TraySizeMode::FixedHeight,
        dismiss_mode: TrayDismissMode::Locked,
        keyboard_dismiss_mode: TrayKeyboardDismissMode::Disabled,
        has_custom_id_base: true,
        has_custom_title: true,
        has_custom_description: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_on_exit_complete: true,
    });

    assert_eq!(resolved.root_state.slot_attr, "tray");
    assert_eq!(resolved.header_state.slot_attr, "tray-header");
    assert_eq!(resolved.title_state.slot_attr, "tray-title");
    assert_eq!(resolved.description_state.slot_attr, "tray-description");
    assert_eq!(resolved.body_state.slot_attr, "tray-body");
    assert_eq!(resolved.footer_state.slot_attr, "tray-footer");
    assert_eq!(resolved.close_state.slot_attr, "tray-close");
}

#[test]
fn tray_mode_enums_map_bool_inputs_to_closed_set() {
    assert_eq!(
        TrayDescriptionMode::from_has_description(true),
        TrayDescriptionMode::WithDescription
    );
    assert_eq!(
        TrayDescriptionMode::from_has_description(false),
        TrayDescriptionMode::TitleOnly
    );
    assert_eq!(
        TrayFooterMode::from_has_footer(true),
        TrayFooterMode::WithFooter
    );
    assert_eq!(
        TrayFooterMode::from_has_footer(false),
        TrayFooterMode::NoFooter
    );
    assert_eq!(
        TrayCloseButtonMode::from_show_close_button(true),
        TrayCloseButtonMode::Shown
    );
    assert_eq!(
        TrayCloseButtonMode::from_show_close_button(false),
        TrayCloseButtonMode::Hidden
    );
    assert_eq!(
        TraySizeMode::from_is_fixed_height(true),
        TraySizeMode::FixedHeight
    );
    assert_eq!(
        TraySizeMode::from_is_fixed_height(false),
        TraySizeMode::AutoHeight
    );
    assert_eq!(
        TrayDismissMode::from_is_dismissable(true),
        TrayDismissMode::Dismissable
    );
    assert_eq!(
        TrayDismissMode::from_is_dismissable(false),
        TrayDismissMode::Locked
    );
    assert_eq!(
        TrayKeyboardDismissMode::from_is_disabled(true),
        TrayKeyboardDismissMode::Disabled
    );
    assert_eq!(
        TrayKeyboardDismissMode::from_is_disabled(false),
        TrayKeyboardDismissMode::Enabled
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
fn normalize_optional_attr_converts_none_to_empty_string() {
    assert_eq!(normalize_optional_attr(None), "");
    assert_eq!(
        normalize_optional_attr(Some("tray-description".to_string())),
        "tray-description"
    );
}
