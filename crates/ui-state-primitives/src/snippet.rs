#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnippetLayout {
    SingleLine,
    MultiLine,
}

impl SnippetLayout {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::SingleLine => "ui-snippet--state-single-line",
            Self::MultiLine => "ui-snippet--state-multiline",
        }
    }

    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::SingleLine => "single-line",
            Self::MultiLine => "multiline",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnippetCopyState {
    Static,
    Disabled,
    Copyable,
}

impl SnippetCopyState {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Static => "ui-snippet--copy-static",
            Self::Disabled => "ui-snippet--copy-disabled",
            Self::Copyable => "ui-snippet--copyable",
        }
    }

    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Static => "static",
            Self::Disabled => "disabled",
            Self::Copyable => "copyable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnippetSource {
    Default,
    Custom,
}

impl SnippetSource {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Default => "ui-snippet--default-copied-label",
            Self::Custom => "ui-snippet--custom-copied-label",
        }
    }

    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnippetStateInput {
    pub is_multiline: bool,
    pub has_text: bool,
    pub has_label: bool,
    pub is_copyable: bool,
    pub has_custom_copied_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnippetState {
    pub layout: SnippetLayout,
    pub copy_state: SnippetCopyState,
    pub copied_label_source: SnippetSource,
    pub is_multiline: bool,
    pub is_empty: bool,
    pub has_label: bool,
    pub is_copyable: bool,
    pub copy_is_actionable: bool,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub copy_state_class: &'static str,
    pub copy_state_attr: &'static str,
    pub copied_label_source_class: &'static str,
    pub copied_label_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: SnippetStateInput) -> SnippetState {
    let layout = if input.is_multiline {
        SnippetLayout::MultiLine
    } else {
        SnippetLayout::SingleLine
    };

    let copy_is_actionable = input.is_copyable && input.has_text;
    let copy_state = if !input.is_copyable {
        SnippetCopyState::Static
    } else if copy_is_actionable {
        SnippetCopyState::Copyable
    } else {
        SnippetCopyState::Disabled
    };

    let copied_label_source = if input.has_custom_copied_label {
        SnippetSource::Custom
    } else {
        SnippetSource::Default
    };

    SnippetState {
        layout,
        copy_state,
        copied_label_source,
        is_multiline: input.is_multiline,
        is_empty: !input.has_text,
        has_label: input.has_label,
        is_copyable: input.is_copyable,
        copy_is_actionable,
        state_class: layout.class_name(),
        state_attr: layout.as_attr(),
        copy_state_class: copy_state.class_name(),
        copy_state_attr: copy_state.as_attr(),
        copied_label_source_class: copied_label_source.class_name(),
        copied_label_source_attr: copied_label_source.as_attr(),
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Done  ".to_string())),
            Some("Done".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_copy_and_label_sources() {
        let state = resolve_state(SnippetStateInput {
            is_multiline: true,
            has_text: true,
            has_label: true,
            is_copyable: true,
            has_custom_copied_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.layout, SnippetLayout::MultiLine);
        assert_eq!(state.copy_state, SnippetCopyState::Copyable);
        assert_eq!(state.copied_label_source, SnippetSource::Custom);
        assert_eq!(state.state_class, "ui-snippet--state-multiline");
        assert_eq!(state.copy_state_class, "ui-snippet--copyable");
        assert_eq!(
            state.copied_label_source_class,
            "ui-snippet--custom-copied-label"
        );
        assert!(!state.is_empty);
        assert!(state.has_label);
        assert!(state.is_copyable);
        assert!(state.copy_is_actionable);
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn resolve_state_marks_empty_copyable_snippet_as_disabled_copy() {
        let state = resolve_state(SnippetStateInput {
            is_multiline: false,
            has_text: false,
            has_label: false,
            is_copyable: true,
            has_custom_copied_label: false,
            has_custom_class_name: false,
        });

        assert_eq!(state.copy_state, SnippetCopyState::Disabled);
        assert_eq!(state.copy_state_attr, "disabled");
        assert!(state.is_empty);
        assert!(!state.copy_is_actionable);
    }

    #[test]
    fn marker_values_are_closed_sets() {
        let allowed_layout = ["single-line", "multiline"];
        let allowed_copy = ["copyable", "disabled", "static"];
        let allowed_source = ["default", "custom"];

        for is_multiline in [false, true] {
            for has_text in [false, true] {
                for has_label in [false, true] {
                    for is_copyable in [false, true] {
                        for has_custom_copied_label in [false, true] {
                            for has_custom_class_name in [false, true] {
                                let state = resolve_state(SnippetStateInput {
                                    is_multiline,
                                    has_text,
                                    has_label,
                                    is_copyable,
                                    has_custom_copied_label,
                                    has_custom_class_name,
                                });

                                assert!(allowed_layout.contains(&state.state_attr));
                                assert!(allowed_copy.contains(&state.copy_state_attr));
                                assert!(allowed_source.contains(&state.copied_label_source_attr));
                            }
                        }
                    }
                }
            }
        }
    }
}
