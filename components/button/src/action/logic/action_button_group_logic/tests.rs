use super::*;

#[test]
fn density_class_names_are_stable() {
    assert_eq!(
        ActionButtonGroupDensity::Regular.class_name(),
        "ui-action-button-group--density-regular"
    );
    assert_eq!(ActionButtonGroupDensity::Regular.as_attr(), "regular");
    assert_eq!(
        ActionButtonGroupDensity::Compact.class_name(),
        "ui-action-button-group--density-compact"
    );
    assert_eq!(ActionButtonGroupDensity::Compact.as_attr(), "compact");
}

#[test]
fn orientation_attributes_match_variants() {
    assert_eq!(
        ActionButtonGroupOrientation::Horizontal.aria_orientation(),
        "horizontal"
    );
    assert_eq!(
        ActionButtonGroupOrientation::Horizontal.as_attr(),
        "horizontal"
    );
    assert_eq!(
        ActionButtonGroupOrientation::Vertical.aria_orientation(),
        "vertical"
    );
    assert_eq!(ActionButtonGroupOrientation::Vertical.as_attr(), "vertical");
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(
        normalize_optional_text(Some("  Group  ".to_string())),
        Some("Group".to_string())
    );
    assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
    assert_eq!(normalize_optional_text(None), None);
}

#[test]
fn normalize_aria_label_uses_trimmed_label_or_fallback() {
    let (label, explicit) = normalize_aria_label(Some("  Actions  ".to_string()));
    assert_eq!(label, "Actions");
    assert!(explicit);

    let (label, explicit) = normalize_aria_label(Some("   ".to_string()));
    assert_eq!(label, "Action button group");
    assert!(!explicit);

    let (label, explicit) = normalize_aria_label(None);
    assert_eq!(label, "Action button group");
    assert!(!explicit);
}

#[test]
fn resolve_state_tracks_orientation_density_and_flags() {
    let state = resolve_state(
        ActionButtonGroupOrientation::Vertical,
        ActionButtonGroupDensity::Compact,
        true,
        true,
        true,
        false,
        true,
    );

    assert_eq!(state.orientation_attr, "vertical");
    assert_eq!(state.density_attr, "compact");
    assert!(!state.is_horizontal);
    assert!(state.is_vertical);
    assert!(!state.is_regular);
    assert!(state.is_compact);
    assert!(state.is_justified);
    assert!(!state.is_not_justified);
    assert!(state.is_quiet);
    assert!(!state.is_filled);
    assert!(state.is_disabled);
    assert!(!state.is_enabled);
    assert!(!state.has_explicit_label);
    assert!(state.has_fallback_label);
    assert!(state.has_custom_class_name);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(
            ActionButtonGroupOrientation::Horizontal,
            ActionButtonGroupDensity::Compact,
            true,
            true,
            true,
            true,
            true,
        ),
    );

    for token in [
        "ui-action-button-group",
        "ui-action-button-group--horizontal",
        "ui-action-button-group--density-compact",
        "ui-action-button-group--justified",
        "ui-action-button-group--quiet",
        "ui-action-button-group--disabled",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
