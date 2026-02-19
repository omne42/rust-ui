use ui_state_primitives::breadcrumbs as breadcrumbs_primitives;
pub use ui_state_primitives::breadcrumbs::{
    BreadcrumbsItemInput, BreadcrumbsState, BreadcrumbsStateInput,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreadcrumbsRootState {
    pub aria_label: String,
    pub aria_source_attr: &'static str,
    pub class_name: String,
    pub class_source_attr: &'static str,
}

pub fn resolve_root_state(
    aria_label: Option<String>,
    class_name: Option<String>,
) -> BreadcrumbsRootState {
    let (aria_label, aria_source_attr) = breadcrumbs_primitives::normalize_aria_label(aria_label);
    let (class_name, class_source_attr) = breadcrumbs_primitives::resolve_root_class(class_name);

    BreadcrumbsRootState {
        aria_label,
        aria_source_attr,
        class_name,
        class_source_attr,
    }
}

pub fn resolve_state(items: &[BreadcrumbItem]) -> BreadcrumbsState {
    let item_inputs: Vec<_> = items
        .iter()
        .enumerate()
        .map(|(index, item)| BreadcrumbsItemInput {
            href: item.href.as_deref(),
            is_last: index + 1 == items.len(),
        })
        .collect();

    breadcrumbs_primitives::resolve_state(BreadcrumbsStateInput {
        items: &item_inputs,
    })
}

pub fn resolve_item_href(item: &BreadcrumbItem, is_last: bool) -> Option<String> {
    if is_last {
        return None;
    }

    breadcrumbs_primitives::normalize_optional_text(item.href.clone())
}

#[cfg(test)]
mod tests {
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
        assert_eq!(root.class_name, "ui-breadcrumbs");
        assert_eq!(root.class_source_attr, "default");
    }

    #[test]
    fn resolve_root_state_tracks_custom_sources() {
        let root = resolve_root_state(
            Some("  Docs trail  ".to_string()),
            Some("  docs-breadcrumbs  ".to_string()),
        );

        assert_eq!(root.aria_label, "Docs trail");
        assert_eq!(root.aria_source_attr, "custom");
        assert_eq!(root.class_name, "ui-breadcrumbs docs-breadcrumbs");
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
            item("Breadcrumbs", None),
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
}
