mod logic;
mod render;
pub mod styles;

pub use logic::{
    DEFAULT_ARIA_LABEL, ViewBackground, ViewBorder, ViewElement, ViewPadding, ViewRadius,
    ViewShadow,
};
pub use render::View;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ViewStateInput {
    pub background: ViewBackground,
    pub border: ViewBorder,
    pub padding: ViewPadding,
    pub radius: ViewRadius,
    pub shadow: ViewShadow,
    pub element: ViewElement,
    pub fluid: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ViewState {
    pub background: ViewBackground,
    pub background_class: &'static str,
    pub background_attr: &'static str,
    pub border: ViewBorder,
    pub border_class: &'static str,
    pub border_attr: &'static str,
    pub padding: ViewPadding,
    pub padding_class: &'static str,
    pub padding_attr: &'static str,
    pub radius: ViewRadius,
    pub radius_class: &'static str,
    pub radius_attr: &'static str,
    pub shadow: ViewShadow,
    pub shadow_class: &'static str,
    pub shadow_attr: &'static str,
    pub element: ViewElement,
    pub element_class: &'static str,
    pub element_attr: &'static str,
    pub is_fluid: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
