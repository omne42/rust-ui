use crate::selection::{
    Key, SelectedKey, SingleSelectionState, SingleSelectionStateOptions, use_single_selection_state,
};

pub const DEFAULT_ARIA_LABEL: &str = "Listbox";
pub const DEFAULT_ITEM_ARIA_LABEL: &str = "Listbox item";
pub const DEFAULT_SECTION_ARIA_LABEL: &str = "Listbox section";

#[derive(Clone, Default)]
pub struct ListStateOptions {
    pub items: Vec<Key>,
    pub selection: SingleSelectionStateOptions,
}

#[derive(Clone)]
pub struct ListState {
    items: Vec<Key>,
    selection: SingleSelectionState,
}

pub fn use_list_state(options: ListStateOptions) -> ListState {
    ListState {
        items: options.items,
        selection: use_single_selection_state(options.selection),
    }
}

impl ListState {
    pub fn items(&self) -> &[Key] {
        &self.items
    }

    pub fn selection(&self) -> &SingleSelectionState {
        &self.selection
    }

    pub fn selection_mut(&mut self) -> &mut SingleSelectionState {
        &mut self.selection
    }

    pub fn selected_key(&self) -> &SelectedKey {
        self.selection.selected_key()
    }

    pub fn selected_key_str(&self) -> Option<&str> {
        self.selection.selected_key_str()
    }

    pub fn selected_index(&self) -> Option<usize> {
        let selected = self.selection.selected_key_str()?;
        self.items.iter().position(|k| k == selected)
    }

    pub fn select_by_index(&mut self, index: usize) {
        let Some(key) = self.items.get(index).cloned() else {
            return;
        };
        self.selection.set_selected_key(SelectedKey::Key(key));
    }

    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let next_index = match self.selected_index() {
            None => 0,
            Some(i) => {
                if i + 1 >= self.items.len() {
                    0
                } else {
                    i + 1
                }
            }
        };

        self.select_by_index(next_index);
    }

    pub fn select_prev(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let prev_index = match self.selected_index() {
            None => self.items.len() - 1,
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
        };

        self.select_by_index(prev_index);
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListAccessibleName {
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
}

pub fn resolve_accessible_name(
    aria_label: Option<String>,
    aria_labelledby: Option<String>,
) -> ListAccessibleName {
    let aria_label = normalize_optional_text(aria_label);
    let aria_labelledby = normalize_optional_text(aria_labelledby);

    if aria_label.is_some() {
        return ListAccessibleName {
            aria_label,
            aria_labelledby: None,
        };
    }

    if aria_labelledby.is_some() {
        return ListAccessibleName {
            aria_label: None,
            aria_labelledby,
        };
    }

    ListAccessibleName {
        aria_label: Some(DEFAULT_ARIA_LABEL.to_string()),
        aria_labelledby: None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListViewStateInput {
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub has_disabled_options: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListViewState {
    pub is_empty: bool,
    pub has_items: bool,
    pub has_selection: bool,
    pub has_disabled_options: bool,
}

pub fn resolve_view_state(input: ListViewStateInput) -> ListViewState {
    let has_items = input.item_count > 0;
    let has_selection = input
        .selected_index
        .filter(|index| *index < input.item_count)
        .is_some();

    ListViewState {
        is_empty: !has_items,
        has_items,
        has_selection,
        has_disabled_options: input.has_disabled_options,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListItemSelectionIndicator {
    Hidden,
    Checkmark,
}

impl ListItemSelectionIndicator {
    pub fn marker(self, is_selected: bool) -> Option<&'static str> {
        match self {
            ListItemSelectionIndicator::Hidden => None,
            ListItemSelectionIndicator::Checkmark => is_selected.then_some("✓"),
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ListItemSelectionIndicator::Hidden => "hidden",
            ListItemSelectionIndicator::Checkmark => "checkmark",
        }
    }
}

pub fn resolve_item_selection_indicator(
    show_selection_indicator: bool,
) -> ListItemSelectionIndicator {
    if show_selection_indicator {
        ListItemSelectionIndicator::Checkmark
    } else {
        ListItemSelectionIndicator::Hidden
    }
}

pub fn normalize_item_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ITEM_ARIA_LABEL.into(), false)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListItemStateInput {
    pub selected: bool,
    pub focused: bool,
    pub disabled: bool,
    pub show_selection_indicator: bool,
    pub has_divider: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListItemState {
    pub is_selected: bool,
    pub is_focused: bool,
    pub is_disabled: bool,
    pub show_selection_indicator: bool,
    pub has_divider: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub data_state_attr: &'static str,
    pub selection_indicator_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn resolve_item_state(input: ListItemStateInput) -> ListItemState {
    let data_state_attr = if input.disabled && input.selected {
        "disabled-selected"
    } else if input.disabled {
        "disabled"
    } else if input.focused && input.selected {
        "focused-selected"
    } else if input.focused {
        "focused"
    } else if input.selected {
        "selected"
    } else {
        "idle"
    };

    let selection_indicator = resolve_item_selection_indicator(input.show_selection_indicator);

    ListItemState {
        is_selected: input.selected,
        is_focused: input.focused,
        is_disabled: input.disabled,
        show_selection_indicator: input.show_selection_indicator,
        has_divider: input.has_divider,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        data_state_attr,
        selection_indicator_attr: selection_indicator.as_attr(),
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ListSectionHeadingTone {
    #[default]
    Default,
    Quiet,
}

impl ListSectionHeadingTone {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Default => "ui-listbox-section--tone-default",
            Self::Quiet => "ui-listbox-section--tone-quiet",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Quiet => "quiet",
        }
    }
}

pub fn normalize_section_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_SECTION_ARIA_LABEL.into(), false)
}

pub fn normalize_section_title(value: Option<String>) -> Option<String> {
    normalize_optional_text(value)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListSectionStateInput {
    pub heading_tone: ListSectionHeadingTone,
    pub item_count: usize,
    pub disabled: bool,
    pub sticky_heading: bool,
    pub show_divider: bool,
    pub has_title: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListSectionState {
    pub heading_tone: ListSectionHeadingTone,
    pub heading_tone_class: &'static str,
    pub heading_tone_attr: &'static str,
    pub item_count: usize,
    pub has_items: bool,
    pub is_empty: bool,
    pub is_disabled: bool,
    pub has_title: bool,
    pub is_sticky_heading: bool,
    pub has_divider: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub title_source_attr: &'static str,
}

pub fn resolve_section_state(input: ListSectionStateInput) -> ListSectionState {
    let has_items = input.item_count > 0;
    let is_empty = !has_items;

    let data_state_attr = if input.disabled && is_empty {
        "disabled-empty"
    } else if input.disabled {
        "disabled"
    } else if is_empty {
        "empty"
    } else if input.sticky_heading {
        "sticky"
    } else if input.show_divider {
        "divided"
    } else {
        "default"
    };

    ListSectionState {
        heading_tone: input.heading_tone,
        heading_tone_class: input.heading_tone.class_name(),
        heading_tone_attr: input.heading_tone.as_attr(),
        item_count: input.item_count,
        has_items,
        is_empty,
        is_disabled: input.disabled,
        has_title: input.has_title,
        is_sticky_heading: input.sticky_heading,
        has_divider: input.show_divider,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        data_state_attr,
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        title_source_attr: if input.has_title { "custom" } else { "none" },
    }
}

#[cfg(test)]
#[path = "test/list.rs"]
mod tests;
