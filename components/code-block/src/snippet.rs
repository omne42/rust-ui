use leptos::prelude::*;
use ui_headless::{SnippetCopyOptions, use_snippet_copy};

#[derive(Clone)]
pub struct SnippetLogic {
    pub copied: Signal<bool>,
    pub copy: Callback<()>,
}

pub fn use_snippet_logic(text: String) -> SnippetLogic {
    let contract = use_snippet_copy(SnippetCopyOptions {
        text,
        is_copyable: true,
        is_copied: None,
        default_copied: Some(false),
        on_copied_change: None,
        on_copy_error: None,
        lang: None,
        dir: None,
    });

    SnippetLogic {
        copied: contract.state.copied,
        copy: contract.handlers.on_copy,
    }
}
