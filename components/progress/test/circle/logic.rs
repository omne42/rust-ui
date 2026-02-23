use super::*;
use leptos::prelude::{Callback, signal};

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
fn normalize_value_axis_reports_controlled_contract() {
    let (controlled, _set_controlled) = signal(Some(36.0));
    let axis = normalize_value_axis(
        Some(controlled.into()),
        Some(8.0),
        Some(Callback::new(|_value: Option<f64>| {})),
    );

    assert!(axis.is_controlled);
    assert!(axis.has_custom_default_value);
    assert!(axis.has_custom_on_value_change);
    assert_eq!(axis.mode_attr, "controlled");
    assert_eq!(axis.value_source_attr, "external");
    assert_eq!(axis.default_value_source_attr, "provided");
    assert_eq!(axis.value_change_source_attr, "provided");
}

#[test]
fn normalize_value_axis_reports_uncontrolled_contract() {
    let axis = normalize_value_axis(None, None, None);

    assert!(!axis.is_controlled);
    assert!(!axis.has_custom_default_value);
    assert!(!axis.has_custom_on_value_change);
    assert_eq!(axis.mode_attr, "uncontrolled");
    assert_eq!(axis.value_source_attr, "default_value");
    assert_eq!(axis.default_value_source_attr, "default");
    assert_eq!(axis.value_change_source_attr, "none");
}

#[test]
fn normalize_range_uses_single_default_source() {
    let range = normalize_range(None, None);
    assert_eq!(
        range,
        ProgressCircleRange::sanitized(DEFAULT_MIN, DEFAULT_MAX)
    );

    let custom = normalize_range(Some(-20.0), Some(220.0));
    assert_eq!(custom, ProgressCircleRange::sanitized(-20.0, 220.0));
}

#[test]
fn normalize_progress_value_uses_logic_fallback() {
    assert_eq!(normalize_progress_value(Some(0.4)), 0.4);
    assert_eq!(normalize_progress_value(None), 0.0);
}

#[test]
fn normalize_mode_maps_bool_to_typed_enum() {
    assert_eq!(normalize_mode(false), ProgressCircleMode::Auto);
    assert_eq!(normalize_mode(true), ProgressCircleMode::Indeterminate);
}

#[test]
fn resolve_kernel_state_centralizes_phase_value_and_label_derivation() {
    let state = resolve_kernel_state(ProgressCircleKernelInput {
        clamped_value: Some(50.0),
        normalized_progress: Some(0.5),
        mode: ProgressCircleMode::Auto,
        value_label_override: Some("50 complete".to_string()),
    });

    assert_eq!(state.mode, ProgressCircleMode::Auto);
    assert!(!state.is_indeterminate);
    assert_eq!(state.phase, ProgressCirclePhase::Determinate);
    assert_eq!(state.progress_value, 0.5);
    assert_eq!(state.aria_value_now, Some(50.0));
    assert_eq!(state.value_label_text.as_deref(), Some("50 complete"));
}

#[test]
fn resolve_kernel_state_derives_indeterminate_from_prop_or_missing_value() {
    let from_prop = resolve_kernel_state(ProgressCircleKernelInput {
        clamped_value: Some(42.0),
        normalized_progress: Some(0.42),
        mode: ProgressCircleMode::Indeterminate,
        value_label_override: Some("ignored".to_string()),
    });
    assert!(from_prop.is_indeterminate);
    assert_eq!(from_prop.phase, ProgressCirclePhase::Indeterminate);
    assert_eq!(from_prop.value_label_text, None);

    let from_missing_value = resolve_kernel_state(ProgressCircleKernelInput {
        clamped_value: None,
        normalized_progress: None,
        mode: ProgressCircleMode::Auto,
        value_label_override: None,
    });
    assert!(from_missing_value.is_indeterminate);
    assert_eq!(from_missing_value.phase, ProgressCirclePhase::Indeterminate);
    assert_eq!(from_missing_value.progress_value, 0.0);
    assert_eq!(from_missing_value.aria_value_now, None);
    assert_eq!(from_missing_value.value_label_text, None);
}

#[test]
fn resolve_stroke_state_uses_typed_input_for_dash_contract() {
    let determinate = resolve_stroke_state(ProgressCircleStrokeInput {
        circumference: 120.0,
        is_indeterminate: false,
        animated_progress: 0.5,
    });
    assert_eq!(determinate.dasharray, "120");
    assert_eq!(determinate.dashoffset, "60");

    let indeterminate = resolve_stroke_state(ProgressCircleStrokeInput {
        circumference: 120.0,
        is_indeterminate: true,
        animated_progress: 0.9,
    });
    assert_eq!(indeterminate.dasharray, "30");
    assert_eq!(indeterminate.dashoffset, "90");
}

#[test]
fn resolve_kernel_state_centralizes_phase_and_label_derivation() {
    let state = resolve_kernel_state(ProgressCircleKernelInput {
        clamped_value: Some(64.0),
        normalized_progress: Some(0.64),
        mode: ProgressCircleMode::Auto,
        value_label_override: None,
    });

    assert_eq!(state.phase, ProgressCirclePhase::Determinate);
    assert!(!state.is_indeterminate);
    assert_eq!(state.progress_value, 0.64);
    assert_eq!(state.aria_value_now, Some(64.0));
    assert_eq!(state.value_label_text, Some("64%".to_string()));
}

#[test]
fn resolve_kernel_state_prefers_indeterminate_rules_over_value_label() {
    let state = resolve_kernel_state(ProgressCircleKernelInput {
        clamped_value: Some(72.0),
        normalized_progress: Some(0.72),
        mode: ProgressCircleMode::Indeterminate,
        value_label_override: Some("custom".to_string()),
    });

    assert_eq!(state.phase, ProgressCirclePhase::Indeterminate);
    assert!(state.is_indeterminate);
    assert_eq!(state.progress_value, 0.72);
    assert_eq!(state.aria_value_now, Some(72.0));
    assert_eq!(state.value_label_text, None);
}

#[test]
fn resolve_stroke_state_maps_phase_to_dash_contract() {
    let determinate = resolve_stroke_state(ProgressCircleStrokeInput {
        circumference: 100.0,
        is_indeterminate: false,
        animated_progress: 0.5,
    });
    assert_eq!(determinate.dasharray, "100");
    assert_eq!(determinate.dashoffset, "50");

    let indeterminate = resolve_stroke_state(ProgressCircleStrokeInput {
        circumference: 100.0,
        is_indeterminate: true,
        animated_progress: 0.9,
    });
    assert_eq!(indeterminate.dasharray, "25");
    assert_eq!(indeterminate.dashoffset, "75");
}
