mod logic;
pub mod styles;
mod view;

pub use view::SidebarFooter;

pub const DEFAULT_ARIA_LABEL: &str = "Sidebar footer";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarFooterStateInput {
    pub disabled: bool,
    pub bordered: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarFooterState {
    pub disabled: bool,
    pub enabled: bool,
    pub bordered: bool,
    pub unbordered: bool,
    pub state_attr: &'static str,
    pub border_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
