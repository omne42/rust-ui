use super::*;

#[test]
fn heading_tone_contract_is_stable() {
    assert_eq!(
        MenuSectionHeadingTone::Default.class_name(),
        "ui-menu-section--tone-default"
    );
    assert_eq!(
        MenuSectionHeadingTone::Quiet.class_name(),
        "ui-menu-section--tone-quiet"
    );
    assert_eq!(MenuSectionHeadingTone::Default.as_attr(), "default");
    assert_eq!(MenuSectionHeadingTone::Quiet.as_attr(), "quiet");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  File operations  ".to_string())),
        Some("File operations".to_string())
    );

    assert_eq!(
        normalize_aria_label(Some("  Action region  ".to_string())),
        ("Action region".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(Some("".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn resolve_state_tracks_markers() {
    let state = resolve_state(MenuSectionStateInput {
        heading_tone: MenuSectionHeadingTone::Quiet,
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
    let state = resolve_state(MenuSectionStateInput {
        heading_tone: MenuSectionHeadingTone::Default,
        item_count: 3,
        disabled: false,
        sticky_heading: true,
        show_divider: true,
        has_title: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-menu-section-custom".to_string()), state);

    for needle in [
        "ui-menu-section",
        "ui-menu-section--tone-default",
        "ui-menu-section--has-title",
        "ui-menu-section--sticky-heading",
        "ui-menu-section--divided",
        "ui-menu-section--custom-class",
        "docs-menu-section-custom",
    ] {
        assert!(
            class_name.contains(needle),
            "MenuSection class list should include `{needle}`"
        );
    }
}
