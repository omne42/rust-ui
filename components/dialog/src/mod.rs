mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_CLOSE_LABEL, DEFAULT_ID_BASE, DEFAULT_SHOW_CLOSE_BUTTON, DEFAULT_SIZE, DEFAULT_TITLE,
    DialogSize,
};
pub use motion::DialogMotion;
pub use view::Dialog;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogSlot {
    Root,
    Header,
    Title,
    Description,
    Body,
    Footer,
    Close,
}

impl DialogSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            DialogSlot::Root => "dialog",
            DialogSlot::Header => "dialog-header",
            DialogSlot::Title => "dialog-title",
            DialogSlot::Description => "dialog-description",
            DialogSlot::Body => "dialog-body",
            DialogSlot::Footer => "dialog-footer",
            DialogSlot::Close => "dialog-close",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            DialogSlot::Root => "ui-dialog",
            DialogSlot::Header => "ui-dialog__header",
            DialogSlot::Title => "ui-dialog__title",
            DialogSlot::Description => "ui-dialog__description",
            DialogSlot::Body => "ui-dialog__body",
            DialogSlot::Footer => "ui-dialog__footer",
            DialogSlot::Close => "ui-dialog__close",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogPartStateInput {
    pub slot: DialogSlot,
    pub size: DialogSize,
    pub has_description: bool,
    pub has_footer: bool,
    pub show_close_button: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_close_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogPartState {
    pub slot: DialogSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub size: DialogSize,
    pub size_attr: &'static str,
    pub size_class: &'static str,
    pub state_attr: &'static str,
    pub description_attr: &'static str,
    pub footer_attr: &'static str,
    pub close_button_attr: &'static str,
    pub show_description: bool,
    pub show_footer: bool,
    pub show_close_button: bool,
    pub has_custom_size: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_close_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
    pub size_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub footer_source_attr: &'static str,
    pub close_source_attr: &'static str,
    pub id_source_attr: &'static str,
    pub title_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics;
