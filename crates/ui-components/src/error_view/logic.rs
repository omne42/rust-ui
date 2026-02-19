use ui_state_primitives::error_view as error_view_state;
pub use ui_state_primitives::error_view::{
    DEFAULT_ARIA_LABEL, DEFAULT_MESSAGE, ErrorViewState, ErrorViewStateInput, ErrorViewTone,
    normalize_aria_label, normalize_message, normalize_optional_text,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorViewNormalizeInput {
    pub tone: Option<ErrorViewTone>,
    pub is_invalid: bool,
    pub is_compact: Option<bool>,
    pub is_bordered: Option<bool>,
    pub message: Option<String>,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
    pub has_icon: bool,
    pub has_actions: bool,
    pub has_children: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorViewNormalizedProps {
    pub state_input: ErrorViewStateInput,
    pub message: String,
    pub aria_label: String,
    pub class_name: Option<String>,
    pub tone_source_attr: &'static str,
    pub compact_source_attr: &'static str,
    pub bordered_source_attr: &'static str,
}

pub fn resolve_state(input: ErrorViewStateInput) -> ErrorViewState {
    error_view_state::resolve_state(input)
}

pub fn compose_class_name(base_class_name: Option<String>, state: ErrorViewState) -> String {
    error_view_state::compose_class_name(base_class_name, state)
}

fn source_attr_from_presence(is_present: bool) -> &'static str {
    if is_present { "prop" } else { "default" }
}

fn resolve_bool_axis(value: Option<bool>, default_value: bool) -> (bool, &'static str) {
    if let Some(value) = value {
        return (value, "is-prop");
    }
    (default_value, "default")
}

pub fn normalize_props(input: ErrorViewNormalizeInput) -> ErrorViewNormalizedProps {
    let tone = input.tone.unwrap_or_default();
    let tone_source_attr = source_attr_from_presence(input.tone.is_some());

    let (compact, compact_source_attr) = resolve_bool_axis(input.is_compact, false);
    let (bordered, bordered_source_attr) = resolve_bool_axis(input.is_bordered, false);

    let (message, has_custom_message) = normalize_message(input.message);
    let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);
    let class_name = normalize_optional_text(input.class_name);

    ErrorViewNormalizedProps {
        state_input: ErrorViewStateInput {
            tone,
            is_invalid: input.is_invalid,
            compact,
            bordered,
            has_icon: input.has_icon,
            has_actions: input.has_actions,
            has_children: input.has_children,
            has_custom_message,
            has_custom_aria_label,
            has_custom_class_name: class_name.is_some(),
            has_custom_motion: input.has_custom_motion,
        },
        message,
        aria_label,
        class_name,
        tone_source_attr,
        compact_source_attr,
        bordered_source_attr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitives_are_reexported_from_ui_state_primitives() {
        assert_eq!(
            normalize_optional_text(Some("  bad input  ".to_string())),
            Some("bad input".to_string())
        );
        assert_eq!(ErrorViewTone::Neutral.as_attr(), "neutral");
    }

    #[test]
    fn normalize_props_centralizes_defaults_and_source_markers() {
        let normalized = normalize_props(ErrorViewNormalizeInput {
            tone: None,
            is_invalid: true,
            is_compact: Some(true),
            is_bordered: None,
            message: Some("  Email invalid  ".to_string()),
            aria_label: None,
            class_name: Some("  docs-error-view  ".to_string()),
            has_icon: true,
            has_actions: false,
            has_children: false,
            has_custom_motion: true,
        });

        assert_eq!(normalized.state_input.tone, ErrorViewTone::Negative);
        assert!(normalized.state_input.is_invalid);
        assert!(normalized.state_input.compact);
        assert!(!normalized.state_input.bordered);
        assert!(normalized.state_input.has_custom_message);
        assert!(!normalized.state_input.has_custom_aria_label);
        assert!(normalized.state_input.has_custom_class_name);
        assert!(normalized.state_input.has_custom_motion);
        assert_eq!(normalized.message, "Email invalid");
        assert_eq!(normalized.aria_label, DEFAULT_ARIA_LABEL);
        assert_eq!(normalized.class_name, Some("docs-error-view".to_string()));
        assert_eq!(normalized.tone_source_attr, "default");
        assert_eq!(normalized.compact_source_attr, "is-prop");
        assert_eq!(normalized.bordered_source_attr, "default");
    }
}
