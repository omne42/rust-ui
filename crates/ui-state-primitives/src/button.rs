pub const BUTTON_ICON_ONLY_FALLBACK_ARIA_LABEL: &str = "Button";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonLabelSource {
    Explicit,
    Fallback,
    None,
}

impl ButtonLabelSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Explicit => "explicit",
            Self::Fallback => "fallback",
            Self::None => "none",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_aria_label(
    aria_label: Option<String>,
    is_icon_only: bool,
    icon_only_fallback_aria_label: Option<String>,
) -> (Option<String>, ButtonLabelSource) {
    if let Some(label) = normalize_optional_text(aria_label) {
        return (Some(label), ButtonLabelSource::Explicit);
    }

    if is_icon_only {
        let fallback = normalize_optional_text(icon_only_fallback_aria_label)
            .unwrap_or_else(|| BUTTON_ICON_ONLY_FALLBACK_ARIA_LABEL.to_string());
        return (Some(fallback), ButtonLabelSource::Fallback);
    }

    (None, ButtonLabelSource::None)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonStateCoreInput {
    pub is_disabled: bool,
    pub is_loading: bool,
    pub is_icon_only: bool,
    pub is_full_width: bool,
    pub has_start_content: bool,
    pub has_end_content: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonStateCore {
    pub is_disabled: bool,
    pub is_loading: bool,
    pub is_icon_only: bool,
    pub is_full_width: bool,
    pub has_start_content: bool,
    pub has_end_content: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub state_attr: &'static str,
}

pub fn resolve_state_core(input: ButtonStateCoreInput) -> ButtonStateCore {
    let is_disabled = input.is_disabled || input.is_loading;
    let state_attr = if input.is_loading {
        "loading"
    } else if is_disabled {
        "disabled"
    } else {
        "ready"
    };

    ButtonStateCore {
        is_disabled,
        is_loading: input.is_loading,
        is_icon_only: input.is_icon_only,
        is_full_width: input.is_full_width,
        has_start_content: input.has_start_content,
        has_end_content: input.has_end_content,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        state_attr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_trims_and_drops_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  button  ".to_string())),
            Some("button".to_string())
        );
    }

    #[test]
    fn resolve_aria_label_uses_fallback_for_icon_only_buttons() {
        assert_eq!(
            resolve_aria_label(Some("  Save  ".to_string()), true, None),
            (Some("Save".to_string()), ButtonLabelSource::Explicit)
        );
        assert_eq!(
            resolve_aria_label(None, true, None),
            (
                Some(BUTTON_ICON_ONLY_FALLBACK_ARIA_LABEL.to_string()),
                ButtonLabelSource::Fallback
            )
        );
        assert_eq!(
            resolve_aria_label(None, false, None),
            (None, ButtonLabelSource::None)
        );
        assert_eq!(
            resolve_aria_label(None, true, Some("  Control  ".to_string())),
            (Some("Control".to_string()), ButtonLabelSource::Fallback)
        );
    }

    #[test]
    fn resolve_state_core_derives_disabled_and_state_attr() {
        let ready = resolve_state_core(ButtonStateCoreInput {
            is_disabled: false,
            is_loading: false,
            is_icon_only: false,
            is_full_width: false,
            has_start_content: false,
            has_end_content: false,
            has_custom_class_name: false,
            has_custom_motion: false,
        });
        assert!(!ready.is_disabled);
        assert_eq!(ready.state_attr, "ready");

        let loading = resolve_state_core(ButtonStateCoreInput {
            is_disabled: false,
            is_loading: true,
            is_icon_only: true,
            is_full_width: true,
            has_start_content: true,
            has_end_content: true,
            has_custom_class_name: true,
            has_custom_motion: true,
        });
        assert!(loading.is_disabled);
        assert_eq!(loading.state_attr, "loading");
    }
}
