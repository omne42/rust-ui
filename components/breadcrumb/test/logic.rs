use super::*;

fn item(label: &str, href: Option<&str>) -> BreadcrumbItem {
    BreadcrumbItem {
        label: label.into(),
        href: href.map(ToString::to_string),
    }
}

#[test]
fn resolve_root_state_tracks_default_sources() {
    let root = resolve_root_state(None, None);

    assert_eq!(
        root.aria_label,
        ui_state_primitives::breadcrumbs::DEFAULT_ARIA_LABEL
    );
    assert_eq!(root.aria_source_attr, "default");
    assert_eq!(root.class_name, "ui-breadcrumb");
    assert_eq!(root.class_source_attr, "default");
}

#[test]
fn resolve_root_state_tracks_custom_sources() {
    let root = resolve_root_state(
        Some("  Docs trail  ".to_string()),
        Some("  docs-breadcrumb  ".to_string()),
    );

    assert_eq!(root.aria_label, "Docs trail");
    assert_eq!(root.aria_source_attr, "custom");
    assert_eq!(root.class_name, "ui-breadcrumb docs-breadcrumb");
    assert_eq!(root.class_source_attr, "custom");
}

#[test]
fn resolve_state_tracks_empty_and_count() {
    let state = resolve_state(&[]);
    assert_eq!(state.item_count, 0);
    assert!(state.is_empty);
    assert!(!state.has_items);
    assert!(!state.has_links);
    assert!(!state.has_current_page);
}

#[test]
fn resolve_state_tracks_links_on_non_last_items() {
    let state = resolve_state(&[
        item("Home", Some("/")),
        item("Components", Some("/components")),
        item("Breadcrumb", None),
    ]);

    assert_eq!(state.item_count, 3);
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert!(state.has_links);
    assert!(state.has_current_page);
}

#[test]
fn resolve_state_ignores_blank_and_last_item_links() {
    let state = resolve_state(&[item("Home", Some("   ")), item("Details", Some("/details"))]);

    assert_eq!(state.item_count, 2);
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert!(!state.has_links);
    assert!(state.has_current_page);
}

#[test]
fn resolve_item_href_sanitizes_non_last_items() {
    assert_eq!(
        resolve_item_href(&item("Home", Some("  /docs  ")), false),
        Some("/docs".to_string())
    );
    assert_eq!(resolve_item_href(&item("Home", Some("  ")), false), None);
    assert_eq!(
        resolve_item_href(&item("Current", Some("/current")), true),
        None
    );
}
