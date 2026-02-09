mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::DEFAULT_CLOSE_LABEL;
pub use motion::BottomSheetMotion;
pub use view::BottomSheet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BottomSheetStateInput {
    pub has_description: bool,
    pub has_footer: bool,
    pub show_handle: bool,
    pub show_close_button: bool,
    pub detached: bool,
    pub bottom_inset_px: f64,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BottomSheetState {
    pub show_description: bool,
    pub description_attr: &'static str,
    pub show_footer: bool,
    pub footer_attr: &'static str,
    pub show_handle: bool,
    pub handle_class: &'static str,
    pub handle_attr: &'static str,
    pub show_close_button: bool,
    pub close_button_class: &'static str,
    pub close_button_attr: &'static str,
    pub detached: bool,
    pub detached_class: &'static str,
    pub detached_attr: &'static str,
    pub inset_class: &'static str,
    pub inset_attr: &'static str,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
