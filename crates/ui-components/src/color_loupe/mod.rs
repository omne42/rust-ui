mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_COLOR};
pub use view::ColorLoupe;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorLoupeStateInput {
    pub open: bool,
    pub disabled: bool,
    pub has_color: bool,
    pub x_percent: f32,
    pub y_percent: f32,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorLoupeState {
    pub is_open: bool,
    pub is_disabled: bool,
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
