use super::*;
use crate::DescriptionStateInput;

#[test]
fn tone_and_element_contracts_are_stable() {
    assert_eq!(
        DescriptionTone::Default.class_name(),
        "ui-description--tone-default"
    );
    assert_eq!(
        DescriptionTone::Muted.class_name(),
        "ui-description--tone-muted"
    );
    assert_eq!(
        DescriptionTone::Negative.class_name(),
        "ui-description--tone-negative"
    );

    assert_eq!(DescriptionTone::Default.as_attr(), "default");
    assert_eq!(DescriptionTone::Muted.as_attr(), "muted");
    assert_eq!(DescriptionTone::Negative.as_attr(), "negative");

    assert_eq!(DescriptionElement::default(), DescriptionElement::Paragraph);
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  helper  ".to_string())),
        Some("helper".to_string())
    );

    assert_eq!(normalize_content(Some("  hint  ".to_string())), "hint");
    assert_eq!(normalize_content(Some(" \n ".to_string())), DEFAULT_TEXT);

    let (label, custom) = normalize_aria_label(Some("  Form help  ".to_string()));
    assert_eq!(label, "Form help");
    assert!(custom);

    let (label, custom) = normalize_aria_label(None);
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom);
}

#[test]
fn resolve_state_tracks_sources_and_priority() {
    let state = resolve_state(DescriptionStateInput {
        tone: DescriptionTone::Muted,
        disabled: false,
        truncate: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.tone_attr, "muted");
    assert!(!state.is_disabled);
    assert!(state.is_truncated);
    assert_eq!(state.data_state_attr, "truncate");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-description".to_string()),
        resolve_state(DescriptionStateInput {
            tone: DescriptionTone::Negative,
            disabled: true,
            truncate: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-description",
        "ui-description--tone-negative",
        "ui-description--disabled",
        "ui-description--truncate",
        "ui-description--custom-class",
        "docs-description",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should contain `{token}`"
        );
    }
}
