use crate::a11y::{A11yDirection, region_attrs};
use ui_state_primitives::surface::SurfaceState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SurfaceHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SurfaceAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_tone: &'static str,
    pub data_elevation: &'static str,
    pub data_state: &'static str,
    pub data_bordered: Option<&'static str>,
    pub data_padded: Option<&'static str>,
    pub data_plain: Option<&'static str>,
    pub data_aria_source: &'static str,
    pub data_custom_class: Option<&'static str>,
    pub data_class_source: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceSemanticState {
    pub tone: &'static str,
    pub elevation: &'static str,
    pub state: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
    pub is_bordered: bool,
    pub is_padded: bool,
    pub is_plain: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SurfaceContract {
    pub attrs: SurfaceAttrs,
    pub handlers: SurfaceHandlers,
    pub state: SurfaceSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SurfaceOptions {
    pub state: SurfaceState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_surface(options: SurfaceOptions) -> SurfaceContract {
    let region = region_attrs(options.aria_label, options.lang, options.dir);

    SurfaceContract {
        attrs: SurfaceAttrs {
            role: region.role,
            aria_label: region.aria_label,
            lang: region.lang,
            dir: region.dir,
            data_tone: options.state.tone_attr,
            data_elevation: options.state.elevation_attr,
            data_state: options.state.data_state_attr,
            data_bordered: options.state.is_bordered.then_some("true"),
            data_padded: options.state.is_padded.then_some("true"),
            data_plain: options.state.is_plain.then_some("true"),
            data_aria_source: options.state.aria_source_attr,
            data_custom_class: options.state.has_custom_class_name.then_some("true"),
            data_class_source: options.state.class_source_attr,
        },
        handlers: SurfaceHandlers,
        state: SurfaceSemanticState {
            tone: options.state.tone_attr,
            elevation: options.state.elevation_attr,
            state: options.state.data_state_attr,
            aria_source: options.state.aria_source_attr,
            class_source: options.state.class_source_attr,
            is_bordered: options.state.is_bordered,
            is_padded: options.state.is_padded,
            is_plain: options.state.is_plain,
            has_custom_class_name: options.state.has_custom_class_name,
        },
    }
}

#[cfg(test)]
#[path = "test/surface.rs"]
mod tests;
