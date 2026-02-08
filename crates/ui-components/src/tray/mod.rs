mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use motion::TrayMotion;
pub use view::Tray;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrayStateInput {
    pub has_description: bool,
    pub has_footer: bool,
    pub show_close_button: bool,
    pub is_fixed_height: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrayState {
    pub show_description: bool,
    pub description_attr: &'static str,
    pub show_footer: bool,
    pub footer_attr: &'static str,
    pub show_close_button: bool,
    pub close_button_class: &'static str,
    pub close_button_attr: &'static str,
    pub is_fixed_height: bool,
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub has_custom_class_name: bool,
}
