use super::*;

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
}

#[test]
fn inline_style_composition_is_stable() {
    let style = compose_inline_style(Some("#000"), Some("#fff"))
        .expect("custom track colors should produce inline css variables");
    assert_eq!(
        style,
        "--ui-color-slider-track-start: #000; --ui-color-slider-track-end: #fff;"
    );
    assert!(
        !style.contains("background:")
            && !style.contains("display:")
            && !style.contains("position:")
            && !style.contains("color:"),
        "inline style contract should only carry custom properties, not business style rules.",
    );
    assert_eq!(compose_inline_style(None, None), None);
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

#[test]
fn source_attrs_are_derived_in_logic_layer() {
    let attrs = resolve_source_attrs(ColorSliderInputPresence {
        has_external_value: true,
        has_default_value: true,
        has_value_change_handler: true,
    });
    assert_eq!(attrs.control_mode_attr, "controlled");
    assert_eq!(attrs.value_source_attr, "external");
    assert_eq!(attrs.value_change_source_attr, "on_value_change");
    assert_eq!(attrs.default_value_source_attr, "custom");

    let attrs = resolve_source_attrs(ColorSliderInputPresence {
        has_external_value: false,
        has_default_value: false,
        has_value_change_handler: false,
    });
    assert_eq!(attrs.control_mode_attr, "uncontrolled");
    assert_eq!(attrs.value_source_attr, "default_value");
    assert_eq!(attrs.value_change_source_attr, "none");
    assert_eq!(attrs.default_value_source_attr, "default");
}

#[test]
fn state_primitive_calls_remain_available() {
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
    assert_eq!(state.track_source_attr, "custom");

    let class = compose_class_name(Some("docs-custom".to_string()), state);
    assert!(class.contains("ui-color-slider--channel-alpha"));
}

#[test]
fn default_value_normalization_stays_in_logic_layer() {
    let resolved = normalize_default_value(ColorSliderChannel::Hue, None, 0.0, 360.0, 1.0);
    assert_eq!(resolved, ColorSliderChannel::Hue.default_value());

    let resolved = normalize_default_value(ColorSliderChannel::Hue, Some(400.2), 0.0, 360.0, 1.0);
    assert_eq!(resolved, 360.0);
}
