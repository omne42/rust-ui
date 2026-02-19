use std::collections::BTreeSet;

use ui_headless::PopoverPlacement;

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
pub struct DropdownMenuIds {
    pub trigger_id: String,
    pub menu_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropdownMenuStateInput {
    pub item_count: usize,
    pub trigger_disabled: bool,
    pub close_on_action: bool,
    pub has_custom_class_name: bool,
    pub has_disabled_items: bool,
    pub has_item_kinds: bool,
    pub is_controlled: bool,
    pub placement: PopoverPlacement,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropdownMenuState {
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_trigger_disabled: bool,
    pub is_enabled: bool,
    pub close_on_action: bool,
    pub keep_open_on_action: bool,
    pub has_custom_class_name: bool,
    pub has_disabled_items: bool,
    pub has_item_kinds: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub placement: PopoverPlacement,
    pub placement_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| "dropdown-menu".to_string())
}

pub fn resolve_ids(id_base: &str) -> DropdownMenuIds {
    DropdownMenuIds {
        trigger_id: format!("{id_base}-trigger"),
        menu_id: format!("{id_base}-menu"),
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

pub fn resolve_trigger_disabled(disabled: bool, item_count: usize) -> bool {
    disabled || item_count == 0
}

pub fn resolve_state(input: DropdownMenuStateInput) -> DropdownMenuState {
    DropdownMenuState {
        item_count: input.item_count,
        is_empty: input.item_count == 0,
        has_items: input.item_count > 0,
        is_trigger_disabled: input.trigger_disabled,
        is_enabled: !input.trigger_disabled,
        close_on_action: input.close_on_action,
        keep_open_on_action: !input.close_on_action,
        has_custom_class_name: input.has_custom_class_name,
        has_disabled_items: input.has_disabled_items,
        has_item_kinds: input.has_item_kinds,
        is_controlled: input.is_controlled,
        is_uncontrolled: !input.is_controlled,
        placement: input.placement,
        placement_attr: input.placement.as_str(),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: DropdownMenuState) -> String {
    let mut classes = vec![
        "ui-dropdown-menu".to_string(),
        format!("ui-dropdown-menu--placement-{}", state.placement_attr),
    ];

    if state.is_trigger_disabled {
        classes.push("ui-dropdown-menu--disabled".to_string());
    }
    if state.has_items {
        classes.push("ui-dropdown-menu--has-items".to_string());
    }
    if state.is_empty {
        classes.push("ui-dropdown-menu--empty".to_string());
    }
    if state.keep_open_on_action {
        classes.push("ui-dropdown-menu--persistent".to_string());
    }
    if state.is_controlled {
        classes.push("ui-dropdown-menu--controlled".to_string());
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
    fn menu_id_derives_from_base() {
        let ids = resolve_ids("demo");
        assert_eq!(ids.trigger_id, "demo-trigger");
        assert_eq!(ids.menu_id, "demo-menu");
    }

    #[test]
    fn normalize_id_base_falls_back_when_blank() {
        assert_eq!(
            normalize_id_base("  demo-dropdown  ".to_string()),
            "demo-dropdown"
        );
        assert_eq!(normalize_id_base("   ".to_string()), "dropdown-menu");
    }

    #[test]
    fn disabled_indices_are_deduped_and_clamped_to_item_count() {
        assert_eq!(normalize_disabled_indices(vec![2, 1, 1, 9], 3), vec![1, 2]);
        assert_eq!(normalize_disabled_indices(vec![4], 0), Vec::<usize>::new());
    }

    #[test]
    fn focus_strategy_for_open_key_maps_arrow_keys() {
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
    fn focus_strategy_default_index() {
        assert_eq!(MenuOpenFocusStrategy::First.default_index(4), 0);
        assert_eq!(MenuOpenFocusStrategy::Last.default_index(4), 3);
        assert_eq!(MenuOpenFocusStrategy::Last.default_index(0), 0);
    }

    #[test]
    fn trigger_disabled_when_component_or_items_disabled() {
        assert!(resolve_trigger_disabled(true, 3));
        assert!(resolve_trigger_disabled(false, 0));
        assert!(!resolve_trigger_disabled(false, 2));
    }

    #[test]
    fn resolve_state_tracks_trigger_items_and_strategy_flags() {
        let state = resolve_state(DropdownMenuStateInput {
            item_count: 3,
            trigger_disabled: false,
            close_on_action: false,
            has_custom_class_name: true,
            has_disabled_items: true,
            has_item_kinds: true,
            is_controlled: true,
            placement: PopoverPlacement::TopEnd,
        });

        assert_eq!(state.item_count, 3);
        assert!(state.has_items);
        assert!(!state.is_empty);
        assert!(!state.is_trigger_disabled);
        assert!(state.is_enabled);
        assert!(!state.close_on_action);
        assert!(state.keep_open_on_action);
        assert!(state.has_custom_class_name);
        assert!(state.has_disabled_items);
        assert!(state.has_item_kinds);
        assert!(state.is_controlled);
        assert!(!state.is_uncontrolled);
        assert_eq!(state.placement_attr, "top-end");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(DropdownMenuStateInput {
                item_count: 0,
                trigger_disabled: true,
                close_on_action: false,
                has_custom_class_name: true,
                has_disabled_items: false,
                has_item_kinds: false,
                is_controlled: true,
                placement: PopoverPlacement::BottomStart,
            }),
        );

        for token in [
            "ui-dropdown-menu",
            "ui-dropdown-menu--placement-bottom-start",
            "ui-dropdown-menu--disabled",
            "ui-dropdown-menu--empty",
            "ui-dropdown-menu--persistent",
            "ui-dropdown-menu--controlled",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
