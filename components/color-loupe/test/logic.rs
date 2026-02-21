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
fn resolve_state_and_class_name_track_flags_and_sources() {
    let state = resolve_state(ColorLoupeStateInput {
        open: true,
        disabled: false,
        has_color: true,
        x_percent: 22.0,
        y_percent: 88.0,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert!(state.is_open);
    assert_eq!(state.data_state_attr, "open");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.x_bucket_attr, "start");
    assert_eq!(state.y_bucket_attr, "end");

    let class_name = compose_class_name(Some("docs-color-loupe".to_string()), state);
    assert!(class_name.contains("ui-color-loupe"));
    assert!(class_name.contains("ui-color-loupe--open"));
    assert!(class_name.contains("ui-color-loupe--custom-class"));
    assert!(class_name.contains("docs-color-loupe"));
}

#[test]
fn normalize_aria_label_uses_default_or_custom_values() {
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        normalize_aria_label(Some("  Accent loupe  ".to_string())),
        ("Accent loupe".to_string(), true)
    );
}

#[test]
fn resolve_component_state_owns_default_position_priority() {
    let state = resolve_component_state(ColorLoupeLogicInput {
        is_open: true,
        is_disabled: false,
        has_color: true,
        x_percent: None,
        y_percent: Some(120.0),
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });

    assert_eq!(state.x_percent, DEFAULT_POSITION_PERCENT);
    assert_eq!(state.y_percent, 100.0);
    assert!(state.is_open);
}

#[test]
fn semantic_markers_use_closed_enumerable_value_sets() {
    let base = ColorLoupeLogicInput {
        is_open: false,
        is_disabled: false,
        has_color: false,
        x_percent: Some(50.0),
        y_percent: Some(50.0),
        has_custom_aria_label: false,
        has_custom_class_name: false,
    };

    let idle = resolve_component_state(base);
    let open = resolve_component_state(ColorLoupeLogicInput {
        is_open: true,
        ..base
    });
    let color = resolve_component_state(ColorLoupeLogicInput {
        has_color: true,
        ..base
    });
    let disabled = resolve_component_state(ColorLoupeLogicInput {
        is_disabled: true,
        ..base
    });
    let start_end = resolve_component_state(ColorLoupeLogicInput {
        x_percent: Some(0.0),
        y_percent: Some(100.0),
        ..base
    });
    let custom_sources = resolve_component_state(ColorLoupeLogicInput {
        has_custom_aria_label: true,
        has_custom_class_name: true,
        ..base
    });

    assert_eq!(idle.data_state_attr, "idle");
    assert_eq!(open.data_state_attr, "open");
    assert_eq!(color.data_state_attr, "color");
    assert_eq!(disabled.data_state_attr, "disabled");

    for marker in [
        idle.data_state_attr,
        open.data_state_attr,
        color.data_state_attr,
        disabled.data_state_attr,
    ] {
        assert!(matches!(marker, "idle" | "color" | "open" | "disabled"));
    }

    for marker in [
        idle.x_bucket_attr,
        idle.y_bucket_attr,
        start_end.x_bucket_attr,
        start_end.y_bucket_attr,
    ] {
        assert!(matches!(marker, "start" | "center" | "end"));
    }

    for marker in [
        idle.aria_source_attr,
        custom_sources.aria_source_attr,
        idle.class_source_attr,
        custom_sources.class_source_attr,
    ] {
        assert!(matches!(marker, "default" | "custom"));
    }
}

#[test]
fn agent_contract_schema_is_type_driven_and_closed() {
    let state = resolve_component_state(ColorLoupeLogicInput {
        is_open: true,
        is_disabled: false,
        has_color: true,
        x_percent: Some(20.0),
        y_percent: Some(80.0),
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    let schema = agent_contract_schema_attr(state, ColorLoupeOutputState::Verified);
    assert_eq!(
        schema,
        "v=1;intent=snapshot;action=render;state=open;output_state=verified;source=aria:custom,class:default;x_bucket=start;y_bucket=end"
    );
    assert!(!schema.contains("<script"));
    assert!(!schema.contains("javascript:"));
}

#[test]
fn output_state_defaults_and_enumeration_are_closed() {
    assert_eq!(
        normalize_output_state(None),
        ColorLoupeOutputState::Verified
    );
    for output_state in [
        ColorLoupeOutputState::Draft,
        ColorLoupeOutputState::Verified,
        ColorLoupeOutputState::Committable,
    ] {
        assert!(matches!(
            output_state.as_attr(),
            "draft" | "verified" | "committable"
        ));
    }
}
