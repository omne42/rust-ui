use crate::button::ButtonSize;
use crate::icon_button::{IconButtonState, IconButtonStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Icon button";

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(aria_label: String) -> (String, bool) {
    let trimmed = aria_label.trim();
    if trimmed.is_empty() {
        (DEFAULT_ARIA_LABEL.to_string(), false)
    } else {
        (trimmed.to_string(), true)
    }
}

pub fn resolve_state(input: IconButtonStateInput) -> IconButtonState {
    let uses_icon_size = matches!(
        input.size,
        ButtonSize::Icon
            | ButtonSize::IconSm
            | ButtonSize::IconLg
            | ButtonSize::IconXs
            | ButtonSize::IconS
            | ButtonSize::IconM
            | ButtonSize::IconL
            | ButtonSize::IconXl
    );

    IconButtonState {
        is_disabled: input.disabled,
        is_enabled: !input.disabled,
        uses_icon_size,
        uses_custom_size: !uses_icon_size,
        has_custom_press_handler: input.has_custom_press_handler,
        has_explicit_aria_label: input.has_explicit_aria_label,
        has_fallback_aria_label: !input.has_explicit_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        state_attr: if input.disabled { "disabled" } else { "ready" },
        size_mode_attr: if uses_icon_size { "icon" } else { "custom" },
        handler_source_attr: if input.has_custom_press_handler {
            "custom"
        } else {
            "default"
        },
        label_source_attr: if input.has_explicit_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        motion_source_attr: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: IconButtonState) -> String {
    let mut classes = vec!["ui-icon-button".to_string()];

    if state.is_disabled {
        classes.push("ui-icon-button--disabled".to_string());
    } else {
        classes.push("ui-icon-button--enabled".to_string());
    }

    if state.uses_custom_size {
        classes.push("ui-icon-button--custom-size".to_string());
    } else {
        classes.push("ui-icon-button--icon-size".to_string());
    }

    if state.has_custom_press_handler {
        classes.push("ui-icon-button--custom-handler".to_string());
    }

    if state.has_fallback_aria_label {
        classes.push("ui-icon-button--fallback-label".to_string());
    }

    if state.has_custom_motion {
        classes.push("ui-icon-button--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-icon-button--custom-class".to_string());
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
    fn normalize_aria_label_trims_and_falls_back() {
        let (label, explicit) = normalize_aria_label("  Close dialog  ".to_string());
        assert_eq!(label, "Close dialog");
        assert!(explicit);

        let (label, explicit) = normalize_aria_label("   ".to_string());
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!explicit);
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(
            normalize_optional_text(Some(" custom ".to_string())),
            Some("custom".to_string())
        );
        assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
        assert_eq!(normalize_optional_text(None), None);
    }

    #[test]
    fn resolve_state_tracks_size_and_source_markers() {
        let icon_state = resolve_state(IconButtonStateInput {
            disabled: false,
            size: ButtonSize::IconSm,
            has_custom_press_handler: true,
            has_explicit_aria_label: true,
            has_custom_class_name: true,
            has_custom_motion: true,
        });
        assert!(icon_state.is_enabled);
        assert!(!icon_state.is_disabled);
        assert!(icon_state.uses_icon_size);
        assert!(!icon_state.uses_custom_size);
        assert!(icon_state.has_custom_press_handler);
        assert!(icon_state.has_explicit_aria_label);
        assert!(!icon_state.has_fallback_aria_label);
        assert!(icon_state.has_custom_class_name);
        assert!(icon_state.has_custom_motion);
        assert_eq!(icon_state.state_attr, "ready");
        assert_eq!(icon_state.size_mode_attr, "icon");
        assert_eq!(icon_state.handler_source_attr, "custom");
        assert_eq!(icon_state.label_source_attr, "custom");
        assert_eq!(icon_state.class_source_attr, "custom");
        assert_eq!(icon_state.motion_source_attr, "custom");

        let fallback_state = resolve_state(IconButtonStateInput {
            disabled: true,
            size: ButtonSize::Lg,
            has_custom_press_handler: false,
            has_explicit_aria_label: false,
            has_custom_class_name: false,
            has_custom_motion: false,
        });
        assert!(!fallback_state.is_enabled);
        assert!(fallback_state.is_disabled);
        assert!(!fallback_state.uses_icon_size);
        assert!(fallback_state.uses_custom_size);
        assert!(!fallback_state.has_custom_press_handler);
        assert!(!fallback_state.has_explicit_aria_label);
        assert!(fallback_state.has_fallback_aria_label);
        assert!(!fallback_state.has_custom_class_name);
        assert!(!fallback_state.has_custom_motion);
        assert_eq!(fallback_state.state_attr, "disabled");
        assert_eq!(fallback_state.size_mode_attr, "custom");
        assert_eq!(fallback_state.handler_source_attr, "default");
        assert_eq!(fallback_state.label_source_attr, "default");
        assert_eq!(fallback_state.class_source_attr, "default");
        assert_eq!(fallback_state.motion_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(IconButtonStateInput {
                disabled: false,
                size: ButtonSize::Lg,
                has_custom_press_handler: true,
                has_explicit_aria_label: false,
                has_custom_class_name: true,
                has_custom_motion: true,
            }),
        );

        for token in [
            "ui-icon-button",
            "ui-icon-button--enabled",
            "ui-icon-button--custom-size",
            "ui-icon-button--custom-handler",
            "ui-icon-button--fallback-label",
            "ui-icon-button--custom-motion",
            "ui-icon-button--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
