#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VisuallyHiddenStateInput {
    pub is_focusable: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VisuallyHiddenState {
    pub is_focusable: bool,
    pub has_custom_class_name: bool,
    pub focusable_class: Option<&'static str>,
    pub focusable_attr: Option<&'static str>,
    pub custom_class_attr: Option<&'static str>,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: VisuallyHiddenStateInput) -> VisuallyHiddenState {
    VisuallyHiddenState {
        is_focusable: input.is_focusable,
        has_custom_class_name: input.has_custom_class_name,
        focusable_class: input
            .is_focusable
            .then_some("ui-visually-hidden--focusable"),
        focusable_attr: input.is_focusable.then_some("true"),
        custom_class_attr: input.has_custom_class_name.then_some("true"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-hidden  ".to_string())),
            Some("docs-hidden".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_focusable_and_class_source_markers() {
        let focusable = resolve_state(VisuallyHiddenStateInput {
            is_focusable: true,
            has_custom_class_name: true,
        });
        assert_eq!(
            focusable,
            VisuallyHiddenState {
                is_focusable: true,
                has_custom_class_name: true,
                focusable_class: Some("ui-visually-hidden--focusable"),
                focusable_attr: Some("true"),
                custom_class_attr: Some("true"),
            }
        );

        let default_state = resolve_state(VisuallyHiddenStateInput {
            is_focusable: false,
            has_custom_class_name: false,
        });
        assert_eq!(default_state.focusable_class, None);
        assert_eq!(default_state.focusable_attr, None);
        assert_eq!(default_state.custom_class_attr, None);
    }
}
