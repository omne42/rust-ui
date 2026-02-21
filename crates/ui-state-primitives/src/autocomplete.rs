use std::collections::BTreeSet;

pub const DEFAULT_LABEL: &str = "Options";
pub const DEFAULT_ID_BASE: &str = "autocomplete";
pub const DEFAULT_PLACEHOLDER: &str = "Type…";
pub const DEFAULT_EMPTY_MESSAGE: &str = "No matches";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AutocompleteStateInput {
    pub item_count: usize,
    pub disabled_option_count: usize,
    pub is_disabled: bool,
    pub has_custom_label: bool,
    pub has_custom_description: bool,
    pub has_custom_error: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_id_base: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub is_controlled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AutocompleteState {
    pub item_count: usize,
    pub disabled_option_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_description: bool,
    pub has_error: bool,
    pub has_disabled_options: bool,
    pub label_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub id_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub has_custom_label: bool,
    pub has_custom_description: bool,
    pub has_custom_error: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_id_base: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AutocompleteInputState {
    pub query: String,
    pub has_typed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AutocompleteInputEvent {
    SyncFromSelection { selected_label: Option<String> },
    InputChanged { query: String },
    OptionCommitted { selected_label: String },
    InputBlurred,
}

pub fn reduce_input_state(
    current: AutocompleteInputState,
    event: AutocompleteInputEvent,
) -> AutocompleteInputState {
    match event {
        AutocompleteInputEvent::SyncFromSelection { selected_label } => AutocompleteInputState {
            query: selected_label.unwrap_or_default(),
            has_typed: false,
        },
        AutocompleteInputEvent::InputChanged { query } => AutocompleteInputState {
            query,
            has_typed: true,
        },
        AutocompleteInputEvent::OptionCommitted { selected_label } => AutocompleteInputState {
            query: selected_label,
            has_typed: false,
        },
        AutocompleteInputEvent::InputBlurred => AutocompleteInputState {
            query: current.query,
            has_typed: false,
        },
    }
}

pub fn normalize_label(label: String) -> String {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        DEFAULT_LABEL.into()
    } else {
        trimmed.into()
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

pub fn resolve_placeholder(placeholder: Option<String>) -> String {
    normalize_optional_text(placeholder).unwrap_or_else(|| DEFAULT_PLACEHOLDER.into())
}

pub fn resolve_empty_message(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_EMPTY_MESSAGE.into())
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

pub fn filter_indices(items: &[String], query: &str, has_typed: bool) -> Vec<usize> {
    if !has_typed {
        return (0..items.len()).collect();
    }

    let q = query.trim().to_ascii_lowercase();
    if q.is_empty() {
        return (0..items.len()).collect();
    }

    items
        .iter()
        .enumerate()
        .filter_map(|(idx, label)| label.to_ascii_lowercase().contains(&q).then_some(idx))
        .collect()
}

pub fn map_selected_to_filtered(
    selected_original: Option<usize>,
    filtered_original_indices: &[usize],
) -> Option<usize> {
    let selected = selected_original?;
    filtered_original_indices
        .iter()
        .position(|&idx| idx == selected)
}

pub fn map_filtered_to_original(
    filtered_index: usize,
    filtered_original_indices: &[usize],
) -> Option<usize> {
    filtered_original_indices.get(filtered_index).copied()
}

pub fn resolve_state(input: AutocompleteStateInput) -> AutocompleteState {
    AutocompleteState {
        item_count: input.item_count,
        disabled_option_count: input.disabled_option_count,
        is_empty: input.item_count == 0,
        has_items: input.item_count > 0,
        is_disabled: input.is_disabled,
        is_enabled: !input.is_disabled,
        has_description: input.has_custom_description,
        has_error: input.has_custom_error,
        has_disabled_options: input.disabled_option_count > 0,
        label_source_attr: if input.has_custom_label {
            "custom"
        } else {
            "default"
        },
        description_source_attr: if input.has_custom_description {
            "custom"
        } else {
            "default"
        },
        error_source_attr: if input.has_custom_error {
            "custom"
        } else {
            "default"
        },
        placeholder_source_attr: if input.has_custom_placeholder {
            "custom"
        } else {
            "default"
        },
        id_source_attr: if input.has_custom_id_base {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        motion_source_attr: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
        has_custom_label: input.has_custom_label,
        has_custom_description: input.has_custom_description,
        has_custom_error: input.has_custom_error,
        has_custom_placeholder: input.has_custom_placeholder,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        is_controlled: input.is_controlled,
        is_uncontrolled: !input.is_controlled,
    }
}

#[cfg(test)]
#[path = "test/autocomplete.rs"]
mod tests;
