use crate::{
    NativeSelectOption, NativeSelectOptionResolved, NativeSelectState, NativeSelectStateInput,
};

pub const DEFAULT_ARIA_LABEL: &str = "Native select";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NativeSelectSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl NativeSelectSize {
    pub fn class_name(self) -> &'static str {
        match self {
            NativeSelectSize::Sm => "ui-native-select--size-sm",
            NativeSelectSize::Md => "ui-native-select--size-md",
            NativeSelectSize::Lg => "ui-native-select--size-lg",
        }
    }

    pub fn data_size(self) -> &'static str {
        match self {
            NativeSelectSize::Sm => "sm",
            NativeSelectSize::Md => "md",
            NativeSelectSize::Lg => "lg",
        }
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

pub fn normalize_placeholder(placeholder: Option<String>) -> Option<String> {
    normalize_optional_text(placeholder)
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

pub fn resolve_state(
    input: NativeSelectStateInput<'_>,
    size: NativeSelectSize,
) -> NativeSelectState {
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
        size_class: size.class_name(),
        size_attr: size.data_size(),
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

pub fn compose_class_name(base_class_name: Option<String>, state: &NativeSelectState) -> String {
    let mut classes = vec!["ui-native-select".to_string(), state.size_class.into()];

    if state.control_disabled {
        classes.push("ui-native-select--disabled".to_string());
    }
    if state.is_invalid {
        classes.push("ui-native-select--invalid".to_string());
    }
    if state.is_empty {
        classes.push("ui-native-select--empty".to_string());
    }
    if state.has_selection {
        classes.push("ui-native-select--selected".to_string());
    }
    if state.has_placeholder {
        classes.push("ui-native-select--has-placeholder".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-native-select--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
