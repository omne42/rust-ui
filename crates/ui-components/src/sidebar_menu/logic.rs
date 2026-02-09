use crate::sidebar_menu::{DEFAULT_ARIA_LABEL, DEFAULT_ID_BASE};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarMenuState {
    pub item_count: usize,
    pub has_items: bool,
    pub is_empty: bool,
    pub disabled: bool,
    pub enabled: bool,
    pub show_badges: bool,
    pub show_actions: bool,
    pub allow_submenu_collapse: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub has_custom_class_name: bool,
    pub has_shortcut: bool,
    pub state_attr: &'static str,
    pub control_attr: &'static str,
    pub class_source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarMenuStateInput {
    pub item_count: usize,
    pub disabled: bool,
    pub show_badges: bool,
    pub show_actions: bool,
    pub allow_submenu_collapse: bool,
    pub is_controlled: bool,
    pub has_custom_class_name: bool,
    pub has_shortcut: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ARIA_LABEL.to_string())
}

pub fn normalize_id_base(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ID_BASE.to_string())
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

pub fn resolve_state(input: SidebarMenuStateInput) -> SidebarMenuState {
    let has_items = input.item_count > 0;
    let is_empty = !has_items;
    let enabled = !input.disabled;
    let is_uncontrolled = !input.is_controlled;

    SidebarMenuState {
        item_count: input.item_count,
        has_items,
        is_empty,
        disabled: input.disabled,
        enabled,
        show_badges: input.show_badges,
        show_actions: input.show_actions,
        allow_submenu_collapse: input.allow_submenu_collapse,
        is_controlled: input.is_controlled,
        is_uncontrolled,
        has_custom_class_name: input.has_custom_class_name,
        has_shortcut: input.has_shortcut,
        state_attr: if input.disabled && is_empty {
            "disabled-empty"
        } else if input.disabled {
            "disabled"
        } else if is_empty {
            "empty"
        } else {
            "ready"
        },
        control_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(class_name: Option<String>, state: SidebarMenuState) -> String {
    let mut classes = vec!["ui-sidebar-menu".to_string()];

    if state.disabled {
        classes.push("ui-sidebar-menu--disabled".to_string());
    }

    if state.is_empty {
        classes.push("ui-sidebar-menu--empty".to_string());
    }

    if state.show_badges {
        classes.push("ui-sidebar-menu--with-badges".to_string());
    }

    if state.show_actions {
        classes.push("ui-sidebar-menu--with-actions".to_string());
    }

    if state.allow_submenu_collapse {
        classes.push("ui-sidebar-menu--collapsible-sub".to_string());
    }

    if state.has_shortcut {
        classes.push("ui-sidebar-menu--with-shortcut".to_string());
    }

    if state.is_controlled {
        classes.push("ui-sidebar-menu--controlled".to_string());
    } else {
        classes.push("ui-sidebar-menu--uncontrolled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-sidebar-menu--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_items_applies_fallbacks_and_trims() {
        let normalized = normalize_items(vec![SidebarMenuItem {
            id: " ".to_string(),
            label: " ".to_string(),
            href: Some(" /demo ".to_string()),
            badge: Some(" 42 ".to_string()),
            action_label: Some(" ⋯ ".to_string()),
            disabled: false,
            sub_items: vec![SidebarMenuSubItem {
                id: " ".to_string(),
                label: " ".to_string(),
                href: Some(" # ".to_string()),
                disabled: false,
            }],
            default_sub_open: true,
        }]);

        assert_eq!(normalized[0].id, "item-0");
        assert_eq!(normalized[0].label, "Item 1");
        assert_eq!(normalized[0].href.as_deref(), Some("/demo"));
        assert_eq!(normalized[0].badge.as_deref(), Some("42"));
        assert_eq!(normalized[0].action_label.as_deref(), Some("⋯"));
        assert_eq!(normalized[0].sub_items[0].id, "item-0-sub-0");
    }

    #[test]
    fn next_id_for_key_tracks_enabled_items() {
        let items = normalize_items(vec![
            SidebarMenuItem::new("a", "A"),
            SidebarMenuItem {
                id: "b".to_string(),
                label: "B".to_string(),
                href: None,
                badge: None,
                action_label: None,
                disabled: true,
                sub_items: vec![SidebarMenuSubItem::new("b-1", "B1")],
                default_sub_open: false,
            },
        ]);

        assert_eq!(next_id_for_key("Home", &items, None).as_deref(), Some("a"));
        assert_eq!(next_id_for_key("End", &items, None).as_deref(), Some("b-1"));
        assert_eq!(
            next_id_for_key("ArrowDown", &items, Some("a".to_string())).as_deref(),
            Some("b-1"),
        );
    }

    #[test]
    fn compose_class_name_includes_state_flags() {
        let class = compose_class_name(
            Some("custom".to_string()),
            resolve_state(SidebarMenuStateInput {
                item_count: 2,
                disabled: false,
                show_badges: true,
                show_actions: true,
                allow_submenu_collapse: true,
                is_controlled: false,
                has_custom_class_name: true,
                has_shortcut: true,
            }),
        );

        for needle in [
            "ui-sidebar-menu",
            "ui-sidebar-menu--with-badges",
            "ui-sidebar-menu--with-actions",
            "ui-sidebar-menu--collapsible-sub",
            "ui-sidebar-menu--with-shortcut",
            "ui-sidebar-menu--uncontrolled",
            "ui-sidebar-menu--custom-class",
            "custom",
        ] {
            assert!(class.contains(needle), "class should contain `{needle}`");
        }
    }
}
