use crate::a11y::{A11yDirection, region_attrs};
use ui_state_primitives::aspect_ratio::AspectRatioState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct AspectRatioHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AspectRatioAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_ratio: &'static str,
    pub data_radius: &'static str,
    pub data_bordered: Option<&'static str>,
    pub data_fill: Option<&'static str>,
    pub data_state: &'static str,
    pub data_aria_source: &'static str,
    pub data_custom_class: Option<&'static str>,
    pub data_class_source: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AspectRatioSemanticState {
    pub ratio: &'static str,
    pub radius: &'static str,
    pub state: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
    pub is_bordered: bool,
    pub is_fill: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AspectRatioContract {
    pub attrs: AspectRatioAttrs,
    pub handlers: AspectRatioHandlers,
    pub state: AspectRatioSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AspectRatioOptions {
    pub state: AspectRatioState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_aspect_ratio(options: AspectRatioOptions) -> AspectRatioContract {
    let region = region_attrs(options.aria_label, options.lang, options.dir);

    AspectRatioContract {
        attrs: AspectRatioAttrs {
            role: region.role,
            aria_label: region.aria_label,
            lang: region.lang,
            dir: region.dir,
            data_ratio: options.state.ratio_attr,
            data_radius: options.state.radius_attr,
            data_bordered: options.state.is_bordered.then_some("true"),
            data_fill: options.state.is_fill.then_some("true"),
            data_state: options.state.data_state_attr,
            data_aria_source: options.state.aria_source_attr,
            data_custom_class: options.state.has_custom_class_name.then_some("true"),
            data_class_source: options.state.class_source_attr,
        },
        handlers: AspectRatioHandlers,
        state: AspectRatioSemanticState {
            ratio: options.state.ratio_attr,
            radius: options.state.radius_attr,
            state: options.state.data_state_attr,
            aria_source: options.state.aria_source_attr,
            class_source: options.state.class_source_attr,
            is_bordered: options.state.is_bordered,
            is_fill: options.state.is_fill,
            has_custom_class_name: options.state.has_custom_class_name,
        },
    }
}

#[cfg(test)]
#[path = "test/aspect_ratio.rs"]
mod tests;
