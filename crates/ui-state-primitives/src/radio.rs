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
mod tests {
    use super::*;

    #[test]
    fn orientation_class_names_are_stable() {
        assert_eq!(
            RadioGroupOrientation::Vertical.class_name(),
            "ui-radio-group--vertical"
        );
        assert_eq!(
            RadioGroupOrientation::Horizontal.class_name(),
            "ui-radio-group--horizontal"
        );
    }

    #[test]
    fn aria_and_data_orientation_values_are_stable() {
        assert_eq!(
            RadioGroupOrientation::Vertical.aria_orientation(),
            "vertical"
        );
        assert_eq!(
            RadioGroupOrientation::Horizontal.aria_orientation(),
            "horizontal"
        );

        assert_eq!(
            RadioGroupOrientation::Vertical.data_orientation(),
            "vertical"
        );
        assert_eq!(
            RadioGroupOrientation::Horizontal.data_orientation(),
            "horizontal"
        );
    }

    #[test]
    fn resolve_state_tracks_empty_disabled_group() {
        let disabled = HashSet::new();
        let state = resolve_state(
            0,
            true,
            &disabled,
            Some(0),
            RadioGroupOrientation::Vertical,
            false,
        );

        assert_eq!(state.item_count, 0);
        assert!(state.is_empty);
        assert!(!state.has_items);
        assert!(state.is_disabled);
        assert!(!state.has_disabled_options);
        assert_eq!(state.disabled_option_count, 0);
        assert_eq!(state.selected_index, None);
        assert!(!state.has_selection);
        assert!(state.selection_empty);
        assert!(!state.is_horizontal);
        assert!(state.is_vertical);
        assert!(!state.has_label);
    }

    #[test]
    fn resolve_state_tracks_selection_and_disabled_options() {
        let disabled = HashSet::from([1_usize, 9_usize]);
        let state = resolve_state(
            3,
            false,
            &disabled,
            Some(2),
            RadioGroupOrientation::Horizontal,
            true,
        );

        assert_eq!(state.item_count, 3);
        assert!(!state.is_empty);
        assert!(state.has_items);
        assert!(!state.is_disabled);
        assert!(state.has_disabled_options);
        assert_eq!(state.disabled_option_count, 1);
        assert_eq!(state.selected_index, Some(2));
        assert!(state.has_selection);
        assert!(!state.selection_empty);
        assert!(state.is_horizontal);
        assert!(!state.is_vertical);
        assert!(state.has_label);
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Size  ".to_string())),
            Some("Size".to_string())
        );
    }

    #[test]
    fn resolve_accessible_name_prefers_aria_label() {
        assert_eq!(
            resolve_accessible_name(
                Some("  Plan selector  ".to_string()),
                Some("external-label".to_string()),
                Some("internal-label".to_string())
            ),
            RadioGroupAccessibleName {
                aria_label: Some("Plan selector".to_string()),
                aria_labelledby: None,
            }
        );
    }

    #[test]
    fn resolve_accessible_name_uses_labelledby_when_available() {
        assert_eq!(
            resolve_accessible_name(
                None,
                Some("external-label".to_string()),
                Some("internal-label".to_string())
            ),
            RadioGroupAccessibleName {
                aria_label: None,
                aria_labelledby: Some("external-label".to_string()),
            }
        );
    }

    #[test]
    fn resolve_accessible_name_falls_back_to_internal_labelledby() {
        assert_eq!(
            resolve_accessible_name(None, None, Some("group-label".to_string())),
            RadioGroupAccessibleName {
                aria_label: None,
                aria_labelledby: Some("group-label".to_string()),
            }
        );
    }

    #[test]
    fn resolve_accessible_name_uses_default_label_when_missing() {
        assert_eq!(
            resolve_accessible_name(None, None, None),
            RadioGroupAccessibleName {
                aria_label: Some(DEFAULT_ARIA_LABEL.into()),
                aria_labelledby: None,
            }
        );
    }

    #[test]
    fn resolve_checked_axis_prefers_is_checked_and_on_checked_change() {
        let state = resolve_checked_axis(RadioCheckedAxisInput {
            has_is_checked: true,
            has_checked: true,
            has_default_checked: true,
            has_on_checked_change: true,
            has_on_change: true,
        });

        assert!(state.is_controlled);
        assert_eq!(state.control_mode_attr, "controlled");
        assert_eq!(state.checked_source_attr, "is_checked");
        assert_eq!(state.default_checked_source_attr, "provided");
        assert_eq!(state.checked_change_source_attr, "on_checked_change");
    }

    #[test]
    fn resolve_checked_axis_defaults_to_uncontrolled_markers() {
        let state = resolve_checked_axis(RadioCheckedAxisInput {
            has_is_checked: false,
            has_checked: false,
            has_default_checked: false,
            has_on_checked_change: false,
            has_on_change: true,
        });

        assert!(!state.is_controlled);
        assert_eq!(state.control_mode_attr, "uncontrolled");
        assert_eq!(state.checked_source_attr, "default");
        assert_eq!(state.default_checked_source_attr, "default");
        assert_eq!(state.checked_change_source_attr, "on_change");
    }
}
