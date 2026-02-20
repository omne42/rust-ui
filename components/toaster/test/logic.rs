use super::*;
use crate::toaster::ToasterStoreSource;

#[test]
fn normalize_helpers_trim_and_guard_limits() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-toaster ".to_string())),
        Some("docs-toaster".to_string())
    );

    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        normalize_aria_label(Some(" Alerts host ".to_string())),
        ("Alerts host".to_string(), true)
    );

    assert_eq!(normalize_max_toasts(0), 1);
    assert_eq!(normalize_max_toasts(2), 2);
}

#[test]
fn normalize_props_centralizes_defaults_and_custom_source_flags() {
    let normalized = normalize_props(ToasterNormalizeInput {
        position: ToasterPosition::TopLeft,
        portal: DEFAULT_PORTAL,
        max_toasts: 0,
        aria_label: Some("  Alerts host ".to_string()),
        class_name: Some(" docs-toaster ".to_string()),
        motion: crate::toast::ToastMotion::default(),
    });

    assert_eq!(normalized.position, ToasterPosition::TopLeft);
    assert_eq!(normalized.portal, DEFAULT_PORTAL);
    assert_eq!(normalized.max_toasts, 1);
    assert_eq!(normalized.aria_label, "Alerts host");
    assert_eq!(normalized.class_name, Some("docs-toaster".to_string()));
    assert!(normalized.has_custom_position);
    assert!(!normalized.has_custom_portal);
    assert!(normalized.has_custom_max_toasts);
    assert!(normalized.has_custom_aria_label);
    assert!(normalized.has_custom_class_name);
    assert!(!normalized.has_custom_motion);
}

#[test]
fn queue_and_state_markers_follow_contract() {
    assert_eq!(state_attr(true), "portal");
    assert_eq!(state_attr(false), "inline");

    assert_eq!(queue_attr(1), "single");
    assert_eq!(queue_attr(3), "bounded");
    assert_eq!(queue_attr(6), "extended");
}

#[test]
fn agent_contract_is_typed_and_stable() {
    let contract = agent_contract();
    assert_eq!(contract.schema_attr, "ui.toaster.v1");
    assert_eq!(
        contract.intent_attr,
        ToasterAgentIntent::NotificationHost.as_attr()
    );
    assert_eq!(
        contract.action_model_attr,
        ToasterAgentActionModel::PushClearDismiss.as_attr()
    );
    assert_eq!(
        contract.stream_support_attr,
        ToasterAgentStreamSupport::Optional.as_attr()
    );
    assert_eq!(
        contract.stream_fallback_attr,
        ToasterAgentStreamFallback::Snapshot.as_attr()
    );
    assert_eq!(
        contract.output_status_attr,
        ToasterAgentOutputStatus::Verified.as_attr()
    );
    assert_eq!(
        contract.state_axis_attr,
        "state|queue|position|portal|max-toasts"
    );
    assert_eq!(
        contract.source_axis_attr,
        "position|portal|max-toasts|aria|class|motion|store"
    );
    assert_eq!(ToasterAgentOutputStatus::Draft.as_attr(), "draft");
    assert_eq!(
        ToasterAgentOutputStatus::Submittable.as_attr(),
        "submittable"
    );
}

#[test]
fn resolve_state_tracks_state_sources_and_store_origin() {
    let state = resolve_state(ToasterPartStateInput {
        slot: ToasterSlot::Root,
        position: ToasterPosition::TopCenter,
        portal: false,
        max_toasts: 0,
        has_custom_position: true,
        has_custom_portal: true,
        has_custom_max_toasts: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        store_source: ToasterStoreSource::Provided,
    });

    assert_eq!(state.slot_attr, "toaster");
    assert_eq!(state.base_class, "ui-toaster");
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
    let root_state = resolve_state(ToasterPartStateInput {
        slot: ToasterSlot::Root,
        position: ToasterPosition::BottomLeft,
        portal: true,
        max_toasts: 5,
        has_custom_position: true,
        has_custom_portal: false,
        has_custom_max_toasts: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        store_source: ToasterStoreSource::Local,
    });

    let class_name = compose_class_name(Some("docs-toaster".to_string()), root_state);
    assert!(class_name.contains("ui-toaster"));
    assert!(class_name.contains("ui-toaster--bottom-left"));
    assert!(class_name.contains("ui-toaster--portal"));
    assert!(class_name.contains("ui-toaster--custom-position"));
    assert!(class_name.contains("ui-toaster--custom-max-toasts"));
    assert!(class_name.contains("ui-toaster--custom-motion"));
    assert!(class_name.contains("ui-toaster--custom-class"));
    assert!(class_name.contains("ui-toaster--custom-aria"));
    assert!(class_name.contains("docs-toaster"));

    let sonner_state = resolve_state(ToasterPartStateInput {
        slot: ToasterSlot::Sonner,
        position: ToasterPosition::TopRight,
        portal: false,
        max_toasts: 2,
        has_custom_position: false,
        has_custom_portal: false,
        has_custom_max_toasts: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
        has_custom_motion: false,
        store_source: ToasterStoreSource::Context,
    });

    let sonner_class = compose_class_name(None, sonner_state);
    assert_eq!(
        sonner_class,
        "ui-toaster__sonner ui-toaster__sonner--top-right ui-toaster__sonner--inline"
    );
}

#[test]
fn map_to_sonner_position_matches_all_variants() {
    assert_eq!(
        map_to_sonner_position(ToasterPosition::TopLeft),
        crate::sonner::SonnerPosition::TopLeft
    );
    assert_eq!(
        map_to_sonner_position(ToasterPosition::TopCenter),
        crate::sonner::SonnerPosition::TopCenter
    );
    assert_eq!(
        map_to_sonner_position(ToasterPosition::TopRight),
        crate::sonner::SonnerPosition::TopRight
    );
    assert_eq!(
        map_to_sonner_position(ToasterPosition::BottomLeft),
        crate::sonner::SonnerPosition::BottomLeft
    );
    assert_eq!(
        map_to_sonner_position(ToasterPosition::BottomCenter),
        crate::sonner::SonnerPosition::BottomCenter
    );
    assert_eq!(
        map_to_sonner_position(ToasterPosition::BottomRight),
        crate::sonner::SonnerPosition::BottomRight
    );
}
