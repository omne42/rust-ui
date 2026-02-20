pub const DEFAULT_TEXT: &str = "Group";
pub const DEFAULT_REQUIRED_INDICATOR: &str = "*";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LegendTone {
    #[default]
    Default,
    Muted,
    Strong,
}

impl LegendTone {
    pub fn class_name(self) -> &'static str {
        match self {
            LegendTone::Default => "ui-legend--tone-default",
            LegendTone::Muted => "ui-legend--tone-muted",
            LegendTone::Strong => "ui-legend--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            LegendTone::Default => "default",
            LegendTone::Muted => "muted",
            LegendTone::Strong => "strong",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LegendStateInput {
    pub tone: LegendTone,
    pub is_required: bool,
    pub is_disabled: bool,
    pub has_custom_text: bool,
    pub has_custom_indicator: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LegendState {
    pub tone: LegendTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_required: bool,
    pub is_optional: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_custom_text: bool,
    pub has_custom_indicator: bool,
    pub has_custom_class_name: bool,
    pub text_source_attr: &'static str,
    pub indicator_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_text(value: Option<String>) -> (String, bool) {
    if let Some(text) = normalize_optional_text(value) {
        (text, true)
    } else {
        (DEFAULT_TEXT.into(), false)
    }
}

pub fn normalize_required_indicator(value: Option<String>) -> (String, bool) {
    if let Some(indicator) = normalize_optional_text(value) {
        (indicator, true)
    } else {
        (DEFAULT_REQUIRED_INDICATOR.into(), false)
    }
}

pub fn source_attr_from_presence(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: LegendStateInput) -> LegendState {
    LegendState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_required: input.is_required,
        is_optional: !input.is_required,
        is_disabled: input.is_disabled,
        is_enabled: !input.is_disabled,
        has_custom_text: input.has_custom_text,
        has_custom_indicator: input.has_custom_indicator,
        has_custom_class_name: input.has_custom_class_name,
        text_source_attr: source_attr_from_presence(input.has_custom_text),
        indicator_source_attr: source_attr_from_presence(input.has_custom_indicator),
        class_source_attr: source_attr_from_presence(input.has_custom_class_name),
    }
}

#[cfg(test)]
#[path = "test/legend.rs"]
mod tests;
