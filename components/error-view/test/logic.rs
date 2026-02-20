use super::*;

#[test]
fn primitives_are_reexported_from_ui_state_primitives() {
    assert_eq!(
        normalize_optional_text(Some("  bad input  ".to_string())),
        Some("bad input".to_string())
    );
    assert_eq!(ErrorViewTone::Neutral.as_attr(), "neutral");
}

#[test]
fn normalize_props_centralizes_defaults_and_source_markers() {
    let normalized = normalize_props(ErrorViewNormalizeInput {
        tone: None,
        is_invalid: true,
        is_compact: Some(true),
        is_bordered: None,
        message: Some("  Email invalid  ".to_string()),
        aria_label: None,
        class_name: Some("  docs-error-view  ".to_string()),
        has_icon: true,
        has_actions: false,
        has_children: false,
        has_custom_motion: true,
    });

    assert_eq!(normalized.state_input.tone, ErrorViewTone::Negative);
    assert!(normalized.state_input.is_invalid);
    assert!(normalized.state_input.compact);
    assert!(!normalized.state_input.bordered);
    assert!(normalized.state_input.has_custom_message);
    assert!(!normalized.state_input.has_custom_aria_label);
    assert!(normalized.state_input.has_custom_class_name);
    assert!(normalized.state_input.has_custom_motion);
    assert_eq!(normalized.message, "Email invalid");
    assert_eq!(normalized.aria_label, DEFAULT_ARIA_LABEL);
    assert_eq!(normalized.class_name, Some("docs-error-view".to_string()));
    assert_eq!(normalized.tone_source_attr, "default");
    assert_eq!(normalized.compact_source_attr, "is-prop");
    assert_eq!(normalized.bordered_source_attr, "default");
}
