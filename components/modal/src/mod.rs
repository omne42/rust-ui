mod logic;
pub mod motion;
pub mod protocol;
pub mod styles;
mod view;

pub use view::Modal;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalSlot {
    Root,
    Title,
    Description,
    Body,
}

impl ModalSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            ModalSlot::Root => "modal",
            ModalSlot::Title => "modal-title",
            ModalSlot::Description => "modal-description",
            ModalSlot::Body => "modal-body",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            ModalSlot::Root => "ui-modal",
            ModalSlot::Title => "ui-modal__title",
            ModalSlot::Description => "ui-modal__description",
            ModalSlot::Body => "ui-modal__body",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalDescriptionState {
    WithDescription,
    TitleOnly,
}

impl ModalDescriptionState {
    pub fn as_state_attr(self) -> &'static str {
        match self {
            Self::WithDescription => "with-description",
            Self::TitleOnly => "title-only",
        }
    }

    pub fn as_description_attr(self) -> &'static str {
        match self {
            Self::WithDescription => "present",
            Self::TitleOnly => "absent",
        }
    }

    pub fn shows_description(self) -> bool {
        matches!(self, Self::WithDescription)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModalPartStateInput {
    pub slot: ModalSlot,
    pub description_state: ModalDescriptionState,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModalPartState {
    pub slot: ModalSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub description_state: ModalDescriptionState,
    pub state_attr: &'static str,
    pub description_attr: &'static str,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
    pub id_source_attr: &'static str,
    pub title_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}

#[cfg(all(test, not(feature = "component-modal")))]
#[path = "../test/semantics.rs"]
mod semantics;
