use super::*;

#[test]
fn channel_contracts_are_stable() {
    assert_eq!(
        ColorSliderChannel::Hue.class_name(),
        "ui-color-slider--channel-hue"
    );
    assert_eq!(ColorSliderChannel::Hue.as_attr(), "hue");
    assert_eq!(ColorSliderChannel::Hue.default_bounds(), (0.0, 360.0));
    assert_eq!(ColorSliderChannel::Hue.default_label(), "Hue");

    assert_eq!(ColorSliderChannel::Alpha.default_bounds(), (0.0, 100.0));
    assert_eq!(ColorSliderChannel::Red.default_bounds(), (0.0, 255.0));
}

#[test]
fn normalize_helpers_keep_defaults_and_trim_values() {
    assert_eq!(
        normalize_label(None, ColorSliderChannel::Hue),
        ("Hue".to_string(), false)
    );
    assert_eq!(
        normalize_label(Some("  Tint  ".to_string()), ColorSliderChannel::Hue),
        ("Tint".to_string(), true)
    );

    assert_eq!(
        normalize_aria_label(None, "Hue", ColorSliderChannel::Hue),
        ("Hue slider".to_string(), false)
    );
    assert_eq!(
        normalize_aria_label(Some("  Tone  ".to_string()), "Hue", ColorSliderChannel::Hue),
        ("Tone".to_string(), true)
    );

    assert_eq!(
        sanitize_track_color(Some(" #09f ".to_string())),
        Some("#09f".to_string())
    );
    assert_eq!(
        sanitize_track_color(Some("javascript:alert(1)".to_string())),
        None
    );
}

#[test]
fn sanitizers_handle_invalid_bounds_step_and_value() {
    let channel = ColorSliderChannel::Hue;
    let (min, max) = sanitize_bounds(channel, 360.0, 0.0);
    assert_eq!((min, max), (0.0, 360.0));

    let step = sanitize_step(channel, f64::NAN, min, max);
    assert_eq!(step, 1.0);

    let value = sanitize_value(channel, 482.5, min, max, step);
    assert_eq!(value, 360.0);

    assert_eq!(resolve_percent(180.0, min, max), 50.0);
}

#[test]
fn inline_style_and_formatting_are_stable() {
    assert_eq!(
        compose_inline_style(Some("#000"), Some("#fff")),
        Some("--ui-color-slider-track-start: #000; --ui-color-slider-track-end: #fff;".to_string())
    );
    assert_eq!(compose_inline_style(None, None), None);

    assert_eq!(format_channel_value(ColorSliderChannel::Hue, 120.4), "120°");
    assert_eq!(format_channel_value(ColorSliderChannel::Alpha, 57.6), "58%");
    assert_eq!(format_channel_value(ColorSliderChannel::Red, 200.2), "200");
}

#[test]
fn resolve_state_and_class_name_track_markers() {
    let state = resolve_state(ColorSliderStateInput {
        disabled: false,
        channel: ColorSliderChannel::Alpha,
        value: 45.0,
        min: 0.0,
        max: 100.0,
        step: 1.0,
        show_value_label: true,
        has_custom_motion: true,
        has_custom_label: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
        has_custom_track: true,
    });

    assert_eq!(state.channel_attr, "alpha");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.track_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");

    let class = compose_class_name(Some("docs-custom".to_string()), state);
    assert!(class.contains("ui-color-slider"));
    assert!(class.contains("ui-color-slider--channel-alpha"));
    assert!(class.contains("ui-color-slider--motion-custom"));
    assert!(class.contains("ui-color-slider--track-custom"));
    assert!(class.contains("docs-custom"));
}

#[test]
fn accessibility_and_agent_contract_markers_are_closed_sets() {
    let accessibility = normalize_accessibility_state(Some(false), true);
    assert!(!accessibility.is_disabled);
    assert_eq!(accessibility.disabled_source_attr, "is_disabled");

    let accessibility = normalize_accessibility_state(None, true);
    assert!(accessibility.is_disabled);
    assert_eq!(accessibility.disabled_source_attr, "disabled");

    let contract = resolve_agent_contract(true);
    assert_eq!(contract.schema_attr, "ui.color-slider.agent-contract.v1");
    assert_eq!(contract.schema_version_attr, "1");
    assert_eq!(contract.stream_support_attr, "unsupported");
    assert_eq!(contract.stream_fallback_attr, "snapshot");
    assert_eq!(contract.stream_mode_attr, "snapshot");
    assert_eq!(contract.output_status_attr, "submittable");
    assert_eq!(contract.intent_attr, "adjust-color-channel");

    assert_eq!(resolve_ui_action(false, false).as_attr(), "idle");
    assert_eq!(resolve_ui_action(false, true).as_attr(), "focus");
    assert_eq!(resolve_ui_action(true, true).as_attr(), "press");
}
