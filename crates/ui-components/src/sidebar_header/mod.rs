mod logic;
pub mod styles;
mod view;

pub use view::SidebarHeader;

pub const DEFAULT_ARIA_LABEL: &str = "Sidebar header";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarHeaderStateInput {
    pub disabled: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarHeaderState {
    pub disabled: bool,
    pub enabled: bool,
    pub state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
