use super::*;

#[test]
fn keyboard_tone_contract_is_stable() {
    assert_eq!(
        KeyboardTone::Default.class_name(),
        "ui-keyboard--tone-default"
    );
    assert_eq!(KeyboardTone::Muted.class_name(), "ui-keyboard--tone-muted");

    assert_eq!(KeyboardTone::Default.as_attr(), "default");
    assert_eq!(KeyboardTone::Muted.as_attr(), "muted");
}

#[test]
fn normalize_optional_text_trims_and_drops_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-keyboard  ".to_string())),
        Some("docs-keyboard".to_string())
    );
}

#[test]
fn normalize_aria_label_uses_fallback_when_missing() {
    let (label, custom) = normalize_aria_label(Some("  Keyboard Command  ".to_string()));
    assert_eq!(label, "Keyboard Command");
    assert!(custom);

    let (label, custom) = normalize_aria_label(Some("  ".to_string()));
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom);
}

#[test]
fn resolve_state_tracks_tone_compact_and_sources() {
    let state = resolve_state(KeyboardStateInput {
        tone: KeyboardTone::Muted,
        compact: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.tone_attr, "muted");
    assert!(state.is_compact);
    assert_eq!(state.data_state_attr, "compact");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}
