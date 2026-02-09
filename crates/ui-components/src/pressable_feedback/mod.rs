mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, PressableFeedbackEffect, PressableFeedbackTone};
pub use motion::PressableFeedbackMotion;
pub use view::PressableFeedback;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PressableFeedbackStateInput {
    pub tone: PressableFeedbackTone,
    pub effect: PressableFeedbackEffect,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub bounded: bool,
    pub has_highlight: bool,
    pub has_ripple: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_press_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PressableFeedbackState {
    pub tone: PressableFeedbackTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub effect: PressableFeedbackEffect,
    pub effect_class: &'static str,
    pub effect_attr: &'static str,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub is_bounded: bool,
    pub is_unbounded: bool,
    pub boundary_class: &'static str,
    pub boundary_attr: &'static str,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub has_highlight: bool,
    pub has_ripple: bool,
    pub highlight_attr: &'static str,
    pub ripple_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub has_custom_press_handler: bool,
}
