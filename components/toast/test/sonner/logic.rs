use super::*;
use crate::sonner::{SonnerPartStateInput, SonnerPosition, SonnerStoreSource};

#[test]
fn normalize_helpers_trim_and_guard_limits() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-sonner ".to_string())),
        Some("docs-sonner".to_string())
    );

    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        normalize_aria_label(Some(" Status host ".to_string())),
        ("Status host".to_string(), true)
    );

    assert_eq!(normalize_max_toasts(0), 1);
    assert_eq!(normalize_max_toasts(2), 2);
}

#[test]
fn normalize_props_centralizes_defaults_and_custom_source_flags() {
    let normalized = normalize_props(SonnerNormalizeInput {
        position: SonnerPosition::TopCenter,
        portal: false,
        max_toasts: 0,
        aria_label: Some(" Status host ".to_string()),
        class_name: Some(" docs-sonner ".to_string()),
        motion: ToastMotion {
            initial_y_px: 22.0,
            initial_scale: 0.94,
            ..ToastMotion::default()
        },
    });

    assert_eq!(normalized.position, SonnerPosition::TopCenter);
    assert!(!normalized.portal);
    assert_eq!(normalized.max_toasts, 1);
    assert_eq!(normalized.aria_label, "Status host");
    assert_eq!(normalized.class_name, Some("docs-sonner".to_string()));
    assert!(normalized.has_custom_position);
    assert!(normalized.has_custom_portal);
    assert!(normalized.has_custom_max_toasts);
    assert!(normalized.has_custom_aria_label);
    assert!(normalized.has_custom_class_name);
    assert!(normalized.has_custom_motion);
}

#[test]
fn queue_and_state_markers_follow_contract() {
    assert_eq!(sonner_state::state_attr(true), "portal");
    assert_eq!(sonner_state::state_attr(false), "inline");

    assert_eq!(sonner_state::queue_attr(1), "single");
    assert_eq!(sonner_state::queue_attr(3), "bounded");
    assert_eq!(sonner_state::queue_attr(6), "extended");
}

#[test]
fn resolve_state_tracks_state_sources_and_store_origin() {
    let state = resolve_state(SonnerPartStateInput {
        slot: SonnerSlot::Root,
        position: SonnerPosition::TopCenter,
        portal: false,
        max_toasts: 0,
        has_custom_position: true,
        has_custom_portal: true,
        has_custom_max_toasts: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        store_source: SonnerStoreSource::Provided,
    });

    assert_eq!(state.slot_attr, "sonner");
    assert_eq!(state.base_class, "ui-sonner");
    assert_eq!(state.position_attr, "top-center");
    assert_eq!(state.state_attr, "inline");
    assert_eq!(state.portal_attr, "false");
    assert_eq!(state.max_toasts, 1);
    assert_eq!(state.queue_attr, "single");
    assert_eq!(state.position_source_attr, "custom");
    assert_eq!(state.portal_source_attr, "custom");
    assert_eq!(state.max_toasts_source_attr, "custom");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.store_source_attr, "provided");
}

#[test]
fn compose_class_name_includes_custom_markers() {
    let root_state = resolve_state(SonnerPartStateInput {
        slot: SonnerSlot::Root,
        position: SonnerPosition::BottomLeft,
        portal: true,
        max_toasts: 5,
        has_custom_position: true,
        has_custom_portal: false,
        has_custom_max_toasts: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        store_source: SonnerStoreSource::Local,
    });

    let class_name = compose_class_name(Some("docs-sonner".to_string()), root_state);
    assert!(class_name.contains("ui-sonner"));
    assert!(class_name.contains("ui-sonner--bottom-left"));
    assert!(class_name.contains("ui-sonner--portal"));
    assert!(class_name.contains("ui-sonner--custom-position"));
    assert!(class_name.contains("ui-sonner--custom-max-toasts"));
    assert!(class_name.contains("ui-sonner--custom-motion"));
    assert!(class_name.contains("ui-sonner--custom-class"));
    assert!(class_name.contains("ui-sonner--custom-aria"));
    assert!(class_name.contains("docs-sonner"));

    let viewport_state = resolve_state(SonnerPartStateInput {
        slot: SonnerSlot::Viewport,
        position: SonnerPosition::TopRight,
        portal: false,
        max_toasts: 2,
        has_custom_position: false,
        has_custom_portal: false,
        has_custom_max_toasts: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
        has_custom_motion: false,
        store_source: SonnerStoreSource::Context,
    });

    let viewport_class = compose_class_name(None, viewport_state);
    assert_eq!(
        viewport_class,
        "ui-sonner__viewport ui-sonner__viewport--top-right ui-sonner__viewport--inline"
    );
}

#[test]
fn agent_contract_is_stable() {
    let contract = agent_contract();

    assert_eq!(contract.schema_attr, "ui.sonner.v1");
    assert_eq!(contract.intent_attr, "notification-host");
    assert_eq!(contract.action_model_attr, "push|clear|dismiss");
    assert_eq!(contract.stream_support_attr, "optional");
    assert_eq!(contract.stream_fallback_attr, "snapshot");
    assert_eq!(contract.output_status_attr, "verified");
    assert_eq!(
        contract.state_axis_attr,
        "state|queue|position|portal|max-toasts"
    );
    assert_eq!(
        contract.source_axis_attr,
        "position|portal|max-toasts|aria|class|motion|store"
    );
}
