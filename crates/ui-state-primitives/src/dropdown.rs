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
#[path = "test/dropdown.rs"]
mod tests;
