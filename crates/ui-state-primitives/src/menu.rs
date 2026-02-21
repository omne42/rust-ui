use std::collections::BTreeSet;

pub const DEFAULT_MENU_ARIA_LABEL: &str = "Menu";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuAccessibleName {
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuState {
    pub is_empty: bool,
    pub has_items: bool,
    pub has_checked_items: bool,
    pub has_disabled_items: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavigationItemInput {
    pub id: String,
    pub label: String,
    pub href: String,
    pub disabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavigationItemResolved {
    pub id: String,
    pub dom_id: String,
    pub label: String,
    pub href: String,
    pub disabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenubarMenuInput {
    pub id: String,
    pub label: String,
    pub items: Vec<String>,
    pub disabled_indices: Vec<usize>,
    pub disabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenubarMenuResolved {
    pub id: String,
    pub label: String,
    pub items: Vec<String>,
    pub disabled_indices: Vec<usize>,
    pub is_trigger_disabled: bool,
    pub has_items: bool,
    pub trigger_id: String,
    pub menu_id: String,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_controlled_prop_alias<T>(is_value: Option<T>, value: Option<T>) -> Option<T> {
    is_value.or(value)
}

pub fn is_controlled_prop<T>(value: &Option<T>) -> bool {
    value.is_some()
}

pub fn normalize_id_base(id_base: String, default_id_base: &str) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| default_id_base.into())
}

pub fn resolve_id_pair(id_base: &str) -> (String, String) {
    (format!("{id_base}-trigger"), format!("{id_base}-menu"))
}

pub fn resolve_aria_label_with_fallback(
    value: Option<String>,
    fallback_aria_label: &str,
    default_aria_label: &str,
) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    let fallback = fallback_aria_label.trim();
    if fallback.is_empty() {
        (default_aria_label.trim().into(), false)
    } else {
        (fallback.into(), false)
    }
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

pub fn normalize_index(index: Option<usize>, item_count: usize) -> Option<usize> {
    index.filter(|index| *index < item_count)
}

pub fn resolve_trigger_disabled(disabled: bool, item_count: usize) -> bool {
    disabled || item_count == 0
}

pub fn item_attr(item_count: usize) -> &'static str {
    if item_count == 0 {
        "empty"
    } else {
        "populated"
    }
}

pub fn action_attr(close_on_action: bool) -> &'static str {
    if close_on_action {
        "close"
    } else {
        "keep-open"
    }
}

pub fn open_mode_attr(is_controlled: bool) -> &'static str {
    if is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    }
}

pub fn disabled_attr(disabled: bool) -> &'static str {
    if disabled { "true" } else { "false" }
}

pub fn context_state_attr(is_open: bool, trigger_disabled: bool) -> &'static str {
    if is_open {
        "open"
    } else if trigger_disabled {
        "disabled"
    } else {
        "closed"
    }
}

pub fn menubar_state_attr(menu_count: usize, has_open_menu: bool) -> &'static str {
    if menu_count == 0 {
        "empty"
    } else if has_open_menu {
        "open"
    } else {
        "closed"
    }
}

pub fn menubar_menu_attr(menu_count: usize) -> &'static str {
    if menu_count == 0 {
        "empty"
    } else {
        "populated"
    }
}

pub fn navigation_state_attr(
    item_count: usize,
    has_selection: bool,
    has_focus: bool,
) -> &'static str {
    if item_count == 0 {
        "empty"
    } else if has_selection {
        "selected"
    } else if has_focus {
        "focused"
    } else {
        "idle"
    }
}

pub fn selected_attr(has_selection: bool) -> &'static str {
    if has_selection { "present" } else { "absent" }
}

pub fn focus_attr(has_focus: bool) -> &'static str {
    if has_focus { "present" } else { "absent" }
}

pub fn focus_activation_attr(activate_on_focus: bool) -> &'static str {
    if activate_on_focus { "auto" } else { "manual" }
}

pub fn resolve_menu_accessible_name(
    aria_label: Option<String>,
    aria_labelledby: Option<String>,
    default_aria_label: &str,
) -> MenuAccessibleName {
    let aria_label = normalize_optional_text(aria_label);
    let aria_labelledby = normalize_optional_text(aria_labelledby);

    if aria_label.is_some() {
        return MenuAccessibleName {
            aria_label,
            aria_labelledby: None,
        };
    }

    if aria_labelledby.is_some() {
        return MenuAccessibleName {
            aria_label: None,
            aria_labelledby,
        };
    }

    let fallback = default_aria_label.trim();
    let aria_label = if fallback.is_empty() {
        DEFAULT_MENU_ARIA_LABEL.into()
    } else {
        fallback.into()
    };

    MenuAccessibleName {
        aria_label: Some(aria_label),
        aria_labelledby: None,
    }
}

pub fn resolve_menu_state(
    item_count: usize,
    has_checked_items: bool,
    has_disabled_items: bool,
) -> MenuState {
    let has_items = item_count > 0;

    MenuState {
        is_empty: !has_items,
        has_items,
        has_checked_items,
        has_disabled_items,
    }
}

pub fn first_enabled_index(disabled_flags: &[bool]) -> Option<usize> {
    disabled_flags.iter().position(|disabled| !*disabled)
}

pub fn last_enabled_index(disabled_flags: &[bool]) -> Option<usize> {
    disabled_flags.iter().rposition(|disabled| !*disabled)
}

pub fn sanitize_enabled_index(index: Option<usize>, disabled_flags: &[bool]) -> Option<usize> {
    let index = normalize_index(index, disabled_flags.len())?;
    (!disabled_flags[index]).then_some(index)
}

pub fn next_enabled_index(
    disabled_flags: &[bool],
    current_index: usize,
    step: isize,
) -> Option<usize> {
    if disabled_flags.is_empty() || step == 0 {
        return None;
    }

    let len = disabled_flags.len() as isize;
    let mut cursor = current_index as isize;

    for _ in 0..disabled_flags.len().saturating_sub(1) {
        cursor = (cursor + step).rem_euclid(len);
        let index = cursor as usize;
        if !disabled_flags[index] {
            return Some(index);
        }
    }

    None
}

pub fn resolve_initial_focus_index(
    selected_index: Option<usize>,
    disabled_flags: &[bool],
) -> Option<usize> {
    sanitize_enabled_index(selected_index, disabled_flags)
        .or_else(|| first_enabled_index(disabled_flags))
}

pub fn sanitize_token(value: &str, fallback: &str) -> String {
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
        return fallback.into();
    }

    out
}

pub fn resolve_navigation_items(
    id_base: &str,
    items: Vec<NavigationItemInput>,
) -> Vec<NavigationItemResolved> {
    let mut seen_ids = BTreeSet::new();
    let mut resolved = Vec::with_capacity(items.len());

    for (index, item) in items.into_iter().enumerate() {
        let fallback_id = format!("item-{}", index + 1);
        let raw_id = normalize_optional_text(Some(item.id)).unwrap_or_else(|| fallback_id.clone());
        let base_id = sanitize_token(&raw_id, &fallback_id);

        let mut unique_id = base_id.clone();
        let mut suffix = 2;
        while seen_ids.contains(&unique_id) {
            unique_id = format!("{base_id}-{suffix}");
            suffix += 1;
        }
        seen_ids.insert(unique_id.clone());

        let label = normalize_optional_text(Some(item.label))
            .unwrap_or_else(|| format!("Item {}", index + 1));
        let href = normalize_optional_text(Some(item.href)).unwrap_or_else(|| "#".to_string());
        let dom_id = format!("{id_base}-{unique_id}");

        resolved.push(NavigationItemResolved {
            id: unique_id,
            dom_id,
            label,
            href,
            disabled: item.disabled,
        });
    }

    resolved
}

pub fn selected_index_for_id(
    items: &[NavigationItemResolved],
    selected_id: Option<String>,
) -> Option<usize> {
    let selected_id = normalize_optional_text(selected_id)?;
    items
        .iter()
        .position(|item| item.id == selected_id && !item.disabled)
}

pub fn sanitize_selected_id(
    selected_id: Option<String>,
    items: &[NavigationItemResolved],
) -> Option<String> {
    let selected_id = normalize_optional_text(selected_id)?;
    items
        .iter()
        .find(|item| item.id == selected_id && !item.disabled)
        .map(|item| item.id.clone())
}

pub fn resolve_menubar_menu_ids(id_base: &str, menu_id: &str) -> (String, String) {
    (
        format!("{id_base}-{menu_id}-trigger"),
        format!("{id_base}-{menu_id}-menu"),
    )
}

pub fn resolve_menubar_menus(
    id_base: &str,
    menus: Vec<MenubarMenuInput>,
) -> Vec<MenubarMenuResolved> {
    let mut seen_ids = BTreeSet::new();
    let mut resolved = Vec::with_capacity(menus.len());

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
        let is_trigger_disabled = menu.disabled || item_count == 0;
        let (trigger_id, menu_id) = resolve_menubar_menu_ids(id_base, &unique_id);

        resolved.push(MenubarMenuResolved {
            id: unique_id,
            label,
            items,
            disabled_indices,
            is_trigger_disabled,
            has_items: item_count > 0,
            trigger_id,
            menu_id,
        });
    }

    resolved
}

pub fn sanitize_open_index_for_trigger_disabled(
    open_index: Option<usize>,
    trigger_disabled: &[bool],
) -> Option<usize> {
    let index = normalize_index(open_index, trigger_disabled.len())?;
    (!trigger_disabled[index]).then_some(index)
}

#[cfg(test)]
#[path = "test/menu.rs"]
mod tests;
