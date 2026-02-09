mod logic;
pub mod styles;
mod view;

pub use view::SidebarMenuAction;

pub const DEFAULT_ARIA_LABEL: &str = "Sidebar menu action";
pub const DEFAULT_LABEL: &str = "⋯";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarMenuActionStateInput {
    pub hover_only: bool,
    pub disabled: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarMenuActionState {
    pub hover_only: bool,
    pub always_visible: bool,
    pub disabled: bool,
    pub enabled: bool,
    pub state_attr: &'static str,
    pub visibility_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
