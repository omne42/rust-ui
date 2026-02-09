mod logic;
pub mod styles;
mod view;

pub use view::SidebarMenuBadge;

pub const DEFAULT_ARIA_LABEL: &str = "Sidebar menu badge";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarMenuBadgeStateInput {
    pub muted: bool,
    pub disabled: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarMenuBadgeState {
    pub muted: bool,
    pub emphasized: bool,
    pub disabled: bool,
    pub enabled: bool,
    pub state_attr: &'static str,
    pub tone_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
