use super::*;
use crate::sidebar_content::DEFAULT_ARIA_LABEL;

#[test]
fn normalize_aria_label_tracks_default_and_custom_sources() {
    assert_eq!(
        normalize_aria_label(Some("  Sidebar section content  ".to_string())),
        ("Sidebar section content".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(Some("\n\t".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn resolve_default_priority_prefers_prefixed_flags() {
    assert!(resolve_disabled(Some(true), false));
    assert!(!resolve_disabled(None, false));
    assert!(resolve_padded(Some(true), false));
    assert!(!resolve_padded(None, false));
    assert!(resolve_scrollable(Some(true), false));
    assert!(!resolve_scrollable(None, false));
}

#[test]
fn resolve_state_reports_padding_scroll_and_source_markers() {
    let state = resolve_state(SidebarContentStateInput {
        disabled: true,
        padded: false,
        scrollable: false,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    assert_eq!(state.state_attr, "disabled");
    assert_eq!(state.padding_attr, "compact");
    assert_eq!(state.scroll_attr, "static");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");
    assert!(state.disabled);
    assert!(!state.enabled);
}

#[test]
fn compose_class_name_includes_state_and_custom_markers() {
    let state = resolve_state(SidebarContentStateInput {
        disabled: true,
        padded: true,
        scrollable: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });
    let class_name = compose_class_name(Some("docs-sidebar-content-custom".to_string()), state);

    for needle in [
        "ui-sidebar__content",
        "ui-sidebar-content",
        "ui-sidebar-content--disabled",
        "ui-sidebar-content--padded",
        "ui-sidebar-content--scrollable",
        "ui-sidebar-content--custom-class",
        "docs-sidebar-content-custom",
    ] {
        assert!(
            class_name.contains(needle),
            "missing `{needle}` in sidebar content class contract"
        );
    }
}
