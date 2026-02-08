use crate::date_input_group::{DateInputGroupState, DateInputGroupStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Date input group";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DateInputGroupVariant {
    #[default]
    Primary,
    Secondary,
}

impl DateInputGroupVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            DateInputGroupVariant::Primary => "ui-date-input-group--variant-primary",
            DateInputGroupVariant::Secondary => "ui-date-input-group--variant-secondary",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            DateInputGroupVariant::Primary => "primary",
            DateInputGroupVariant::Secondary => "secondary",
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

pub fn resolve_state(input: DateInputGroupStateInput) -> DateInputGroupState {
    let width_class = if input.full_width {
        "ui-date-input-group--full-width"
    } else {
        "ui-date-input-group--fit-width"
    };

    let width_attr = if input.full_width { "full" } else { "fit" };

    let data_state_attr = if input.disabled && input.invalid {
        "disabled-invalid"
    } else if input.disabled {
        "disabled"
    } else if input.invalid {
        "invalid"
    } else if input.segmented {
        "segmented"
    } else {
        "default"
    };

    DateInputGroupState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_attr(),
        width_class,
        width_attr,
        is_full_width: input.full_width,
        is_disabled: input.disabled,
        is_invalid: input.invalid,
        is_segmented: input.segmented,
        has_prefix: input.has_prefix,
        has_suffix: input.has_suffix,
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
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: DateInputGroupState) -> String {
    let mut classes = vec![
        "ui-date-input-group".to_string(),
        state.variant_class.to_string(),
        state.width_class.to_string(),
    ];

    if state.is_disabled {
        classes.push("ui-date-input-group--disabled".to_string());
    }

    if state.is_invalid {
        classes.push("ui-date-input-group--invalid".to_string());
    }

    if state.is_segmented {
        classes.push("ui-date-input-group--segmented".to_string());
    }

    if state.has_prefix {
        classes.push("ui-date-input-group--has-prefix".to_string());
    }

    if state.has_suffix {
        classes.push("ui-date-input-group--has-suffix".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-date-input-group--custom-class".to_string());
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
            DateInputGroupVariant::Primary.class_name(),
            "ui-date-input-group--variant-primary"
        );
        assert_eq!(
            DateInputGroupVariant::Secondary.class_name(),
            "ui-date-input-group--variant-secondary"
        );

        assert_eq!(DateInputGroupVariant::Primary.as_attr(), "primary");
        assert_eq!(DateInputGroupVariant::Secondary.as_attr(), "secondary");
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Booking controls  ".to_string())),
            Some("Booking controls".to_string())
        );

        assert_eq!(
            normalize_aria_label(Some("  Date segments  ".to_string())),
            ("Date segments".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(Some("  ".to_string())),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
    }

    #[test]
    fn resolve_state_tracks_markers() {
        let state = resolve_state(DateInputGroupStateInput {
            variant: DateInputGroupVariant::Secondary,
            full_width: true,
            disabled: false,
            invalid: true,
            segmented: true,
            has_prefix: true,
            has_suffix: false,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.variant_attr, "secondary");
        assert_eq!(state.width_attr, "full");
        assert_eq!(state.data_state_attr, "invalid");
        assert!(state.is_full_width);
        assert!(state.is_invalid);
        assert!(state.is_segmented);
        assert!(state.has_prefix);
        assert!(!state.has_suffix);
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(DateInputGroupStateInput {
            variant: DateInputGroupVariant::Primary,
            full_width: false,
            disabled: true,
            invalid: false,
            segmented: true,
            has_prefix: true,
            has_suffix: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-date-input-group".to_string()), state);

        for token in [
            "ui-date-input-group",
            "ui-date-input-group--variant-primary",
            "ui-date-input-group--fit-width",
            "ui-date-input-group--disabled",
            "ui-date-input-group--segmented",
            "ui-date-input-group--has-prefix",
            "ui-date-input-group--has-suffix",
            "ui-date-input-group--custom-class",
            "docs-date-input-group",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
