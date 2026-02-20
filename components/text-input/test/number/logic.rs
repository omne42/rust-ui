use super::*;

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-static-number  ".to_string())),
        Some("docs-static-number".to_string())
    );
}

#[test]
fn resolve_decimal_separator_defaults_and_detects_custom_source() {
    assert_eq!(
        resolve_decimal_separator(None),
        (DEFAULT_DECIMAL_SEPARATOR.into(), false)
    );
    assert_eq!(
        resolve_decimal_separator(Some(" . ".to_string())),
        (DEFAULT_DECIMAL_SEPARATOR.into(), false)
    );
    assert_eq!(
        resolve_decimal_separator(Some(" , ".to_string())),
        (",".to_string(), true)
    );
}

#[test]
fn resolve_thousand_separator_reports_source() {
    assert_eq!(resolve_thousand_separator(None), (None, false));
    assert_eq!(
        resolve_thousand_separator(Some("\n\t".to_string())),
        (None, false)
    );
    assert_eq!(
        resolve_thousand_separator(Some(" , ".to_string())),
        (Some(",".to_string()), true)
    );
}

#[test]
fn sanitize_decimal_places_caps_at_twelve() {
    assert_eq!(sanitize_decimal_places(None), None);
    assert_eq!(sanitize_decimal_places(Some(2)), Some(2));
    assert_eq!(sanitize_decimal_places(Some(30)), Some(12));
}

#[test]
fn sanitize_number_handles_non_finite_values() {
    assert_eq!(sanitize_number(42.0), 42.0);
    assert_eq!(sanitize_number(f64::NAN), 0.0);
    assert_eq!(sanitize_number(f64::INFINITY), 0.0);
}

#[test]
fn resolve_sign_maps_sign_variants() {
    assert_eq!(resolve_sign(-1.0), NumberSign::Negative);
    assert_eq!(resolve_sign(0.0), NumberSign::Zero);
    assert_eq!(resolve_sign(1.0), NumberSign::Positive);
    assert_eq!(resolve_sign(f64::NAN), NumberSign::Zero);
}

#[test]
fn resolve_static_state_tracks_source_contracts() {
    let state = resolve_static_number_state(StaticNumberStateInput {
        value: -12.3,
        has_custom_decimal_separator: true,
        has_custom_decimal_places: false,
        has_custom_thousand_separator: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.sign, NumberSign::Negative);
    assert_eq!(state.sign_class, "ui-static-number--sign-negative");
    assert_eq!(state.sign_attr, "negative");
    assert_eq!(
        state.decimal_separator_source_class,
        "ui-static-number--decimal-separator-custom"
    );
    assert_eq!(
        state.decimal_places_source_class,
        "ui-static-number--decimal-places-auto"
    );
    assert_eq!(
        state.thousand_separator_source_class,
        "ui-static-number--thousand-separator-custom"
    );
    assert_eq!(state.decimal_separator_source_attr, "custom");
    assert_eq!(state.decimal_places_source_attr, "auto");
    assert_eq!(state.thousand_separator_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_static_class_name_includes_state_markers() {
    let class_name = compose_static_number_class_name(
        Some("docs-static-number-custom".to_string()),
        resolve_static_number_state(StaticNumberStateInput {
            value: 0.0,
            has_custom_decimal_separator: false,
            has_custom_decimal_places: true,
            has_custom_thousand_separator: true,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-static-number",
        "ui-static-number--sign-zero",
        "ui-static-number--decimal-separator-default",
        "ui-static-number--decimal-places-custom",
        "ui-static-number--thousand-separator-custom",
        "ui-static-number--custom-class",
        "docs-static-number-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn sliding_phase_mappings_are_stable() {
    assert_eq!(
        SlidingNumberPhase::Animated.class_name(),
        "ui-sliding-number--state-animated"
    );
    assert_eq!(
        SlidingNumberPhase::Static.class_name(),
        "ui-sliding-number--state-static"
    );
    assert_eq!(SlidingNumberPhase::Animated.as_str(), "animated");
    assert_eq!(SlidingNumberPhase::Static.as_str(), "static");
}

#[test]
fn resolve_sliding_state_tracks_source_contracts() {
    let state = resolve_sliding_number_state(SlidingNumberStateInput {
        value: -42.0,
        animate: true,
        has_custom_decimal_separator: true,
        has_custom_decimal_places: false,
        has_custom_thousand_separator: true,
        has_custom_motion: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.sign, NumberSign::Negative);
    assert_eq!(state.phase, SlidingNumberPhase::Animated);
    assert_eq!(state.sign_class, "ui-sliding-number--sign-negative");
    assert_eq!(state.phase_class, "ui-sliding-number--state-animated");
    assert_eq!(state.sign_attr, "negative");
    assert_eq!(state.phase_attr, "animated");
    assert!(state.is_animated);
    assert!(!state.is_static);
    assert_eq!(
        state.decimal_separator_source_class,
        "ui-sliding-number--decimal-separator-custom"
    );
    assert_eq!(
        state.decimal_places_source_class,
        "ui-sliding-number--decimal-places-auto"
    );
    assert_eq!(
        state.thousand_separator_source_class,
        "ui-sliding-number--thousand-separator-custom"
    );
    assert_eq!(
        state.motion_source_class,
        "ui-sliding-number--motion-custom"
    );
    assert_eq!(state.decimal_separator_source_attr, "custom");
    assert_eq!(state.decimal_places_source_attr, "auto");
    assert_eq!(state.thousand_separator_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_sliding_class_name_includes_state_markers() {
    let class_name = compose_sliding_number_class_name(
        Some("docs-sliding-number-custom".to_string()),
        resolve_sliding_number_state(SlidingNumberStateInput {
            value: 3.5,
            animate: false,
            has_custom_decimal_separator: true,
            has_custom_decimal_places: true,
            has_custom_thousand_separator: false,
            has_custom_motion: true,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-sliding-number",
        "ui-sliding-number--sign-positive",
        "ui-sliding-number--state-static",
        "ui-sliding-number--decimal-separator-custom",
        "ui-sliding-number--decimal-places-custom",
        "ui-sliding-number--thousand-separator-none",
        "ui-sliding-number--motion-custom",
        "ui-sliding-number--custom-class",
        "docs-sliding-number-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn formats_negative_and_decimals() {
    let out = format_static_number(
        -12.345,
        NumberFormatOptions {
            decimal_places: Some(2),
            ..Default::default()
        },
    );
    assert_eq!(out, "-12.35");
}

#[test]
fn supports_thousand_separator() {
    let out = format_static_number(
        12345.0,
        NumberFormatOptions {
            thousand_separator: Some(","),
            ..Default::default()
        },
    );
    assert_eq!(out, "12,345");
}

#[test]
fn supports_custom_decimal_separator() {
    let out = format_static_number(
        1.5,
        NumberFormatOptions {
            decimal_separator: ",",
            ..Default::default()
        },
    );
    assert_eq!(out, "1,5");
}
