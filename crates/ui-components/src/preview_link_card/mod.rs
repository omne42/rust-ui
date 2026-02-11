mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_CLOSE_DELAY_MS, DEFAULT_DESCRIPTION, DEFAULT_DISABLED, DEFAULT_OPEN_DELAY_MS,
    DEFAULT_SITE_LABEL, DEFAULT_TITLE, DEFAULT_URL,
};
pub use motion::PreviewLinkCardMotion;
pub use view::PreviewLinkCard;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewLinkCardSlot {
    Root,
    Trigger,
    Panel,
}

impl PreviewLinkCardSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            PreviewLinkCardSlot::Root => "preview-link-card",
            PreviewLinkCardSlot::Trigger => "preview-link-card-trigger",
            PreviewLinkCardSlot::Panel => "preview-link-card-panel",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            PreviewLinkCardSlot::Root => "ui-preview-link-card",
            PreviewLinkCardSlot::Trigger => "ui-preview-link-card__trigger",
            PreviewLinkCardSlot::Panel => "ui-preview-link-card__panel",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PreviewLinkCardPartStateInput {
    pub slot: PreviewLinkCardSlot,
    pub disabled: bool,
    pub has_image: bool,
    pub has_custom_class_name: bool,
    pub has_custom_delays: bool,
    pub has_custom_id: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_url: bool,
    pub site_label_source_attr: &'static str,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PreviewLinkCardPartState {
    pub slot: PreviewLinkCardSlot,
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
    pub site_label_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_link_card_slot_contracts_are_stable() {
        assert_eq!(PreviewLinkCardSlot::Root.as_attr(), "preview-link-card");
        assert_eq!(
            PreviewLinkCardSlot::Root.base_class(),
            "ui-preview-link-card"
        );
        assert_eq!(
            PreviewLinkCardSlot::Trigger.as_attr(),
            "preview-link-card-trigger"
        );
        assert_eq!(
            PreviewLinkCardSlot::Trigger.base_class(),
            "ui-preview-link-card__trigger"
        );
        assert_eq!(
            PreviewLinkCardSlot::Panel.as_attr(),
            "preview-link-card-panel"
        );
        assert_eq!(
            PreviewLinkCardSlot::Panel.base_class(),
            "ui-preview-link-card__panel"
        );
    }
}
