use std::collections::BTreeSet;

use crate::navigation_menu::{
    NavigationMenuItem, NavigationMenuItemResolved, NavigationMenuPartState,
    NavigationMenuPartStateInput, NavigationMenuSlot,
};

pub const DEFAULT_ID_BASE: &str = "navigation-menu";
pub const DEFAULT_ARIA_LABEL: &str = "Main navigation";
pub const DEFAULT_ACTIVATE_ON_FOCUS: bool = true;

pub fn state_attr(item_count: usize, has_selection: bool, has_focus: bool) -> &'static str {
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

pub fn item_attr(item_count: usize) -> &'static str {
    if item_count == 0 {
        "empty"
    } else {
        "populated"
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

pub fn selection_mode_attr(is_controlled: bool) -> &'static str {
    if is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| DEFAULT_ID_BASE.to_string())
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

    (DEFAULT_ARIA_LABEL.to_string(), false)
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

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: NavigationMenuPartStateInput) -> NavigationMenuPartState {
    let has_items = input.item_count > 0;
    let is_empty = !has_items;
    let has_selection = input.selected_index.is_some();
    let has_focus = input.focused_index.is_some();

    NavigationMenuPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.item_count, has_selection, has_focus),
        item_attr: item_attr(input.item_count),
        selected_attr: selected_attr(has_selection),
        focus_attr: focus_attr(has_focus),
        focus_activation_attr: focus_activation_attr(input.activate_on_focus),
        selection_mode_attr: selection_mode_attr(input.is_controlled),
        open_attr: has_selection.then_some("true"),
        closed_attr: (!has_selection).then_some("true"),
        item_count: input.item_count,
        selected_index: input.selected_index,
        focused_index: input.focused_index,
        is_empty,
        has_items,
        has_selection,
        has_focus,
        has_disabled_items: input.has_disabled_items,
        activate_on_focus: input.activate_on_focus,
        is_controlled: input.is_controlled,
        is_uncontrolled: !input.is_controlled,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_activate_on_focus: input.has_custom_activate_on_focus,
        has_custom_selected_id: input.has_custom_selected_id,
        has_custom_default_selected_id: input.has_custom_default_selected_id,
        has_custom_on_selected_id_change: input.has_custom_on_selected_id_change,
        has_custom_motion: input.has_custom_motion,
        id_source_attr: source_attr(input.has_custom_id_base),
        aria_label_source_attr: source_attr(input.has_custom_aria_label),
        class_source_attr: source_attr(input.has_custom_class_name),
        activate_on_focus_source_attr: source_attr(input.has_custom_activate_on_focus),
        selected_id_source_attr: source_attr(input.has_custom_selected_id),
        default_selected_id_source_attr: source_attr(input.has_custom_default_selected_id),
        selected_id_change_source_attr: source_attr(input.has_custom_on_selected_id_change),
        motion_source_attr: source_attr(input.has_custom_motion),
    }
}

pub fn compose_class_name(
    base_class_name: Option<String>,
    state: NavigationMenuPartState,
) -> String {
    let mut classes = vec![state.base_class.to_string()];

    if matches!(state.slot, NavigationMenuSlot::Root) {
        if state.is_empty {
            classes.push("ui-navigation-menu--empty".to_string());
        } else {
            classes.push("ui-navigation-menu--has-items".to_string());
        }

        if state.has_selection {
            classes.push("ui-navigation-menu--selected".to_string());
        } else {
            classes.push("ui-navigation-menu--unselected".to_string());
        }

        if state.has_focus {
            classes.push("ui-navigation-menu--focused".to_string());
        }

        if state.has_disabled_items {
            classes.push("ui-navigation-menu--has-disabled-items".to_string());
        }

        if state.activate_on_focus {
            classes.push("ui-navigation-menu--auto-activation".to_string());
        } else {
            classes.push("ui-navigation-menu--manual-activation".to_string());
        }

        if state.is_controlled {
            classes.push("ui-navigation-menu--controlled".to_string());
        } else {
            classes.push("ui-navigation-menu--uncontrolled".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-navigation-menu--custom-motion".to_string());
        }

        if state.has_custom_id_base {
            classes.push("ui-navigation-menu--custom-id".to_string());
        }

        if state.has_custom_aria_label {
            classes.push("ui-navigation-menu--custom-aria-label".to_string());
        }

        if state.has_custom_activate_on_focus {
            classes.push("ui-navigation-menu--custom-activate-on-focus".to_string());
        }

        if state.has_custom_selected_id {
            classes.push("ui-navigation-menu--custom-selected-id".to_string());
        }

        if state.has_custom_default_selected_id {
            classes.push("ui-navigation-menu--custom-default-selected-id".to_string());
        }

        if state.has_custom_on_selected_id_change {
            classes.push("ui-navigation-menu--custom-selected-id-change".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-navigation-menu--custom-class".to_string());
            if let Some(base_class_name) = normalize_optional_text(base_class_name) {
                classes.push(base_class_name);
            }
        }
    } else if let Some(base_class_name) = normalize_optional_text(base_class_name) {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::navigation_menu::{NavigationMenuPartStateInput, NavigationMenuSlot};

    #[test]
    fn id_base_and_aria_label_have_stable_defaults() {
        assert_eq!(
            normalize_id_base("  primary-nav  ".to_string()),
            "primary-nav"
        );
        assert_eq!(normalize_id_base(" ".to_string()), DEFAULT_ID_BASE);

        assert_eq!(
            resolve_aria_label(None),
            (DEFAULT_ARIA_LABEL.to_string(), false)
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
    fn resolve_state_tracks_source_and_selection_contracts() {
        let state = resolve_state(NavigationMenuPartStateInput {
            slot: NavigationMenuSlot::Root,
            item_count: 3,
            selected_index: Some(1),
            focused_index: Some(1),
            has_disabled_items: true,
            activate_on_focus: false,
            is_controlled: true,
            has_custom_id_base: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            has_custom_activate_on_focus: true,
            has_custom_selected_id: true,
            has_custom_default_selected_id: true,
            has_custom_on_selected_id_change: true,
            has_custom_motion: true,
        });

        assert_eq!(state.slot_attr, "navigation-menu");
        assert_eq!(state.state_attr, "selected");
        assert_eq!(state.item_attr, "populated");
        assert_eq!(state.selected_attr, "present");
        assert_eq!(state.focus_attr, "present");
        assert_eq!(state.focus_activation_attr, "manual");
        assert_eq!(state.selection_mode_attr, "controlled");
        assert_eq!(state.id_source_attr, "custom");
        assert_eq!(state.aria_label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.activate_on_focus_source_attr, "custom");
        assert_eq!(state.selected_id_source_attr, "custom");
        assert_eq!(state.default_selected_id_source_attr, "custom");
        assert_eq!(state.selected_id_change_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(NavigationMenuPartStateInput {
                slot: NavigationMenuSlot::Root,
                item_count: 3,
                selected_index: Some(1),
                focused_index: Some(1),
                has_disabled_items: true,
                activate_on_focus: false,
                is_controlled: true,
                has_custom_id_base: false,
                has_custom_aria_label: true,
                has_custom_class_name: true,
                has_custom_activate_on_focus: true,
                has_custom_selected_id: true,
                has_custom_default_selected_id: false,
                has_custom_on_selected_id_change: true,
                has_custom_motion: true,
            }),
        );

        for token in [
            "ui-navigation-menu",
            "ui-navigation-menu--has-items",
            "ui-navigation-menu--selected",
            "ui-navigation-menu--focused",
            "ui-navigation-menu--has-disabled-items",
            "ui-navigation-menu--manual-activation",
            "ui-navigation-menu--controlled",
            "ui-navigation-menu--custom-motion",
            "ui-navigation-menu--custom-aria-label",
            "ui-navigation-menu--custom-activate-on-focus",
            "ui-navigation-menu--custom-selected-id",
            "ui-navigation-menu--custom-selected-id-change",
            "ui-navigation-menu--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
