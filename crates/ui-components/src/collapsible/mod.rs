mod logic;
mod motion;
pub mod styles;
mod view;

pub use crate::disclosure::DisclosureMotion as CollapsibleMotion;
pub use view::Collapsible;

pub const DEFAULT_ID_BASE: &str = "collapsible";
pub const DEFAULT_TITLE: &str = "Collapsible";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CollapsibleStateInput {
    pub is_open: bool,
    pub is_disabled: bool,
    pub is_controlled: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CollapsibleState {
    pub is_open: bool,
    pub is_closed: bool,
    pub is_disabled: bool,
    pub is_controlled: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub state_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}
