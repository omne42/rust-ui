mod logic;
pub mod styles;
mod view;

pub use view::SidebarContent;

pub const DEFAULT_ARIA_LABEL: &str = "Sidebar content";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarContentStateInput {
    pub disabled: bool,
    pub padded: bool,
    pub scrollable: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarContentState {
    pub disabled: bool,
    pub enabled: bool,
    pub padded: bool,
    pub compact: bool,
    pub scrollable: bool,
    pub static_layout: bool,
    pub state_attr: &'static str,
    pub padding_attr: &'static str,
    pub scroll_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
