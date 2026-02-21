use crate::button::normalize_optional_text;
use std::collections::BTreeSet;

pub const DEFAULT_TOGGLE_GROUP_ARIA_LABEL: &str = "Toggle group";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ToggleGroupOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Horizontal => "ui-toggle-group--horizontal",
            Self::Vertical => "ui-toggle-group--vertical",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleGroupSelectionMode {
    #[default]
    Multiple,
    Single,
}

impl ToggleGroupSelectionMode {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Multiple => "ui-toggle-group--mode-multiple",
            Self::Single => "ui-toggle-group--mode-single",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Multiple => "multiple",
            Self::Single => "single",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToggleGroupItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
}

impl ToggleGroupItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub fn normalize_toggle_group_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_TOGGLE_GROUP_ARIA_LABEL.to_string(), false)
}

pub fn normalize_toggle_group_items(items: Vec<ToggleGroupItem>) -> Vec<ToggleGroupItem> {
    items
        .into_iter()
        .enumerate()
        .map(|(index, mut item)| {
            let fallback_id = format!("toggle-{}", index + 1);
            item.id = normalize_optional_text(Some(item.id)).unwrap_or(fallback_id);
            item.label =
                normalize_optional_text(Some(item.label)).unwrap_or_else(|| item.id.clone());
            item
        })
        .collect()
}

pub fn collect_toggle_group_item_ids(items: &[ToggleGroupItem]) -> BTreeSet<String> {
    items.iter().map(|item| item.id.clone()).collect()
}

fn toggle_group_item_is_disabled(id: &str, items: &[ToggleGroupItem]) -> bool {
    items
        .iter()
        .find(|item| item.id == id)
        .map(|item| item.disabled)
        .unwrap_or(true)
}

pub fn sanitize_toggle_group_selected_ids(
    selected_ids: BTreeSet<String>,
    item_ids: &BTreeSet<String>,
    items: &[ToggleGroupItem],
    selection_mode: ToggleGroupSelectionMode,
) -> BTreeSet<String> {
    let mut selected_ids: BTreeSet<String> = selected_ids
        .into_iter()
        .filter(|id| item_ids.contains(id) && !toggle_group_item_is_disabled(id, items))
        .collect();

    if matches!(selection_mode, ToggleGroupSelectionMode::Single) && selected_ids.len() > 1 {
        let first = selected_ids.iter().next().cloned();
        selected_ids.clear();
        if let Some(first) = first {
            selected_ids.insert(first);
        }
    }

    selected_ids
}

pub fn toggle_toggle_group_selected_id(
    selected_ids: BTreeSet<String>,
    id: &str,
    item_ids: &BTreeSet<String>,
    items: &[ToggleGroupItem],
    selection_mode: ToggleGroupSelectionMode,
    next_selected: bool,
) -> BTreeSet<String> {
    if !item_ids.contains(id) || toggle_group_item_is_disabled(id, items) {
        return selected_ids;
    }

    match selection_mode {
        ToggleGroupSelectionMode::Single => {
            if next_selected {
                BTreeSet::from([id.into()])
            } else {
                BTreeSet::new()
            }
        }
        ToggleGroupSelectionMode::Multiple => {
            let mut next = selected_ids;
            if next_selected {
                next.insert(id.into());
            } else {
                next.remove(id);
            }
            next
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToggleGroupStateInput {
    pub orientation: ToggleGroupOrientation,
    pub selection_mode: ToggleGroupSelectionMode,
    pub disabled: bool,
    pub attached: bool,
    pub item_count: usize,
    pub selected_count: usize,
    pub disabled_item_count: usize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToggleGroupState {
    pub orientation: ToggleGroupOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub selection_mode: ToggleGroupSelectionMode,
    pub selection_mode_class: &'static str,
    pub selection_mode_attr: &'static str,
    pub is_disabled: bool,
    pub is_attached: bool,
    pub item_count: usize,
    pub selected_count: usize,
    pub disabled_item_count: usize,
    pub has_selection: bool,
    pub is_empty: bool,
    pub has_disabled_items: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn resolve_toggle_group_state(input: ToggleGroupStateInput) -> ToggleGroupState {
    let has_selection = input.selected_count > 0;
    let is_empty = input.item_count == 0;
    let has_disabled_items = input.disabled_item_count > 0;

    let data_state_attr = if input.disabled {
        "disabled"
    } else if is_empty {
        "empty"
    } else if has_selection {
        "selected"
    } else {
        "default"
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

    ToggleGroupState {
        orientation: input.orientation,
        orientation_class: input.orientation.class_name(),
        orientation_attr: input.orientation.as_attr(),
        selection_mode: input.selection_mode,
        selection_mode_class: input.selection_mode.class_name(),
        selection_mode_attr: input.selection_mode.as_attr(),
        is_disabled: input.disabled,
        is_attached: input.attached,
        item_count: input.item_count,
        selected_count: input.selected_count,
        disabled_item_count: input.disabled_item_count,
        has_selection,
        is_empty,
        has_disabled_items,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToggleButtonState {
    pub is_selected: bool,
    pub is_unselected: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub is_pressed: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub is_focus_visible: bool,
}

impl ToggleButtonState {
    pub fn data_state(self) -> &'static str {
        if self.is_selected {
            "selected"
        } else {
            "unselected"
        }
    }
}

pub fn resolve_toggle_button_state(
    is_selected: bool,
    is_disabled: bool,
    is_pressed: bool,
    is_hovered: bool,
    is_focused: bool,
    is_focus_visible: bool,
) -> ToggleButtonState {
    let is_enabled = !is_disabled;

    ToggleButtonState {
        is_selected,
        is_unselected: !is_selected,
        is_disabled,
        is_enabled,
        is_pressed: is_pressed && is_enabled,
        is_hovered: is_hovered && is_enabled,
        is_focused: is_focused && is_enabled,
        is_focus_visible: is_focus_visible && is_enabled,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ToggleButtonGroupOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Horizontal => "ui-toggle-button-group--horizontal",
            Self::Vertical => "ui-toggle-button-group--vertical",
        }
    }

    pub fn data_orientation(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToggleButtonGroupState {
    pub is_horizontal: bool,
    pub is_vertical: bool,
    pub is_attached: bool,
    pub is_detached: bool,
    pub has_explicit_label: bool,
    pub has_fallback_label: bool,
}

pub fn normalize_toggle_button_group_aria_label(aria_label: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(aria_label) {
        return (label, true);
    }

    (DEFAULT_TOGGLE_GROUP_ARIA_LABEL.to_string(), false)
}

pub fn resolve_toggle_button_group_state(
    orientation: ToggleButtonGroupOrientation,
    attached: bool,
    has_explicit_label: bool,
) -> ToggleButtonGroupState {
    ToggleButtonGroupState {
        is_horizontal: matches!(orientation, ToggleButtonGroupOrientation::Horizontal),
        is_vertical: matches!(orientation, ToggleButtonGroupOrientation::Vertical),
        is_attached: attached,
        is_detached: !attached,
        has_explicit_label,
        has_fallback_label: !has_explicit_label,
    }
}

#[cfg(test)]
#[path = "test/toggle_button.rs"]
mod tests;
