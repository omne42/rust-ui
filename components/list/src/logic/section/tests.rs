use super::*;

#[test]
fn heading_tone_contract_is_stable() {
    assert_eq!(
        ListSectionHeadingTone::Default.class_name(),
        "ui-listbox-section--tone-default"
    );
    assert_eq!(
        ListSectionHeadingTone::Quiet.class_name(),
        "ui-listbox-section--tone-quiet"
    );
    assert_eq!(ListSectionHeadingTone::Default.as_attr(), "default");
    assert_eq!(ListSectionHeadingTone::Quiet.as_attr(), "quiet");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_title(None), None);
    assert_eq!(normalize_title(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_title(Some("  Favorite regions  ".to_string())),
        Some("Favorite regions".to_string())
    );

    assert_eq!(
        normalize_aria_label(Some("  Region choices  ".to_string())),
        ("Region choices".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(Some("".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn resolve_state_tracks_markers() {
    let state = resolve_state(ListSectionStateInput {
        heading_tone: ListSectionHeadingTone::Quiet,
        item_count: 0,
        disabled: true,
        sticky_heading: true,
        show_divider: true,
        has_title: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert!(state.is_empty);
    assert!(!state.has_items);
    assert!(state.is_disabled);
    assert!(state.has_title);
    assert!(state.is_sticky_heading);
    assert!(state.has_divider);
    assert_eq!(state.data_state_attr, "disabled-empty");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.title_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let state = resolve_state(ListSectionStateInput {
        heading_tone: ListSectionHeadingTone::Default,
        item_count: 3,
        disabled: false,
        sticky_heading: true,
        show_divider: true,
        has_title: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-listbox-section-custom".to_string()), state);

    for needle in [
        "ui-listbox-section",
        "ui-listbox-section--tone-default",
        "ui-listbox-section--has-title",
        "ui-listbox-section--sticky-heading",
        "ui-listbox-section--divided",
        "ui-listbox-section--custom-class",
        "docs-listbox-section-custom",
    ] {
        assert!(
            class_name.contains(needle),
            "ListSection class list should include `{needle}`"
        );
    }
}

#[test]
fn normalize_item_count_and_title_text_apply_single_default_source() {
    assert_eq!(normalize_item_count(None), 1);
    assert_eq!(normalize_item_count(Some(3)), 3);
    assert_eq!(resolve_title_text(None), "");
    assert_eq!(
        resolve_title_text(Some("Preferred regions".to_string())),
        "Preferred regions"
    );
}
