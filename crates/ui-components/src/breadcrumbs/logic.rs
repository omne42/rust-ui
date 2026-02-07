#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbsState {
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub has_links: bool,
    pub has_current_page: bool,
}

pub fn resolve_state(items: &[BreadcrumbItem]) -> BreadcrumbsState {
    let item_count = items.len();
    let has_items = item_count > 0;
    let has_links = items.iter().enumerate().any(|(index, item)| {
        index + 1 < item_count
            && item
                .href
                .as_ref()
                .map(|href| href.trim())
                .is_some_and(|href| !href.is_empty())
    });

    BreadcrumbsState {
        item_count,
        is_empty: !has_items,
        has_items,
        has_links,
        has_current_page: has_items,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(label: &str, href: Option<&str>) -> BreadcrumbItem {
        BreadcrumbItem {
            label: label.to_string(),
            href: href.map(ToString::to_string),
        }
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
}
