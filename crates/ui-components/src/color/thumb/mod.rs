pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_COLOR};
pub use motion::ColorThumbMotion;
pub use view::ColorThumb;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorThumbStateInput {
    pub disabled: bool,
    pub focused: bool,
    pub dragging: bool,
    pub show_loupe: bool,
    pub has_color: bool,
    pub x_percent: f32,
    pub y_percent: f32,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorThumbState {
    pub is_disabled: bool,
    pub is_focused: bool,
    pub is_dragging: bool,
    pub loupe_visible: bool,
    pub has_color: bool,
    pub x_percent: f32,
    pub y_percent: f32,
    pub x_bucket_class: &'static str,
    pub y_bucket_class: &'static str,
    pub x_bucket_attr: &'static str,
    pub y_bucket_attr: &'static str,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
