use crate::clear_button::{ClearButtonState, ClearButtonStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Clear";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ClearButtonVariant {
    #[default]
    Default,
    OverBackground,
}

impl ClearButtonVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            ClearButtonVariant::Default => "ui-clear-button--variant-default",
            ClearButtonVariant::OverBackground => "ui-clear-button--variant-over-background",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ClearButtonVariant::Default => "default",
            ClearButtonVariant::OverBackground => "over-background",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>, default: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (default.to_string(), false)
}

pub fn resolve_state(input: ClearButtonStateInput) -> ClearButtonState {
    let data_state_attr = if input.disabled && input.inset {
        "disabled-inset"
    } else if input.disabled {
        "disabled"
    } else if input.prevent_focus {
        "prevent-focus"
    } else if input.exclude_from_tab_order {
        "exclude-tab"
    } else if input.inset {
        "inset"
    } else {
        "ready"
    };

    ClearButtonState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_attr(),
        is_inset: input.inset,
        is_disabled: input.disabled,
        prevent_focus: input.prevent_focus,
        exclude_from_tab_order: input.exclude_from_tab_order,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_press_handler: input.has_custom_press_handler,
        data_state_attr,
        focus_mode_attr: if input.prevent_focus {
            "prevent"
        } else if input.exclude_from_tab_order {
            "exclude-tab"
        } else {
            "default"
        },
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

pub fn compose_class_name(base_class_name: Option<String>, state: ClearButtonState) -> String {
    let mut classes = vec![
        "ui-clear-button".to_string(),
        state.variant_class.to_string(),
    ];

    if state.is_inset {
        classes.push("ui-clear-button--inset".to_string());
    }

    if state.is_disabled {
        classes.push("ui-clear-button--disabled".to_string());
    }

    if state.prevent_focus {
        classes.push("ui-clear-button--prevent-focus".to_string());
    }

    if state.exclude_from_tab_order {
        classes.push("ui-clear-button--exclude-tab".to_string());
    }

    if state.has_custom_press_handler {
        classes.push("ui-clear-button--custom-handler".to_string());
    }

    if state.has_custom_aria_label {
        classes.push("ui-clear-button--custom-aria-label".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-clear-button--custom-class".to_string());
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
            ClearButtonVariant::Default.class_name(),
            "ui-clear-button--variant-default"
        );
        assert_eq!(
            ClearButtonVariant::OverBackground.class_name(),
            "ui-clear-button--variant-over-background"
        );

        assert_eq!(ClearButtonVariant::Default.as_attr(), "default");
        assert_eq!(
            ClearButtonVariant::OverBackground.as_attr(),
            "over-background"
        );
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-clear-button  ".to_string())),
            Some("docs-clear-button".to_string())
        );

        let (aria_label, custom) =
            normalize_aria_label(Some("  Clear query  ".to_string()), DEFAULT_ARIA_LABEL);
        assert_eq!(aria_label, "Clear query");
        assert!(custom);

        let (aria_label, custom) = normalize_aria_label(None, DEFAULT_ARIA_LABEL);
        assert_eq!(aria_label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_state_tracks_variant_focus_mode_and_sources() {
        let state = resolve_state(ClearButtonStateInput {
            variant: ClearButtonVariant::OverBackground,
            inset: true,
            disabled: false,
            prevent_focus: true,
            exclude_from_tab_order: false,
            has_custom_aria_label: true,
            has_custom_class_name: false,
            has_custom_press_handler: true,
        });

        assert_eq!(state.variant_attr, "over-background");
        assert!(state.is_inset);
        assert!(!state.is_disabled);
        assert!(state.prevent_focus);
        assert!(!state.exclude_from_tab_order);
        assert_eq!(state.data_state_attr, "prevent-focus");
        assert_eq!(state.focus_mode_attr, "prevent");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-clear-button-custom".to_string()),
            resolve_state(ClearButtonStateInput {
                variant: ClearButtonVariant::Default,
                inset: false,
                disabled: true,
                prevent_focus: false,
                exclude_from_tab_order: true,
                has_custom_aria_label: false,
                has_custom_class_name: true,
                has_custom_press_handler: true,
            }),
        );

        for token in [
            "ui-clear-button",
            "ui-clear-button--variant-default",
            "ui-clear-button--disabled",
            "ui-clear-button--exclude-tab",
            "ui-clear-button--custom-handler",
            "ui-clear-button--custom-class",
            "docs-clear-button-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
