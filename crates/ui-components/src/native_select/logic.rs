use crate::native_select::{
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
mod tests {
    use super::*;

    #[test]
    fn size_class_names_and_attrs_are_stable() {
        assert_eq!(
            NativeSelectSize::Sm.class_name(),
            "ui-native-select--size-sm"
        );
        assert_eq!(
            NativeSelectSize::Md.class_name(),
            "ui-native-select--size-md"
        );
        assert_eq!(
            NativeSelectSize::Lg.class_name(),
            "ui-native-select--size-lg"
        );

        assert_eq!(NativeSelectSize::Sm.data_size(), "sm");
        assert_eq!(NativeSelectSize::Md.data_size(), "md");
        assert_eq!(NativeSelectSize::Lg.data_size(), "lg");
    }

    #[test]
    fn normalize_aria_label_prefers_custom_and_has_fallback() {
        let (label, custom) = normalize_aria_label(Some("  Frequency  ".to_string()));
        assert_eq!(label, "Frequency");
        assert!(custom);

        let (label, custom) = normalize_aria_label(Some("   ".to_string()));
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);

        let (label, custom) = normalize_aria_label(None);
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_options_normalizes_ids_labels_and_values() {
        let options = vec![
            NativeSelectOption::new("", ""),
            NativeSelectOption::new("manual", " Manual ").disabled(true),
        ];

        let normalized = normalize_options(options);
        assert_eq!(normalized[0].value, "option-1");
        assert_eq!(normalized[0].label, "option-1");
        assert_eq!(normalized[1].value, "manual");
        assert_eq!(normalized[1].label, "Manual");

        let resolved = resolve_options("docs-native-select", &normalized);
        assert_eq!(resolved[0].id, "docs-native-select-option-0");
        assert_eq!(resolved[1].id, "docs-native-select-option-1");
        assert!(resolved[1].disabled);
    }

    #[test]
    fn selected_index_and_lookup_skip_disabled_options() {
        let options = resolve_options(
            "x",
            &normalize_options(vec![
                NativeSelectOption::new("system", "System"),
                NativeSelectOption::new("manual", "Manual").disabled(true),
            ]),
        );

        assert_eq!(sanitize_selected_index(Some(0), &options), Some(0));
        assert_eq!(sanitize_selected_index(Some(1), &options), None);
        assert_eq!(sanitize_selected_index(Some(8), &options), None);

        assert_eq!(find_index_by_value("system", &options), Some(0));
        assert_eq!(find_index_by_value("manual", &options), None);
        assert_eq!(find_index_by_value("missing", &options), None);
    }

    #[test]
    fn resolve_state_tracks_disabled_invalid_selection_and_counts() {
        let options = resolve_options(
            "docs",
            &normalize_options(vec![
                NativeSelectOption::new("system", "System"),
                NativeSelectOption::new("manual", "Manual").disabled(true),
            ]),
        );

        let state = resolve_state(
            NativeSelectStateInput {
                disabled: false,
                invalid: true,
                required: true,
                has_placeholder: true,
                selected_index: Some(0),
                options: &options,
                has_custom_aria_label: false,
                has_custom_class_name: true,
            },
            NativeSelectSize::Lg,
        );

        assert_eq!(state.size_class, "ui-native-select--size-lg");
        assert_eq!(state.size_attr, "lg");
        assert!(state.has_options);
        assert!(!state.is_empty);
        assert!(state.has_selection);
        assert_eq!(state.selected_index, Some(0));
        assert_eq!(state.selected_value.as_deref(), Some("system"));
        assert!(state.is_invalid);
        assert!(state.is_required);
        assert!(state.has_placeholder);
        assert!(state.has_disabled_options);
        assert_eq!(state.disabled_option_count, 1);
        assert!(state.has_enabled_options);
        assert!(!state.control_disabled);
        assert_eq!(state.data_state_attr, "invalid");
        assert_eq!(state.aria_source_attr, "default");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_adds_state_and_custom_markers() {
        let state = NativeSelectState {
            size_class: "ui-native-select--size-md",
            size_attr: "md",
            is_disabled: false,
            control_disabled: true,
            is_invalid: true,
            is_required: false,
            has_placeholder: true,
            is_empty: true,
            has_options: false,
            option_count: 0,
            selected_index: None,
            selected_value: None,
            has_selection: false,
            has_disabled_options: false,
            has_enabled_options: false,
            disabled_option_count: 0,
            data_state_attr: "disabled",
            aria_source_attr: "custom",
            class_source_attr: "custom",
            has_custom_class_name: true,
        };

        let class = compose_class_name(Some("docs-native-select".to_string()), &state);
        assert!(class.contains("ui-native-select"));
        assert!(class.contains("ui-native-select--size-md"));
        assert!(class.contains("ui-native-select--disabled"));
        assert!(class.contains("ui-native-select--invalid"));
        assert!(class.contains("ui-native-select--empty"));
        assert!(class.contains("ui-native-select--has-placeholder"));
        assert!(class.contains("ui-native-select--custom-class"));
        assert!(class.contains("docs-native-select"));
    }
}
