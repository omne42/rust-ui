mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_CLOSE_DELAY_MS, DEFAULT_DESCRIPTION, DEFAULT_DISABLED, DEFAULT_OPEN_DELAY_MS,
    DEFAULT_SITE_LABEL, DEFAULT_TITLE, DEFAULT_URL,
};
pub use motion::PreviewCardMotion;
pub use view::PreviewCard;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewCardSiteLabelSource {
    Default,
    Derived,
    Custom,
}

impl PreviewCardSiteLabelSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            PreviewCardSiteLabelSource::Default => "default",
            PreviewCardSiteLabelSource::Derived => "derived",
            PreviewCardSiteLabelSource::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewCardSlot {
    Root,
    Trigger,
    Panel,
}

impl PreviewCardSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            PreviewCardSlot::Root => "preview-card",
            PreviewCardSlot::Trigger => "preview-card-trigger",
            PreviewCardSlot::Panel => "preview-card-panel",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            PreviewCardSlot::Root => "ui-preview-card",
            PreviewCardSlot::Trigger => "ui-preview-card__trigger",
            PreviewCardSlot::Panel => "ui-preview-card__panel",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PreviewCardPartStateInput {
    pub slot: PreviewCardSlot,
    pub is_disabled: bool,
    pub has_image: bool,
    pub has_custom_class_name: bool,
    pub has_custom_delays: bool,
    pub has_custom_id: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_url: bool,
    pub site_label_source: PreviewCardSiteLabelSource,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PreviewCardPartState {
    pub slot: PreviewCardSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub content_attr: &'static str,
    pub is_disabled: bool,
    pub has_image: bool,
    pub has_custom_class_name: bool,
    pub has_custom_delays: bool,
    pub has_custom_id: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_url: bool,
    pub has_custom_motion: bool,
    pub class_source_attr: &'static str,
    pub delay_source_attr: &'static str,
    pub id_source_attr: &'static str,
    pub title_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub url_source_attr: &'static str,
    pub site_label_source: PreviewCardSiteLabelSource,
    pub motion_source_attr: &'static str,
}

#[cfg(test)]
#[path = "../test/mod.rs"]
mod tests;
