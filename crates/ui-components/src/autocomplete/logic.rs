use std::collections::BTreeSet;

pub const DEFAULT_LABEL: &str = "Options";
pub const DEFAULT_ID_BASE: &str = "autocomplete";
pub const DEFAULT_PLACEHOLDER: &str = "Type…";

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

pub fn normalize_label(label: String) -> String {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        DEFAULT_LABEL.to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| DEFAULT_ID_BASE.to_string())
}

pub fn resolve_placeholder(placeholder: Option<String>) -> String {
    normalize_optional_text(placeholder).unwrap_or_else(|| DEFAULT_PLACEHOLDER.to_string())
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

pub fn compose_class_name(base_class_name: Option<String>, state: AutocompleteState) -> String {
    let mut classes = vec!["ui-autocomplete".to_string()];

    if state.is_disabled {
        classes.push("ui-autocomplete--disabled".to_string());
    }
    if state.is_empty {
        classes.push("ui-autocomplete--empty".to_string());
    }
    if state.has_description {
        classes.push("ui-autocomplete--has-description".to_string());
    }
    if state.has_error {
        classes.push("ui-autocomplete--has-error".to_string());
    }
    if state.has_disabled_options {
        classes.push("ui-autocomplete--has-disabled-options".to_string());
    }
    if state.is_controlled {
        classes.push("ui-autocomplete--controlled".to_string());
    }
    if state.has_custom_label {
        classes.push("ui-autocomplete--custom-label".to_string());
    }
    if state.has_custom_description {
        classes.push("ui-autocomplete--custom-description".to_string());
    }
    if state.has_custom_error {
        classes.push("ui-autocomplete--custom-error".to_string());
    }
    if state.has_custom_placeholder {
        classes.push("ui-autocomplete--custom-placeholder".to_string());
    }
    if state.has_custom_id_base {
        classes.push("ui-autocomplete--custom-id".to_string());
    }
    if state.has_custom_motion {
        classes.push("ui-autocomplete--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-autocomplete--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn items() -> Vec<String> {
        vec!["Apple".into(), "Banana".into(), "Apricot".into()]
    }

    #[test]
    fn normalize_label_trims_and_defaults() {
        assert_eq!(normalize_label("  City  ".to_string()), "City");
        assert_eq!(normalize_label("   ".to_string()), DEFAULT_LABEL);
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Pick a city  ".to_string())),
            Some("Pick a city".to_string())
        );
    }

    #[test]
    fn normalize_id_base_falls_back_when_blank() {
        assert_eq!(
            normalize_id_base("  city-autocomplete  ".to_string()),
            "city-autocomplete"
        );
        assert_eq!(normalize_id_base("   ".to_string()), DEFAULT_ID_BASE);
    }

    #[test]
    fn resolve_placeholder_uses_fallback() {
        assert_eq!(
            resolve_placeholder(Some("  Search  ".to_string())),
            "Search"
        );
        assert_eq!(
            resolve_placeholder(Some("   ".to_string())),
            DEFAULT_PLACEHOLDER
        );
        assert_eq!(resolve_placeholder(None), DEFAULT_PLACEHOLDER);
    }

    #[test]
    fn disabled_indices_are_deduped_and_clamped_to_item_count() {
        assert_eq!(normalize_disabled_indices(vec![2, 1, 1, 9], 3), vec![1, 2]);
        assert_eq!(normalize_disabled_indices(vec![4], 0), Vec::<usize>::new());
    }

    #[test]
    fn resolve_state_tracks_component_flags() {
        let state = resolve_state(AutocompleteStateInput {
            item_count: 5,
            disabled_option_count: 2,
            is_disabled: false,
            has_custom_label: true,
            has_custom_description: true,
            has_custom_error: true,
            has_custom_placeholder: true,
            has_custom_id_base: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            is_controlled: true,
        });

        assert_eq!(state.item_count, 5);
        assert_eq!(state.disabled_option_count, 2);
        assert!(!state.is_empty);
        assert!(state.has_items);
        assert!(!state.is_disabled);
        assert!(state.is_enabled);
        assert!(state.has_description);
        assert!(state.has_error);
        assert!(state.has_disabled_options);
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.description_source_attr, "custom");
        assert_eq!(state.error_source_attr, "custom");
        assert_eq!(state.placeholder_source_attr, "custom");
        assert_eq!(state.id_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert!(state.has_custom_label);
        assert!(state.has_custom_class_name);
        assert!(state.has_custom_motion);
        assert!(state.is_controlled);
        assert!(!state.is_uncontrolled);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(AutocompleteStateInput {
                item_count: 0,
                disabled_option_count: 1,
                is_disabled: true,
                has_custom_label: true,
                has_custom_description: true,
                has_custom_error: true,
                has_custom_placeholder: true,
                has_custom_id_base: true,
                has_custom_class_name: true,
                has_custom_motion: true,
                is_controlled: true,
            }),
        );

        for token in [
            "ui-autocomplete",
            "ui-autocomplete--disabled",
            "ui-autocomplete--empty",
            "ui-autocomplete--has-description",
            "ui-autocomplete--has-error",
            "ui-autocomplete--has-disabled-options",
            "ui-autocomplete--controlled",
            "ui-autocomplete--custom-label",
            "ui-autocomplete--custom-description",
            "ui-autocomplete--custom-error",
            "ui-autocomplete--custom-placeholder",
            "ui-autocomplete--custom-id",
            "ui-autocomplete--custom-motion",
            "ui-autocomplete--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }

    #[test]
    fn filter_returns_all_when_not_typed_or_query_empty() {
        let items = items();
        assert_eq!(filter_indices(&items, "ap", false), vec![0, 1, 2]);
        assert_eq!(filter_indices(&items, " ", true), vec![0, 1, 2]);
    }

    #[test]
    fn filter_is_case_insensitive_contains() {
        let items = items();
        assert_eq!(filter_indices(&items, "ap", true), vec![0, 2]);
        assert_eq!(filter_indices(&items, "BAN", true), vec![1]);
    }

    #[test]
    fn selected_index_maps_to_filtered_position() {
        let filtered = vec![2, 0];
        assert_eq!(map_selected_to_filtered(Some(0), &filtered), Some(1));
        assert_eq!(map_selected_to_filtered(Some(2), &filtered), Some(0));
        assert_eq!(map_selected_to_filtered(Some(1), &filtered), None);
        assert_eq!(map_selected_to_filtered(None, &filtered), None);
    }

    #[test]
    fn filtered_to_original_maps_by_lookup() {
        let filtered = vec![2, 0];
        assert_eq!(map_filtered_to_original(0, &filtered), Some(2));
        assert_eq!(map_filtered_to_original(1, &filtered), Some(0));
        assert_eq!(map_filtered_to_original(2, &filtered), None);
    }
}
