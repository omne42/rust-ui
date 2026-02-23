use super::*;
use leptos::prelude::Callback;

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

#[test]
fn normalize_value_axis_reports_controlled_contract() {
    let axis = normalize_value_axis(
        Some(72.0),
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
    assert_eq!(axis.value, Some(72.0));
}

#[test]
fn normalize_value_axis_reports_uncontrolled_contract() {
    let axis = normalize_value_axis(None, Some(18.0), None);

    assert!(!axis.is_controlled);
    assert!(axis.has_custom_default_value);
    assert!(!axis.has_custom_on_value_change);
    assert_eq!(axis.mode_attr, "uncontrolled");
    assert_eq!(axis.value_source_attr, "default_value");
    assert_eq!(axis.default_value_source_attr, "provided");
    assert_eq!(axis.value_change_source_attr, "none");
    assert_eq!(axis.value, Some(18.0));
}

#[test]
fn normalize_max_uses_logic_default_source() {
    assert_eq!(normalize_max(None), DEFAULT_MAX);
    assert_eq!(normalize_max(Some(240.0)), 240.0);
}

#[test]
fn normalize_mode_maps_bool_to_typed_enum() {
    assert_eq!(normalize_mode(false), ProgressBarMode::Auto);
    assert_eq!(normalize_mode(true), ProgressBarMode::Indeterminate);
    assert!(!normalize_mode(false).is_indeterminate());
    assert!(normalize_mode(true).is_indeterminate());
}
