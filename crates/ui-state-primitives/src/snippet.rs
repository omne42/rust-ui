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
        (!trimmed.is_empty()).then(|| trimmed.into())
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
#[path = "test/snippet.rs"]
mod tests;
