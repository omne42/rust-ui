mod logic;
pub mod styles;
mod view;

pub use logic::{AspectRatioPreset, AspectRatioRadius, DEFAULT_ARIA_LABEL};
pub use view::AspectRatio;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AspectRatioStateInput {
    pub ratio: AspectRatioPreset,
    pub radius: AspectRatioRadius,
    pub bordered: bool,
    pub fill: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AspectRatioState {
    pub ratio: AspectRatioPreset,
    pub ratio_class: &'static str,
    pub ratio_attr: &'static str,
    pub radius: AspectRatioRadius,
    pub radius_class: &'static str,
    pub radius_attr: &'static str,
    pub is_bordered: bool,
    pub bordered_class: &'static str,
    pub is_fill: bool,
    pub fill_class: &'static str,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
