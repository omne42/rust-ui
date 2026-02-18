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
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_text(value: Option<String>) -> (String, bool) {
    if let Some(text) = normalize_optional_text(value) {
        (text, true)
    } else {
        (DEFAULT_TEXT.to_string(), false)
    }
}

pub fn normalize_required_indicator(value: Option<String>) -> (String, bool) {
    if let Some(indicator) = normalize_optional_text(value) {
        (indicator, true)
    } else {
        (DEFAULT_REQUIRED_INDICATOR.to_string(), false)
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
mod tests {
    use super::*;

    #[test]
    fn tone_class_names_and_attrs_are_stable() {
        assert_eq!(LegendTone::Default.class_name(), "ui-legend--tone-default");
        assert_eq!(LegendTone::Muted.class_name(), "ui-legend--tone-muted");
        assert_eq!(LegendTone::Strong.class_name(), "ui-legend--tone-strong");

        assert_eq!(LegendTone::Default.as_attr(), "default");
        assert_eq!(LegendTone::Muted.as_attr(), "muted");
        assert_eq!(LegendTone::Strong.as_attr(), "strong");
    }

    #[test]
    fn normalize_helpers_fallback_to_defaults() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Preferences  ".to_string())),
            Some("Preferences".to_string())
        );

        assert_eq!(
            normalize_text(Some("  Notification settings  ".to_string())),
            ("Notification settings".to_string(), true)
        );
        assert_eq!(normalize_text(None), (DEFAULT_TEXT.to_string(), false));

        assert_eq!(
            normalize_required_indicator(Some("  (required)  ".to_string())),
            ("(required)".to_string(), true)
        );
        assert_eq!(
            normalize_required_indicator(None),
            (DEFAULT_REQUIRED_INDICATOR.to_string(), false)
        );
    }

    #[test]
    fn resolve_state_tracks_required_disabled_and_sources() {
        let state = resolve_state(LegendStateInput {
            tone: LegendTone::Strong,
            is_required: true,
            is_disabled: true,
            has_custom_text: true,
            has_custom_indicator: false,
            has_custom_class_name: true,
        });

        assert_eq!(state.tone_attr, "strong");
        assert!(state.is_required);
        assert!(!state.is_optional);
        assert!(state.is_disabled);
        assert_eq!(state.text_source_attr, "custom");
        assert_eq!(state.indicator_source_attr, "default");
        assert_eq!(state.class_source_attr, "custom");
    }
}
