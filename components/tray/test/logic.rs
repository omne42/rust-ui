use super::*;
use crate::tray::TrayPartStateInput;
use leptos::prelude::{Callable, Get, Signal};
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

    assert_eq!(
        resolve_dismiss_policy(false, true),
        TrayDismissPolicy::Locked
    );
    assert_eq!(
        resolve_dismiss_policy(true, false),
        TrayDismissPolicy::DismissableKeyboardEnabled
    );
    assert_eq!(
        resolve_dismiss_policy(true, true),
        TrayDismissPolicy::DismissableKeyboardDisabled
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

#[test]
fn normalize_open_state_delegates_mode_contract_to_primitives() {
    let open_state = normalize_open_state(TrayOpenStateInput {
        is_open: None,
        default_open: Some(true),
        on_open_change: None,
    });

    assert_eq!(open_state.mode, TrayOpenMode::Uncontrolled);
    assert_eq!(open_state.open_source_attr, "uncontrolled");
    assert!(open_state.default_open);
    assert!(can_request_open_change(
        open_state.mode,
        open_state.has_open_change_handler
    ));
}

#[test]
fn normalize_defaults_centralizes_default_priority() {
    let defaults = normalize_defaults(TrayDefaultsInput {
        is_show_close_button: None,
        close_label: None,
        is_fixed_height: None,
        is_dismissable: None,
        is_keyboard_dismiss_disabled: None,
        motion: None,
    });

    assert_eq!(defaults.is_show_close_button, DEFAULT_SHOW_CLOSE_BUTTON);
    assert_eq!(defaults.close_label, DEFAULT_CLOSE_LABEL);
    assert_eq!(defaults.is_fixed_height, DEFAULT_FIXED_HEIGHT);
    assert_eq!(defaults.is_dismissable, DEFAULT_DISMISSABLE);
    assert_eq!(
        defaults.is_keyboard_dismiss_disabled,
        DEFAULT_KEYBOARD_DISMISS_DISABLED
    );
    assert_eq!(defaults.motion, crate::tray::TrayMotion::default());
}

#[test]
fn normalize_text_centralizes_text_default_priority() {
    let normalized = normalize_text(TrayTextInput {
        id_base: "   ".to_string(),
        title: "  ".to_string(),
        description: Some("   ".to_string()),
        class_name: Some("   ".to_string()),
    });

    assert_eq!(normalized.id_base, DEFAULT_ID_BASE);
    assert_eq!(normalized.title, DEFAULT_TITLE);
    assert_eq!(normalized.description, None);
    assert_eq!(normalized.class_name, None);
    assert!(!normalized.has_custom_id_base);
    assert!(!normalized.has_custom_title);

    let custom = normalize_text(TrayTextInput {
        id_base: "docs-tray".to_string(),
        title: "My Tray".to_string(),
        description: Some("Details".to_string()),
        class_name: Some("custom".to_string()),
    });

    assert_eq!(custom.id_base, "docs-tray");
    assert_eq!(custom.title, "My Tray");
    assert_eq!(custom.description.as_deref(), Some("Details"));
    assert_eq!(custom.class_name.as_deref(), Some("custom"));
    assert!(custom.has_custom_id_base);
    assert!(custom.has_custom_title);
}

#[test]
fn resolve_open_signal_prefers_controlled_value_when_present() {
    let controlled = Signal::derive(|| true);
    let fallback = Signal::derive(|| false);
    let resolved = resolve_open_signal(Some(controlled), fallback);

    assert!(resolved.get());
}

#[test]
fn resolve_close_effects_centralizes_close_transition_rules() {
    let uncontrolled = resolve_close_effects(TrayOpenMode::Uncontrolled, false);
    assert!(uncontrolled.should_close_uncontrolled);
    assert!(uncontrolled.should_emit_open_change);

    let controlled_without_handler = resolve_close_effects(TrayOpenMode::Controlled, false);
    assert!(!controlled_without_handler.should_close_uncontrolled);
    assert!(!controlled_without_handler.should_emit_open_change);

    let controlled_with_handler = resolve_close_effects(TrayOpenMode::Controlled, true);
    assert!(!controlled_with_handler.should_close_uncontrolled);
    assert!(controlled_with_handler.should_emit_open_change);
}

#[test]
fn normalize_state_inputs_centralizes_boundary_to_state_mapping() {
    let normalized = normalize_state_inputs(TrayStateBoundaryInput {
        has_description: true,
        has_footer: false,
        is_show_close_button: true,
        is_fixed_height: false,
        dismiss_policy: TrayDismissPolicy::DismissableKeyboardDisabled,
        has_custom_id_base: true,
        has_custom_title: false,
        has_custom_description: true,
        has_custom_class_name: false,
        has_custom_motion: true,
        has_on_exit_complete: false,
    });

    assert_eq!(
        normalized.description_mode,
        TrayDescriptionMode::WithDescription
    );
    assert_eq!(normalized.footer_mode, TrayFooterMode::NoFooter);
    assert_eq!(normalized.close_button_mode, TrayCloseButtonMode::Shown);
    assert_eq!(normalized.size_mode, TraySizeMode::AutoHeight);
    assert_eq!(normalized.dismiss_mode, TrayDismissMode::Dismissable);
    assert_eq!(
        normalized.keyboard_dismiss_mode,
        TrayKeyboardDismissMode::Disabled
    );
    assert!(normalized.has_custom_id_base);
    assert!(!normalized.has_custom_title);
    assert!(normalized.has_custom_description);
    assert!(!normalized.has_custom_class_name);
    assert!(normalized.has_custom_motion);
    assert!(!normalized.has_on_exit_complete);
}
