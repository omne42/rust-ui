mod logic;
pub mod styles;
mod view;

pub use logic::EmptyMediaVariant;
pub use view::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptySlot {
    Root,
    Header,
    Title,
    Description,
    Content,
    Media,
}

impl EmptySlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            EmptySlot::Root => "empty",
            EmptySlot::Header => "empty-header",
            EmptySlot::Title => "empty-title",
            EmptySlot::Description => "empty-description",
            EmptySlot::Content => "empty-content",
            EmptySlot::Media => "empty-icon",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            EmptySlot::Root => "ui-empty",
            EmptySlot::Header => "ui-empty__header",
            EmptySlot::Title => "ui-empty__title",
            EmptySlot::Description => "ui-empty__description",
            EmptySlot::Content => "ui-empty__content",
            EmptySlot::Media => "ui-empty__media",
        }
    }

    pub fn state_attr(self) -> &'static str {
        match self {
            EmptySlot::Root => "root",
            EmptySlot::Header => "header",
            EmptySlot::Title => "title",
            EmptySlot::Description => "description",
            EmptySlot::Content => "content",
            EmptySlot::Media => "media",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmptyPartStateInput {
    pub slot: EmptySlot,
    pub media_variant: EmptyMediaVariant,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmptyPartState {
    pub slot: EmptySlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub media_variant: EmptyMediaVariant,
    pub media_variant_attr: &'static str,
    pub has_custom_class_name: bool,
    pub class_source_attr: &'static str,
    pub variant_source_attr: &'static str,
}
