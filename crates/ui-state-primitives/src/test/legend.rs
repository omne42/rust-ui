use super::*;

#[test]
fn tone_class_names_and_attrs_are_stable() {
    assert_eq!(LegendTone::Default.class_name(), "ui-legend--tone-default");
    assert_eq!(LegendTone::Muted.class_name(), "ui-legend--tone-muted");
    assert_eq!(LegendTone::Strong.class_name(), "ui-legend--tone-strong");

    assert_eq!(LegendTone::Default.as_attr(), "default");
    assert_eq!(LegendTone::Muted.as_attr(), "muted");
    assert_eq!(LegendTone::Strong.as_attr(), "strong");
}

#[test]
fn normalize_helpers_fallback_to_defaults() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Preferences  ".to_string())),
        Some("Preferences".to_string())
    );

    assert_eq!(
        normalize_text(Some("  Notification settings  ".to_string())),
        ("Notification settings".to_string(), true)
    );
    assert_eq!(normalize_text(None), (DEFAULT_TEXT.into(), false));

    assert_eq!(
        normalize_required_indicator(Some("  (required)  ".to_string())),
        ("(required)".to_string(), true)
    );
    assert_eq!(
        normalize_required_indicator(None),
        (DEFAULT_REQUIRED_INDICATOR.into(), false)
    );
}

#[test]
fn normalize_required_and_disabled_states_track_sources() {
    let required = normalize_required_state(Some(false), true);
    assert!(!required.is_required);
    assert_eq!(required.required_source_attr, "is_required");

    let required = normalize_required_state(None, true);
    assert!(required.is_required);
    assert_eq!(required.required_source_attr, "required");

    let required = normalize_required_state(None, false);
    assert!(!required.is_required);
    assert_eq!(required.required_source_attr, "default");

    let disabled = normalize_accessibility_state(Some(false), true);
    assert!(!disabled.is_disabled);
    assert_eq!(disabled.disabled_source_attr, "is_disabled");

    let disabled = normalize_accessibility_state(None, true);
    assert!(disabled.is_disabled);
    assert_eq!(disabled.disabled_source_attr, "disabled");

    let disabled = normalize_accessibility_state(None, false);
    assert!(!disabled.is_disabled);
    assert_eq!(disabled.disabled_source_attr, "default");
}

#[test]
fn resolve_state_tracks_required_disabled_and_sources() {
    let state = resolve_state(LegendStateInput {
        tone: LegendTone::Strong,
        is_required: true,
        is_disabled: true,
        has_custom_text: true,
        has_custom_indicator: false,
        has_custom_class_name: true,
    });

    assert_eq!(state.tone_attr, "strong");
    assert!(state.is_required);
    assert!(!state.is_optional);
    assert!(state.is_disabled);
    assert_eq!(state.text_source_attr, "custom");
    assert_eq!(state.indicator_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");
}
