use super::*;

#[test]
fn emphasis_class_names_and_attrs_are_stable() {
    assert_eq!(
        LabelEmphasis::Default.class_name(),
        "ui-label--emphasis-default"
    );
    assert_eq!(
        LabelEmphasis::Subtle.class_name(),
        "ui-label--emphasis-subtle"
    );
    assert_eq!(
        LabelEmphasis::Strong.class_name(),
        "ui-label--emphasis-strong"
    );

    assert_eq!(LabelEmphasis::Default.as_attr(), "default");
    assert_eq!(LabelEmphasis::Subtle.as_attr(), "subtle");
    assert_eq!(LabelEmphasis::Strong.as_attr(), "strong");
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" Name ".to_string())),
        Some("Name".to_string())
    );
}

#[test]
fn normalize_helpers_fallback_to_defaults() {
    let (label, custom_label) = normalize_label_text(Some("  Username  ".to_string()));
    assert_eq!(label, "Username");
    assert!(custom_label);

    let (label, custom_label) = normalize_label_text(None);
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom_label);

    let (indicator, custom_indicator) = normalize_required_indicator(Some(" (req) ".to_string()));
    assert_eq!(indicator, "(req)");
    assert!(custom_indicator);

    let (indicator, custom_indicator) = normalize_required_indicator(None);
    assert_eq!(indicator, DEFAULT_REQUIRED_INDICATOR);
    assert!(!custom_indicator);
}

#[test]
fn resolve_state_tracks_required_disabled_and_sources() {
    let state = resolve_state(LabelStateInput {
        emphasis: LabelEmphasis::Strong,
        required: true,
        disabled: true,
        has_for_id: true,
        has_custom_label: true,
        has_custom_indicator: false,
        has_custom_class_name: true,
    });

    assert_eq!(state.emphasis_attr, "strong");
    assert!(state.is_required);
    assert!(!state.is_optional);
    assert!(state.is_disabled);
    assert!(state.has_for_id);
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.indicator_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let state = resolve_state(LabelStateInput {
        emphasis: LabelEmphasis::Subtle,
        required: true,
        disabled: false,
        has_for_id: true,
        has_custom_label: false,
        has_custom_indicator: true,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-label".to_string()), state);
    for token in [
        "ui-label",
        "ui-label--emphasis-subtle",
        "ui-label--required",
        "ui-label--for",
        "ui-label--indicator-custom",
        "ui-label--custom-class",
        "docs-label",
    ] {
        assert!(class_name.contains(token), "class should contain `{token}`");
    }
}
