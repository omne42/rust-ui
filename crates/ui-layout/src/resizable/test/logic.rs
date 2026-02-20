use super::*;

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  layout splitter  ".to_string())),
        Some("layout splitter".to_string())
    );

    assert_eq!(normalize_aria_label(None), DEFAULT_ARIA_LABEL);
    assert_eq!(
        normalize_aria_label(Some("  Pane split control  ".to_string())),
        "Pane split control"
    );
}

#[test]
fn normalize_value_axis_supports_canonical_and_legacy_names() {
    let bounds = normalize_bounds(0.0, 100.0);
    let (_value, set_value) = signal(0.0_f64);

    let normalized = normalize_value_axis(ResizableValueAxisInput {
        value: None,
        split_percent: None,
        default_value: Some(64.0),
        default_split_percent: Some(36.0),
        on_value_change: None,
        on_split_percent_change: Some(Callback::new(move |next| set_value.set(next))),
        bounds,
    });

    assert_eq!(normalized.control_mode_attr, "uncontrolled");
    assert_eq!(normalized.value_source_attr, "default_value");
    assert_eq!(normalized.default_value_source_attr, "default_value");
    assert_eq!(
        normalized.value_change_source_attr,
        "on_split_percent_change"
    );
    assert_eq!(normalized.default_value, 64.0);
    assert!(normalized.on_value_change.is_some());
}

#[test]
fn normalize_disabled_and_handle_track_sources() {
    let disabled = normalize_disabled(ResizableDisabledInput {
        is_disabled: Some(true),
        disabled: false,
    });
    assert!(disabled.is_disabled);
    assert_eq!(disabled.disabled_source_attr, "is_disabled");

    let handle = normalize_handle(ResizableHandleInput {
        is_with_handle: None,
        with_handle: true,
    });
    assert!(handle.with_handle);
    assert_eq!(handle.with_handle_source_attr, "with_handle");
}

#[test]
fn compose_class_name_surface_all_markers() {
    let state = ui_state_primitives::resizable::resolve_state(
        ui_state_primitives::resizable::ResizableStateInput {
            orientation: ResizableOrientation::Vertical,
            split_percent: 88.0,
            bounds: ui_state_primitives::resizable::SplitBounds {
                min: 20.0,
                max: 80.0,
            },
            disabled: false,
            dragging: true,
            is_controlled: true,
            with_handle: true,
            has_custom_class_name: true,
        },
    );

    let class_name = compose_class_name(Some("docs-resizable".to_string()), state);
    for token in [
        "ui-resizable",
        "ui-resizable--vertical",
        "ui-resizable--dragging",
        "ui-resizable--with-handle",
        "ui-resizable--controlled",
        "ui-resizable--custom-class",
        "docs-resizable",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}

#[test]
fn resolve_agent_contract_tracks_change_handler_presence() {
    let contract = resolve_agent_contract(ResizableValueChangeSource::OnValueChange);
    assert_eq!(contract.schema_attr, "ui.resizable.agent-contract.v1");
    assert_eq!(contract.intent_attr, "adjust-split");
    assert_eq!(contract.action_model_attr, "pointer+keyboard");
    assert_eq!(
        contract.state_axis_attr,
        "orientation:split:dragging:disabled:control:handle"
    );
    assert_eq!(
        contract.source_axis_attr,
        "value:default:value_change:disabled:handle:class"
    );
    assert_eq!(contract.stream_support_attr, "unsupported");
    assert_eq!(contract.stream_fallback_attr, "snapshot");
    assert_eq!(contract.stream_mode_attr, "snapshot");
    assert_eq!(contract.output_status_attr, "submittable");

    let readonly_contract = resolve_agent_contract(ResizableValueChangeSource::None);
    assert_eq!(readonly_contract.output_status_attr, "verified");
}
