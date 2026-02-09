mod logic;
pub mod styles;
mod view;

pub use view::SidebarTrigger;

pub const DEFAULT_ARIA_LABEL: &str = "Toggle sidebar";
pub const DEFAULT_LABEL: &str = "Toggle sidebar";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarTriggerStateInput {
    pub open: bool,
    pub disabled: bool,
    pub is_controlled: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarTriggerState {
    pub open: bool,
    pub closed: bool,
    pub disabled: bool,
    pub enabled: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub state_attr: &'static str,
    pub control_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
