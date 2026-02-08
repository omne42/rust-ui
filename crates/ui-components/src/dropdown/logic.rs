use crate::dropdown::{DropdownState, DropdownStateInput};
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
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| DEFAULT_ID_BASE.to_string())
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
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

pub fn compose_class_name(base_class_name: Option<String>, state: DropdownState) -> String {
    let mut classes = vec!["ui-dropdown".to_string()];

    if state.is_disabled {
        classes.push("ui-dropdown--disabled".to_string());
    }
    if state.has_items {
        classes.push("ui-dropdown--has-items".to_string());
    }
    if state.is_empty {
        classes.push("ui-dropdown--empty".to_string());
    }
    if state.keep_open_on_action {
        classes.push("ui-dropdown--persistent".to_string());
    }
    if state.is_controlled {
        classes.push("ui-dropdown--controlled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-dropdown--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dropdown::DropdownStateInput;

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
            (DEFAULT_ARIA_LABEL.to_string(), false)
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

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(DropdownStateInput {
            item_count: 0,
            disabled: true,
            close_on_action: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            is_controlled: false,
            has_disabled_items: false,
            has_item_kinds: false,
        });

        let class_name = compose_class_name(Some("docs-dropdown-custom".to_string()), state);

        assert!(class_name.contains("ui-dropdown"));
        assert!(class_name.contains("ui-dropdown--disabled"));
        assert!(class_name.contains("ui-dropdown--empty"));
        assert!(class_name.contains("ui-dropdown--custom-class"));
        assert!(class_name.contains("docs-dropdown-custom"));
    }
}
