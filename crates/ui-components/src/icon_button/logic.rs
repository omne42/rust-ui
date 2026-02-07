use crate::button::ButtonSize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconButtonState {
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub uses_icon_size: bool,
    pub uses_custom_size: bool,
    pub has_custom_press_handler: bool,
    pub has_explicit_aria_label: bool,
    pub has_fallback_aria_label: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_aria_label(aria_label: String) -> (String, bool) {
    let trimmed = aria_label.trim();
    if trimmed.is_empty() {
        ("Icon button".to_string(), false)
    } else {
        (trimmed.to_string(), true)
    }
}

pub fn normalize_class_name(class_name: Option<String>) -> Option<String> {
    class_name.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(
    disabled: bool,
    size: ButtonSize,
    has_custom_press_handler: bool,
    has_explicit_aria_label: bool,
    has_custom_class_name: bool,
) -> IconButtonState {
    let uses_icon_size = matches!(
        size,
        ButtonSize::Icon | ButtonSize::IconSm | ButtonSize::IconLg
    );

    IconButtonState {
        is_disabled: disabled,
        is_enabled: !disabled,
        uses_icon_size,
        uses_custom_size: !uses_icon_size,
        has_custom_press_handler,
        has_explicit_aria_label,
        has_fallback_aria_label: !has_explicit_aria_label,
        has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: IconButtonState) -> String {
    let mut classes = vec!["ui-icon-button".to_string()];

    if state.uses_custom_size {
        classes.push("ui-icon-button--custom-size".to_string());
    }
    if state.is_enabled {
        classes.push("ui-icon-button--enabled".to_string());
    }
    if state.is_disabled {
        classes.push("ui-icon-button--disabled".to_string());
    }
    if state.has_custom_press_handler {
        classes.push("ui-icon-button--custom-handler".to_string());
    }
    if state.has_fallback_aria_label {
        classes.push("ui-icon-button--fallback-label".to_string());
    }
    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
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
        assert_eq!(label, "Icon button");
        assert!(!explicit);
    }

    #[test]
    fn normalize_class_name_filters_blank_values() {
        assert_eq!(
            normalize_class_name(Some(" custom ".to_string())),
            Some("custom".to_string())
        );
        assert_eq!(normalize_class_name(Some("   ".to_string())), None);
        assert_eq!(normalize_class_name(None), None);
    }

    #[test]
    fn resolve_state_tracks_size_enablement_and_metadata() {
        let icon_state = resolve_state(false, ButtonSize::IconSm, true, true, true);
        assert!(icon_state.is_enabled);
        assert!(!icon_state.is_disabled);
        assert!(icon_state.uses_icon_size);
        assert!(!icon_state.uses_custom_size);
        assert!(icon_state.has_custom_press_handler);
        assert!(icon_state.has_explicit_aria_label);
        assert!(!icon_state.has_fallback_aria_label);
        assert!(icon_state.has_custom_class_name);

        let fallback_state = resolve_state(true, ButtonSize::Lg, false, false, false);
        assert!(!fallback_state.is_enabled);
        assert!(fallback_state.is_disabled);
        assert!(!fallback_state.uses_icon_size);
        assert!(fallback_state.uses_custom_size);
        assert!(!fallback_state.has_custom_press_handler);
        assert!(!fallback_state.has_explicit_aria_label);
        assert!(fallback_state.has_fallback_aria_label);
        assert!(!fallback_state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(false, ButtonSize::Lg, true, false, true),
        );

        for token in [
            "ui-icon-button",
            "ui-icon-button--custom-size",
            "ui-icon-button--enabled",
            "ui-icon-button--custom-handler",
            "ui-icon-button--fallback-label",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
