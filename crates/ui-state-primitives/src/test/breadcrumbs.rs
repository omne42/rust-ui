use super::*;

#[test]
fn normalize_aria_label_uses_custom_and_default_sources() {
    assert_eq!(
        normalize_aria_label(Some("  Docs trail  ".to_string())),
        ("Docs trail".to_string(), "custom")
    );
    assert_eq!(
        normalize_aria_label(Some("   ".to_string())),
        (DEFAULT_ARIA_LABEL.into(), "default")
    );
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), "default")
    );
}

#[test]
fn resolve_root_class_tracks_class_source() {
    assert_eq!(
        resolve_root_class(Some("  docs-breadcrumbs  ".to_string())),
        ("ui-breadcrumbs docs-breadcrumbs".to_string(), "custom")
    );
    assert_eq!(
        resolve_root_class(None),
        ("ui-breadcrumbs".to_string(), "default")
    );
}

#[test]
fn resolve_state_tracks_links_and_current_page_flags() {
    let items = [
        BreadcrumbsItemInput {
            href: Some("/"),
            is_last: false,
        },
        BreadcrumbsItemInput {
            href: Some("/components"),
            is_last: false,
        },
        BreadcrumbsItemInput {
            href: Some("/components/breadcrumbs"),
            is_last: true,
        },
    ];

    let state = resolve_state(BreadcrumbsStateInput { items: &items });
    assert_eq!(state.item_count, 3);
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert!(state.has_links);
    assert!(state.has_current_page);
}

#[test]
fn resolve_state_ignores_last_item_href_and_blank_values() {
    let items = [
        BreadcrumbsItemInput {
            href: Some("   "),
            is_last: false,
        },
        BreadcrumbsItemInput {
            href: Some("/details"),
            is_last: true,
        },
    ];

    let state = resolve_state(BreadcrumbsStateInput { items: &items });
    assert_eq!(state.item_count, 2);
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert!(!state.has_links);
    assert!(state.has_current_page);
}

#[test]
fn source_attr_from_presence_reports_expected_markers() {
    assert_eq!(source_attr_from_presence(true), "custom");
    assert_eq!(source_attr_from_presence(false), "default");
}
