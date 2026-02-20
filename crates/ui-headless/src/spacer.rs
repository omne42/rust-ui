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
#[path = "test/spacer.rs"]
mod tests;
