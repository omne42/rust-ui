use super::*;
use leptos::prelude::{Callback, signal};

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
fn normalize_value_axis_reports_controlled_contract() {
    let (controlled, _set_controlled) = signal(Some(42.0));
    let axis = normalize_value_axis(
        Some(controlled.into()),
        Some(12.0),
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
    assert_eq!(range, ProgressRange::sanitized(DEFAULT_MIN, DEFAULT_MAX));

    let custom = normalize_range(Some(-10.0), Some(200.0));
    assert_eq!(custom, ProgressRange::sanitized(-10.0, 200.0));
}

#[test]
fn normalize_progress_value_uses_logic_fallback() {
    assert_eq!(normalize_progress_value(Some(0.25)), 0.25);
    assert_eq!(normalize_progress_value(None), 0.0);
}

#[test]
fn normalize_mode_maps_bool_to_typed_enum() {
    assert_eq!(normalize_mode(false), ProgressMode::Auto);
    assert_eq!(normalize_mode(true), ProgressMode::Indeterminate);
}

#[test]
fn resolve_render_state_centralizes_phase_value_and_label_derivation() {
    let state = resolve_render_state(ProgressRenderInput {
        clamped_value: Some(25.0),
        normalized_progress: Some(0.25),
        mode: ProgressMode::Auto,
        value_label_override: Some("25 complete".to_string()),
    });

    assert_eq!(state.mode, ProgressMode::Auto);
    assert!(!state.is_indeterminate);
    assert_eq!(state.phase, ProgressPhase::Determinate);
    assert_eq!(state.progress_value, 0.25);
    assert_eq!(state.aria_value_now, Some(25.0));
    assert_eq!(state.value_label_text.as_deref(), Some("25 complete"));
}

#[test]
fn resolve_render_state_derives_indeterminate_from_prop_or_missing_value() {
    let from_prop = resolve_render_state(ProgressRenderInput {
        clamped_value: Some(42.0),
        normalized_progress: Some(0.42),
        mode: ProgressMode::Indeterminate,
        value_label_override: Some("ignored".to_string()),
    });
    assert!(from_prop.is_indeterminate);
    assert_eq!(from_prop.phase, ProgressPhase::Indeterminate);
    assert_eq!(from_prop.value_label_text, None);

    let from_missing_value = resolve_render_state(ProgressRenderInput {
        clamped_value: None,
        normalized_progress: None,
        mode: ProgressMode::Auto,
        value_label_override: None,
    });
    assert!(from_missing_value.is_indeterminate);
    assert_eq!(from_missing_value.phase, ProgressPhase::Indeterminate);
    assert_eq!(from_missing_value.progress_value, 0.0);
    assert_eq!(from_missing_value.aria_value_now, None);
    assert_eq!(from_missing_value.value_label_text, None);
}

#[test]
fn resolve_render_state_centralizes_phase_and_label_derivation() {
    let state = resolve_render_state(ProgressRenderInput {
        clamped_value: Some(25.0),
        normalized_progress: Some(0.25),
        mode: ProgressMode::Auto,
        value_label_override: None,
    });

    assert_eq!(state.phase, ProgressPhase::Determinate);
    assert!(!state.is_indeterminate);
    assert_eq!(state.progress_value, 0.25);
    assert_eq!(state.aria_value_now, Some(25.0));
    assert_eq!(state.value_label_text, Some("25%".to_string()));
}

#[test]
fn resolve_render_state_prefers_indeterminate_and_custom_label_rules() {
    let state = resolve_render_state(ProgressRenderInput {
        clamped_value: Some(40.0),
        normalized_progress: Some(0.4),
        mode: ProgressMode::Indeterminate,
        value_label_override: Some("custom".to_string()),
    });

    assert_eq!(state.phase, ProgressPhase::Indeterminate);
    assert!(state.is_indeterminate);
    assert_eq!(state.progress_value, 0.4);
    assert_eq!(state.aria_value_now, Some(40.0));
    assert_eq!(state.value_label_text, None);
}
