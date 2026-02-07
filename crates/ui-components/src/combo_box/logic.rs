use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComboBoxStateInput {
    pub item_count: usize,
    pub disabled_option_count: usize,
    pub is_disabled: bool,
    pub has_description: bool,
    pub has_error: bool,
    pub has_custom_class_name: bool,
    pub is_controlled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComboBoxState {
    pub item_count: usize,
    pub disabled_option_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_description: bool,
    pub has_error: bool,
    pub has_disabled_options: bool,
    pub has_custom_class_name: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
}

pub fn normalize_label(label: String) -> String {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        "Options".to_string()
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
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| "combo-box".to_string())
}

pub fn resolve_placeholder(placeholder: Option<String>) -> String {
    normalize_optional_text(placeholder).unwrap_or_else(|| "Select…".to_string())
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

pub fn resolve_state(input: ComboBoxStateInput) -> ComboBoxState {
    ComboBoxState {
        item_count: input.item_count,
        disabled_option_count: input.disabled_option_count,
        is_empty: input.item_count == 0,
        has_items: input.item_count > 0,
        is_disabled: input.is_disabled,
        is_enabled: !input.is_disabled,
        has_description: input.has_description,
        has_error: input.has_error,
        has_disabled_options: input.disabled_option_count > 0,
        has_custom_class_name: input.has_custom_class_name,
        is_controlled: input.is_controlled,
        is_uncontrolled: !input.is_controlled,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ComboBoxState) -> String {
    let mut classes = vec!["ui-combo-box".to_string()];

    if state.is_disabled {
        classes.push("ui-combo-box--disabled".to_string());
    }
    if state.is_empty {
        classes.push("ui-combo-box--empty".to_string());
    }
    if state.has_description {
        classes.push("ui-combo-box--has-description".to_string());
    }
    if state.has_error {
        classes.push("ui-combo-box--has-error".to_string());
    }
    if state.has_disabled_options {
        classes.push("ui-combo-box--has-disabled-options".to_string());
    }
    if state.is_controlled {
        classes.push("ui-combo-box--controlled".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_label_trims_and_defaults() {
        assert_eq!(normalize_label("  Language  ".to_string()), "Language");
        assert_eq!(normalize_label("   ".to_string()), "Options");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Pick one  ".to_string())),
            Some("Pick one".to_string())
        );
    }

    #[test]
    fn normalize_id_base_falls_back_when_blank() {
        assert_eq!(
            normalize_id_base("  language-box  ".to_string()),
            "language-box"
        );
        assert_eq!(normalize_id_base("   ".to_string()), "combo-box");
    }

    #[test]
    fn resolve_placeholder_uses_fallback() {
        assert_eq!(
            resolve_placeholder(Some("  Choose  ".to_string())),
            "Choose"
        );
        assert_eq!(resolve_placeholder(Some("   ".to_string())), "Select…");
        assert_eq!(resolve_placeholder(None), "Select…");
    }

    #[test]
    fn disabled_indices_are_deduped_and_clamped_to_item_count() {
        assert_eq!(normalize_disabled_indices(vec![2, 1, 1, 9], 3), vec![1, 2]);
        assert_eq!(normalize_disabled_indices(vec![4], 0), Vec::<usize>::new());
    }

    #[test]
    fn resolve_state_tracks_component_flags() {
        let state = resolve_state(ComboBoxStateInput {
            item_count: 5,
            disabled_option_count: 2,
            is_disabled: false,
            has_description: true,
            has_error: true,
            has_custom_class_name: true,
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
        assert!(state.has_custom_class_name);
        assert!(state.is_controlled);
        assert!(!state.is_uncontrolled);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(ComboBoxStateInput {
                item_count: 0,
                disabled_option_count: 1,
                is_disabled: true,
                has_description: true,
                has_error: true,
                has_custom_class_name: true,
                is_controlled: true,
            }),
        );

        for token in [
            "ui-combo-box",
            "ui-combo-box--disabled",
            "ui-combo-box--empty",
            "ui-combo-box--has-description",
            "ui-combo-box--has-error",
            "ui-combo-box--has-disabled-options",
            "ui-combo-box--controlled",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
