pub(crate) mod logic;
pub(crate) mod motion;
pub mod styles;
mod view;

pub use logic::DEFAULT_ARIA_LABEL;
pub use motion::ColorWheelMotion;
pub use view::ColorWheel;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorWheelStateInput {
    pub disabled: bool,
    pub value: f64,
    pub step: f64,
    pub show_value_label: bool,
    pub has_custom_motion: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorWheelState {
    pub is_disabled: bool,
    pub value: f64,
    pub step: f64,
    pub value_percent: f64,
    pub show_value_label: bool,
    pub data_state_attr: &'static str,
    pub motion_source_class: &'static str,
    pub motion_source_attr: &'static str,
    pub label_source_class: &'static str,
    pub label_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
