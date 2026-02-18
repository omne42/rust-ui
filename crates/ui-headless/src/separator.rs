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
mod tests {
    use super::*;
    use ui_state_primitives::separator::{
        SeparatorElementType, SeparatorOrientation, SeparatorStateInput, resolve_state,
    };

    #[test]
    fn separator_contract_maps_semantic_a11y_attrs() {
        let state = resolve_state(SeparatorStateInput {
            orientation: SeparatorOrientation::Vertical,
            element_type: SeparatorElementType::Div,
            decorative: false,
            has_custom_class_name: false,
        });

        let separator = use_separator(SeparatorOptions {
            state,
            lang: Some("  en-US ".to_string()),
            dir: Some(A11yDirection::Rtl),
        });

        assert_eq!(separator.attrs.role, Some("separator"));
        assert_eq!(separator.attrs.aria_orientation, Some("vertical"));
        assert_eq!(separator.attrs.aria_hidden, None);
        assert_eq!(separator.attrs.lang.as_deref(), Some("en-US"));
        assert_eq!(separator.attrs.dir, Some("rtl"));
        assert!(separator.state.is_semantic);
        assert!(!separator.state.is_decorative);
    }

    #[test]
    fn separator_contract_maps_decorative_a11y_attrs() {
        let state = resolve_state(SeparatorStateInput {
            orientation: SeparatorOrientation::Horizontal,
            element_type: SeparatorElementType::Hr,
            decorative: true,
            has_custom_class_name: true,
        });

        let separator = use_separator(SeparatorOptions {
            state,
            lang: None,
            dir: None,
        });

        assert_eq!(separator.attrs.role, None);
        assert_eq!(separator.attrs.aria_orientation, None);
        assert_eq!(separator.attrs.aria_hidden, Some("true"));
        assert_eq!(separator.attrs.lang, None);
        assert_eq!(separator.attrs.dir, None);
        assert!(!separator.state.is_semantic);
        assert!(separator.state.is_decorative);
    }
}
