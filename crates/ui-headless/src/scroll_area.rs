use crate::a11y::{A11yDirection, region_attrs};
use ui_state_primitives::scroll_area::ScrollAreaState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ScrollAreaHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScrollAreaRootAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_orientation: &'static str,
    pub data_disabled: Option<&'static str>,
    pub data_max_height: &'static str,
    pub data_aria_source: &'static str,
    pub data_class_source: &'static str,
    pub data_custom_class: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollAreaViewportAttrs {
    pub tabindex: i32,
    pub aria_disabled: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollAreaSemanticState {
    pub orientation: &'static str,
    pub is_disabled: bool,
    pub has_custom_max_height: bool,
    pub max_height_source: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScrollAreaContract {
    pub root_attrs: ScrollAreaRootAttrs,
    pub viewport_attrs: ScrollAreaViewportAttrs,
    pub handlers: ScrollAreaHandlers,
    pub state: ScrollAreaSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScrollAreaOptions {
    pub state: ScrollAreaState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_scroll_area(options: ScrollAreaOptions) -> ScrollAreaContract {
    let region = region_attrs(options.aria_label, options.lang, options.dir);
    let state = options.state;

    ScrollAreaContract {
        root_attrs: ScrollAreaRootAttrs {
            role: region.role,
            aria_label: region.aria_label,
            lang: region.lang,
            dir: region.dir,
            data_orientation: state.orientation_attr,
            data_disabled: state.disabled.then_some("true"),
            data_max_height: state.max_height_attr.as_attr(),
            data_aria_source: state.aria_source_attr.as_attr(),
            data_class_source: state.class_source_attr.as_attr(),
            data_custom_class: state.has_custom_class_name.then_some("true"),
        },
        viewport_attrs: ScrollAreaViewportAttrs {
            tabindex: if state.disabled { -1 } else { 0 },
            aria_disabled: state.disabled.then_some("true"),
        },
        handlers: ScrollAreaHandlers,
        state: ScrollAreaSemanticState {
            orientation: state.orientation_attr,
            is_disabled: state.disabled,
            has_custom_max_height: state.has_custom_max_height,
            max_height_source: state.max_height_attr.as_attr(),
            aria_source: state.aria_source_attr.as_attr(),
            class_source: state.class_source_attr.as_attr(),
            has_custom_class_name: state.has_custom_class_name,
        },
    }
}

#[cfg(test)]
#[path = "test/scroll_area.rs"]
mod tests;
