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
pub struct BreadcrumbRootState {
    pub aria_label: String,
    pub aria_source_attr: &'static str,
    pub class_name: String,
    pub class_source_attr: &'static str,
}

pub fn resolve_root_state(
    aria_label: Option<String>,
    class_name: Option<String>,
) -> BreadcrumbRootState {
    let (aria_label, aria_source_attr) = breadcrumbs_primitives::normalize_aria_label(aria_label);
    let normalized_class_name = breadcrumbs_primitives::normalize_optional_text(class_name);
    let has_custom_class_name = normalized_class_name.is_some();
    let class_name = if let Some(class_name) = normalized_class_name {
        format!("ui-breadcrumb {class_name}")
    } else {
        "ui-breadcrumb".to_string()
    };
    let class_source_attr =
        breadcrumbs_primitives::source_attr_from_presence(has_custom_class_name);

    BreadcrumbRootState {
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
#[path = "../test/logic.rs"]
mod tests;
