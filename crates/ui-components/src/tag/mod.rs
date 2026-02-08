mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_REMOVE_ARIA_LABEL, TagSize, TagVariant};
pub use view::Tag;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagStateInput {
    pub variant: TagVariant,
    pub size: TagSize,
    pub disabled: bool,
    pub removable: bool,
    pub has_remove_handler: bool,
    pub has_custom_remove_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagState {
    pub variant: TagVariant,
    pub size: TagSize,
    pub variant_class: &'static str,
    pub size_class: &'static str,
    pub variant_attr: &'static str,
    pub size_attr: &'static str,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub is_enabled: bool,
    pub is_disabled: bool,
    pub is_removable: bool,
    pub is_static: bool,
    pub has_remove_handler: bool,
    pub has_custom_remove_aria_label: bool,
    pub remove_label_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub class_source_attr: &'static str,
}
