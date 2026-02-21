use std::borrow::Cow;

use leptos::prelude::{Callback, Signal};

pub use ui_state_primitives::code_block::{
    CodeBlockContentInput, CodeBlockViewState, normalize_optional_text, resolve_state_from_content,
};

pub const DEFAULT_IS_COPYABLE: bool = true;
pub const DEFAULT_COPIED: bool = false;

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodeBlockStateInput {
    pub is_multiline: bool,
    pub is_empty: bool,
    pub has_label: bool,
    pub has_language: bool,
    pub copyable: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[cfg(test)]
pub fn resolve_state(input: CodeBlockStateInput) -> CodeBlockViewState {
    let code = if input.is_empty {
        ""
    } else if input.is_multiline {
        "line one\nline two"
    } else {
        "line one"
    };

    resolve_state_from_content(CodeBlockContentInput {
        code,
        label: input.has_label.then_some("label"),
        language: input.has_language.then_some("language"),
        copyable: input.copyable,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeBlockCopyableSource {
    Default,
    IsCopyableProp,
    LegacyCopyableProp,
}

impl CodeBlockCopyableSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::IsCopyableProp => "is_copyable",
            Self::LegacyCopyableProp => "copyable_legacy",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodeBlockCopyableContract {
    pub is_copyable: bool,
    pub source: CodeBlockCopyableSource,
}

pub fn resolve_copyable_contract(
    is_copyable: Option<bool>,
    copyable: Option<bool>,
) -> CodeBlockCopyableContract {
    if let Some(value) = is_copyable {
        return CodeBlockCopyableContract {
            is_copyable: value,
            source: CodeBlockCopyableSource::IsCopyableProp,
        };
    }

    if let Some(value) = copyable {
        return CodeBlockCopyableContract {
            is_copyable: value,
            source: CodeBlockCopyableSource::LegacyCopyableProp,
        };
    }

    CodeBlockCopyableContract {
        is_copyable: DEFAULT_IS_COPYABLE,
        source: CodeBlockCopyableSource::Default,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeBlockCopiedSource {
    Controlled,
    Uncontrolled,
}

impl CodeBlockCopiedSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone)]
pub struct CodeBlockCopiedContract {
    pub copied: Option<Signal<bool>>,
    pub default_copied: bool,
    pub on_copied_change: Option<Callback<bool>>,
    pub source: CodeBlockCopiedSource,
}

pub fn resolve_copied_contract(
    is_copied: Option<Signal<bool>>,
    copied: Option<Signal<bool>>,
    default_copied: Option<bool>,
    on_copied_change: Option<Callback<bool>>,
) -> CodeBlockCopiedContract {
    let copied = is_copied.or(copied);
    let source = if copied.is_some() {
        CodeBlockCopiedSource::Controlled
    } else {
        CodeBlockCopiedSource::Uncontrolled
    };

    CodeBlockCopiedContract {
        copied,
        default_copied: default_copied.unwrap_or(DEFAULT_COPIED),
        on_copied_change,
        source,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeBlockLogicInput {
    pub code: String,
    pub label: Option<String>,
    pub language: Option<String>,
    pub is_copyable: bool,
    pub class_name: Option<String>,
    pub has_custom_motion: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeBlockRenderModel {
    pub code: String,
    pub label: Option<String>,
    pub language: Option<String>,
    pub class_name: String,
    pub state: CodeBlockViewState,
}

pub fn resolve_render_model(input: CodeBlockLogicInput) -> CodeBlockRenderModel {
    let label = normalize_optional_text(input.label);
    let language = normalize_optional_text(input.language);
    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    let state = resolve_state_from_content(CodeBlockContentInput {
        code: &input.code,
        label: label.as_deref(),
        language: language.as_deref(),
        copyable: input.is_copyable,
        has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
    });
    let class_name = compose_class_name(class_name, state);

    CodeBlockRenderModel {
        code: input.code,
        label,
        language,
        class_name,
        state,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: CodeBlockViewState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-code-block"),
        Cow::Borrowed(state.state_class),
        Cow::Borrowed(state.header_class),
        Cow::Borrowed(state.motion_source_class),
    ];

    if state.copyable {
        classes.push(Cow::Borrowed("ui-code-block--copyable"));
    }
    if state.has_label {
        classes.push(Cow::Borrowed("ui-code-block--with-label"));
    }
    if state.has_language {
        classes.push(Cow::Borrowed("ui-code-block--with-language"));
    }
    if state.is_empty {
        classes.push(Cow::Borrowed("ui-code-block--empty"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-code-block--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
