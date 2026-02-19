use std::collections::BTreeSet;

pub const DEFAULT_ID_BASE: &str = "dropdown";
pub const DEFAULT_ARIA_LABEL: &str = "Open dropdown";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DropdownOpenFocusStrategy {
    #[default]
    First,
    Last,
}

impl DropdownOpenFocusStrategy {
    pub fn default_index(self, item_count: usize) -> usize {
        match self {
            Self::First => 0,
            Self::Last => item_count.saturating_sub(1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropdownStateInput {
    pub item_count: usize,
    pub disabled: bool,
    pub close_on_action: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub is_controlled: bool,
    pub has_disabled_items: bool,
    pub has_item_kinds: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropdownState {
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_disabled: bool,
    pub close_on_action: bool,
    pub keep_open_on_action: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_disabled_items: bool,
    pub has_item_kinds: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn focus_strategy_for_open_key(key: &str) -> Option<DropdownOpenFocusStrategy> {
    match key {
        "ArrowDown" => Some(DropdownOpenFocusStrategy::First),
        "ArrowUp" => Some(DropdownOpenFocusStrategy::Last),
        _ => None,
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| DEFAULT_ID_BASE.into())
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
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

pub fn resolve_state(input: DropdownStateInput) -> DropdownState {
    let data_state_attr = if input.disabled {
        "disabled"
    } else if input.item_count == 0 {
        "empty"
    } else if input.close_on_action {
        "close-on-action"
    } else {
        "persistent"
    };

    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    DropdownState {
        item_count: input.item_count,
        is_empty: input.item_count == 0,
        has_items: input.item_count > 0,
        is_disabled: input.disabled,
        close_on_action: input.close_on_action,
        keep_open_on_action: !input.close_on_action,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_disabled_items: input.has_disabled_items,
        has_item_kinds: input.has_item_kinds,
        is_controlled: input.is_controlled,
        is_uncontrolled: !input.is_controlled,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_id_base_trims_or_falls_back() {
        assert_eq!(
            normalize_id_base("  docs-dropdown  ".to_string()),
            "docs-dropdown"
        );
        assert_eq!(normalize_id_base("   ".to_string()), DEFAULT_ID_BASE);
    }

    #[test]
    fn normalize_aria_label_trims_or_falls_back() {
        assert_eq!(
            normalize_aria_label(Some("  Actions menu  ".to_string())),
            ("Actions menu".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(Some("\n\t".to_string())),
            (DEFAULT_ARIA_LABEL.into(), false)
        );
    }

    #[test]
    fn normalize_disabled_indices_dedupes_and_clamps() {
        assert_eq!(normalize_disabled_indices(vec![2, 1, 1, 9], 3), vec![1, 2]);
        assert_eq!(normalize_disabled_indices(vec![4], 0), Vec::<usize>::new());
    }

    #[test]
    fn focus_strategy_for_open_key_maps_arrow_keys() {
        assert_eq!(
            focus_strategy_for_open_key("ArrowDown"),
            Some(DropdownOpenFocusStrategy::First)
        );
        assert_eq!(
            focus_strategy_for_open_key("ArrowUp"),
            Some(DropdownOpenFocusStrategy::Last)
        );
        assert_eq!(focus_strategy_for_open_key("Enter"), None);
    }

    #[test]
    fn resolve_state_tracks_state_and_sources() {
        let state = resolve_state(DropdownStateInput {
            item_count: 3,
            disabled: false,
            close_on_action: false,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            is_controlled: true,
            has_disabled_items: true,
            has_item_kinds: true,
        });

        assert_eq!(state.data_state_attr, "persistent");
        assert!(state.has_items);
        assert!(!state.is_empty);
        assert!(state.keep_open_on_action);
        assert!(state.is_controlled);
        assert!(!state.is_uncontrolled);
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }
}
