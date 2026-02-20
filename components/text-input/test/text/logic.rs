use super::*;
use crate::text_input::text::TextStateInput;

#[test]
fn class_and_attr_contracts_are_stable() {
    assert_eq!(TextTone::Default.class_name(), "ui-text--tone-default");
    assert_eq!(TextAlign::Center.class_name(), "ui-text--align-center");
    assert_eq!(TextWeight::Bold.class_name(), "ui-text--weight-bold");

    assert_eq!(TextTone::Subtle.as_attr(), "subtle");
    assert_eq!(TextAlign::Justify.as_attr(), "justify");
    assert_eq!(TextWeight::Medium.as_attr(), "medium");
}

#[test]
fn normalization_helpers_use_defaults() {
    assert_eq!(normalize_content(Some("  hello  ".to_string())), "hello");
    assert_eq!(normalize_content(Some("   ".to_string())), DEFAULT_TEXT);

    let (aria, is_custom) = normalize_aria_label(None);
    assert_eq!(aria, DEFAULT_ARIA_LABEL);
    assert!(!is_custom);
}

#[test]
fn resolve_state_tracks_sources_and_flags() {
    let state = resolve_state(TextStateInput {
        tone: TextTone::Strong,
        align: TextAlign::End,
        weight: TextWeight::Semibold,
        disabled: false,
        truncate: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.tone_attr, "strong");
    assert_eq!(state.align_attr, "end");
    assert_eq!(state.weight_attr, "semibold");
    assert_eq!(state.data_state_attr, "truncate");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}

#[test]
fn compose_class_name_includes_markers() {
    let class_name = compose_class_name(
        Some("docs-text".to_string()),
        resolve_state(TextStateInput {
            tone: TextTone::Subtle,
            align: TextAlign::Center,
            weight: TextWeight::Bold,
            disabled: true,
            truncate: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-text",
        "ui-text--tone-subtle",
        "ui-text--align-center",
        "ui-text--weight-bold",
        "ui-text--disabled",
        "ui-text--truncate",
        "ui-text--custom-class",
        "docs-text",
    ] {
        assert!(class_name.contains(token), "class should include `{token}`");
    }
}
