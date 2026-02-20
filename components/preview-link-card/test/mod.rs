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
