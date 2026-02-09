mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_MESSAGE, ErrorViewTone};
pub use motion::ErrorViewMotion;
pub use view::ErrorView;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorViewStateInput {
    pub tone: ErrorViewTone,
    pub is_invalid: bool,
    pub compact: bool,
    pub bordered: bool,
    pub has_icon: bool,
    pub has_actions: bool,
    pub has_children: bool,
    pub has_custom_message: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorViewState {
    pub tone: ErrorViewTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_visible: bool,
    pub is_hidden: bool,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub is_compact: bool,
    pub is_bordered: bool,
    pub has_icon: bool,
    pub has_actions: bool,
    pub has_children: bool,
    pub content_attr: &'static str,
    pub message_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
