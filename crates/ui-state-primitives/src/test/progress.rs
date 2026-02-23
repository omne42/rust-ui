use super::*;

#[test]
fn range_sanitizes_invalid_bounds() {
    assert_eq!(
        ProgressRange::sanitized(10.0, 2.0),
        ProgressRange { min: 0.0, max: 1.0 }
    );
    assert_eq!(
        ProgressRange::sanitized(f64::NAN, f64::INFINITY),
        ProgressRange { min: 0.0, max: 1.0 }
    );
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-progress  ".to_string())),
        Some("docs-progress".to_string())
    );
}

#[test]
fn resolve_aria_label_defaults_and_detects_custom_source() {
    assert_eq!(resolve_aria_label(None), (DEFAULT_ARIA_LABEL.into(), false));
    assert_eq!(
        resolve_aria_label(Some("\n\t".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        resolve_aria_label(Some("  Progress  ".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        resolve_aria_label(Some("  Uploading dataset  ".to_string())),
        ("Uploading dataset".to_string(), true)
    );
}

#[test]
fn resolve_value_label_reports_source() {
    assert_eq!(resolve_value_label(None), (None, false));
    assert_eq!(resolve_value_label(Some("  ".to_string())), (None, false));
    assert_eq!(
        resolve_value_label(Some("  72 complete  ".to_string())),
        (Some("72 complete".to_string()), true)
    );
}

#[test]
fn clamp_and_normalize_are_consistent() {
    let range = ProgressRange::sanitized(0.0, 100.0);
    let value = clamp_to_range(25.0, range);
    assert_eq!(value, 25.0);
    assert!((normalize_progress(value, range) - 0.25).abs() < 1e-9);
}

#[test]
fn clamp_treats_non_finite_as_min() {
    let range = ProgressRange::sanitized(10.0, 20.0);
    assert_eq!(clamp_to_range(f64::NAN, range), 10.0);
    assert_eq!(clamp_to_range(f64::INFINITY, range), 10.0);
    assert_eq!(clamp_to_range(f64::NEG_INFINITY, range), 10.0);
}

#[test]
fn phase_mapping_is_stable() {
    assert_eq!(
        ProgressPhase::Determinate.class_name(),
        "ui-progress--state-determinate"
    );
    assert_eq!(
        ProgressPhase::Indeterminate.class_name(),
        "ui-progress--state-indeterminate"
    );
    assert_eq!(ProgressPhase::Determinate.as_str(), "determinate");
    assert_eq!(ProgressPhase::Indeterminate.as_str(), "indeterminate");
}

#[test]
fn resolve_state_tracks_source_contracts() {
    let state = resolve_state(ProgressStateInput {
        has_custom_aria_label: true,
        has_custom_value_label: false,
        has_custom_motion: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.label_source_class, "ui-progress--label-custom");
    assert_eq!(
        state.value_label_source_class,
        "ui-progress--value-label-auto"
    );
    assert_eq!(state.motion_source_class, "ui-progress--motion-custom");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.value_label_source_attr, "auto");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(ProgressStateInput {
            has_custom_aria_label: false,
            has_custom_value_label: true,
            has_custom_motion: true,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-progress",
        "ui-progress--value-label-custom",
        "ui-progress--motion-custom",
        "ui-progress--custom-class",
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
    assert_eq!(normalize_mode(false), ProgressMode::Auto);
    assert_eq!(normalize_mode(true), ProgressMode::Indeterminate);
    assert_eq!(normalize_mode(false).as_str(), "auto");
    assert_eq!(normalize_mode(true).as_str(), "indeterminate");
}

#[test]
fn resolve_value_axis_tracks_controlled_contract() {
    let controlled = resolve_value_axis(ProgressValueAxisInput {
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

    let uncontrolled = resolve_value_axis(ProgressValueAxisInput {
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
