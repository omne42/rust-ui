pub const DEFAULT_LABEL: &str = "Textarea";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaVisualStateAttr {
    Disabled,
    Invalid,
    Readonly,
    Ready,
}

impl TextareaVisualStateAttr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Invalid => "invalid",
            Self::Readonly => "readonly",
            Self::Ready => "ready",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaValueAttr {
    Filled,
    Empty,
}

impl TextareaValueAttr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Filled => "filled",
            Self::Empty => "empty",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaRequirementAttr {
    Required,
    Optional,
}

impl TextareaRequirementAttr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaSourceAttr {
    Custom,
    Default,
}

impl TextareaSourceAttr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Custom => "custom",
            Self::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextareaStateInput {
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub invalid: bool,
    pub has_value: bool,
    pub has_custom_label: bool,
    pub has_custom_description: bool,
    pub has_custom_error: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_rows: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextareaState {
    pub state_attr: TextareaVisualStateAttr,
    pub value_attr: TextareaValueAttr,
    pub requirement_attr: TextareaRequirementAttr,
    pub label_source_attr: TextareaSourceAttr,
    pub description_source_attr: TextareaSourceAttr,
    pub error_source_attr: TextareaSourceAttr,
    pub placeholder_source_attr: TextareaSourceAttr,
    pub rows_source_attr: TextareaSourceAttr,
    pub class_source_attr: TextareaSourceAttr,
    pub has_custom_class_name: bool,
}

pub fn resolve_label(value: String) -> (String, bool) {
    resolve_label_with_fallback(value, DEFAULT_LABEL)
}

pub fn resolve_label_with_fallback(value: String, fallback_label: &str) -> (String, bool) {
    let trimmed = value.trim();

    if !trimmed.is_empty() {
        return (trimmed.to_string(), true);
    }

    let fallback_trimmed = fallback_label.trim();
    if !fallback_trimmed.is_empty() {
        return (fallback_trimmed.to_string(), false);
    }

    (DEFAULT_LABEL.to_string(), false)
}

pub fn resolve_state(input: TextareaStateInput) -> TextareaState {
    TextareaState {
        state_attr: if input.disabled {
            TextareaVisualStateAttr::Disabled
        } else if input.invalid {
            TextareaVisualStateAttr::Invalid
        } else if input.read_only {
            TextareaVisualStateAttr::Readonly
        } else {
            TextareaVisualStateAttr::Ready
        },
        value_attr: if input.has_value {
            TextareaValueAttr::Filled
        } else {
            TextareaValueAttr::Empty
        },
        requirement_attr: if input.required {
            TextareaRequirementAttr::Required
        } else {
            TextareaRequirementAttr::Optional
        },
        label_source_attr: if input.has_custom_label {
            TextareaSourceAttr::Custom
        } else {
            TextareaSourceAttr::Default
        },
        description_source_attr: if input.has_custom_description {
            TextareaSourceAttr::Custom
        } else {
            TextareaSourceAttr::Default
        },
        error_source_attr: if input.has_custom_error {
            TextareaSourceAttr::Custom
        } else {
            TextareaSourceAttr::Default
        },
        placeholder_source_attr: if input.has_custom_placeholder {
            TextareaSourceAttr::Custom
        } else {
            TextareaSourceAttr::Default
        },
        rows_source_attr: if input.has_custom_rows {
            TextareaSourceAttr::Custom
        } else {
            TextareaSourceAttr::Default
        },
        class_source_attr: if input.has_custom_class_name {
            TextareaSourceAttr::Custom
        } else {
            TextareaSourceAttr::Default
        },
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_label_uses_default_for_blank_values() {
        assert_eq!(
            resolve_label("  ".to_string()),
            (DEFAULT_LABEL.to_string(), false)
        );
        assert_eq!(
            resolve_label("  Release summary  ".to_string()),
            ("Release summary".to_string(), true)
        );
    }

    #[test]
    fn resolve_label_with_fallback_prefers_props_then_i18n_then_default() {
        assert_eq!(
            resolve_label_with_fallback("  Summary  ".to_string(), "Localized Textarea"),
            ("Summary".to_string(), true)
        );
        assert_eq!(
            resolve_label_with_fallback("   ".to_string(), "  Localized Textarea  "),
            ("Localized Textarea".to_string(), false)
        );
        assert_eq!(
            resolve_label_with_fallback("   ".to_string(), "   "),
            (DEFAULT_LABEL.to_string(), false)
        );
    }

    #[test]
    fn resolve_state_tracks_sources_and_rows_markers() {
        let state = resolve_state(TextareaStateInput {
            disabled: false,
            read_only: true,
            required: true,
            invalid: false,
            has_value: true,
            has_custom_label: true,
            has_custom_description: true,
            has_custom_error: false,
            has_custom_placeholder: true,
            has_custom_rows: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.state_attr, TextareaVisualStateAttr::Readonly);
        assert_eq!(state.value_attr, TextareaValueAttr::Filled);
        assert_eq!(state.requirement_attr, TextareaRequirementAttr::Required);
        assert_eq!(state.label_source_attr, TextareaSourceAttr::Custom);
        assert_eq!(state.description_source_attr, TextareaSourceAttr::Custom);
        assert_eq!(state.error_source_attr, TextareaSourceAttr::Default);
        assert_eq!(state.placeholder_source_attr, TextareaSourceAttr::Custom);
        assert_eq!(state.rows_source_attr, TextareaSourceAttr::Custom);
        assert_eq!(state.class_source_attr, TextareaSourceAttr::Default);
    }

    #[test]
    fn resolve_state_uses_closed_enumerated_marker_values() {
        for disabled in [false, true] {
            for read_only in [false, true] {
                for required in [false, true] {
                    for invalid in [false, true] {
                        for has_value in [false, true] {
                            for has_custom_label in [false, true] {
                                for has_custom_description in [false, true] {
                                    for has_custom_error in [false, true] {
                                        for has_custom_placeholder in [false, true] {
                                            for has_custom_rows in [false, true] {
                                                for has_custom_class_name in [false, true] {
                                                    let state = resolve_state(TextareaStateInput {
                                                        disabled,
                                                        read_only,
                                                        required,
                                                        invalid,
                                                        has_value,
                                                        has_custom_label,
                                                        has_custom_description,
                                                        has_custom_error,
                                                        has_custom_placeholder,
                                                        has_custom_rows,
                                                        has_custom_class_name,
                                                    });

                                                    assert!(
                                                        matches!(
                                                            state.state_attr,
                                                            TextareaVisualStateAttr::Disabled
                                                                | TextareaVisualStateAttr::Invalid
                                                                | TextareaVisualStateAttr::Readonly
                                                                | TextareaVisualStateAttr::Ready
                                                        ),
                                                        "unexpected `data-state` value: {}",
                                                        state.state_attr.as_str()
                                                    );
                                                    assert!(
                                                        matches!(
                                                            state.value_attr,
                                                            TextareaValueAttr::Filled
                                                                | TextareaValueAttr::Empty
                                                        ),
                                                        "unexpected `data-value` value: {}",
                                                        state.value_attr.as_str()
                                                    );
                                                    assert!(
                                                        matches!(
                                                            state.requirement_attr,
                                                            TextareaRequirementAttr::Required
                                                                | TextareaRequirementAttr::Optional
                                                        ),
                                                        "unexpected `data-requirement` value: {}",
                                                        state.requirement_attr.as_str()
                                                    );
                                                    assert!(
                                                        matches!(
                                                            state.label_source_attr,
                                                            TextareaSourceAttr::Custom
                                                                | TextareaSourceAttr::Default
                                                        ),
                                                        "unexpected `data-label-source` value: {}",
                                                        state.label_source_attr.as_str()
                                                    );
                                                    assert!(
                                                        matches!(
                                                            state.description_source_attr,
                                                            TextareaSourceAttr::Custom
                                                                | TextareaSourceAttr::Default
                                                        ),
                                                        "unexpected `data-description-source` value: {}",
                                                        state.description_source_attr.as_str()
                                                    );
                                                    assert!(
                                                        matches!(
                                                            state.error_source_attr,
                                                            TextareaSourceAttr::Custom
                                                                | TextareaSourceAttr::Default
                                                        ),
                                                        "unexpected `data-error-source` value: {}",
                                                        state.error_source_attr.as_str()
                                                    );
                                                    assert!(
                                                        matches!(
                                                            state.placeholder_source_attr,
                                                            TextareaSourceAttr::Custom
                                                                | TextareaSourceAttr::Default
                                                        ),
                                                        "unexpected `data-placeholder-source` value: {}",
                                                        state.placeholder_source_attr.as_str()
                                                    );
                                                    assert!(
                                                        matches!(
                                                            state.rows_source_attr,
                                                            TextareaSourceAttr::Custom
                                                                | TextareaSourceAttr::Default
                                                        ),
                                                        "unexpected `data-rows-source` value: {}",
                                                        state.rows_source_attr.as_str()
                                                    );
                                                    assert!(
                                                        matches!(
                                                            state.class_source_attr,
                                                            TextareaSourceAttr::Custom
                                                                | TextareaSourceAttr::Default
                                                        ),
                                                        "unexpected `data-class-source` value: {}",
                                                        state.class_source_attr.as_str()
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
