pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_COPIED_LABEL, DEFAULT_COPY_LABEL, SnippetLogic, SnippetLogicOptions,
    SnippetTextContract, SnippetTextFallbacks, normalize_optional_text, resolve_text_contract,
    use_snippet_logic, use_snippet_logic_with_options,
};
pub use motion::SnippetMotion;
pub use view::Snippet;
