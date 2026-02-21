use crate::logic::CodeBlockCopiedSource;
use leptos::prelude::*;
use ui_headless::{SnippetCopyOptions, a11y::A11yDirection, use_snippet_copy};

#[derive(Clone)]
pub struct SnippetLogic {
    pub copied: Signal<bool>,
    pub is_loading: ReadSignal<bool>,
    pub has_error: ReadSignal<bool>,
    pub aria_busy: Signal<Option<&'static str>>,
    pub copy: Callback<()>,
    pub copied_source: CodeBlockCopiedSource,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct SnippetLogicOptions {
    pub text: String,
    pub copied: Option<Signal<bool>>,
    pub default_copied: bool,
    pub on_copied_change: Option<Callback<bool>>,
    pub copied_source: CodeBlockCopiedSource,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_snippet_logic_with_options(options: SnippetLogicOptions) -> SnippetLogic {
    let contract = use_snippet_copy(SnippetCopyOptions {
        text: options.text,
        is_copyable: true,
        is_copied: options.copied,
        default_copied: Some(options.default_copied),
        on_copied_change: options.on_copied_change,
        on_copy_error: None,
        lang: options.lang,
        dir: options.dir,
    });

    SnippetLogic {
        copied: contract.state.copied,
        is_loading: contract.state.is_loading,
        has_error: contract.state.has_error,
        aria_busy: contract.attrs.aria_busy,
        copy: contract.handlers.on_copy,
        copied_source: options.copied_source,
        lang: contract.attrs.lang,
        dir: contract.attrs.dir,
    }
}

#[cfg(test)]
pub fn use_snippet_logic(text: String) -> SnippetLogic {
    use_snippet_logic_with_options(SnippetLogicOptions {
        text,
        copied: None,
        default_copied: crate::logic::DEFAULT_COPIED,
        on_copied_change: None,
        copied_source: CodeBlockCopiedSource::Uncontrolled,
        lang: None,
        dir: None,
    })
}

#[cfg(test)]
#[path = "../test/snippet.rs"]
mod tests;
