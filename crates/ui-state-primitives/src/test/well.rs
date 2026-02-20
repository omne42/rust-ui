use super::*;

#[test]
fn tone_class_names_and_attrs_are_stable() {
    assert_eq!(WellTone::Default.class_name(), "ui-well--tone-default");
    assert_eq!(WellTone::Quiet.class_name(), "ui-well--tone-quiet");
    assert_eq!(WellTone::Strong.class_name(), "ui-well--tone-strong");

    assert_eq!(WellTone::Default.as_attr(), "default");
    assert_eq!(WellTone::Quiet.as_attr(), "quiet");
    assert_eq!(WellTone::Strong.as_attr(), "strong");
}

#[test]
fn density_class_names_and_attrs_are_stable() {
    assert_eq!(
        WellDensity::Comfortable.class_name(),
        "ui-well--density-comfortable"
    );
    assert_eq!(
        WellDensity::Compact.class_name(),
        "ui-well--density-compact"
    );

    assert_eq!(WellDensity::Comfortable.as_attr(), "comfortable");
    assert_eq!(WellDensity::Compact.as_attr(), "compact");
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-well ".to_string())),
        Some("docs-well".to_string())
    );
}

#[test]
fn normalize_aria_label_uses_trimmed_text_or_fallback() {
    let (label, custom) = normalize_aria_label(Some("  Selection summary  ".to_string()));
    assert_eq!(label, "Selection summary");
    assert!(custom);

    let (label, custom) = normalize_aria_label(Some("  ".to_string()));
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom);
}

#[test]
fn resolve_state_tracks_density_inset_and_sources() {
    let state = resolve_state(WellStateInput {
        tone: WellTone::Strong,
        density: WellDensity::Compact,
        inset: true,
        has_custom_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.tone_attr, "strong");
    assert_eq!(state.density_attr, "compact");
    assert!(state.is_inset);
    assert!(!state.is_not_inset);
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}
