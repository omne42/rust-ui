#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListAccessibleName {
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListState {
    pub is_empty: bool,
    pub has_items: bool,
    pub has_selection: bool,
    pub has_disabled_options: bool,
}

pub use item::ListItemSelectionIndicator;
pub use section::ListSectionHeadingTone;

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_accessible_name(
    aria_label: Option<String>,
    aria_labelledby: Option<String>,
) -> ListAccessibleName {
    let aria_label = normalize_optional_text(aria_label);
    let aria_labelledby = normalize_optional_text(aria_labelledby);

    if aria_label.is_some() {
        return ListAccessibleName {
            aria_label,
            aria_labelledby: None,
        };
    }

    if aria_labelledby.is_some() {
        return ListAccessibleName {
            aria_label: None,
            aria_labelledby,
        };
    }

    ListAccessibleName {
        aria_label: Some("Listbox".to_string()),
        aria_labelledby: None,
    }
}

pub fn resolve_state(
    item_count: usize,
    selected_index: Option<usize>,
    has_disabled_options: bool,
) -> ListState {
    let has_items = item_count > 0;
    let has_selection = selected_index.filter(|index| *index < item_count).is_some();

    ListState {
        is_empty: !has_items,
        has_items,
        has_selection,
        has_disabled_options,
    }
}

pub(crate) mod item {
    use super::normalize_optional_text;

    pub(crate) const DEFAULT_ARIA_LABEL: &str = "Listbox item";

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ListItemSelectionIndicator {
        Hidden,
        Checkmark,
    }

    impl ListItemSelectionIndicator {
        pub fn marker(self, is_selected: bool) -> Option<&'static str> {
            match self {
                ListItemSelectionIndicator::Hidden => None,
                ListItemSelectionIndicator::Checkmark => is_selected.then_some("✓"),
            }
        }

        pub fn as_attr(self) -> &'static str {
            match self {
                ListItemSelectionIndicator::Hidden => "hidden",
                ListItemSelectionIndicator::Checkmark => "checkmark",
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ListItemStateInput {
        pub selected: bool,
        pub focused: bool,
        pub disabled: bool,
        pub show_selection_indicator: bool,
        pub has_divider: bool,
        pub has_custom_aria_label: bool,
        pub has_custom_class_name: bool,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ListItemState {
        pub is_selected: bool,
        pub is_focused: bool,
        pub is_disabled: bool,
        pub show_selection_indicator: bool,
        pub has_divider: bool,
        pub has_custom_aria_label: bool,
        pub has_custom_class_name: bool,
        pub data_state_attr: &'static str,
        pub selection_indicator_attr: &'static str,
        pub aria_source_attr: &'static str,
        pub class_source_attr: &'static str,
    }

    pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
        if let Some(label) = normalize_optional_text(value) {
            return (label, true);
        }

        (DEFAULT_ARIA_LABEL.to_string(), false)
    }

    pub fn normalize_class_name(value: Option<String>) -> Option<String> {
        normalize_optional_text(value)
    }

    pub fn resolve_selection_indicator(
        show_selection_indicator: bool,
    ) -> ListItemSelectionIndicator {
        if show_selection_indicator {
            ListItemSelectionIndicator::Checkmark
        } else {
            ListItemSelectionIndicator::Hidden
        }
    }

    pub fn resolve_state(input: ListItemStateInput) -> ListItemState {
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

        ListItemState {
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

    pub fn compose_class_name(base_class_name: Option<String>, state: ListItemState) -> String {
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
            assert_eq!(normalize_class_name(None), None);
            assert_eq!(normalize_class_name(Some("  \n\t  ".to_string())), None);
            assert_eq!(
                normalize_class_name(Some("  San Jose  ".to_string())),
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
                ListItemSelectionIndicator::Hidden
            );
            assert_eq!(
                resolve_selection_indicator(true),
                ListItemSelectionIndicator::Checkmark
            );
            assert_eq!(ListItemSelectionIndicator::Hidden.as_attr(), "hidden");
            assert_eq!(ListItemSelectionIndicator::Checkmark.as_attr(), "checkmark");
        }

        #[test]
        fn resolve_state_tracks_selection_focus_and_sources() {
            let state = resolve_state(ListItemStateInput {
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
            let state = resolve_state(ListItemStateInput {
                selected: true,
                focused: false,
                disabled: false,
                show_selection_indicator: true,
                has_divider: true,
                has_custom_aria_label: false,
                has_custom_class_name: true,
            });

            let class_name =
                compose_class_name(Some("docs-listbox-item-custom".to_string()), state);

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
                    "ListItem class list should include `{needle}`"
                );
            }
        }
    }
}

pub(crate) mod section {
    use super::normalize_optional_text;

    pub(crate) const DEFAULT_ARIA_LABEL: &str = "Listbox section";

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub enum ListSectionHeadingTone {
        #[default]
        Default,
        Quiet,
    }

    impl ListSectionHeadingTone {
        pub fn class_name(self) -> &'static str {
            match self {
                Self::Default => "ui-listbox-section--tone-default",
                Self::Quiet => "ui-listbox-section--tone-quiet",
            }
        }

        pub fn as_attr(self) -> &'static str {
            match self {
                Self::Default => "default",
                Self::Quiet => "quiet",
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ListSectionStateInput {
        pub heading_tone: ListSectionHeadingTone,
        pub item_count: usize,
        pub disabled: bool,
        pub sticky_heading: bool,
        pub show_divider: bool,
        pub has_title: bool,
        pub has_custom_aria_label: bool,
        pub has_custom_class_name: bool,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ListSectionState {
        pub heading_tone: ListSectionHeadingTone,
        pub heading_tone_class: &'static str,
        pub heading_tone_attr: &'static str,
        pub item_count: usize,
        pub has_items: bool,
        pub is_empty: bool,
        pub is_disabled: bool,
        pub has_title: bool,
        pub is_sticky_heading: bool,
        pub has_divider: bool,
        pub has_custom_aria_label: bool,
        pub has_custom_class_name: bool,
        pub data_state_attr: &'static str,
        pub aria_source_attr: &'static str,
        pub class_source_attr: &'static str,
        pub title_source_attr: &'static str,
    }

    pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
        if let Some(label) = normalize_optional_text(value) {
            return (label, true);
        }

        (DEFAULT_ARIA_LABEL.to_string(), false)
    }

    pub fn normalize_title(value: Option<String>) -> Option<String> {
        normalize_optional_text(value)
    }

    pub fn normalize_class_name(value: Option<String>) -> Option<String> {
        normalize_optional_text(value)
    }

    pub fn resolve_state(input: ListSectionStateInput) -> ListSectionState {
        let has_items = input.item_count > 0;
        let is_empty = !has_items;

        let data_state_attr = if input.disabled && is_empty {
            "disabled-empty"
        } else if input.disabled {
            "disabled"
        } else if is_empty {
            "empty"
        } else if input.sticky_heading {
            "sticky"
        } else if input.show_divider {
            "divided"
        } else {
            "default"
        };

        ListSectionState {
            heading_tone: input.heading_tone,
            heading_tone_class: input.heading_tone.class_name(),
            heading_tone_attr: input.heading_tone.as_attr(),
            item_count: input.item_count,
            has_items,
            is_empty,
            is_disabled: input.disabled,
            has_title: input.has_title,
            is_sticky_heading: input.sticky_heading,
            has_divider: input.show_divider,
            has_custom_aria_label: input.has_custom_aria_label,
            has_custom_class_name: input.has_custom_class_name,
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
            title_source_attr: if input.has_title { "custom" } else { "none" },
        }
    }

    pub fn compose_class_name(base_class_name: Option<String>, state: ListSectionState) -> String {
        let mut classes = vec![
            "ui-listbox-section".to_string(),
            state.heading_tone_class.to_string(),
        ];

        if state.has_title {
            classes.push("ui-listbox-section--has-title".to_string());
        }

        if state.is_empty {
            classes.push("ui-listbox-section--empty".to_string());
        }

        if state.is_disabled {
            classes.push("ui-listbox-section--disabled".to_string());
        }

        if state.is_sticky_heading {
            classes.push("ui-listbox-section--sticky-heading".to_string());
        }

        if state.has_divider {
            classes.push("ui-listbox-section--divided".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-listbox-section--custom-class".to_string());
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
        fn heading_tone_contract_is_stable() {
            assert_eq!(
                ListSectionHeadingTone::Default.class_name(),
                "ui-listbox-section--tone-default"
            );
            assert_eq!(
                ListSectionHeadingTone::Quiet.class_name(),
                "ui-listbox-section--tone-quiet"
            );
            assert_eq!(ListSectionHeadingTone::Default.as_attr(), "default");
            assert_eq!(ListSectionHeadingTone::Quiet.as_attr(), "quiet");
        }

        #[test]
        fn normalize_helpers_trim_and_fallback() {
            assert_eq!(normalize_title(None), None);
            assert_eq!(normalize_title(Some(" \n\t ".to_string())), None);
            assert_eq!(
                normalize_title(Some("  Favorite regions  ".to_string())),
                Some("Favorite regions".to_string())
            );

            assert_eq!(
                normalize_aria_label(Some("  Region choices  ".to_string())),
                ("Region choices".to_string(), true)
            );
            assert_eq!(
                normalize_aria_label(Some("".to_string())),
                (DEFAULT_ARIA_LABEL.to_string(), false)
            );
        }

        #[test]
        fn resolve_state_tracks_markers() {
            let state = resolve_state(ListSectionStateInput {
                heading_tone: ListSectionHeadingTone::Quiet,
                item_count: 0,
                disabled: true,
                sticky_heading: true,
                show_divider: true,
                has_title: true,
                has_custom_aria_label: true,
                has_custom_class_name: true,
            });

            assert!(state.is_empty);
            assert!(!state.has_items);
            assert!(state.is_disabled);
            assert!(state.has_title);
            assert!(state.is_sticky_heading);
            assert!(state.has_divider);
            assert_eq!(state.data_state_attr, "disabled-empty");
            assert_eq!(state.aria_source_attr, "custom");
            assert_eq!(state.class_source_attr, "custom");
            assert_eq!(state.title_source_attr, "custom");
        }

        #[test]
        fn compose_class_name_includes_state_markers() {
            let state = resolve_state(ListSectionStateInput {
                heading_tone: ListSectionHeadingTone::Default,
                item_count: 3,
                disabled: false,
                sticky_heading: true,
                show_divider: true,
                has_title: true,
                has_custom_aria_label: false,
                has_custom_class_name: true,
            });

            let class_name =
                compose_class_name(Some("docs-listbox-section-custom".to_string()), state);

            for needle in [
                "ui-listbox-section",
                "ui-listbox-section--tone-default",
                "ui-listbox-section--has-title",
                "ui-listbox-section--sticky-heading",
                "ui-listbox-section--divided",
                "ui-listbox-section--custom-class",
                "docs-listbox-section-custom",
            ] {
                assert!(
                    class_name.contains(needle),
                    "ListSection class list should include `{needle}`"
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_accessible_name_prefers_explicit_aria_label() {
        assert_eq!(
            resolve_accessible_name(
                Some("  Fruit options  ".to_string()),
                Some("trigger-id".to_string())
            ),
            ListAccessibleName {
                aria_label: Some("Fruit options".to_string()),
                aria_labelledby: None,
            }
        );
    }

    #[test]
    fn resolve_accessible_name_uses_labelledby_when_label_missing() {
        assert_eq!(
            resolve_accessible_name(None, Some("  trigger-id  ".to_string())),
            ListAccessibleName {
                aria_label: None,
                aria_labelledby: Some("trigger-id".to_string()),
            }
        );
    }

    #[test]
    fn resolve_accessible_name_defaults_when_none_provided() {
        assert_eq!(
            resolve_accessible_name(None, None),
            ListAccessibleName {
                aria_label: Some("Listbox".to_string()),
                aria_labelledby: None,
            }
        );
    }

    #[test]
    fn resolve_accessible_name_ignores_blank_inputs() {
        assert_eq!(
            resolve_accessible_name(Some("  ".to_string()), Some("".to_string())),
            ListAccessibleName {
                aria_label: Some("Listbox".to_string()),
                aria_labelledby: None,
            }
        );
    }

    #[test]
    fn resolve_state_tracks_item_and_selection_flags() {
        let state = resolve_state(4, Some(2), true);
        assert!(!state.is_empty);
        assert!(state.has_items);
        assert!(state.has_selection);
        assert!(state.has_disabled_options);
    }

    #[test]
    fn resolve_state_treats_out_of_range_selection_as_empty_selection() {
        let state = resolve_state(2, Some(9), false);
        assert!(!state.is_empty);
        assert!(state.has_items);
        assert!(!state.has_selection);
        assert!(!state.has_disabled_options);
    }

    #[test]
    fn resolve_state_handles_empty_listbox() {
        let state = resolve_state(0, None, false);
        assert!(state.is_empty);
        assert!(!state.has_items);
        assert!(!state.has_selection);
        assert!(!state.has_disabled_options);
    }
}
