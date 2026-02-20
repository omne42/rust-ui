use super::*;

#[test]
fn phase_contract_is_stable() {
    assert_eq!(
        SliderPhase::Enabled.class_name(),
        "ui-slider--state-enabled"
    );
    assert_eq!(SliderPhase::Enabled.as_attr(), "enabled");
    assert_eq!(
        SliderPhase::Disabled.class_name(),
        "ui-slider--state-disabled"
    );
    assert_eq!(SliderPhase::Disabled.as_attr(), "disabled");
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-slider ".to_string())),
        Some("docs-slider".to_string())
    );
}

#[test]
fn resolve_label_falls_back_to_default_for_empty_text() {
    assert_eq!(
        resolve_label("  ".to_string()),
        (DEFAULT_LABEL.into(), false)
    );
    assert_eq!(
        resolve_label(" Volume ".to_string()),
        ("Volume".to_string(), true)
    );
}

#[test]
fn sanitize_bounds_swaps_or_falls_back_for_invalid_range() {
    assert_eq!(sanitize_bounds(10.0, 0.0), (0.0, 10.0));
    assert_eq!(sanitize_bounds(f64::NAN, f64::INFINITY), (0.0, 100.0));
    assert_eq!(sanitize_bounds(5.0, 5.0), (0.0, 100.0));
}

#[test]
fn sanitize_step_and_value_align_to_range() {
    assert_eq!(sanitize_step(0.0, 0.0, 10.0), 1.0);
    assert_eq!(sanitize_step(200.0, 0.0, 10.0), 10.0);
    assert_eq!(sanitize_value(11.2, 0.0, 10.0, 0.5), 10.0);
    assert_eq!(sanitize_value(0.24, 0.0, 10.0, 0.5), 0.0);
    assert_eq!(sanitize_value(0.26, 0.0, 10.0, 0.5), 0.5);
    assert_eq!(sanitize_value(f64::NAN, 0.0, 10.0, 1.0), 0.0);
}

#[test]
fn parse_value_reads_trimmed_numbers() {
    assert_eq!(parse_value(" 42.5 "), Some(42.5));
    assert_eq!(parse_value(""), None);
    assert_eq!(parse_value("not-a-number"), None);
}

#[test]
fn resolve_percent_is_clamped() {
    assert_eq!(resolve_percent(50.0, 0.0, 100.0), 50.0);
    assert_eq!(resolve_percent(-1.0, 0.0, 100.0), 0.0);
    assert_eq!(resolve_percent(101.0, 0.0, 100.0), 100.0);
}

#[test]
fn resolve_state_tracks_source_markers() {
    let state = resolve_state(SliderStateInput {
        value: 88.0,
        min: 0.0,
        max: 120.0,
        step: 2.0,
        is_disabled: true,
        has_custom_motion: true,
        has_custom_class_name: true,
        has_custom_label: true,
    });

    assert_eq!(state.phase_class, "ui-slider--state-disabled");
    assert_eq!(state.phase_attr, "disabled");
    assert!(state.is_disabled);
    assert!(!state.is_enabled);
    assert_eq!(state.motion_source_class, "ui-slider--motion-custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.label_source_class, "ui-slider--label-custom");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let state = resolve_state(SliderStateInput {
        value: 45.0,
        min: 0.0,
        max: 100.0,
        step: 1.0,
        is_disabled: false,
        has_custom_motion: false,
        has_custom_class_name: true,
        has_custom_label: false,
    });
    let class_name = compose_class_name(Some("docs-slider".to_string()), state);

    for token in [
        "ui-slider",
        "ui-slider--state-enabled",
        "ui-slider--motion-default",
        "ui-slider--label-default",
        "ui-slider--custom-class",
        "docs-slider",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
