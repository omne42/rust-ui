use crate::{TextState, TextStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Text";
pub const DEFAULT_TEXT: &str = "—";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextTone {
    #[default]
    Default,
    Subtle,
    Strong,
}

impl TextTone {
    pub fn class_name(self) -> &'static str {
        match self {
            TextTone::Default => "ui-text--tone-default",
            TextTone::Subtle => "ui-text--tone-subtle",
            TextTone::Strong => "ui-text--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TextTone::Default => "default",
            TextTone::Subtle => "subtle",
            TextTone::Strong => "strong",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Start,
    Center,
    End,
    Justify,
}

impl TextAlign {
    pub fn class_name(self) -> &'static str {
        match self {
            TextAlign::Start => "ui-text--align-start",
            TextAlign::Center => "ui-text--align-center",
            TextAlign::End => "ui-text--align-end",
            TextAlign::Justify => "ui-text--align-justify",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TextAlign::Start => "start",
            TextAlign::Center => "center",
            TextAlign::End => "end",
            TextAlign::Justify => "justify",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextWeight {
    #[default]
    Regular,
    Medium,
    Semibold,
    Bold,
}

impl TextWeight {
    pub fn class_name(self) -> &'static str {
        match self {
            TextWeight::Regular => "ui-text--weight-regular",
            TextWeight::Medium => "ui-text--weight-medium",
            TextWeight::Semibold => "ui-text--weight-semibold",
            TextWeight::Bold => "ui-text--weight-bold",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TextWeight::Regular => "regular",
            TextWeight::Medium => "medium",
            TextWeight::Semibold => "semibold",
            TextWeight::Bold => "bold",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextElement {
    Span,
    #[default]
    Paragraph,
    Div,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_content(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_TEXT.into())
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_slot_kind_attr(slot: Option<&str>) -> &'static str {
    match slot {
        None => "none",
        Some(slot) if slot.eq_ignore_ascii_case("label") => "label",
        Some(slot) if slot.eq_ignore_ascii_case("description") => "description",
        Some(slot) if slot.eq_ignore_ascii_case("icon") => "icon",
        Some(_) => "custom",
    }
}

pub fn resolve_state(input: TextStateInput) -> TextState {
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

    let data_state_attr = if input.disabled {
        "disabled"
    } else if input.truncate {
        "truncate"
    } else {
        "default"
    };

    TextState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        align: input.align,
        align_class: input.align.class_name(),
        align_attr: input.align.as_attr(),
        weight: input.weight,
        weight_class: input.weight.class_name(),
        weight_attr: input.weight.as_attr(),
        is_disabled: input.disabled,
        is_truncated: input.truncate,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
        slot_kind_attr: input.slot_kind_attr,
        has_named_slot: input.has_named_slot,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: TextState) -> String {
    let mut classes = vec![
        "ui-text".to_string(),
        state.tone_class.into(),
        state.align_class.into(),
        state.weight_class.into(),
    ];

    if state.is_disabled {
        classes.push("ui-text--disabled".to_string());
    }
    if state.is_truncated {
        classes.push("ui-text--truncate".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-text--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/logic.rs"]
mod tests;
