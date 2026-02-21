use super::*;

#[test]
fn sanitize_percent_clamps_and_falls_back_for_invalid_numbers() {
    assert_eq!(sanitize_percent(-1.0), 0.0);
    assert_eq!(sanitize_percent(38.5), 38.5);
    assert_eq!(sanitize_percent(101.0), 100.0);
    assert_eq!(sanitize_percent(f32::NAN), DEFAULT_POSITION_PERCENT);
}

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
    let state = resolve_state(ColorThumbStateInput {
        interaction_state: ColorThumbInteractionState::Dragging,
        show_loupe: true,
        loupe_source: ColorThumbInputSource::External,
        has_color: true,
        x_percent: 22.0,
        y_percent: 88.0,
        x_source: ColorThumbInputSource::External,
        y_source: ColorThumbInputSource::External,
        has_custom_aria_label: true,
        aria_value_text_source: ColorThumbAriaValueTextSource::Custom,
        has_custom_class_name: true,
    });

    assert_eq!(state.data_state_attr, "dragging");
    assert_eq!(state.interaction_source_attr, "external");
    assert!(state.loupe_visible);
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.aria_value_text_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.loupe_source_attr, "external");
    assert_eq!(state.x_source_attr, "external");
    assert_eq!(state.y_source_attr, "external");
    assert_eq!(state.x_bucket_attr, "start");
    assert_eq!(state.y_bucket_attr, "end");

    let class_name = compose_class_name(Some("docs-thumb".to_string()), state);
    assert!(class_name.contains("ui-color-thumb"));
    assert!(class_name.contains("ui-color-thumb--x-start"));
    assert!(class_name.contains("ui-color-thumb--y-end"));
    assert!(class_name.contains("ui-color-thumb--dragging"));
    assert!(class_name.contains("ui-color-thumb--custom-class"));
    assert!(class_name.contains("docs-thumb"));
}

#[test]
fn resolve_component_state_owns_default_value_priority() {
    let state = resolve_component_state(ColorThumbLogicInput {
        interaction_state: ColorThumbInteractionState::Dragging,
        is_loupe_visible: None,
        has_color: true,
        x_percent: None,
        y_percent: Some(120.0),
        has_custom_aria_label: false,
        aria_value_text_source: ColorThumbAriaValueTextSource::Default,
        has_custom_class_name: false,
    });

    assert_eq!(state.x_percent, DEFAULT_POSITION_PERCENT);
    assert_eq!(state.y_percent, 100.0);
    assert!(state.loupe_visible);
    assert_eq!(state.loupe_source_attr, "default");
    assert_eq!(state.x_source_attr, "default");
    assert_eq!(state.y_source_attr, "external");
}

#[test]
fn interaction_state_from_flags_uses_exclusive_priority() {
    assert_eq!(
        interaction_state_from_flags(false, false, false),
        ColorThumbInteractionState::Idle
    );
    assert_eq!(
        interaction_state_from_flags(false, true, false),
        ColorThumbInteractionState::Focused
    );
    assert_eq!(
        interaction_state_from_flags(false, true, true),
        ColorThumbInteractionState::Dragging
    );
    assert_eq!(
        interaction_state_from_flags(true, true, true),
        ColorThumbInteractionState::Disabled
    );
}

#[test]
fn normalize_aria_value_text_prefers_prop_then_color_then_default() {
    assert_eq!(
        normalize_aria_value_text(
            Some("  Hue 120 deg, 80%  ".to_string()),
            Some("#09f".to_string())
        ),
        (
            "Hue 120 deg, 80%".to_string(),
            ColorThumbAriaValueTextSource::Custom
        )
    );
    assert_eq!(
        normalize_aria_value_text(None, Some(" #09f ".to_string())),
        ("#09f".to_string(), ColorThumbAriaValueTextSource::Color)
    );
    assert_eq!(
        normalize_aria_value_text(Some("   ".to_string()), None),
        (
            DEFAULT_ARIA_VALUE_TEXT.to_string(),
            ColorThumbAriaValueTextSource::Default
        )
    );
}
