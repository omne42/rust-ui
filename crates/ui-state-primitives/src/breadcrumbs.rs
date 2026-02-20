pub use crate::button::normalize_optional_text;

pub const DEFAULT_ARIA_LABEL: &str = "Breadcrumb";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbsItemInput<'a> {
    pub href: Option<&'a str>,
    pub is_last: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbsStateInput<'a> {
    pub items: &'a [BreadcrumbsItemInput<'a>],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbsState {
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub has_links: bool,
    pub has_current_page: bool,
}

pub fn source_attr_from_presence(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn normalize_aria_label(value: Option<String>) -> (String, &'static str) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, "custom");
    }

    (DEFAULT_ARIA_LABEL.into(), "default")
}

pub fn resolve_root_class(class_name: Option<String>) -> (String, &'static str) {
    const BASE_CLASS: &str = "ui-breadcrumbs";

    if let Some(class_name) = normalize_optional_text(class_name) {
        return (format!("{BASE_CLASS} {class_name}"), "custom");
    }

    (BASE_CLASS.to_string(), "default")
}

pub fn resolve_state(input: BreadcrumbsStateInput<'_>) -> BreadcrumbsState {
    let item_count = input.items.len();
    let has_items = item_count > 0;
    let has_links = input.items.iter().any(|item| {
        !item.is_last
            && item
                .href
                .is_some_and(|href| normalize_optional_text(Some(href.into())).is_some())
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
#[path = "test/breadcrumbs.rs"]
mod tests;
