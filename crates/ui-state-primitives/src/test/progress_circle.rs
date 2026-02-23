use super::*;

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-progress-circle  ".to_string())),
        Some("docs-progress-circle".to_string())
    );
}

#[test]
fn resolve_aria_label_defaults_and_trims_values() {
    assert_eq!(resolve_aria_label(None), (DEFAULT_ARIA_LABEL.into(), false));
    assert_eq!(
        resolve_aria_label(Some("\n\t".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        resolve_aria_label(Some(" Progress ".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        resolve_aria_label(Some("  Syncing mailbox  ".to_string())),
        ("Syncing mailbox".to_string(), true)
    );
}

#[test]
fn resolve_value_label_reports_source() {
    assert_eq!(resolve_value_label(None), (None, false));
    assert_eq!(resolve_value_label(Some("  ".to_string())), (None, false));
    assert_eq!(
        resolve_value_label(Some("  42 percent  ".to_string())),
        (Some("42 percent".to_string()), true)
    );
}

#[test]
fn sanitize_dimension_rejects_invalid_values() {
    assert_eq!(sanitize_dimension(None, 24.0), (24.0, false));
    assert_eq!(sanitize_dimension(Some(0.0), 24.0), (24.0, false));
    assert_eq!(sanitize_dimension(Some(-1.0), 24.0), (24.0, false));
    assert_eq!(sanitize_dimension(Some(f64::NAN), 24.0), (24.0, false));
    assert_eq!(sanitize_dimension(Some(26.0), 24.0), (26.0, true));
}

#[test]
fn range_sanitizes_invalid_bounds() {
    assert_eq!(
        ProgressCircleRange::sanitized(10.0, 2.0),
        ProgressCircleRange { min: 0.0, max: 1.0 }
    );
    assert_eq!(
        ProgressCircleRange::sanitized(f64::NAN, f64::INFINITY),
        ProgressCircleRange { min: 0.0, max: 1.0 }
    );
}

#[test]
fn clamp_and_normalize_are_consistent() {
    let range = ProgressCircleRange::sanitized(0.0, 100.0);
    let value = clamp_to_range(25.0, range);
    assert_eq!(value, 25.0);
    assert!((normalize_progress(value, range) - 0.25).abs() < 1e-9);
}

#[test]
fn clamp_treats_non_finite_as_min() {
    let range = ProgressCircleRange::sanitized(10.0, 20.0);
    assert_eq!(clamp_to_range(f64::NAN, range), 10.0);
    assert_eq!(clamp_to_range(f64::INFINITY, range), 10.0);
    assert_eq!(clamp_to_range(f64::NEG_INFINITY, range), 10.0);
}

#[test]
fn resolve_metrics_sanitizes_inputs_and_tracks_sources() {
    let resolved = resolve_metrics(ProgressCircleMetricsInput {
        size_px: Some(f64::NAN),
        stroke_width_px: Some(-1.0),
    });

    assert_eq!(resolved.metrics.size_px, DEFAULT_SIZE_PX);
    assert_eq!(resolved.metrics.stroke_width_px, DEFAULT_STROKE_WIDTH_PX);
    assert!(resolved.metrics.radius_px > 0.0);
    assert!(resolved.metrics.circumference > 0.0);
    assert!(!resolved.has_custom_size);
    assert!(!resolved.has_custom_stroke_width);
}

#[test]
fn phase_mapping_is_stable() {
    assert_eq!(
        ProgressCirclePhase::Determinate.class_name(),
        "ui-progress-circle--state-determinate"
    );
    assert_eq!(
        ProgressCirclePhase::Indeterminate.class_name(),
        "ui-progress-circle--state-indeterminate"
    );
    assert_eq!(ProgressCirclePhase::Determinate.as_str(), "determinate");
    assert_eq!(ProgressCirclePhase::Indeterminate.as_str(), "indeterminate");
}

#[test]
fn resolve_state_tracks_source_contracts() {
    let state = resolve_state(ProgressCircleStateInput {
        has_custom_aria_label: true,
        has_custom_value_label: true,
        has_custom_size: true,
        has_custom_stroke_width: false,
        has_custom_motion: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.label_source_class, "ui-progress-circle--label-custom");
    assert_eq!(
        state.value_label_source_class,
        "ui-progress-circle--value-label-custom"
    );
    assert_eq!(state.size_source_class, "ui-progress-circle--size-custom");
    assert_eq!(
        state.stroke_source_class,
        "ui-progress-circle--stroke-default"
    );
    assert_eq!(
        state.motion_source_class,
        "ui-progress-circle--motion-custom"
    );
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.value_label_source_attr, "custom");
    assert_eq!(state.size_source_attr, "custom");
    assert_eq!(state.stroke_source_attr, "default");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(ProgressCircleStateInput {
            has_custom_aria_label: false,
            has_custom_value_label: true,
            has_custom_size: false,
            has_custom_stroke_width: true,
            has_custom_motion: true,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-progress-circle",
        "ui-progress-circle--value-label-custom",
        "ui-progress-circle--stroke-custom",
        "ui-progress-circle--motion-custom",
        "ui-progress-circle--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn normalize_mode_maps_to_typed_mode() {
    assert_eq!(normalize_mode(false), ProgressCircleMode::Auto);
    assert_eq!(normalize_mode(true), ProgressCircleMode::Indeterminate);
    assert_eq!(normalize_mode(false).as_str(), "auto");
    assert_eq!(normalize_mode(true).as_str(), "indeterminate");
}

#[test]
fn resolve_value_axis_tracks_controlled_contract() {
    let controlled = resolve_value_axis(ProgressCircleValueAxisInput {
        is_controlled: true,
        has_default_value: true,
        has_on_value_change: true,
    });
    assert!(controlled.is_controlled);
    assert!(controlled.has_default_value);
    assert!(controlled.has_on_value_change);
    assert_eq!(controlled.mode_attr, "controlled");
    assert_eq!(controlled.value_source_attr, "external");
    assert_eq!(controlled.default_value_source_attr, "provided");
    assert_eq!(controlled.value_change_source_attr, "provided");

    let uncontrolled = resolve_value_axis(ProgressCircleValueAxisInput {
        is_controlled: false,
        has_default_value: false,
        has_on_value_change: false,
    });
    assert!(!uncontrolled.is_controlled);
    assert!(!uncontrolled.has_default_value);
    assert!(!uncontrolled.has_on_value_change);
    assert_eq!(uncontrolled.mode_attr, "uncontrolled");
    assert_eq!(uncontrolled.value_source_attr, "default_value");
    assert_eq!(uncontrolled.default_value_source_attr, "default");
    assert_eq!(uncontrolled.value_change_source_attr, "none");
}
