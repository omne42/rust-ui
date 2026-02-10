mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_TRIGGER_LABEL};
pub use view::Sidenav;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidenavStateInput {
    pub disabled: bool,
    pub show_trigger: bool,
    pub enable_shortcut: bool,
    pub is_controlled: bool,
    pub initial_open: bool,
    pub has_custom_shortcut_key: bool,
    pub has_custom_trigger_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_open_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidenavState {
    pub is_disabled: bool,
    pub show_trigger: bool,
    pub enable_shortcut: bool,
    pub is_controlled: bool,
    pub initial_open: bool,
    pub has_custom_shortcut_key: bool,
    pub has_custom_trigger_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_open_handler: bool,
    pub state_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub initial_open_attr: &'static str,
    pub trigger_mode_attr: &'static str,
    pub shortcut_mode_attr: &'static str,
    pub label_source_attr: &'static str,
    pub trigger_source_attr: &'static str,
    pub shortcut_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub handler_source_attr: &'static str,
}
