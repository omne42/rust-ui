use std::collections::BTreeSet;
use std::sync::Arc;

use crate::MenuItemKind;
use ui_headless::PopoverPlacement;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenubarMenu {
    pub id: String,
    pub label: String,
    pub items: Vec<String>,
    pub disabled_indices: Vec<usize>,
    pub item_kinds: Vec<MenuItemKind>,
    pub disabled: bool,
}

impl MenubarMenu {
    pub fn new(id: impl Into<String>, label: impl Into<String>, items: Vec<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            items,
            disabled_indices: Vec::new(),
            item_kinds: Vec::new(),
            disabled: false,
        }
    }

    pub fn disabled_indices(mut self, disabled_indices: Vec<usize>) -> Self {
        self.disabled_indices = disabled_indices;
        self
    }

    pub fn item_kinds(mut self, item_kinds: Vec<MenuItemKind>) -> Self {
        self.item_kinds = item_kinds;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MenuOpenFocusStrategy {
    #[default]
    First,
    Last,
}

impl MenuOpenFocusStrategy {
    pub fn default_index(self, item_count: usize) -> usize {
        match self {
            Self::First => 0,
            Self::Last => item_count.saturating_sub(1),
        }
    }
}

pub fn focus_strategy_for_open_key(key: &str) -> Option<MenuOpenFocusStrategy> {
    match key {
        "ArrowDown" => Some(MenuOpenFocusStrategy::First),
        "ArrowUp" => Some(MenuOpenFocusStrategy::Last),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenubarMenuIds {
    pub trigger_id: String,
    pub menu_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenubarMenuResolved {
    pub id: String,
    pub label: String,
    pub items: Arc<[String]>,
    pub disabled_indices: Vec<usize>,
    pub item_kinds: Vec<MenuItemKind>,
    pub is_trigger_disabled: bool,
    pub has_items: bool,
    pub trigger_id: String,
    pub menu_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenubarStateInput {
    pub menu_count: usize,
    pub open_index: Option<usize>,
    pub has_disabled_menus: bool,
    pub has_custom_class_name: bool,
    pub is_controlled: bool,
    pub placement: PopoverPlacement,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenubarState {
    pub menu_count: usize,
    pub is_empty: bool,
    pub has_menus: bool,
    pub open_index: Option<usize>,
    pub has_open_menu: bool,
    pub has_disabled_menus: bool,
    pub has_custom_class_name: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub placement: PopoverPlacement,
    pub placement_attr: &'static str,
    pub data_state_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| "menubar".to_string())
}

fn sanitize_token(value: &str, fallback: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_dash = false;
            continue;
        }

        if (ch == '-' || ch == '_' || ch == ' ') && !last_dash {
            out.push('-');
            last_dash = true;
        }
    }

    while out.ends_with('-') {
        out.pop();
    }

    if out.is_empty() {
        return fallback.to_string();
    }

    out
}

pub fn resolve_menu_ids(id_base: &str, menu_id: &str) -> MenubarMenuIds {
    MenubarMenuIds {
        trigger_id: format!("{id_base}-{menu_id}-trigger"),
        menu_id: format!("{id_base}-{menu_id}-menu"),
    }
}

pub fn normalize_open_index(open_index: Option<usize>, menu_count: usize) -> Option<usize> {
    open_index.filter(|index| *index < menu_count)
}

pub fn normalize_disabled_indices(disabled_indices: Vec<usize>, item_count: usize) -> Vec<usize> {
    let mut unique = BTreeSet::new();
    for index in disabled_indices {
        if index < item_count {
            unique.insert(index);
        }
    }

    unique.into_iter().collect()
}

pub fn resolve_menus(id_base: &str, menus: Vec<MenubarMenu>) -> Vec<MenubarMenuResolved> {
    let mut seen_ids = BTreeSet::new();
    let mut resolved = Vec::new();

    for (index, menu) in menus.into_iter().enumerate() {
        let fallback_id = format!("menu-{}", index + 1);
        let raw_id = normalize_optional_text(Some(menu.id)).unwrap_or_else(|| fallback_id.clone());
        let base_id = sanitize_token(&raw_id, &fallback_id);

        let mut unique_id = base_id.clone();
        let mut suffix = 2;
        while seen_ids.contains(&unique_id) {
            unique_id = format!("{base_id}-{suffix}");
            suffix += 1;
        }
        seen_ids.insert(unique_id.clone());

        let label = normalize_optional_text(Some(menu.label))
            .unwrap_or_else(|| format!("Menu {}", index + 1));

        let items: Vec<String> = menu
            .items
            .into_iter()
            .filter_map(|item| normalize_optional_text(Some(item)))
            .collect();

        let item_count = items.len();
        let disabled_indices = normalize_disabled_indices(menu.disabled_indices, item_count);
        let item_kinds: Vec<MenuItemKind> = menu.item_kinds.into_iter().take(item_count).collect();

        let is_trigger_disabled = menu.disabled || item_count == 0;
        let ids = resolve_menu_ids(id_base, &unique_id);

        resolved.push(MenubarMenuResolved {
            id: unique_id,
            label,
            items: Arc::<[String]>::from(items),
            disabled_indices,
            item_kinds,
            is_trigger_disabled,
            has_items: item_count > 0,
            trigger_id: ids.trigger_id,
            menu_id: ids.menu_id,
        });
    }

    resolved
}

pub fn sanitize_open_index_for_menus(
    open_index: Option<usize>,
    menus: &[MenubarMenuResolved],
) -> Option<usize> {
    let index = normalize_open_index(open_index, menus.len())?;
    (!menus[index].is_trigger_disabled).then_some(index)
}

pub fn next_enabled_menu_index(
    menus: &[MenubarMenuResolved],
    current_index: usize,
    step: isize,
) -> Option<usize> {
    if menus.is_empty() || step == 0 {
        return None;
    }

    let len = menus.len() as isize;
    let mut cursor = current_index as isize;

    for _ in 0..menus.len().saturating_sub(1) {
        cursor = (cursor + step).rem_euclid(len);
        let index = cursor as usize;
        if !menus[index].is_trigger_disabled {
            return Some(index);
        }
    }

    None
}

pub fn resolve_state(input: MenubarStateInput) -> MenubarState {
    let has_menus = input.menu_count > 0;
    let has_open_menu = input.open_index.is_some();

    let data_state_attr = if !has_menus {
        "empty"
    } else if has_open_menu {
        "open"
    } else {
        "closed"
    };

    MenubarState {
        menu_count: input.menu_count,
        is_empty: !has_menus,
        has_menus,
        open_index: input.open_index,
        has_open_menu,
        has_disabled_menus: input.has_disabled_menus,
        has_custom_class_name: input.has_custom_class_name,
        is_controlled: input.is_controlled,
        is_uncontrolled: !input.is_controlled,
        placement: input.placement,
        placement_attr: input.placement.as_str(),
        data_state_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: MenubarState) -> String {
    let mut classes = vec![
        "ui-menubar".to_string(),
        format!("ui-menubar--placement-{}", state.placement_attr),
    ];

    if state.is_empty {
        classes.push("ui-menubar--empty".to_string());
    }
    if state.has_open_menu {
        classes.push("ui-menubar--open".to_string());
    }
    if state.has_disabled_menus {
        classes.push("ui-menubar--has-disabled-menus".to_string());
    }
    if state.is_controlled {
        classes.push("ui-menubar--controlled".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_id_base_falls_back_when_blank() {
        assert_eq!(
            normalize_id_base("  menubar-root  ".to_string()),
            "menubar-root"
        );
        assert_eq!(normalize_id_base("   ".to_string()), "menubar");
    }

    #[test]
    fn resolve_menus_normalizes_ids_items_and_disabled_indices() {
        let menus = resolve_menus(
            "docs-menubar",
            vec![
                MenubarMenu::new(
                    " File ",
                    " File ",
                    vec![
                        "New".to_string(),
                        "  ".to_string(),
                        "Open".to_string(),
                        "Save".to_string(),
                    ],
                )
                .disabled_indices(vec![2, 2, 6]),
                MenubarMenu::new("File", "Edit", vec!["Undo".to_string()]),
            ],
        );

        assert_eq!(menus.len(), 2);
        assert_eq!(menus[0].id, "file");
        assert_eq!(menus[1].id, "file-2");
        assert_eq!(menus[0].label, "File");
        assert_eq!(menus[0].items.len(), 3);
        assert_eq!(menus[0].disabled_indices, vec![2]);
        assert_eq!(menus[0].trigger_id, "docs-menubar-file-trigger");
        assert_eq!(menus[0].menu_id, "docs-menubar-file-menu");
    }

    #[test]
    fn disabled_or_empty_menu_disables_trigger() {
        let menus = resolve_menus(
            "docs-menubar",
            vec![
                MenubarMenu::new("view", "View", Vec::new()),
                MenubarMenu::new("help", "Help", vec!["About".to_string()]).disabled(true),
            ],
        );

        assert!(menus[0].is_trigger_disabled);
        assert!(menus[1].is_trigger_disabled);
    }

    #[test]
    fn next_enabled_menu_skips_disabled_and_wraps() {
        let menus = resolve_menus(
            "docs-menubar",
            vec![
                MenubarMenu::new("file", "File", vec!["New".to_string()]),
                MenubarMenu::new("edit", "Edit", vec!["Undo".to_string()]).disabled(true),
                MenubarMenu::new("view", "View", vec!["Zoom".to_string()]),
            ],
        );

        assert_eq!(next_enabled_menu_index(&menus, 0, 1), Some(2));
        assert_eq!(next_enabled_menu_index(&menus, 2, 1), Some(0));
        assert_eq!(next_enabled_menu_index(&menus, 0, -1), Some(2));
    }

    #[test]
    fn sanitize_open_index_rejects_invalid_or_disabled_menu() {
        let menus = resolve_menus(
            "docs-menubar",
            vec![
                MenubarMenu::new("file", "File", vec!["New".to_string()]),
                MenubarMenu::new("help", "Help", vec!["About".to_string()]).disabled(true),
            ],
        );

        assert_eq!(sanitize_open_index_for_menus(Some(0), &menus), Some(0));
        assert_eq!(sanitize_open_index_for_menus(Some(1), &menus), None);
        assert_eq!(sanitize_open_index_for_menus(Some(8), &menus), None);
    }

    #[test]
    fn focus_strategy_maps_arrow_open_keys() {
        assert_eq!(
            focus_strategy_for_open_key("ArrowDown"),
            Some(MenuOpenFocusStrategy::First)
        );
        assert_eq!(
            focus_strategy_for_open_key("ArrowUp"),
            Some(MenuOpenFocusStrategy::Last)
        );
        assert_eq!(focus_strategy_for_open_key("Enter"), None);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(MenubarStateInput {
                menu_count: 2,
                open_index: Some(1),
                has_disabled_menus: true,
                has_custom_class_name: true,
                is_controlled: true,
                placement: PopoverPlacement::BottomStart,
            }),
        );

        for token in [
            "ui-menubar",
            "ui-menubar--placement-bottom-start",
            "ui-menubar--open",
            "ui-menubar--has-disabled-menus",
            "ui-menubar--controlled",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
