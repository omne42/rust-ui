use crate::a11y::{A11yDirection, locale_attrs};
use ui_state_primitives::spacer::SpacerState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SpacerHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpacerAttrs {
    pub role: &'static str,
    pub aria_hidden: &'static str,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_slot: &'static str,
    pub data_axis: &'static str,
    pub data_size: &'static str,
    pub data_state: &'static str,
    pub data_vertical: Option<&'static str>,
    pub data_horizontal: Option<&'static str>,
    pub data_custom_class: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpacerSemanticState {
    pub axis: &'static str,
    pub size: &'static str,
    pub is_vertical: bool,
    pub is_horizontal: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpacerContract {
    pub attrs: SpacerAttrs,
    pub handlers: SpacerHandlers,
    pub state: SpacerSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpacerOptions {
    pub state: SpacerState,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_spacer(options: SpacerOptions) -> SpacerContract {
    let locale = locale_attrs(options.lang, options.dir);

    SpacerContract {
        attrs: SpacerAttrs {
            role: "presentation",
            aria_hidden: "true",
            lang: locale.lang,
            dir: locale.dir,
            data_slot: "spacer",
            data_axis: options.state.axis_attr,
            data_size: options.state.size_attr,
            data_state: options.state.axis_attr,
            data_vertical: options.state.is_vertical.then_some("true"),
            data_horizontal: options.state.is_horizontal.then_some("true"),
            data_custom_class: options.state.has_custom_class_name.then_some("true"),
        },
        handlers: SpacerHandlers,
        state: SpacerSemanticState {
            axis: options.state.axis_attr,
            size: options.state.size_attr,
            is_vertical: options.state.is_vertical,
            is_horizontal: options.state.is_horizontal,
            has_custom_class_name: options.state.has_custom_class_name,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_state_primitives::spacer::{SpacerAxis, SpacerSize, SpacerStateInput, resolve_state};

    #[test]
    fn use_spacer_maps_locale_and_semantic_attrs() {
        let state = resolve_state(SpacerStateInput {
            axis: SpacerAxis::Horizontal,
            size: SpacerSize::Lg,
            has_custom_class_name: true,
        });

        let contract = use_spacer(SpacerOptions {
            state,
            lang: Some("  zh-CN ".to_string()),
            dir: Some(A11yDirection::Rtl),
        });

        assert_eq!(contract.attrs.role, "presentation");
        assert_eq!(contract.attrs.aria_hidden, "true");
        assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(contract.attrs.dir, Some("rtl"));
        assert_eq!(contract.attrs.data_slot, "spacer");
        assert_eq!(contract.attrs.data_axis, "horizontal");
        assert_eq!(contract.attrs.data_size, "lg");
        assert_eq!(contract.attrs.data_state, "horizontal");
        assert_eq!(contract.attrs.data_vertical, None);
        assert_eq!(contract.attrs.data_horizontal, Some("true"));
        assert_eq!(contract.attrs.data_custom_class, Some("true"));
        assert_eq!(contract.state.axis, "horizontal");
        assert_eq!(contract.state.size, "lg");
        assert!(contract.state.is_horizontal);
        assert!(!contract.state.is_vertical);
        assert!(contract.state.has_custom_class_name);
    }

    #[test]
    fn use_spacer_omits_optional_markers_for_default_vertical_case() {
        let state = resolve_state(SpacerStateInput {
            axis: SpacerAxis::Vertical,
            size: SpacerSize::Md,
            has_custom_class_name: false,
        });

        let contract = use_spacer(SpacerOptions {
            state,
            lang: None,
            dir: None,
        });

        assert_eq!(contract.attrs.data_axis, "vertical");
        assert_eq!(contract.attrs.data_size, "md");
        assert_eq!(contract.attrs.data_vertical, Some("true"));
        assert_eq!(contract.attrs.data_horizontal, None);
        assert_eq!(contract.attrs.data_custom_class, None);
        assert_eq!(contract.attrs.lang, None);
        assert_eq!(contract.attrs.dir, None);
    }
}
