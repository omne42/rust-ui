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
fn width_and_status_contract_are_stable() {
    assert_eq!(
        DateInputGroupWidth::Fit.class_name(),
        "ui-date-input-group--fit-width"
    );
    assert_eq!(
        DateInputGroupWidth::Full.class_name(),
        "ui-date-input-group--full-width"
    );
    assert_eq!(DateInputGroupWidth::Fit.as_attr(), "fit");
    assert_eq!(DateInputGroupWidth::Full.as_attr(), "full");
    assert!(!DateInputGroupWidth::Fit.is_full());
    assert!(DateInputGroupWidth::Full.is_full());

    assert_eq!(DateInputGroupStatus::Default.as_attr(), "default");
    assert_eq!(DateInputGroupStatus::Invalid.as_attr(), "invalid");
    assert_eq!(DateInputGroupStatus::Disabled.as_attr(), "disabled");
    assert_eq!(
        DateInputGroupStatus::DisabledInvalid.as_attr(),
        "disabled-invalid"
    );
    assert!(!DateInputGroupStatus::Default.is_disabled());
    assert!(DateInputGroupStatus::Disabled.is_disabled());
    assert!(DateInputGroupStatus::DisabledInvalid.is_disabled());
    assert!(!DateInputGroupStatus::Default.is_invalid());
    assert!(DateInputGroupStatus::Invalid.is_invalid());
    assert!(DateInputGroupStatus::DisabledInvalid.is_invalid());

    assert_eq!(resolve_width(false), DateInputGroupWidth::Fit);
    assert_eq!(resolve_width(true), DateInputGroupWidth::Full);
    assert_eq!(resolve_status(false, false), DateInputGroupStatus::Default);
    assert_eq!(resolve_status(false, true), DateInputGroupStatus::Invalid);
    assert_eq!(resolve_status(true, false), DateInputGroupStatus::Disabled);
    assert_eq!(
        resolve_status(true, true),
        DateInputGroupStatus::DisabledInvalid
    );
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
        width: DateInputGroupWidth::Full,
        status: DateInputGroupStatus::Invalid,
        segmented: true,
        has_prefix: true,
        has_suffix: false,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.variant_attr, "secondary");
    assert_eq!(state.width_attr, "full");
    assert_eq!(state.data_state_attr, "invalid");
    assert_eq!(state.width, DateInputGroupWidth::Full);
    assert_eq!(state.status, DateInputGroupStatus::Invalid);
    assert!(state.is_full_width);
    assert!(state.is_invalid);
    assert!(state.is_segmented);
    assert!(state.has_prefix);
    assert!(!state.has_suffix);
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn resolve_state_prefers_disabled_invalid_state() {
    let state = resolve_state(DateInputGroupStateInput {
        variant: DateInputGroupVariant::Primary,
        width: DateInputGroupWidth::Fit,
        status: DateInputGroupStatus::DisabledInvalid,
        segmented: true,
        has_prefix: false,
        has_suffix: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });

    assert_eq!(state.data_state_attr, "disabled-invalid");
}
