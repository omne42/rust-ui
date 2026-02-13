use std::collections::BTreeSet;

pub const DEFAULT_LABEL: &str = "Options";
pub const DEFAULT_ID_BASE: &str = "combo-box";
pub const DEFAULT_PLACEHOLDER: &str = "Select…";
pub const DEFAULT_EMPTY_MESSAGE: &str = "No options";
pub const DEFAULT_TOGGLE_ARIA_LABEL: &str = "Toggle options";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComboBoxStateInput {
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

pub fn resolve_empty_message(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_EMPTY_MESSAGE.to_string())
}

pub fn resolve_toggle_aria_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_TOGGLE_ARIA_LABEL.to_string())
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
    if state.has_custom_label {
        classes.push("ui-combo-box--custom-label".to_string());
    }
    if state.has_custom_description {
        classes.push("ui-combo-box--custom-description".to_string());
    }
    if state.has_custom_error {
        classes.push("ui-combo-box--custom-error".to_string());
    }
    if state.has_custom_placeholder {
        classes.push("ui-combo-box--custom-placeholder".to_string());
    }
    if state.has_custom_id_base {
        classes.push("ui-combo-box--custom-id".to_string());
    }
    if state.has_custom_motion {
        classes.push("ui-combo-box--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-combo-box--custom-class".to_string());
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
    fn normalize_label_trims_and_defaults() {
        assert_eq!(normalize_label("  Language  ".to_string()), "Language");
        assert_eq!(normalize_label("   ".to_string()), DEFAULT_LABEL);
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
        assert_eq!(normalize_id_base("   ".to_string()), DEFAULT_ID_BASE);
    }

    #[test]
    fn resolve_placeholder_uses_fallback() {
        assert_eq!(
            resolve_placeholder(Some("  Choose  ".to_string())),
            "Choose"
        );
        assert_eq!(
            resolve_placeholder(Some("   ".to_string())),
            DEFAULT_PLACEHOLDER
        );
        assert_eq!(resolve_placeholder(None), DEFAULT_PLACEHOLDER);
    }

    #[test]
    fn resolve_empty_message_uses_fallback() {
        assert_eq!(
            resolve_empty_message(Some("  Nothing matched  ".to_string())),
            "Nothing matched"
        );
        assert_eq!(
            resolve_empty_message(Some("   ".to_string())),
            DEFAULT_EMPTY_MESSAGE
        );
        assert_eq!(resolve_empty_message(None), DEFAULT_EMPTY_MESSAGE);
    }

    #[test]
    fn resolve_toggle_aria_label_uses_fallback() {
        assert_eq!(
            resolve_toggle_aria_label(Some("  Expand options  ".to_string())),
            "Expand options"
        );
        assert_eq!(
            resolve_toggle_aria_label(Some("   ".to_string())),
            DEFAULT_TOGGLE_ARIA_LABEL
        );
        assert_eq!(resolve_toggle_aria_label(None), DEFAULT_TOGGLE_ARIA_LABEL);
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
            resolve_state(ComboBoxStateInput {
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
            "ui-combo-box",
            "ui-combo-box--disabled",
            "ui-combo-box--empty",
            "ui-combo-box--has-description",
            "ui-combo-box--has-error",
            "ui-combo-box--has-disabled-options",
            "ui-combo-box--controlled",
            "ui-combo-box--custom-label",
            "ui-combo-box--custom-description",
            "ui-combo-box--custom-error",
            "ui-combo-box--custom-placeholder",
            "ui-combo-box--custom-id",
            "ui-combo-box--custom-motion",
            "ui-combo-box--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
