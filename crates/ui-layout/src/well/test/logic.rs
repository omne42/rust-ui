use super::*;

#[test]
fn normalize_defaults_are_explicit_and_stable() {
    assert_eq!(normalize_tone(None), WellTone::Default);
    assert_eq!(normalize_tone(Some(WellTone::Strong)), WellTone::Strong);

    assert_eq!(normalize_density(None), WellDensity::Comfortable);
    assert_eq!(
        normalize_density(Some(WellDensity::Compact)),
        WellDensity::Compact
    );

    assert!(!normalize_is_inset(None));
    assert!(normalize_is_inset(Some(true)));
}

#[test]
fn normalize_props_centralizes_state_input_and_sources() {
    let normalized = normalize_props(WellNormalizeInput {
        tone: None,
        density: Some(WellDensity::Compact),
        is_inset: None,
        aria_label: Some("  Selection summary  ".to_string()),
        fallback_aria_label: "Content well".to_string(),
        class_name: Some(" docs-well ".to_string()),
    });

    assert_eq!(normalized.state_input.tone, WellTone::Default);
    assert_eq!(normalized.state_input.density, WellDensity::Compact);
    assert!(!normalized.state_input.inset);
    assert!(normalized.state_input.has_custom_label);
    assert!(normalized.state_input.has_custom_class_name);
    assert_eq!(normalized.aria_label, "Selection summary");
    assert_eq!(normalized.class_name, Some("docs-well".to_string()));
    assert_eq!(normalized.tone_source_attr, "default");
    assert_eq!(normalized.density_source_attr, "prop");
    assert_eq!(normalized.inset_source_attr, "default");
}

#[test]
fn normalize_aria_label_with_fallback_prefers_explicit_and_sanitizes_fallback() {
    assert_eq!(
        normalize_aria_label_with_fallback(Some("  Summary  ".to_string()), "Localized default"),
        ("Summary".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label_with_fallback(None, "  Localized default  "),
        ("Localized default".to_string(), false)
    );
    assert_eq!(
        normalize_aria_label_with_fallback(None, "   "),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn compose_class_name_includes_state_markers() {
    let state = resolve_state(WellStateInput {
        tone: WellTone::Quiet,
        density: WellDensity::Comfortable,
        inset: true,
        has_custom_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-well".to_string()), state);
    for token in [
        "ui-well",
        "ui-well--tone-quiet",
        "ui-well--density-comfortable",
        "ui-well--inset",
        "ui-well--custom-class",
        "docs-well",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}

#[test]
fn source_attr_from_presence_is_closed_set() {
    assert_eq!(source_attr_from_presence(false), "default");
    assert_eq!(source_attr_from_presence(true), "prop");
}
