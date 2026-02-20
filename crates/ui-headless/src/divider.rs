use crate::a11y::{A11yDirection, locale_attrs};
use ui_state_primitives::divider::DividerState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DividerHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DividerAttrs {
    pub role: &'static str,
    pub aria_orientation: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_orientation: &'static str,
    pub data_state: &'static str,
    pub data_horizontal: Option<&'static str>,
    pub data_vertical: Option<&'static str>,
    pub data_custom_class: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DividerSemanticState {
    pub orientation: &'static str,
    pub is_horizontal: bool,
    pub is_vertical: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DividerContract {
    pub attrs: DividerAttrs,
    pub handlers: DividerHandlers,
    pub state: DividerSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DividerOptions {
    pub state: DividerState,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_divider(options: DividerOptions) -> DividerContract {
    let locale = locale_attrs(options.lang, options.dir);

    DividerContract {
        attrs: DividerAttrs {
            role: "separator",
            aria_orientation: options.state.aria_orientation,
            lang: locale.lang,
            dir: locale.dir,
            data_orientation: options.state.orientation_attr,
            data_state: options.state.orientation_attr,
            data_horizontal: options.state.is_horizontal.then_some("true"),
            data_vertical: options.state.is_vertical.then_some("true"),
            data_custom_class: options.state.has_custom_class_name.then_some("true"),
        },
        handlers: DividerHandlers,
        state: DividerSemanticState {
            orientation: options.state.orientation_attr,
            is_horizontal: options.state.is_horizontal,
            is_vertical: options.state.is_vertical,
            has_custom_class_name: options.state.has_custom_class_name,
        },
    }
}

#[cfg(test)]
#[path = "test/divider.rs"]
mod tests;
