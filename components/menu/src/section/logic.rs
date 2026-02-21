use crate::menu::section::{MenuSectionState, MenuSectionStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Menu section";
pub const DEFAULT_ITEM_COUNT: usize = 1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuSectionNormalizedProps {
    pub title_text: String,
    pub has_title: bool,
    pub item_count: usize,
    pub disabled: bool,
}

pub struct MenuSectionNormalizeInput {
    pub title: Option<String>,
    pub item_count: Option<usize>,
    pub is_disabled: Option<bool>,
    pub disabled: bool,
}

pub fn normalize_props(input: MenuSectionNormalizeInput) -> MenuSectionNormalizedProps {
    let title = normalize_optional_text(input.title);

    MenuSectionNormalizedProps {
        title_text: title.clone().unwrap_or_default(),
        has_title: title.is_some(),
        item_count: input.item_count.unwrap_or(DEFAULT_ITEM_COUNT),
        disabled: input.is_disabled.unwrap_or(input.disabled),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MenuSectionHeadingTone {
    #[default]
    Default,
    Quiet,
}

impl MenuSectionHeadingTone {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Default => "ui-menu-section--tone-default",
            Self::Quiet => "ui-menu-section--tone-quiet",
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
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_state(input: MenuSectionStateInput) -> MenuSectionState {
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

    MenuSectionState {
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

pub fn compose_class_name(base_class_name: Option<String>, state: MenuSectionState) -> String {
    let mut classes = vec![
        "ui-menu-section".to_string(),
        state.heading_tone_class.into(),
    ];

    if state.has_title {
        classes.push("ui-menu-section--has-title".to_string());
    }

    if state.is_empty {
        classes.push("ui-menu-section--empty".to_string());
    }

    if state.is_disabled {
        classes.push("ui-menu-section--disabled".to_string());
    }

    if state.is_sticky_heading {
        classes.push("ui-menu-section--sticky-heading".to_string());
    }

    if state.has_divider {
        classes.push("ui-menu-section--divided".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-menu-section--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/section/logic.rs"]
mod tests;
