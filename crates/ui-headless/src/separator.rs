use crate::a11y::{A11yDirection, locale_attrs};
use ui_state_primitives::separator::SeparatorState;

#[derive(Clone, Debug)]
pub struct SeparatorOptions {
    pub state: SeparatorState,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone, Debug)]
pub struct SeparatorHandlers {}

#[derive(Clone, Debug)]
pub struct SeparatorAttrs {
    pub role: Option<&'static str>,
    pub aria_orientation: Option<&'static str>,
    pub aria_hidden: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeparatorSemanticState {
    pub is_semantic: bool,
    pub is_decorative: bool,
}

#[derive(Clone, Debug)]
pub struct SeparatorContract {
    pub attrs: SeparatorAttrs,
    pub handlers: SeparatorHandlers,
    pub state: SeparatorSemanticState,
}

pub fn use_separator(options: SeparatorOptions) -> SeparatorContract {
    let SeparatorOptions { state, lang, dir } = options;
    let locale = locale_attrs(lang, dir);

    let role = state.is_semantic.then_some("separator");
    let aria_orientation = state
        .is_semantic
        .then_some(state.aria_orientation)
        .flatten();
    let aria_hidden = state.is_decorative.then_some("true");

    SeparatorContract {
        attrs: SeparatorAttrs {
            role,
            aria_orientation,
            aria_hidden,
            lang: locale.lang,
            dir: locale.dir,
        },
        handlers: SeparatorHandlers {},
        state: SeparatorSemanticState {
            is_semantic: state.is_semantic,
            is_decorative: state.is_decorative,
        },
    }
}

#[cfg(test)]
#[path = "test/separator.rs"]
mod tests;
