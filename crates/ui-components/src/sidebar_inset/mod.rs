mod logic;
pub mod styles;
mod view;

pub use view::SidebarInset;

pub const DEFAULT_ARIA_LABEL: &str = "Sidebar inset";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarInsetStateInput {
    pub side: crate::sidebar::SidebarSide,
    pub padded: bool,
    pub recessed: bool,
    pub disabled: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarInsetState {
    pub side: crate::sidebar::SidebarSide,
    pub side_attr: &'static str,
    pub padded: bool,
    pub compact: bool,
    pub recessed: bool,
    pub plain: bool,
    pub disabled: bool,
    pub enabled: bool,
    pub state_attr: &'static str,
    pub padding_attr: &'static str,
    pub surface_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
