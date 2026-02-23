use super::*;

#[test]
fn preview_link_card_slot_contracts_are_stable() {
    assert_eq!(
        logic::PreviewLinkCardSlot::Root.as_attr(),
        "preview-link-card"
    );
    assert_eq!(
        logic::PreviewLinkCardSlot::Root.base_class(),
        "ui-preview-link-card"
    );
    assert_eq!(
        logic::PreviewLinkCardSlot::Trigger.as_attr(),
        "preview-link-card-trigger"
    );
    assert_eq!(
        logic::PreviewLinkCardSlot::Trigger.base_class(),
        "ui-preview-link-card__trigger"
    );
    assert_eq!(
        logic::PreviewLinkCardSlot::Panel.as_attr(),
        "preview-link-card-panel"
    );
    assert_eq!(
        logic::PreviewLinkCardSlot::Panel.base_class(),
        "ui-preview-link-card__panel"
    );
}
