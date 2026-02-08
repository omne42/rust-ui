use crate::logic_button::{LogicButtonState, LogicButtonStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Logic operator";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LogicButtonVariant {
    #[default]
    And,
    Or,
}

impl LogicButtonVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            LogicButtonVariant::And => "ui-logic-button--variant-and",
            LogicButtonVariant::Or => "ui-logic-button--variant-or",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            LogicButtonVariant::And => "and",
            LogicButtonVariant::Or => "or",
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

pub fn resolve_state(input: LogicButtonStateInput) -> LogicButtonState {
    LogicButtonState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_attr(),
        is_disabled: input.disabled,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_press_handler: input.has_custom_press_handler,
        data_state_attr: if input.disabled { "disabled" } else { "ready" },
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

pub fn compose_class_name(base_class_name: Option<String>, state: LogicButtonState) -> String {
    let mut classes = vec![
        "ui-logic-button".to_string(),
        state.variant_class.to_string(),
    ];

    if state.is_disabled {
        classes.push("ui-logic-button--disabled".to_string());
    }

    if state.has_custom_press_handler {
        classes.push("ui-logic-button--custom-handler".to_string());
    }

    if state.has_custom_aria_label {
        classes.push("ui-logic-button--custom-aria-label".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-logic-button--custom-class".to_string());
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
    fn variant_contract_is_stable() {
        assert_eq!(
            LogicButtonVariant::And.class_name(),
            "ui-logic-button--variant-and"
        );
        assert_eq!(
            LogicButtonVariant::Or.class_name(),
            "ui-logic-button--variant-or"
        );

        assert_eq!(LogicButtonVariant::And.as_attr(), "and");
        assert_eq!(LogicButtonVariant::Or.as_attr(), "or");
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-logic-button  ".to_string())),
            Some("docs-logic-button".to_string())
        );

        let (aria_label, custom) = normalize_aria_label(Some("  Logical operator  ".to_string()));
        assert_eq!(aria_label, "Logical operator");
        assert!(custom);

        let (aria_label, custom) = normalize_aria_label(None);
        assert_eq!(aria_label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_state_tracks_variant_and_sources() {
        let state = resolve_state(LogicButtonStateInput {
            variant: LogicButtonVariant::Or,
            disabled: false,
            has_custom_aria_label: true,
            has_custom_class_name: false,
            has_custom_press_handler: true,
        });

        assert_eq!(state.variant_attr, "or");
        assert!(!state.is_disabled);
        assert_eq!(state.data_state_attr, "ready");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-logic-button-custom".to_string()),
            resolve_state(LogicButtonStateInput {
                variant: LogicButtonVariant::And,
                disabled: true,
                has_custom_aria_label: false,
                has_custom_class_name: true,
                has_custom_press_handler: true,
            }),
        );

        for token in [
            "ui-logic-button",
            "ui-logic-button--variant-and",
            "ui-logic-button--disabled",
            "ui-logic-button--custom-handler",
            "ui-logic-button--custom-class",
            "docs-logic-button-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
