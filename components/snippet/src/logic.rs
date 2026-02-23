use std::borrow::Cow;

use leptos::prelude::*;
use ui_headless::{A11yDirection, SnippetCopyOptions, use_snippet_copy};

pub use ui_state_primitives::snippet::{SnippetStateInput, normalize_optional_text, resolve_state};
pub type SnippetViewState = ui_state_primitives::snippet::SnippetState;

pub const DEFAULT_IS_COPYABLE: bool = true;
pub const DEFAULT_COPY_LABEL: &str = "Copy";
pub const DEFAULT_COPIED_LABEL: &str = "Copied";
pub const DEFAULT_COPY_ARIA_LABEL: &str = "Copy to clipboard";
pub const DEFAULT_COPY_ERROR_LABEL: &str = "Copy failed. Activate again to retry.";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnippetTextContract {
    pub copy_label: String,
    pub copied_label: String,
    pub copy_aria_label: String,
    pub copy_error_label: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SnippetTextFallbacks {
    pub copy_label: Option<String>,
    pub copied_label: Option<String>,
    pub copy_aria_label: Option<String>,
    pub copy_error_label: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnippetCopyableSource {
    Default,
    IsCopyableProp,
    LegacyCopyableProp,
}

impl SnippetCopyableSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::IsCopyableProp => "is_copyable",
            Self::LegacyCopyableProp => "copyable_legacy",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnippetCopyableContract {
    pub is_copyable: bool,
    pub source: SnippetCopyableSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnippetCopiedSource {
    Controlled,
    Uncontrolled,
}

impl SnippetCopiedSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone)]
pub struct SnippetControlledCopied {
    pub value: Option<Signal<bool>>,
    pub source: SnippetCopiedSource,
}

#[derive(Clone)]
pub struct SnippetLogicOptions {
    pub text: String,
    pub is_copyable: bool,
    pub is_copied: Option<Signal<bool>>,
    pub default_copied: Option<bool>,
    pub on_copied_change: Option<Callback<bool>>,
    pub on_copy_error: Option<Callback<()>>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct SnippetLogic {
    pub copied: Signal<bool>,
    pub is_loading: ReadSignal<bool>,
    pub has_error: ReadSignal<bool>,
    pub is_copying: ReadSignal<bool>,
    pub has_copy_error: ReadSignal<bool>,
    pub copy: Callback<()>,
    pub retry_copy: Callback<()>,
    pub aria_busy: Signal<Option<&'static str>>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

pub fn resolve_copyable_contract(
    is_copyable: Option<bool>,
    copyable: Option<bool>,
) -> SnippetCopyableContract {
    if let Some(value) = is_copyable {
        return SnippetCopyableContract {
            is_copyable: value,
            source: SnippetCopyableSource::IsCopyableProp,
        };
    }

    if let Some(value) = copyable {
        return SnippetCopyableContract {
            is_copyable: value,
            source: SnippetCopyableSource::LegacyCopyableProp,
        };
    }

    SnippetCopyableContract {
        is_copyable: DEFAULT_IS_COPYABLE,
        source: SnippetCopyableSource::Default,
    }
}

pub fn resolve_controlled_copied(
    is_copied: Option<Signal<bool>>,
    copied: Option<Signal<bool>>,
) -> SnippetControlledCopied {
    let value = is_copied.or(copied);
    let source = if value.is_some() {
        SnippetCopiedSource::Controlled
    } else {
        SnippetCopiedSource::Uncontrolled
    };
    SnippetControlledCopied { value, source }
}

pub fn resolve_text_contract(
    copy_label: Option<String>,
    copied_label: Option<String>,
    copy_aria_label: Option<String>,
    copy_error_label: Option<String>,
    fallbacks: SnippetTextFallbacks,
) -> SnippetTextContract {
    let SnippetTextFallbacks {
        copy_label: fallback_copy_label,
        copied_label: fallback_copied_label,
        copy_aria_label: fallback_copy_aria_label,
        copy_error_label: fallback_copy_error_label,
    } = fallbacks;

    let copy_label = normalize_optional_text(copy_label)
        .or_else(|| normalize_optional_text(fallback_copy_label))
        .unwrap_or_else(|| DEFAULT_COPY_LABEL.into());
    let copied_label = normalize_optional_text(copied_label)
        .or_else(|| normalize_optional_text(fallback_copied_label))
        .unwrap_or_else(|| DEFAULT_COPIED_LABEL.into());
    let copy_aria_label = normalize_optional_text(copy_aria_label)
        .or_else(|| normalize_optional_text(fallback_copy_aria_label))
        .unwrap_or_else(|| DEFAULT_COPY_ARIA_LABEL.into());
    let copy_error_label = normalize_optional_text(copy_error_label)
        .or_else(|| normalize_optional_text(fallback_copy_error_label))
        .unwrap_or_else(|| DEFAULT_COPY_ERROR_LABEL.into());

    SnippetTextContract {
        copy_label,
        copied_label,
        copy_aria_label,
        copy_error_label,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SnippetViewState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-snippet"),
        Cow::Borrowed(state.state_class),
        Cow::Borrowed(state.copy_state_class),
        Cow::Borrowed(state.copied_label_source_class),
    ];

    if state.has_label {
        classes.push(Cow::Borrowed("ui-snippet--with-label"));
    }
    if state.is_empty {
        classes.push(Cow::Borrowed("ui-snippet--empty"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-snippet--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .iter()
        .map(Cow::as_ref)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn use_snippet_logic_with_options(options: SnippetLogicOptions) -> SnippetLogic {
    let contract = use_snippet_copy(SnippetCopyOptions {
        text: options.text,
        is_copyable: options.is_copyable,
        is_copied: options.is_copied,
        default_copied: options.default_copied,
        on_copied_change: options.on_copied_change,
        on_copy_error: options.on_copy_error,
        lang: options.lang,
        dir: options.dir,
    });

    SnippetLogic {
        copied: contract.state.copied,
        is_loading: contract.state.is_loading,
        has_error: contract.state.has_error,
        is_copying: contract.state.is_loading,
        has_copy_error: contract.state.has_error,
        copy: contract.handlers.on_copy,
        retry_copy: contract.handlers.on_retry,
        aria_busy: contract.attrs.aria_busy,
        lang: contract.attrs.lang,
        dir: contract.attrs.dir,
    }
}

pub fn use_snippet_logic(text: String) -> SnippetLogic {
    use_snippet_logic_with_options(SnippetLogicOptions {
        text,
        is_copyable: true,
        is_copied: None,
        default_copied: Some(false),
        on_copied_change: None,
        on_copy_error: None,
        lang: None,
        dir: None,
    })
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
