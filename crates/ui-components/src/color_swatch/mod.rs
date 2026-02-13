mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    ColorSwatchAlpha, ColorSwatchRounding, ColorSwatchShape, ColorSwatchSize, DEFAULT_ARIA_LABEL,
};
pub use motion::ColorSwatchMotion;
pub use view::ColorSwatch;

pub fn sanitize_color_value(value: Option<String>) -> Option<String> {
    logic::sanitize_color_value(value)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorSwatchStateInput {
    pub size: ColorSwatchSize,
    pub rounding: ColorSwatchRounding,
    pub shape: ColorSwatchShape,
    pub bordered: bool,
    pub alpha: ColorSwatchAlpha,
    pub has_color: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorSwatchState {
    pub size: ColorSwatchSize,
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub rounding: ColorSwatchRounding,
    pub rounding_class: &'static str,
    pub rounding_attr: &'static str,
    pub shape: ColorSwatchShape,
    pub shape_class: &'static str,
    pub shape_attr: &'static str,
    pub alpha: ColorSwatchAlpha,
    pub alpha_class: &'static str,
    pub alpha_attr: &'static str,
    pub is_bordered: bool,
    pub has_color: bool,
    pub has_custom_class_name: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
}
