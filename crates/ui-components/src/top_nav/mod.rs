pub use crate::navigation_menu::NavigationMenuItem as TopNavItem;
pub use crate::navigation_menu::NavigationMenuMotion as TopNavMotion;

mod logic;
pub mod styles;
mod view;

pub use view::TopNav;

pub const DEFAULT_LABEL: &str = "Top navigation";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TopNavStateInput {
    pub is_controlled: bool,
    pub has_default_selected_id: bool,
    pub activate_on_focus: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TopNavState {
    pub state_attr: &'static str,
    pub selection_mode_attr: &'static str,
    pub default_selection_attr: &'static str,
    pub focus_activation_attr: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub has_default_selected_id: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}
