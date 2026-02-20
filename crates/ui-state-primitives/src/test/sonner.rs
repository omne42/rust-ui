use super::*;

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
fn queue_and_state_markers_follow_contract() {
    assert_eq!(state_attr(true), "portal");
    assert_eq!(state_attr(false), "inline");

    assert_eq!(queue_attr(1), "single");
    assert_eq!(queue_attr(3), "bounded");
    assert_eq!(queue_attr(6), "extended");
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
