use super::*;
use crate::toast::{ToastMotion, ToastSlot, ToastStoreSource, ToastViewportSlot};
use ui_headless::LiveRegionPriority;

fn with_store(max_toasts: usize, f: impl FnOnce(ToastStore)) {
    Owner::new().with(|| {
        let store = provide_toast_store(ToastStoreOptions { max_toasts });
        f(store);
    });
}

#[test]
fn variant_aria_live_matches_severity() {
    assert_eq!(ToastVariant::Default.aria_live(), "polite");
    assert_eq!(ToastVariant::Accent.aria_live(), "polite");
    assert_eq!(ToastVariant::Danger.aria_live(), "assertive");
}

#[test]
fn discrete_state_axes_are_enum_closed_sets() {
    let variants = [
        ToastVariant::Default,
        ToastVariant::Accent,
        ToastVariant::Danger,
    ];
    assert_eq!(
        variants.map(ToastVariant::as_attr),
        ["default", "accent", "danger"]
    );
    assert_eq!(
        variants.map(ToastVariant::class_name),
        [
            "ui-toast--variant-default",
            "ui-toast--variant-accent",
            "ui-toast--variant-danger",
        ]
    );

    let slots = [
        ToastSlot::Root,
        ToastSlot::Content,
        ToastSlot::Title,
        ToastSlot::Description,
        ToastSlot::Close,
    ];
    assert_eq!(
        slots.map(ToastSlot::as_attr),
        [
            "toast",
            "toast-content",
            "title",
            "description",
            "toast-close"
        ]
    );

    let viewport_slots = [ToastViewportSlot::Root];
    assert_eq!(
        viewport_slots.map(ToastViewportSlot::as_attr),
        ["toast-viewport"]
    );

    let store_sources = [
        ToastStoreSource::Provided,
        ToastStoreSource::Context,
        ToastStoreSource::Local,
    ];
    assert_eq!(
        store_sources.map(ToastStoreSource::as_attr),
        ["provided", "context", "local"]
    );
}

#[test]
fn agent_contracts_are_typed_and_stable() {
    let toast_contract = toast_agent_contract();
    assert_eq!(toast_contract.schema_attr, "ui.toast.v1");
    assert_eq!(
        toast_contract.intent_attr,
        ToastAgentIntent::NotificationItem.as_attr()
    );
    assert_eq!(
        toast_contract.action_model_attr,
        ToastAgentActionModel::DismissClose.as_attr()
    );
    assert_eq!(
        toast_contract.state_axis_attr,
        "state|variant|description|close-mode|open"
    );
    assert_eq!(
        toast_contract.source_axis_attr,
        "id|description|class|motion|close|exit|open"
    );

    let viewport_contract = toast_viewport_agent_contract();
    assert_eq!(viewport_contract.schema_attr, "ui.toast.viewport.v1");
    assert_eq!(
        viewport_contract.intent_attr,
        ToastAgentIntent::NotificationViewport.as_attr()
    );
    assert_eq!(
        viewport_contract.action_model_attr,
        ToastAgentActionModel::QueueDismissRemove.as_attr()
    );
    assert_eq!(
        viewport_contract.state_axis_attr,
        "state|queue|portal|max-toasts"
    );
    assert_eq!(
        viewport_contract.source_axis_attr,
        "portal|max-toasts|class|motion|store"
    );
}

#[test]
fn store_push_adds_toast_and_returns_id() {
    with_store(3, |store| {
        let id = store.push.run(ToastOptions::simple("Hello"));
        assert!(!id.trim().is_empty());

        let toasts = store.toasts().get_untracked();
        assert_eq!(toasts.len(), 1);
        assert_eq!(toasts[0].id, id);
        assert!(toasts[0].is_open);
    });
}

#[test]
fn store_overflow_marks_oldest_closing_and_rotates_to_end() {
    with_store(2, |store| {
        let id1 = store.push.run(ToastOptions::simple("One"));
        let id2 = store.push.run(ToastOptions::simple("Two"));
        let id3 = store.push.run(ToastOptions::simple("Three"));

        let toasts = store.toasts().get_untracked();
        assert_eq!(toasts.len(), 3);

        assert_eq!(toasts[0].id, id2);
        assert_eq!(toasts[1].id, id3);
        assert_eq!(toasts[2].id, id1);

        assert!(toasts[0].is_open);
        assert!(toasts[1].is_open);
        assert!(!toasts[2].is_open);
    });
}

#[test]
fn store_dismiss_marks_toast_closed() {
    with_store(3, |store| {
        let id1 = store.push.run(ToastOptions::simple("One"));
        let id2 = store.push.run(ToastOptions::simple("Two"));

        store.dismiss.run(id1.clone());

        let toasts = store.toasts().get_untracked();
        let t1 = toasts.iter().find(|t| t.id == id1).unwrap();
        let t2 = toasts.iter().find(|t| t.id == id2).unwrap();
        assert!(!t1.is_open);
        assert!(t2.is_open);
    });
}

#[test]
fn store_clear_marks_all_toasts_closed() {
    with_store(3, |store| {
        store.push.run(ToastOptions::simple("One"));
        store.push.run(ToastOptions::simple("Two"));

        store.clear.run(());

        let toasts = store.toasts().get_untracked();
        assert!(!toasts.is_empty());
        for toast in toasts {
            assert!(!toast.is_open);
        }
    });
}

#[test]
fn store_remove_drops_toast_by_id() {
    with_store(3, |store| {
        let id1 = store.push.run(ToastOptions::simple("One"));
        let id2 = store.push.run(ToastOptions::simple("Two"));

        store.remove(&id1);

        let toasts = store.toasts().get_untracked();
        assert_eq!(toasts.len(), 1);
        assert_eq!(toasts[0].id, id2);
    });
}

#[test]
fn toast_options_helper_sets_defaults() {
    let opts = ToastOptions::simple("Hello");
    assert_eq!(opts.title, "Hello");
    assert_eq!(opts.variant, ToastVariant::Default);
    assert!(opts.duration_ms.is_some());
}

#[test]
fn normalize_helpers_trim_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-toast  ".to_string())),
        Some("docs-toast".to_string())
    );

    assert_eq!(normalize_title("  Saved  ".to_string()), "Saved");
    assert_eq!(normalize_title("\n\t".to_string()), "Notification");
    assert_eq!(
        normalize_description(Some("  done  ".to_string())),
        Some("done".to_string())
    );
}

#[test]
fn open_state_config_prefers_is_open_and_sets_default_open_fallback() {
    let (is_open_raw, _set_is_open_raw) = signal(false);
    let config = resolve_open_state_config(Some(is_open_raw.into()), None, None);

    assert!(config.is_controlled);
    assert_eq!(config.open_source_attr, "is_open");
    assert_eq!(config.default_open, Some(true));
    assert!(!config.has_custom_default_open);
    assert!(!config.has_custom_on_open_change);
}

#[test]
fn open_state_config_uncontrolled_axis_keeps_pairing_semantics() {
    let on_open_change = Callback::new(|_| {});
    let config = resolve_open_state_config(None, Some(false), Some(on_open_change));

    assert!(!config.is_controlled);
    assert_eq!(config.open_source_attr, "implicit");
    assert_eq!(config.default_open, Some(false));
    assert!(config.has_custom_default_open);
    assert!(config.has_custom_on_open_change);
}

#[test]
fn normalize_props_centralizes_state_input_and_source_markers() {
    let normalized = normalize_props(ToastNormalizeInput {
        title: "  Saved  ".to_string(),
        id: Some("  toast-1  ".to_string()),
        description: Some("  done  ".to_string()),
        class_name: Some("  docs-toast  ".to_string()),
        motion: ToastMotion {
            initial_y_px: 18.0,
            initial_scale: 0.95,
            ..ToastMotion::default()
        },
    });

    assert_eq!(normalized.title, "Saved");
    assert_eq!(normalized.id, Some("toast-1".to_string()));
    assert_eq!(normalized.description, Some("done".to_string()));
    assert_eq!(normalized.class_name, Some("docs-toast".to_string()));
    assert!(normalized.has_custom_id);
    assert!(normalized.has_description);
    assert!(normalized.has_custom_description);
    assert!(normalized.has_custom_class_name);
    assert!(normalized.has_custom_motion);
}

#[test]
fn open_state_source_markers_are_derived_in_logic() {
    let controlled = resolve_open_state_config(Some(signal(true).0.into()), None, None);
    let controlled_markers = resolve_open_state_markers(&controlled);
    assert_eq!(controlled_markers.control_mode_attr, "controlled");
    assert_eq!(controlled_markers.default_open_source_attr, "implicit");
    assert_eq!(controlled_markers.open_change_source_attr, "none");

    let uncontrolled = resolve_open_state_config(None, Some(false), Some(Callback::new(|_| {})));
    let uncontrolled_markers = resolve_open_state_markers(&uncontrolled);
    assert_eq!(uncontrolled_markers.control_mode_attr, "uncontrolled");
    assert_eq!(uncontrolled_markers.default_open_source_attr, "provided");
    assert_eq!(uncontrolled_markers.open_change_source_attr, "provided");
}

#[test]
fn toast_state_derivation_is_centralized_in_logic() {
    let state = resolve_toast_part_state(ToastStateDerivationInput {
        variant: ToastVariant::Accent,
        is_open: false,
        has_description: true,
        has_custom_id: true,
        has_custom_description: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_custom_on_close: true,
        has_custom_on_exit_complete: true,
    });

    assert_eq!(state.slot, ToastSlot::Root);
    assert_eq!(state.variant, ToastVariant::Accent);
    assert!(!state.is_open);
    assert!(state.has_description);
    assert!(state.has_custom_id);
    assert!(state.has_custom_description);
    assert!(state.has_custom_class_name);
    assert!(state.has_custom_motion);
    assert!(state.has_custom_on_close);
    assert!(state.has_custom_on_exit_complete);
}

#[test]
fn viewport_state_derivation_is_centralized_in_logic() {
    let state = resolve_toast_viewport_state(ToastViewportStateDerivationInput {
        is_portal: false,
        max_toasts: 6,
        has_custom_portal: true,
        has_custom_max_toasts: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        store_source: ToastStoreSource::Context,
    });

    assert_eq!(
        state.slot_attr,
        crate::toast::ToastViewportSlot::Root.as_attr()
    );
    assert!(!state.portal);
    assert_eq!(state.max_toasts, 6);
    assert!(state.has_custom_portal);
    assert!(state.has_custom_max_toasts);
    assert!(state.has_custom_class_name);
    assert!(state.has_custom_motion);
    assert_eq!(state.store_source, ToastStoreSource::Context);
}

#[test]
fn live_region_priority_is_derived_in_logic() {
    assert_eq!(
        resolve_live_region_priority(ToastVariant::Default),
        LiveRegionPriority::Polite
    );
    assert_eq!(
        resolve_live_region_priority(ToastVariant::Accent),
        LiveRegionPriority::Polite
    );
    assert_eq!(
        resolve_live_region_priority(ToastVariant::Danger),
        LiveRegionPriority::Assertive
    );
}

#[test]
fn callback_and_label_defaults_are_centralized_in_logic() {
    let callbacks = resolve_callbacks_config(None, None);
    assert!(!callbacks.has_custom_on_close);
    assert!(!callbacks.has_custom_on_exit_complete);

    callbacks.on_close.run(());
    callbacks.on_exit_complete.run(());

    assert_eq!(
        resolve_close_aria_label(Some("  Dismiss toast  ".to_string()), "Close"),
        "Dismiss toast"
    );
    assert_eq!(resolve_close_aria_label(None, "Close"), "Close");
}

#[test]
fn viewport_defaults_and_record_fallbacks_are_centralized_in_logic() {
    let config = resolve_viewport_config(DEFAULT_VIEWPORT_PORTAL, DEFAULT_VIEWPORT_MAX_TOASTS);
    assert!(!config.has_custom_portal);
    assert!(!config.has_custom_max_toasts);

    let custom = resolve_viewport_config(!DEFAULT_VIEWPORT_PORTAL, DEFAULT_VIEWPORT_MAX_TOASTS + 1);
    assert!(custom.has_custom_portal);
    assert!(custom.has_custom_max_toasts);

    let toasts = vec![ToastInstance {
        id: "id-1".to_string(),
        title: "Saved".to_string(),
        description: Some("done".to_string()),
        variant: ToastVariant::Default,
        is_open: true,
    }];
    assert!(resolve_instance_open(&toasts, "id-1"));
    assert!(!resolve_instance_open(&toasts, "id-404"));
    assert_eq!(
        resolve_instance_description(Some("desc".to_string())),
        "desc"
    );
    assert_eq!(resolve_instance_description(None), "");
}

#[test]
fn viewport_state_normalization_and_store_source_are_centralized_in_logic() {
    let normalized = normalize_viewport_props(ToastViewportNormalizeInput {
        is_portal: false,
        max_toasts: 0,
        class_name: Some("  docs-viewport  ".to_string()),
        motion: ToastMotion {
            initial_y_px: 24.0,
            initial_scale: 0.92,
            ..ToastMotion::default()
        },
    });

    assert!(!normalized.viewport.is_portal);
    assert!(normalized.viewport.has_custom_portal);
    assert!(normalized.viewport.has_custom_max_toasts);
    assert_eq!(normalized.normalized_max_toasts, 1);
    assert_eq!(normalized.class_name, Some("docs-viewport".to_string()));
    assert!(normalized.has_custom_class_name);
    assert!(normalized.has_custom_motion);

    Owner::new().with(|| {
        let provided = provide_toast_store(ToastStoreOptions { max_toasts: 2 });
        let (_, source) = resolve_viewport_store(Some(provided), normalized.normalized_max_toasts);
        assert_eq!(source, ToastStoreSource::Provided);
    });

    Owner::new().with(|| {
        let _context = provide_toast_store(ToastStoreOptions { max_toasts: 2 });
        let (_, source) = resolve_viewport_store(None, normalized.normalized_max_toasts);
        assert_eq!(source, ToastStoreSource::Context);
    });

    Owner::new().with(|| {
        let (_, source) = resolve_viewport_store(None, normalized.normalized_max_toasts);
        assert_eq!(source, ToastStoreSource::Local);
    });
}

#[test]
fn compose_toast_class_name_tracks_state_markers() {
    let class_name = compose_class_name(
        Some("docs-toast-custom".to_string()),
        resolve_state(ToastPartStateInput {
            slot: ToastSlot::Root,
            variant: ToastVariant::Accent,
            is_open: false,
            has_description: true,
            has_custom_id: true,
            has_custom_description: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_on_close: true,
            has_custom_on_exit_complete: true,
        }),
    );

    for token in [
        "ui-toast",
        "ui-toast--variant-accent",
        "ui-toast--closing",
        "ui-toast--with-description",
        "ui-toast--custom-class",
        "docs-toast-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}
