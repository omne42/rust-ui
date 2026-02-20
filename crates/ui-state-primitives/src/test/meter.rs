use super::*;

#[test]
fn variant_and_size_mappings_are_stable() {
    assert_eq!(
        MeterVariant::Default.class_name(),
        "ui-meter--variant-default"
    );
    assert_eq!(
        MeterVariant::Danger.class_name(),
        "ui-meter--variant-danger"
    );
    assert_eq!(MeterVariant::Default.as_str(), "default");
    assert_eq!(MeterVariant::Danger.as_str(), "danger");

    assert_eq!(MeterSize::Sm.class_name(), "ui-meter--size-sm");
    assert_eq!(MeterSize::Default.class_name(), "ui-meter--size-default");
    assert_eq!(MeterSize::Lg.class_name(), "ui-meter--size-lg");

    assert_eq!(MeterSize::Sm.as_str(), "sm");
    assert_eq!(MeterSize::Default.as_str(), "default");
    assert_eq!(MeterSize::Lg.as_str(), "lg");
}

#[test]
fn range_sanitizes_invalid_bounds() {
    assert_eq!(
        MeterRange::sanitized(10.0, 2.0),
        MeterRange {
            min: 0.0,
            max: 100.0
        }
    );
    assert_eq!(
        MeterRange::sanitized(f64::NAN, f64::INFINITY),
        MeterRange {
            min: 0.0,
            max: 100.0
        }
    );
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-meter  ".to_string())),
        Some("docs-meter".to_string())
    );
}

#[test]
fn resolve_aria_label_defaults_and_detects_custom_source() {
    assert_eq!(
        resolve_aria_label(None, None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        resolve_aria_label(Some("\n\t".to_string()), None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        resolve_aria_label(None, Some("  Completion  ".to_string())),
        ("Completion".to_string(), true)
    );
    assert_eq!(
        resolve_aria_label(Some("  Meter  ".to_string()), Some("  Label  ".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        resolve_aria_label(
            Some("  Upload progress  ".to_string()),
            Some("  Label  ".to_string())
        ),
        ("Upload progress".to_string(), true)
    );
}

#[test]
fn resolve_value_label_reports_source() {
    assert_eq!(resolve_value_label(None), (None, false));
    assert_eq!(resolve_value_label(Some("\n\t".to_string())), (None, false));
    assert_eq!(
        resolve_value_label(Some("  72 complete  ".to_string())),
        (Some("72 complete".to_string()), true)
    );
}

#[test]
fn clamp_and_normalize_are_consistent() {
    let range = MeterRange::sanitized(0.0, 100.0);
    let value = clamp_to_range(25.0, range);
    assert_eq!(value, 25.0);
    assert!((normalize_progress(value, range) - 0.25).abs() < 1e-9);
}

#[test]
fn clamp_treats_non_finite_as_min() {
    let range = MeterRange::sanitized(10.0, 20.0);
    assert_eq!(clamp_to_range(f64::NAN, range), 10.0);
    assert_eq!(clamp_to_range(f64::INFINITY, range), 10.0);
    assert_eq!(clamp_to_range(f64::NEG_INFINITY, range), 10.0);
}

#[test]
fn phase_mappings_are_stable() {
    assert_eq!(
        MeterPhase::Determinate.class_name(),
        "ui-meter--state-determinate"
    );
    assert_eq!(
        MeterPhase::Indeterminate.class_name(),
        "ui-meter--state-indeterminate"
    );
    assert_eq!(MeterPhase::Determinate.as_str(), "determinate");
    assert_eq!(MeterPhase::Indeterminate.as_str(), "indeterminate");
}

#[test]
fn resolve_state_tracks_source_contracts() {
    let state = resolve_state(MeterStateInput {
        variant: MeterVariant::Danger,
        size: MeterSize::Lg,
        has_custom_aria_label: true,
        has_custom_value_label: false,
        has_custom_motion: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.variant_attr, "danger");
    assert_eq!(state.size_attr, "lg");
    assert_eq!(state.label_source_class, "ui-meter--label-custom");
    assert_eq!(state.value_label_source_class, "ui-meter--value-label-auto");
    assert_eq!(state.motion_source_class, "ui-meter--motion-custom");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.value_label_source_attr, "auto");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-meter-custom".to_string()),
        resolve_state(MeterStateInput {
            variant: MeterVariant::Default,
            size: MeterSize::Sm,
            has_custom_aria_label: false,
            has_custom_value_label: true,
            has_custom_motion: true,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-meter",
        "ui-meter--size-sm",
        "ui-meter--value-label-custom",
        "ui-meter--motion-custom",
        "ui-meter--custom-class",
        "docs-meter-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
