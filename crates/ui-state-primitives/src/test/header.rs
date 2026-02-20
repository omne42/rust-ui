use super::*;

#[test]
fn header_tone_contract_is_stable() {
    assert_eq!(HeaderTone::Default.class_name(), "ui-header--tone-default");
    assert_eq!(HeaderTone::Strong.class_name(), "ui-header--tone-strong");

    assert_eq!(HeaderTone::Default.as_attr(), "default");
    assert_eq!(HeaderTone::Strong.as_attr(), "strong");
}

#[test]
fn normalize_optional_text_trims_and_drops_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-header  ".to_string())),
        Some("docs-header".to_string())
    );
}

#[test]
fn normalize_aria_label_uses_fallback_when_missing() {
    let (label, custom) = normalize_aria_label(Some("  Dialog Header  ".to_string()));
    assert_eq!(label, "Dialog Header");
    assert!(custom);

    let (label, custom) = normalize_aria_label(Some("  ".to_string()));
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom);
}

#[test]
fn resolve_state_tracks_flags_and_sources() {
    let state = resolve_state(HeaderStateInput {
        tone: HeaderTone::Strong,
        bordered: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.tone_attr, "strong");
    assert!(state.is_bordered);
    assert_eq!(state.data_state_attr, "strong-bordered");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}
