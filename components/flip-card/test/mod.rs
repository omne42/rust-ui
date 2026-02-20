use super::*;

#[test]
fn flip_card_slot_contracts_are_stable() {
    assert_eq!(FlipCardSlot::Root.as_attr(), "flip-card");
    assert_eq!(FlipCardSlot::Root.base_class(), "ui-flip-card");
    assert_eq!(FlipCardSlot::Front.as_attr(), "flip-card-front");
    assert_eq!(
        FlipCardSlot::Front.base_class(),
        "ui-flip-card__face ui-flip-card__front"
    );
    assert_eq!(FlipCardSlot::Back.as_attr(), "flip-card-back");
    assert_eq!(
        FlipCardSlot::Back.base_class(),
        "ui-flip-card__face ui-flip-card__back"
    );
}
