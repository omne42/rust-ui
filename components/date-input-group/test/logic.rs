use super::*;

#[test]
fn variant_contract_is_stable() {
    assert_eq!(
        DateInputGroupVariant::Primary.class_name(),
        "ui-date-input-group--variant-primary"
    );
    assert_eq!(
        DateInputGroupVariant::Secondary.class_name(),
        "ui-date-input-group--variant-secondary"
    );

    assert_eq!(DateInputGroupVariant::Primary.as_attr(), "primary");
    assert_eq!(DateInputGroupVariant::Secondary.as_attr(), "secondary");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Booking controls  ".to_string())),
        Some("Booking controls".to_string())
    );

    assert_eq!(
        normalize_aria_label(Some("  Date segments  ".to_string())),
        ("Date segments".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(Some("  ".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn resolve_state_tracks_markers() {
    let state = resolve_state(DateInputGroupStateInput {
        variant: DateInputGroupVariant::Secondary,
        full_width: true,
        disabled: false,
        invalid: true,
        segmented: true,
        has_prefix: true,
        has_suffix: false,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.variant_attr, "secondary");
    assert_eq!(state.width_attr, "full");
    assert_eq!(state.data_state_attr, "invalid");
    assert!(state.is_full_width);
    assert!(state.is_invalid);
    assert!(state.is_segmented);
    assert!(state.has_prefix);
    assert!(!state.has_suffix);
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let state = resolve_state(DateInputGroupStateInput {
        variant: DateInputGroupVariant::Primary,
        full_width: false,
        disabled: true,
        invalid: false,
        segmented: true,
        has_prefix: true,
        has_suffix: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-date-input-group".to_string()), state);

    for token in [
        "ui-date-input-group",
        "ui-date-input-group--variant-primary",
        "ui-date-input-group--fit-width",
        "ui-date-input-group--disabled",
        "ui-date-input-group--segmented",
        "ui-date-input-group--has-prefix",
        "ui-date-input-group--has-suffix",
        "ui-date-input-group--custom-class",
        "docs-date-input-group",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
