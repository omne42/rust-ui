use super::*;

#[test]
fn normalize_optional_text_trims_and_drops_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-infield-button  ".to_string())),
        Some("docs-infield-button".to_string())
    );
}

#[test]
fn normalize_aria_label_uses_fallback_when_missing() {
    let (label, custom) = normalize_aria_label(Some("  Open picker  ".to_string()));
    assert_eq!(label, "Open picker");
    assert!(custom);

    let (label, custom) = normalize_aria_label(None);
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom);
}

#[test]
fn resolve_state_tracks_quiet_invalid_disabled_sources() {
    let state = resolve_state(InfieldButtonStateInput {
        quiet: true,
        invalid: true,
        disabled: false,
        forced_active: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
        has_custom_press_handler: true,
    });

    assert!(state.is_quiet);
    assert!(state.is_invalid);
    assert!(!state.is_disabled);
    assert!(state.is_forced_active);
    assert_eq!(state.quiet_attr, "true");
    assert_eq!(state.invalid_attr, "true");
    assert_eq!(state.disabled_attr, "false");
    assert_eq!(state.active_mode_attr, "forced");
    assert_eq!(state.data_state_attr, "invalid");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-infield-button-custom".to_string()),
        resolve_state(InfieldButtonStateInput {
            quiet: true,
            invalid: true,
            disabled: true,
            forced_active: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            has_custom_press_handler: true,
        }),
    );

    for expected in [
        "ui-infield-button",
        "ui-infield-button--quiet",
        "ui-infield-button--invalid",
        "ui-infield-button--disabled",
        "ui-infield-button--active",
        "ui-infield-button--custom-handler",
        "ui-infield-button--custom-aria-label",
        "ui-infield-button--custom-class",
        "docs-infield-button-custom",
    ] {
        assert!(
            class_name.contains(expected),
            "expected composed class name to contain `{expected}`"
        );
    }
}
