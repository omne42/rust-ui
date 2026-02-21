use super::*;

#[test]
fn sanitize_color_rejects_unsafe_values() {
    assert_eq!(
        sanitize_color(Some(" #09f ".to_string())),
        Some("#09f".to_string())
    );
    assert_eq!(
        sanitize_color(Some("javascript:alert(1)".to_string())),
        None
    );
}

#[test]
fn normalize_aria_label_uses_default_or_custom_values() {
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        normalize_aria_label(Some("  Accent handle  ".to_string())),
        ("Accent handle".to_string(), true)
    );
}

#[test]
fn resolve_state_and_class_name_track_sources_and_flags() {
    let state = resolve_state(ColorHandleStateInput {
        is_disabled: false,
        is_focused: true,
        is_dragging: true,
        is_loupe_visible: true,
        has_color: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.data_state_attr, "dragging");
    assert!(state.loupe_visible);
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");

    let class_name = compose_class_name(Some("docs-color-handle".to_string()), state);
    assert!(class_name.contains("ui-color-handle"));
    assert!(class_name.contains("ui-color-handle--focused"));
    assert!(class_name.contains("ui-color-handle--dragging"));
    assert!(class_name.contains("ui-color-handle--custom-class"));
    assert!(class_name.contains("docs-color-handle"));
}

#[test]
fn resolve_agent_contract_is_schema_typed_and_traceable() {
    let state = resolve_state(ColorHandleStateInput {
        is_disabled: false,
        is_focused: true,
        is_dragging: true,
        is_loupe_visible: true,
        has_color: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });
    let contract = resolve_agent_contract(state, "custom");

    assert_eq!(contract.schema_name, "ui.color-handle.agent-contract");
    assert_eq!(contract.schema_version.as_str(), "1");
    assert_eq!(contract.intent.as_str(), "color-selection");
    assert_eq!(contract.action.as_str(), "drag-update");
    assert_eq!(contract.state.as_str(), "dragging");
    assert_eq!(contract.source.as_str(), "drag-interaction");
    assert_eq!(contract.stream_support.as_str(), "optional");
    assert_eq!(contract.stream_fallback.as_str(), "snapshot");
    assert_eq!(contract.output_status.as_str(), "submittable");
    assert!(contract.capabilities.can_drag);
    assert!(contract.capabilities.can_focus);
}
