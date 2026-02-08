mod logic;
pub mod styles;
mod view;

pub use crate::slider::SliderMotion as ColorSliderMotion;
pub use logic::{ColorSliderChannel, DEFAULT_ARIA_LABEL};
pub use view::ColorSlider;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorSliderStateInput {
    pub disabled: bool,
    pub channel: ColorSliderChannel,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub show_value_label: bool,
    pub has_custom_motion: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_track: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorSliderState {
    pub is_disabled: bool,
    pub channel: ColorSliderChannel,
    pub channel_class: &'static str,
    pub channel_attr: &'static str,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub value: f64,
    pub value_percent: f64,
    pub show_value_label: bool,
    pub data_state_attr: &'static str,
    pub motion_source_class: &'static str,
    pub motion_source_attr: &'static str,
    pub label_source_class: &'static str,
    pub label_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub track_source_class: &'static str,
    pub track_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub has_custom_track: bool,
}
