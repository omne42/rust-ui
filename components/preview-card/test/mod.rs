use super::*;

#[test]
fn preview_card_slot_contracts_are_stable() {
    assert_eq!(PreviewCardSlot::Root.as_attr(), "preview-card");
    assert_eq!(PreviewCardSlot::Root.base_class(), "ui-preview-card");
    assert_eq!(PreviewCardSlot::Trigger.as_attr(), "preview-card-trigger");
    assert_eq!(
        PreviewCardSlot::Trigger.base_class(),
        "ui-preview-card__trigger"
    );
    assert_eq!(PreviewCardSlot::Panel.as_attr(), "preview-card-panel");
    assert_eq!(
        PreviewCardSlot::Panel.base_class(),
        "ui-preview-card__panel"
    );
}
