use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SidebarMenuSubItem {
    pub id: String,
    pub label: String,
    pub href: Option<String>,
    pub disabled: bool,
}

impl SidebarMenuSubItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            href: None,
            disabled: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SidebarMenuItem {
    pub id: String,
    pub label: String,
    pub href: Option<String>,
    pub badge: Option<String>,
    pub action_label: Option<String>,
    pub disabled: bool,
    pub sub_items: Vec<SidebarMenuSubItem>,
    pub default_sub_open: bool,
}

impl SidebarMenuItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            href: None,
            badge: None,
            action_label: None,
            disabled: false,
            sub_items: Vec::new(),
            default_sub_open: false,
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_items(items: Vec<SidebarMenuItem>) -> Vec<SidebarMenuItem> {
    items
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            let fallback_id = format!("item-{index}");
            let fallback_label = format!("Item {}", index + 1);

            let id = normalize_optional_text(Some(item.id)).unwrap_or(fallback_id);
            let label = normalize_optional_text(Some(item.label)).unwrap_or(fallback_label);
            let href = normalize_optional_text(item.href);
            let badge = normalize_optional_text(item.badge);
            let action_label = normalize_optional_text(item.action_label);

            let sub_items = item
                .sub_items
                .into_iter()
                .enumerate()
                .map(|(sub_index, sub_item)| {
                    let fallback_id = format!("{id}-sub-{sub_index}");
                    let fallback_label = format!("Sub {}", sub_index + 1);

                    SidebarMenuSubItem {
                        id: normalize_optional_text(Some(sub_item.id)).unwrap_or(fallback_id),
                        label: normalize_optional_text(Some(sub_item.label))
                            .unwrap_or(fallback_label),
                        href: normalize_optional_text(sub_item.href),
                        disabled: sub_item.disabled,
                    }
                })
                .collect();

            SidebarMenuItem {
                id,
                label,
                href,
                badge,
                action_label,
                disabled: item.disabled,
                sub_items,
                default_sub_open: item.default_sub_open,
            }
        })
        .collect()
}

pub fn default_open_sub_ids(items: &[SidebarMenuItem]) -> Vec<String> {
    items
        .iter()
        .filter(|item| item.default_sub_open && !item.sub_items.is_empty())
        .map(|item| item.id.clone())
        .collect()
}

pub fn default_open_sub_id_set(items: &[SidebarMenuItem]) -> BTreeSet<String> {
    default_open_sub_ids(items).into_iter().collect()
}

pub fn submenu_root_id_set(items: &[SidebarMenuItem]) -> BTreeSet<String> {
    items
        .iter()
        .filter(|item| !item.sub_items.is_empty())
        .map(|item| item.id.clone())
        .collect()
}

pub fn normalize_open_sub_id_set(
    open_sub_ids: &BTreeSet<String>,
    items: &[SidebarMenuItem],
) -> BTreeSet<String> {
    let valid = submenu_root_id_set(items);
    open_sub_ids
        .iter()
        .filter(|id| valid.contains(id.as_str()))
        .cloned()
        .collect()
}

pub fn toggle_open_sub_id(
    open_sub_ids: &BTreeSet<String>,
    id: &str,
    items: &[SidebarMenuItem],
) -> BTreeSet<String> {
    let valid = submenu_root_id_set(items);
    if !valid.contains(id) {
        return normalize_open_sub_id_set(open_sub_ids, items);
    }

    let mut next = normalize_open_sub_id_set(open_sub_ids, items);
    if next.contains(id) {
        next.remove(id);
    } else {
        next.insert(id.into());
    }
    next
}

pub fn default_active_id(items: &[SidebarMenuItem], requested: Option<String>) -> Option<String> {
    if let Some(requested) = normalize_optional_text(requested)
        && contains_id(items, &requested)
    {
        return Some(requested);
    }

    first_enabled_id(items)
}

pub fn contains_id(items: &[SidebarMenuItem], id: &str) -> bool {
    items.iter().any(|item| {
        item.id == id
            || item
                .sub_items
                .iter()
                .any(|sub_item| sub_item.id == id && !sub_item.disabled)
    })
}

pub fn first_enabled_id(items: &[SidebarMenuItem]) -> Option<String> {
    for item in items {
        if !item.disabled {
            return Some(item.id.clone());
        }

        for sub_item in &item.sub_items {
            if !sub_item.disabled {
                return Some(sub_item.id.clone());
            }
        }
    }

    None
}

pub fn linear_enabled_ids(items: &[SidebarMenuItem]) -> Vec<String> {
    let mut ids = Vec::new();

    for item in items {
        if !item.disabled {
            ids.push(item.id.clone());
        }

        for sub_item in &item.sub_items {
            if !sub_item.disabled {
                ids.push(sub_item.id.clone());
            }
        }
    }

    ids
}

pub fn next_enabled_id(
    items: &[SidebarMenuItem],
    current: Option<String>,
    step: i32,
) -> Option<String> {
    let linear_ids = linear_enabled_ids(items);
    if linear_ids.is_empty() {
        return None;
    }

    let current_index = current
        .as_ref()
        .and_then(|current| linear_ids.iter().position(|id| id == current))
        .unwrap_or(0);

    let len = linear_ids.len() as i32;
    let next_index = (current_index as i32 + step).rem_euclid(len) as usize;
    Some(linear_ids[next_index].clone())
}

pub fn next_id_for_key(
    key: &str,
    items: &[SidebarMenuItem],
    current: Option<String>,
) -> Option<String> {
    match key {
        "ArrowDown" => next_enabled_id(items, current, 1),
        "ArrowUp" => next_enabled_id(items, current, -1),
        "Home" => first_enabled_id(items),
        "End" => linear_enabled_ids(items).last().cloned(),
        _ => None,
    }
}

pub fn resolve_active_index(linear_ids: &[String], current: Option<&str>) -> usize {
    current
        .and_then(|current| linear_ids.iter().position(|id| id == current))
        .unwrap_or(0)
}

pub fn active_index_for_current(items: &[SidebarMenuItem], current: Option<&str>) -> usize {
    let linear_ids = linear_enabled_ids(items);
    resolve_active_index(&linear_ids, current)
}

#[cfg(test)]
#[path = "test/sidebar_menu.rs"]
mod tests;
