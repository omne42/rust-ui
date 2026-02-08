use crate::listbox_section::{ListBoxSectionState, ListBoxSectionStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Listbox section";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ListBoxSectionHeadingTone {
    #[default]
    Default,
    Quiet,
}

impl ListBoxSectionHeadingTone {
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

pub fn resolve_state(input: ListBoxSectionStateInput) -> ListBoxSectionState {
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

    ListBoxSectionState {
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

pub fn compose_class_name(base_class_name: Option<String>, state: ListBoxSectionState) -> String {
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
            ListBoxSectionHeadingTone::Default.class_name(),
            "ui-listbox-section--tone-default"
        );
        assert_eq!(
            ListBoxSectionHeadingTone::Quiet.class_name(),
            "ui-listbox-section--tone-quiet"
        );
        assert_eq!(ListBoxSectionHeadingTone::Default.as_attr(), "default");
        assert_eq!(ListBoxSectionHeadingTone::Quiet.as_attr(), "quiet");
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Favorite regions  ".to_string())),
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
        let state = resolve_state(ListBoxSectionStateInput {
            heading_tone: ListBoxSectionHeadingTone::Quiet,
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
        let state = resolve_state(ListBoxSectionStateInput {
            heading_tone: ListBoxSectionHeadingTone::Default,
            item_count: 3,
            disabled: false,
            sticky_heading: true,
            show_divider: true,
            has_title: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-listbox-section-custom".to_string()), state);

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
                "ListBoxSection class list should include `{needle}`"
            );
        }
    }
}
