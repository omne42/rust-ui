pub use crate::button::normalize_optional_text;

pub const DEFAULT_ARIA_LABEL: &str = "Native select";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeSelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl NativeSelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeSelectOptionResolved {
    pub id: String,
    pub index: usize,
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeSelectStateInput<'a> {
    pub disabled: bool,
    pub invalid: bool,
    pub required: bool,
    pub has_placeholder: bool,
    pub selected_index: Option<usize>,
    pub options: &'a [NativeSelectOptionResolved],
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeSelectState {
    pub is_disabled: bool,
    pub control_disabled: bool,
    pub is_invalid: bool,
    pub is_required: bool,
    pub has_placeholder: bool,
    pub is_empty: bool,
    pub has_options: bool,
    pub option_count: usize,
    pub selected_index: Option<usize>,
    pub selected_value: Option<String>,
    pub has_selection: bool,
    pub has_disabled_options: bool,
    pub has_enabled_options: bool,
    pub disabled_option_count: usize,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn normalize_options(options: Vec<NativeSelectOption>) -> Vec<NativeSelectOption> {
    options
        .into_iter()
        .enumerate()
        .map(|(index, mut option)| {
            let fallback = format!("option-{}", index + 1);

            option.value =
                normalize_optional_text(Some(option.value)).unwrap_or_else(|| fallback.clone());
            option.label =
                normalize_optional_text(Some(option.label)).unwrap_or_else(|| option.value.clone());

            option
        })
        .collect()
}

pub fn resolve_options(
    id_base: &str,
    options: &[NativeSelectOption],
) -> Vec<NativeSelectOptionResolved> {
    options
        .iter()
        .enumerate()
        .map(|(index, option)| NativeSelectOptionResolved {
            id: format!("{id_base}-option-{index}"),
            index,
            value: option.value.clone(),
            label: option.label.clone(),
            disabled: option.disabled,
        })
        .collect()
}

pub fn find_index_by_value(value: &str, options: &[NativeSelectOptionResolved]) -> Option<usize> {
    options
        .iter()
        .position(|option| option.value == value)
        .filter(|index| !options[*index].disabled)
}

pub fn sanitize_selected_index(
    selected_index: Option<usize>,
    options: &[NativeSelectOptionResolved],
) -> Option<usize> {
    selected_index.filter(|index| {
        options
            .get(*index)
            .map(|option| !option.disabled)
            .unwrap_or(false)
    })
}

pub fn resolve_state(input: NativeSelectStateInput<'_>) -> NativeSelectState {
    let option_count = input.options.len();
    let selected_index = sanitize_selected_index(input.selected_index, input.options);
    let selected_value = selected_index
        .and_then(|index| input.options.get(index).map(|option| option.value.clone()));

    let has_selection = selected_index.is_some();
    let is_empty = option_count == 0;
    let has_options = !is_empty;

    let disabled_option_count = input
        .options
        .iter()
        .filter(|option| option.disabled)
        .count();
    let has_disabled_options = disabled_option_count > 0;
    let has_enabled_options = input.options.iter().any(|option| !option.disabled);

    let control_disabled = input.disabled || !has_enabled_options;

    let data_state_attr = if control_disabled {
        "disabled"
    } else if input.invalid {
        "invalid"
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

    NativeSelectState {
        is_disabled: input.disabled,
        control_disabled,
        is_invalid: input.invalid,
        is_required: input.required,
        has_placeholder: input.has_placeholder,
        is_empty,
        has_options,
        option_count,
        selected_index,
        selected_value,
        has_selection,
        has_disabled_options,
        has_enabled_options,
        disabled_option_count,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "test/native_select.rs"]
mod tests;
