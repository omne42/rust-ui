use std::collections::BTreeSet;

pub use crate::swatch::normalize_optional_text;

pub const DEFAULT_ARIA_LABEL: &str = "Color swatches";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwatchPickerItem {
    pub color: String,
    pub color_name: Option<String>,
    pub disabled: bool,
}

impl SwatchPickerItem {
    pub fn new(color: impl Into<String>) -> Self {
        Self {
            color: color.into(),
            color_name: None,
            disabled: false,
        }
    }

    pub fn named(color: impl Into<String>, color_name: impl Into<String>) -> Self {
        Self {
            color: color.into(),
            color_name: Some(color_name.into()),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwatchPickerStateInput {
    pub disabled: bool,
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub disabled_item_count: usize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwatchPickerState {
    pub is_disabled: bool,
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub has_selection: bool,
    pub selection_empty: bool,
    pub is_empty: bool,
    pub has_items: bool,
    pub disabled_item_count: usize,
    pub has_disabled_items: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

fn canonical_color_key(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

pub fn normalize_items(items: Vec<SwatchPickerItem>) -> Vec<SwatchPickerItem> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();

    for item in items {
        let Some(color) = crate::swatch::sanitize_color_value(Some(item.color)) else {
            continue;
        };

        let key = canonical_color_key(&color);
        if !seen.insert(key) {
            continue;
        }

        out.push(SwatchPickerItem {
            color,
            color_name: normalize_optional_text(item.color_name),
            disabled: item.disabled,
        });
    }

    out
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn sanitize_selected_color(selected_color: Option<String>) -> Option<String> {
    crate::swatch::sanitize_color_value(selected_color)
}

pub fn resolve_selected_index(
    items: &[SwatchPickerItem],
    selected_color: Option<String>,
) -> Option<usize> {
    let selected_color = sanitize_selected_color(selected_color)?;
    let selected_key = canonical_color_key(&selected_color);

    items
        .iter()
        .position(|item| canonical_color_key(&item.color) == selected_key)
}

pub fn resolve_selected_color(
    items: &[SwatchPickerItem],
    selected_index: Option<usize>,
) -> Option<String> {
    let index = selected_index?;
    items.get(index).map(|item| item.color.clone())
}

pub fn resolve_option_label(item: &SwatchPickerItem, index: usize) -> String {
    if let Some(color_name) = normalize_optional_text(item.color_name.clone()) {
        return color_name;
    }

    if !item.color.trim().is_empty() {
        return format!("Color {} ({})", index + 1, item.color);
    }

    format!("Color {}", index + 1)
}

pub fn resolve_state(input: SwatchPickerStateInput) -> SwatchPickerState {
    let has_items = input.item_count > 0;
    let has_selection = input.selected_index.is_some();

    let data_state_attr = if input.disabled {
        "disabled"
    } else if !has_items {
        "empty"
    } else if has_selection {
        "selected"
    } else {
        "default"
    };

    SwatchPickerState {
        is_disabled: input.disabled,
        item_count: input.item_count,
        selected_index: input.selected_index,
        has_selection,
        selection_empty: !has_selection,
        is_empty: !has_items,
        has_items,
        disabled_item_count: input.disabled_item_count,
        has_disabled_items: input.disabled_item_count > 0,
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
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "test/swatch_picker.rs"]
mod tests;
