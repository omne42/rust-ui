pub const DEFAULT_TITLE: &str = "Nothing to show";
pub const DEFAULT_DESCRIPTION: &str = "Try adjusting filters or refreshing data.";
pub const DEFAULT_ARIA_LABEL: &str = "Empty state";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EmptyStateTone {
    #[default]
    Default,
    Muted,
    Accent,
}

impl EmptyStateTone {
    pub fn class_name(self) -> &'static str {
        match self {
            EmptyStateTone::Default => "ui-empty-state--tone-default",
            EmptyStateTone::Muted => "ui-empty-state--tone-muted",
            EmptyStateTone::Accent => "ui-empty-state--tone-accent",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            EmptyStateTone::Default => "default",
            EmptyStateTone::Muted => "muted",
            EmptyStateTone::Accent => "accent",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EmptyStateAlign {
    #[default]
    Start,
    Center,
}

impl EmptyStateAlign {
    pub fn class_name(self) -> &'static str {
        match self {
            EmptyStateAlign::Start => "ui-empty-state--align-start",
            EmptyStateAlign::Center => "ui-empty-state--align-center",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            EmptyStateAlign::Start => "start",
            EmptyStateAlign::Center => "center",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmptyStateStateInput {
    pub tone: EmptyStateTone,
    pub align: EmptyStateAlign,
    pub compact: bool,
    pub bordered: bool,
    pub has_icon: bool,
    pub has_actions: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmptyStateState {
    pub tone: EmptyStateTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub align: EmptyStateAlign,
    pub align_class: &'static str,
    pub align_attr: &'static str,
    pub is_compact: bool,
    pub is_bordered: bool,
    pub has_icon: bool,
    pub has_actions: bool,
    pub data_state_attr: &'static str,
    pub title_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_title(value: Option<String>, default: &str) -> (String, bool) {
    normalize_optional_text(value)
        .map(|title| (title, true))
        .unwrap_or_else(|| (default.into(), false))
}

pub fn normalize_description(value: Option<String>, default: &str) -> (String, bool) {
    normalize_optional_text(value)
        .map(|description| (description, true))
        .unwrap_or_else(|| (default.into(), false))
}

pub fn normalize_aria_label(value: Option<String>, default: &str) -> (String, bool) {
    normalize_optional_text(value)
        .map(|label| (label, true))
        .unwrap_or_else(|| (default.into(), false))
}

pub fn resolve_state(input: EmptyStateStateInput) -> EmptyStateState {
    let data_state_attr = if input.has_icon && input.has_actions {
        "rich"
    } else if input.has_actions {
        "actions"
    } else if input.has_icon {
        "icon"
    } else {
        "plain"
    };

    let title_source_attr = if input.has_custom_title {
        "custom"
    } else {
        "default"
    };

    let description_source_attr = if input.has_custom_description {
        "custom"
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

    EmptyStateState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        align: input.align,
        align_class: input.align.class_name(),
        align_attr: input.align.as_attr(),
        is_compact: input.compact,
        is_bordered: input.bordered,
        has_icon: input.has_icon,
        has_actions: input.has_actions,
        data_state_attr,
        title_source_attr,
        description_source_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: EmptyStateState) -> String {
    let mut classes = vec![
        "ui-empty-state".to_string(),
        state.tone_class.into(),
        state.align_class.into(),
    ];

    if state.is_compact {
        classes.push("ui-empty-state--compact".to_string());
    }

    if state.is_bordered {
        classes.push("ui-empty-state--bordered".to_string());
    }

    if state.has_icon {
        classes.push("ui-empty-state--with-icon".to_string());
    }

    if state.has_actions {
        classes.push("ui-empty-state--with-actions".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-empty-state--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/empty_state.rs"]
mod tests;
