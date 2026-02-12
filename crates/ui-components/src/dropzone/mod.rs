mod logic;
pub mod styles;
mod view;

pub use view::Dropzone;

pub const DEFAULT_LABEL: &str = "Drop files";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropzoneStateInput {
    pub disabled: bool,
    pub has_custom_label: bool,
    pub aria_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_drop_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropzoneState {
    pub state_attr: &'static str,
    pub label_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub drop_handler_source_attr: &'static str,
    pub has_custom_label: bool,
    pub has_custom_aria: bool,
    pub has_custom_drop_handler: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}
