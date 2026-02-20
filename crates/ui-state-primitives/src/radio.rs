use std::collections::HashSet;

pub const DEFAULT_ARIA_LABEL: &str = "Radio group";
pub const DEFAULT_CHECKED: bool = false;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RadioGroupOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl RadioGroupOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            RadioGroupOrientation::Vertical => "ui-radio-group--vertical",
            RadioGroupOrientation::Horizontal => "ui-radio-group--horizontal",
        }
    }

    pub fn aria_orientation(self) -> &'static str {
        match self {
            RadioGroupOrientation::Vertical => "vertical",
            RadioGroupOrientation::Horizontal => "horizontal",
        }
    }

    pub fn data_orientation(self) -> &'static str {
        self.aria_orientation()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RadioCheckedAxisInput {
    pub has_is_checked: bool,
    pub has_checked: bool,
    pub has_default_checked: bool,
    pub has_on_checked_change: bool,
    pub has_on_change: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RadioCheckedAxisState {
    pub is_controlled: bool,
    pub control_mode_attr: &'static str,
    pub checked_source_attr: &'static str,
    pub default_checked_source_attr: &'static str,
    pub checked_change_source_attr: &'static str,
}

pub fn resolve_checked_axis(input: RadioCheckedAxisInput) -> RadioCheckedAxisState {
    let is_controlled = input.has_is_checked || input.has_checked;
    let control_mode_attr = if is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    };
    let checked_source_attr = if input.has_is_checked {
        "is_checked"
    } else if input.has_checked {
        "checked"
    } else {
        "default"
    };
    let default_checked_source_attr = if input.has_default_checked {
        "provided"
    } else {
        "default"
    };
    let checked_change_source_attr = if input.has_on_checked_change {
        "on_checked_change"
    } else if input.has_on_change {
        "on_change"
    } else {
        "none"
    };

    RadioCheckedAxisState {
        is_controlled,
        control_mode_attr,
        checked_source_attr,
        default_checked_source_attr,
        checked_change_source_attr,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RadioGroupState {
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_disabled: bool,
    pub has_disabled_options: bool,
    pub disabled_option_count: usize,
    pub selected_index: Option<usize>,
    pub has_selection: bool,
    pub selection_empty: bool,
    pub is_horizontal: bool,
    pub is_vertical: bool,
    pub has_label: bool,
}

pub fn resolve_state(
    item_count: usize,
    is_disabled: bool,
    disabled_indices: &HashSet<usize>,
    selected_index: Option<usize>,
    orientation: RadioGroupOrientation,
    has_label: bool,
) -> RadioGroupState {
    let has_items = item_count > 0;
    let selected_index = selected_index.filter(|index| *index < item_count);
    let has_selection = selected_index.is_some();
    let disabled_option_count = disabled_indices
        .iter()
        .filter(|index| **index < item_count)
        .count();

    RadioGroupState {
        item_count,
        is_empty: !has_items,
        has_items,
        is_disabled,
        has_disabled_options: disabled_option_count > 0,
        disabled_option_count,
        selected_index,
        has_selection,
        selection_empty: !has_selection,
        is_horizontal: matches!(orientation, RadioGroupOrientation::Horizontal),
        is_vertical: matches!(orientation, RadioGroupOrientation::Vertical),
        has_label,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioGroupAccessibleName {
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_accessible_name(
    aria_label: Option<String>,
    aria_labelledby: Option<String>,
    fallback_labelledby: Option<String>,
) -> RadioGroupAccessibleName {
    let aria_label = normalize_optional_text(aria_label);
    let aria_labelledby = normalize_optional_text(aria_labelledby);

    if aria_label.is_some() {
        return RadioGroupAccessibleName {
            aria_label,
            aria_labelledby: None,
        };
    }

    if aria_labelledby.is_some() {
        return RadioGroupAccessibleName {
            aria_label: None,
            aria_labelledby,
        };
    }

    if fallback_labelledby.is_some() {
        return RadioGroupAccessibleName {
            aria_label: None,
            aria_labelledby: fallback_labelledby,
        };
    }

    RadioGroupAccessibleName {
        aria_label: Some(DEFAULT_ARIA_LABEL.into()),
        aria_labelledby: None,
    }
}

#[cfg(test)]
#[path = "test/radio.rs"]
mod tests;
