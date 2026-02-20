use super::{DisclosureGroupState, DisclosureGroupStateInput};
use std::collections::BTreeSet;

pub const DEFAULT_ARIA_LABEL: &str = "DisclosureGroup";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisclosureGroupExpandedAxisState {
    pub control_mode_attr: &'static str,
    pub default_expanded_source_attr: &'static str,
    pub is_controlled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DisclosureGroupSelectionMode {
    Single,
    #[default]
    Multiple,
}

impl DisclosureGroupSelectionMode {
    pub fn class_name(self) -> &'static str {
        match self {
            DisclosureGroupSelectionMode::Single => "ui-disclosure-group--selection-single",
            DisclosureGroupSelectionMode::Multiple => "ui-disclosure-group--selection-multiple",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            DisclosureGroupSelectionMode::Single => "single",
            DisclosureGroupSelectionMode::Multiple => "multiple",
        }
    }
}

pub fn resolve_expanded_axis_state(
    has_controlled_expanded_indices: bool,
    has_default_expanded_indices: bool,
) -> DisclosureGroupExpandedAxisState {
    DisclosureGroupExpandedAxisState {
        control_mode_attr: if has_controlled_expanded_indices {
            "controlled"
        } else {
            "uncontrolled"
        },
        default_expanded_source_attr: if has_default_expanded_indices {
            "prop"
        } else {
            "implicit-default"
        },
        is_controlled: has_controlled_expanded_indices,
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn normalize_expanded_indices(
    mode: DisclosureGroupSelectionMode,
    expanded: &BTreeSet<usize>,
    item_count: usize,
) -> BTreeSet<usize> {
    let mut next = expanded
        .iter()
        .copied()
        .filter(|&index| index < item_count)
        .collect::<BTreeSet<_>>();

    if mode == DisclosureGroupSelectionMode::Single && next.len() > 1 {
        let first = next.iter().copied().next();
        next.clear();
        if let Some(first) = first {
            next.insert(first);
        }
    }

    next
}

pub fn resolve_state(input: DisclosureGroupStateInput) -> DisclosureGroupState {
    let has_items = input.item_count > 0;

    let mut expanded_count = input.expanded_count.min(input.item_count);
    if input.selection_mode == DisclosureGroupSelectionMode::Single {
        expanded_count = expanded_count.min(1);
    }

    let has_expanded_items = expanded_count > 0;
    let has_multiple_expanded = expanded_count > 1;

    let data_state_attr = if !has_items {
        "empty"
    } else if input.disabled {
        "disabled"
    } else if has_multiple_expanded {
        "multiple-expanded"
    } else if has_expanded_items {
        "expanded"
    } else {
        "collapsed"
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

    DisclosureGroupState {
        selection_mode: input.selection_mode,
        selection_mode_class: input.selection_mode.class_name(),
        selection_mode_attr: input.selection_mode.as_attr(),
        item_count: input.item_count,
        expanded_count,
        is_empty: !has_items,
        has_items,
        has_expanded_items,
        has_multiple_expanded,
        is_disabled: input.disabled,
        has_disabled_items: input.has_disabled_items,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: DisclosureGroupState) -> String {
    let mut classes = vec![
        "ui-disclosure-group".to_string(),
        state.selection_mode_class.into(),
    ];

    if state.is_empty {
        classes.push("ui-disclosure-group--empty".to_string());
    }

    if state.is_disabled {
        classes.push("ui-disclosure-group--disabled".to_string());
    }

    if state.has_multiple_expanded {
        classes.push("ui-disclosure-group--multiple-expanded".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-disclosure-group--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/group/logic.rs"]
mod tests;
