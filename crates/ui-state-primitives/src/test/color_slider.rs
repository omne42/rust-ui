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
fn track_color_and_formatting_are_stable() {
    assert_eq!(
        sanitize_track_color(Some(" #09f ".to_string())),
        Some("#09f".to_string())
    );
    assert_eq!(
        sanitize_track_color(Some("javascript:alert(1)".to_string())),
        None
    );

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
