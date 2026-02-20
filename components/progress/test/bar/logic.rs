use super::*;

#[test]
fn variant_and_size_mappings_are_stable() {
    assert_eq!(
        ProgressBarVariant::Default.class_name(),
        "ui-progress-bar--variant-default"
    );
    assert_eq!(
        ProgressBarVariant::Accent.class_name(),
        "ui-progress-bar--variant-accent"
    );
    assert_eq!(
        ProgressBarVariant::Danger.class_name(),
        "ui-progress-bar--variant-danger"
    );

    assert_eq!(ProgressBarVariant::Default.as_str(), "default");
    assert_eq!(ProgressBarVariant::Accent.as_str(), "accent");
    assert_eq!(ProgressBarVariant::Danger.as_str(), "danger");

    assert_eq!(ProgressBarSize::Sm.class_name(), "ui-progress-bar--size-sm");
    assert_eq!(ProgressBarSize::Md.class_name(), "ui-progress-bar--size-md");
    assert_eq!(ProgressBarSize::Lg.class_name(), "ui-progress-bar--size-lg");

    assert_eq!(ProgressBarSize::Sm.as_str(), "sm");
    assert_eq!(ProgressBarSize::Md.as_str(), "md");
    assert_eq!(ProgressBarSize::Lg.as_str(), "lg");
}

#[test]
fn phase_mappings_are_stable() {
    assert_eq!(
        ProgressBarPhase::Determinate.class_name(),
        "ui-progress-bar--state-determinate"
    );
    assert_eq!(
        ProgressBarPhase::Indeterminate.class_name(),
        "ui-progress-bar--state-indeterminate"
    );
    assert_eq!(ProgressBarPhase::Determinate.as_str(), "determinate");
    assert_eq!(ProgressBarPhase::Indeterminate.as_str(), "indeterminate");
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-progress-bar  ".to_string())),
        Some("docs-progress-bar".to_string())
    );
}

#[test]
fn resolve_aria_label_defaults_and_detects_custom_source() {
    assert_eq!(
        resolve_aria_label("\n\t".to_string()),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        resolve_aria_label("Progress".to_string()),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        resolve_aria_label("  Upload progress ".to_string()),
        ("Upload progress".to_string(), true)
    );
}

#[test]
fn sanitize_numeric_inputs() {
    assert_eq!(sanitize_max(DEFAULT_MAX), DEFAULT_MAX);
    assert_eq!(sanitize_max(0.0), MIN_MAX);
    assert_eq!(sanitize_max(f64::NAN), MIN_MAX);

    assert_eq!(sanitize_value(Some(12.0), 100.0), Some(12.0));
    assert_eq!(sanitize_value(Some(-5.0), 100.0), Some(0.0));
    assert_eq!(sanitize_value(Some(120.0), 100.0), Some(100.0));
    assert_eq!(sanitize_value(Some(f64::NAN), 100.0), None);
}

#[test]
fn resolve_state_tracks_phase_and_source_contracts() {
    let state = resolve_state(ProgressBarStateInput {
        variant: ProgressBarVariant::Accent,
        size: ProgressBarSize::Lg,
        value: Some(42.0),
        max: 100.0,
        indeterminate: false,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.variant_attr, "accent");
    assert_eq!(state.size_attr, "lg");
    assert_eq!(state.phase_attr, "determinate");
    assert_eq!(state.value, Some(42.0));
    assert_eq!(state.max, 100.0);
    assert!(state.is_determinate);
    assert!(!state.is_indeterminate);
    assert_eq!(state.label_source_class, "ui-progress-bar--label-custom");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn resolve_state_marks_indeterminate_when_value_missing() {
    let state = resolve_state(ProgressBarStateInput {
        variant: ProgressBarVariant::Default,
        size: ProgressBarSize::Md,
        value: None,
        max: 100.0,
        indeterminate: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });

    assert_eq!(state.phase, ProgressBarPhase::Indeterminate);
    assert!(!state.has_value);
    assert!(state.is_indeterminate);
    assert!(!state.is_determinate);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(ProgressBarStateInput {
            variant: ProgressBarVariant::Danger,
            size: ProgressBarSize::Sm,
            value: Some(5.0),
            max: 10.0,
            indeterminate: false,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-progress-bar",
        "ui-progress-bar--variant-danger",
        "ui-progress-bar--size-sm",
        "ui-progress-bar--state-determinate",
        "ui-progress-bar--label-custom",
        "ui-progress-bar--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
