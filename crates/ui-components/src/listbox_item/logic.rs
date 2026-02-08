use crate::listbox_item::{ListBoxItemState, ListBoxItemStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Listbox item";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListBoxItemSelectionIndicator {
    Hidden,
    Checkmark,
}

impl ListBoxItemSelectionIndicator {
    pub fn marker(self, is_selected: bool) -> Option<&'static str> {
        match self {
            ListBoxItemSelectionIndicator::Hidden => None,
            ListBoxItemSelectionIndicator::Checkmark => is_selected.then_some("✓"),
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ListBoxItemSelectionIndicator::Hidden => "hidden",
            ListBoxItemSelectionIndicator::Checkmark => "checkmark",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn resolve_selection_indicator(
    show_selection_indicator: bool,
) -> ListBoxItemSelectionIndicator {
    if show_selection_indicator {
        ListBoxItemSelectionIndicator::Checkmark
    } else {
        ListBoxItemSelectionIndicator::Hidden
    }
}

pub fn resolve_state(input: ListBoxItemStateInput) -> ListBoxItemState {
    let data_state_attr = if input.disabled && input.selected {
        "disabled-selected"
    } else if input.disabled {
        "disabled"
    } else if input.focused && input.selected {
        "focused-selected"
    } else if input.focused {
        "focused"
    } else if input.selected {
        "selected"
    } else {
        "idle"
    };

    let selection_indicator = resolve_selection_indicator(input.show_selection_indicator);

    ListBoxItemState {
        is_selected: input.selected,
        is_focused: input.focused,
        is_disabled: input.disabled,
        show_selection_indicator: input.show_selection_indicator,
        has_divider: input.has_divider,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        data_state_attr,
        selection_indicator_attr: selection_indicator.as_attr(),
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
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ListBoxItemState) -> String {
    let mut classes = vec!["ui-listbox-item".to_string()];

    if state.is_selected {
        classes.push("ui-listbox-item--selected".to_string());
    }

    if state.is_focused {
        classes.push("ui-listbox-item--focused".to_string());
    }

    if state.is_disabled {
        classes.push("ui-listbox-item--disabled".to_string());
    }

    if state.show_selection_indicator {
        classes.push("ui-listbox-item--selection-indicator".to_string());
    }

    if state.has_divider {
        classes.push("ui-listbox-item--divider".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-listbox-item--custom-class".to_string());
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
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  San Jose  ".to_string())),
            Some("San Jose".to_string())
        );

        assert_eq!(
            normalize_aria_label(Some("  Favorite city  ".to_string())),
            ("Favorite city".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(Some("".to_string())),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
    }

    #[test]
    fn selection_indicator_contract_is_stable() {
        assert_eq!(
            resolve_selection_indicator(false),
            ListBoxItemSelectionIndicator::Hidden
        );
        assert_eq!(
            resolve_selection_indicator(true),
            ListBoxItemSelectionIndicator::Checkmark
        );
        assert_eq!(ListBoxItemSelectionIndicator::Hidden.as_attr(), "hidden");
        assert_eq!(
            ListBoxItemSelectionIndicator::Checkmark.as_attr(),
            "checkmark"
        );
    }

    #[test]
    fn resolve_state_tracks_selection_focus_and_sources() {
        let state = resolve_state(ListBoxItemStateInput {
            selected: true,
            focused: true,
            disabled: false,
            show_selection_indicator: true,
            has_divider: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert!(state.is_selected);
        assert!(state.is_focused);
        assert!(!state.is_disabled);
        assert!(state.show_selection_indicator);
        assert!(state.has_divider);
        assert_eq!(state.data_state_attr, "focused-selected");
        assert_eq!(state.selection_indicator_attr, "checkmark");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(ListBoxItemStateInput {
            selected: true,
            focused: false,
            disabled: false,
            show_selection_indicator: true,
            has_divider: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-listbox-item-custom".to_string()), state);

        for needle in [
            "ui-listbox-item",
            "ui-listbox-item--selected",
            "ui-listbox-item--selection-indicator",
            "ui-listbox-item--divider",
            "ui-listbox-item--custom-class",
            "docs-listbox-item-custom",
        ] {
            assert!(
                class_name.contains(needle),
                "ListBoxItem class list should include `{needle}`"
            );
        }
    }
}
