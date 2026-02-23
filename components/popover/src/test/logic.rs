use super::*;
use leptos::prelude::*;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

#[test]
fn state_and_modal_attrs_follow_contract() {
    assert_eq!(state_attr_for_open(true), "open");
    assert_eq!(state_attr_for_open(false), "closed");
    assert_eq!(modal_attr(true), "modal");
    assert_eq!(modal_attr(false), "non-modal");
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-popover  ".to_string())),
        Some("docs-popover".to_string())
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
fn normalize_open_state_prefers_is_open_and_tracks_sources() {
    let (is_open_raw, _set_is_open_raw) = signal(true);
    let (open_raw, _set_open_raw) = signal(false);

    let normalized = normalize_open_state(PopoverOpenStateInput {
        is_open: Some(is_open_raw.into()),
        open: Some(open_raw.into()),
        default_open: Some(true),
        on_open_change: None,
        on_close: None,
    });

    assert_eq!(normalized.mode, PopoverOpenMode::Controlled);
    assert_eq!(normalized.mode.as_attr(), "controlled");
    assert_eq!(normalized.open_state_source_attr, "external");
    assert_eq!(normalized.open_prop_source_attr, "is_open");
    assert_eq!(normalized.default_open_source_attr, "default_open");
    assert_eq!(normalized.open_change_source_attr, "none");
    assert!(normalized.has_custom_open);
    assert!(normalized.has_custom_default_open);
    assert!(!normalized.has_custom_on_open_change);
    assert!(!normalized.has_custom_on_close);
    assert!(normalized.open.expect("open signal").get_untracked());
}

#[test]
fn normalize_open_state_maps_legacy_on_close_to_on_open_change() {
    let closed = Arc::new(AtomicBool::new(false));
    let closed_for_handler = Arc::clone(&closed);
    let normalized = normalize_open_state(PopoverOpenStateInput {
        is_open: None,
        open: None,
        default_open: None,
        on_open_change: None,
        on_close: Some(Callback::new(move |_| {
            closed_for_handler.store(true, Ordering::SeqCst);
        })),
    });

    assert_eq!(normalized.mode, PopoverOpenMode::Uncontrolled);
    assert_eq!(normalized.open_state_source_attr, "implicit-default");
    assert_eq!(normalized.open_prop_source_attr, "none");
    assert_eq!(
        normalized.default_open,
        ui_state_primitives::popover::DEFAULT_OPEN
    );
    assert_eq!(normalized.open_change_source_attr, "on_close");
    assert!(normalized.has_custom_on_close);
    assert!(!normalized.has_custom_on_open_change);

    let on_open_change = normalized.on_open_change.expect("mapped close callback");
    on_open_change.run(true);
    assert!(!closed.load(Ordering::SeqCst));
    on_open_change.run(false);
    assert!(closed.load(Ordering::SeqCst));
}

#[test]
fn normalize_open_state_exposes_default_source_when_uncontrolled_default_is_provided() {
    let normalized = normalize_open_state(PopoverOpenStateInput {
        is_open: None,
        open: None,
        default_open: Some(true),
        on_open_change: None,
        on_close: None,
    });

    assert_eq!(normalized.mode, PopoverOpenMode::Uncontrolled);
    assert_eq!(normalized.open_state_source_attr, "default");
    assert_eq!(normalized.default_open_source_attr, "default_open");
}

#[test]
fn normalize_open_state_prefers_on_open_change_over_on_close_alias() {
    let canonical_called = Arc::new(AtomicBool::new(false));
    let canonical_called_for_handler = Arc::clone(&canonical_called);
    let close_called = Arc::new(AtomicBool::new(false));
    let close_called_for_handler = Arc::clone(&close_called);

    let normalized = normalize_open_state(PopoverOpenStateInput {
        is_open: None,
        open: None,
        default_open: None,
        on_open_change: Some(Callback::new(move |_| {
            canonical_called_for_handler.store(true, Ordering::SeqCst);
        })),
        on_close: Some(Callback::new(move |_| {
            close_called_for_handler.store(true, Ordering::SeqCst);
        })),
    });

    assert_eq!(normalized.open_change_source_attr, "on_open_change");
    assert!(normalized.has_custom_on_open_change);
    assert!(normalized.has_custom_on_close);

    normalized
        .on_open_change
        .expect("canonical open-change callback")
        .run(false);
    assert!(canonical_called.load(Ordering::SeqCst));
    assert!(!close_called.load(Ordering::SeqCst));
}

#[test]
fn resolve_states_centralizes_slot_state_derivation() {
    let resolved = resolve_states(PopoverStateInputs {
        open: true,
        modal_mode: PopoverModalMode::NonModal,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_custom_placement: true,
        has_on_exit_complete: true,
    });

    assert_eq!(resolved.root_state.slot_attr, "popover");
    assert_eq!(resolved.root_state.state_attr, "open");
    assert_eq!(resolved.panel_state.slot_attr, "popover-panel");
    assert_eq!(resolved.panel_state.state_attr, "panel");
}

#[test]
fn popover_modal_mode_enum_maps_bool_inputs_to_closed_set() {
    assert_eq!(
        PopoverModalMode::from_is_modal(true),
        PopoverModalMode::Modal
    );
    assert_eq!(
        PopoverModalMode::from_is_modal(false),
        PopoverModalMode::NonModal
    );
}

#[test]
fn resolve_state_tracks_source_markers() {
    let state = resolve_state(PopoverPartStateInput {
        slot: PopoverSlot::Root,
        open: true,
        is_modal: false,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_custom_placement: true,
        has_on_exit_complete: true,
    });

    assert_eq!(state.slot_attr, "popover");
    assert_eq!(state.base_class, "ui-popover");
    assert_eq!(state.state_attr, "open");
    assert_eq!(state.modal_attr, "non-modal");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.placement_source_attr, "custom");
    assert_eq!(state.modal_source_attr, "custom");
    assert_eq!(state.exit_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_custom_markers() {
    let class_name = compose_class_name(
        Some("docs-popover-state".to_string()),
        resolve_state(PopoverPartStateInput {
            slot: PopoverSlot::Root,
            open: false,
            is_modal: false,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_placement: true,
            has_on_exit_complete: true,
        }),
    );

    for token in [
        "ui-popover",
        "ui-popover--custom-motion",
        "ui-popover--custom-placement",
        "ui-popover--non-modal",
        "ui-popover--custom-modal",
        "ui-popover--custom-exit",
        "ui-popover--custom-class",
        "docs-popover-state",
    ] {
        assert!(
            class_name.contains(token),
            "popover class name should include `{token}`"
        );
    }
}

#[test]
fn compose_panel_vars_formats_css_custom_properties() {
    assert_eq!(
        compose_panel_vars(16.5, 24.0, 320.0),
        "--ui-popover-top: 16.5px; --ui-popover-left: 24px; --ui-popover-anchor-width: 320px;"
    );
}

#[test]
fn should_close_on_escape_requires_topmost_non_composing_non_prevented_escape() {
    assert!(should_close_on_escape("Escape", true, false, false));
    assert!(!should_close_on_escape("Enter", true, false, false));
    assert!(!should_close_on_escape("Escape", false, false, false));
    assert!(!should_close_on_escape("Escape", true, true, false));
    assert!(!should_close_on_escape("Escape", true, false, true));
}
