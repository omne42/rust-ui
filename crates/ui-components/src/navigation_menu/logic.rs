use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavigationMenuItem {
    pub id: String,
    pub label: String,
    pub href: String,
    pub disabled: bool,
}

impl NavigationMenuItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            href: href.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavigationMenuItemResolved {
    pub id: String,
    pub dom_id: String,
    pub label: String,
    pub href: String,
    pub disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NavigationMenuStateInput {
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub focused_index: Option<usize>,
    pub has_disabled_items: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NavigationMenuState {
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub selected_index: Option<usize>,
    pub focused_index: Option<usize>,
    pub has_selection: bool,
    pub has_focus: bool,
    pub has_disabled_items: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub data_state_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| "navigation-menu".to_string())
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

pub fn resolve_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    ("Main navigation".to_string(), false)
}

pub fn resolve_items(
    id_base: &str,
    items: Vec<NavigationMenuItem>,
) -> Vec<NavigationMenuItemResolved> {
    let mut seen_ids = BTreeSet::new();
    let mut resolved = Vec::new();

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

        resolved.push(NavigationMenuItemResolved {
            id: unique_id,
            dom_id,
            label,
            href,
            disabled: item.disabled,
        });
    }

    resolved
}

pub fn first_enabled_index(items: &[NavigationMenuItemResolved]) -> Option<usize> {
    items.iter().position(|item| !item.disabled)
}

pub fn last_enabled_index(items: &[NavigationMenuItemResolved]) -> Option<usize> {
    items.iter().rposition(|item| !item.disabled)
}

pub fn next_enabled_index(
    items: &[NavigationMenuItemResolved],
    current_index: usize,
    step: isize,
) -> Option<usize> {
    if items.is_empty() || step == 0 {
        return None;
    }

    let len = items.len() as isize;
    let mut cursor = current_index as isize;

    for _ in 0..items.len().saturating_sub(1) {
        cursor = (cursor + step).rem_euclid(len);
        let index = cursor as usize;
        if !items[index].disabled {
            return Some(index);
        }
    }

    None
}

pub fn selected_index_for_id(
    items: &[NavigationMenuItemResolved],
    selected_id: Option<String>,
) -> Option<usize> {
    let selected_id = selected_id?;
    items
        .iter()
        .position(|item| item.id == selected_id && !item.disabled)
}

pub fn sanitize_selected_id(
    selected_id: Option<String>,
    items: &[NavigationMenuItemResolved],
) -> Option<String> {
    let selected_id = normalize_optional_text(selected_id)?;
    items
        .iter()
        .find(|item| item.id == selected_id && !item.disabled)
        .map(|item| item.id.clone())
}

pub fn sanitize_focused_index(
    focused_index: Option<usize>,
    items: &[NavigationMenuItemResolved],
) -> Option<usize> {
    let index = focused_index?;
    if index >= items.len() || items[index].disabled {
        return None;
    }

    Some(index)
}

pub fn resolve_initial_focus_index(
    items: &[NavigationMenuItemResolved],
    selected_index: Option<usize>,
) -> Option<usize> {
    sanitize_focused_index(selected_index, items).or_else(|| first_enabled_index(items))
}

pub fn resolve_state(input: NavigationMenuStateInput) -> NavigationMenuState {
    let has_items = input.item_count > 0;
    let has_selection = input.selected_index.is_some();
    let has_focus = input.focused_index.is_some();

    let data_state_attr = if !has_items {
        "empty"
    } else if has_selection {
        "selected"
    } else if has_focus {
        "focused"
    } else {
        "idle"
    };

    NavigationMenuState {
        item_count: input.item_count,
        is_empty: !has_items,
        has_items,
        selected_index: input.selected_index,
        focused_index: input.focused_index,
        has_selection,
        has_focus,
        has_disabled_items: input.has_disabled_items,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        data_state_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: NavigationMenuState) -> String {
    let mut classes = vec!["ui-navigation-menu".to_string()];

    if state.is_empty {
        classes.push("ui-navigation-menu--empty".to_string());
    }
    if state.has_selection {
        classes.push("ui-navigation-menu--selected".to_string());
    }
    if state.has_disabled_items {
        classes.push("ui-navigation-menu--has-disabled-items".to_string());
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
    fn id_base_and_aria_label_have_stable_defaults() {
        assert_eq!(
            normalize_id_base("  primary-nav  ".to_string()),
            "primary-nav"
        );
        assert_eq!(normalize_id_base(" ".to_string()), "navigation-menu");

        assert_eq!(
            resolve_aria_label(None),
            ("Main navigation".to_string(), false)
        );
        assert_eq!(
            resolve_aria_label(Some("  App sections  ".to_string())),
            ("App sections".to_string(), true)
        );
    }

    #[test]
    fn resolve_items_normalizes_ids_labels_and_href() {
        let items = resolve_items(
            "docs-nav",
            vec![
                NavigationMenuItem::new("Docs", "Docs", "/docs"),
                NavigationMenuItem::new("Docs", "", " "),
                NavigationMenuItem::new(" ", "Blog", "/blog"),
            ],
        );

        assert_eq!(items.len(), 3);
        assert_eq!(items[0].id, "docs");
        assert_eq!(items[1].id, "docs-2");
        assert_eq!(items[2].id, "item-3");
        assert_eq!(items[1].label, "Item 2");
        assert_eq!(items[1].href, "#");
        assert_eq!(items[0].dom_id, "docs-nav-docs");
    }

    #[test]
    fn enabled_index_helpers_skip_disabled_items() {
        let items = vec![
            NavigationMenuItemResolved {
                id: "a".to_string(),
                dom_id: "a".to_string(),
                label: "A".to_string(),
                href: "/a".to_string(),
                disabled: false,
            },
            NavigationMenuItemResolved {
                id: "b".to_string(),
                dom_id: "b".to_string(),
                label: "B".to_string(),
                href: "/b".to_string(),
                disabled: true,
            },
            NavigationMenuItemResolved {
                id: "c".to_string(),
                dom_id: "c".to_string(),
                label: "C".to_string(),
                href: "/c".to_string(),
                disabled: false,
            },
        ];

        assert_eq!(first_enabled_index(&items), Some(0));
        assert_eq!(last_enabled_index(&items), Some(2));
        assert_eq!(next_enabled_index(&items, 0, 1), Some(2));
        assert_eq!(next_enabled_index(&items, 2, -1), Some(0));
    }

    #[test]
    fn selected_id_and_focus_are_sanitized() {
        let items = resolve_items(
            "docs-nav",
            vec![
                NavigationMenuItem::new("home", "Home", "/").disabled(true),
                NavigationMenuItem::new("docs", "Docs", "/docs"),
            ],
        );

        assert_eq!(
            sanitize_selected_id(Some("docs".to_string()), &items),
            Some("docs".to_string())
        );
        assert_eq!(sanitize_selected_id(Some("home".to_string()), &items), None);
        assert_eq!(
            selected_index_for_id(&items, Some("docs".to_string())),
            Some(1)
        );
        assert_eq!(
            selected_index_for_id(&items, Some("home".to_string())),
            None
        );
        assert_eq!(sanitize_focused_index(Some(1), &items), Some(1));
        assert_eq!(sanitize_focused_index(Some(0), &items), None);
        assert_eq!(resolve_initial_focus_index(&items, Some(0)), Some(1));
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(NavigationMenuStateInput {
                item_count: 3,
                selected_index: Some(1),
                focused_index: Some(1),
                has_disabled_items: true,
                has_custom_aria_label: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-navigation-menu",
            "ui-navigation-menu--selected",
            "ui-navigation-menu--has-disabled-items",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
