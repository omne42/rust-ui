mod logic;
pub mod styles;
mod view;

pub use view::SidebarRail;

pub const DEFAULT_ARIA_LABEL: &str = "Toggle sidebar";
pub const DEFAULT_LABEL: &str = "toggle sidebar";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarRailStateInput {
    pub open: bool,
    pub side: crate::sidebar::SidebarSide,
    pub disabled: bool,
    pub is_controlled: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarRailState {
    pub open: bool,
    pub closed: bool,
    pub side: crate::sidebar::SidebarSide,
    pub side_attr: &'static str,
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
