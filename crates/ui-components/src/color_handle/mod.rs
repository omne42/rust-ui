mod logic;
pub mod styles;
mod view;

pub use logic::DEFAULT_ARIA_LABEL;
pub use view::ColorHandle;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorHandleStateInput {
    pub disabled: bool,
    pub focused: bool,
    pub dragging: bool,
    pub show_loupe: bool,
    pub has_color: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorHandleState {
    pub is_disabled: bool,
    pub is_focused: bool,
    pub is_dragging: bool,
    pub loupe_visible: bool,
    pub has_color: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
