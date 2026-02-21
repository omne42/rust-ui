use super::*;

#[test]
fn helpers_keep_state_and_mode_contracts_stable() {
    assert_eq!(state_attr(true), "flipped");
    assert_eq!(state_attr(false), "default");
    assert_eq!(flip_mode_attr(true), "hover");
    assert_eq!(flip_mode_attr(false), "toggle");
    assert_eq!(
        FlipCardFlipMode::from_hover_flag(true),
        FlipCardFlipMode::Hover
    );
    assert_eq!(
        FlipCardFlipMode::from_hover_flag(false),
        FlipCardFlipMode::Toggle
    );
    assert!(FlipCardFlipMode::Hover.is_hover());
    assert!(!FlipCardFlipMode::Toggle.is_hover());

    assert_eq!(
        normalize_optional_text(Some("  flip  ".to_string())),
        Some("flip".to_string())
    );
    assert_eq!(normalize_optional_text(Some("   ".to_string())), None);

    assert_eq!(
        resolve_id(Some(" docs-flip ".to_string()), "fallback".to_string()),
        ("docs-flip".to_string(), true)
    );
    assert_eq!(
        resolve_id(None, "generated".to_string()),
        ("generated".to_string(), false)
    );
}

#[test]
fn normalize_behavior_flags_keeps_mode_and_source_contracts() {
    let custom = normalize_behavior_flags(FlipCardBehaviorFlagsInput {
        is_disabled: Some(true),
        disabled: Some(false),
        flip_mode: Some(FlipCardFlipMode::Toggle),
        is_flip_on_hover: Some(true),
        flip_on_hover: Some(true),
    });
    assert!(custom.is_disabled);
    assert_eq!(custom.flip_mode, FlipCardFlipMode::Toggle);
    assert_eq!(custom.disabled_source_attr, "is_disabled");
    assert_eq!(custom.flip_mode_source_attr, "flip_mode");

    let legacy_alias = normalize_behavior_flags(FlipCardBehaviorFlagsInput {
        is_disabled: None,
        disabled: None,
        flip_mode: None,
        is_flip_on_hover: None,
        flip_on_hover: Some(true),
    });
    assert_eq!(legacy_alias.flip_mode, FlipCardFlipMode::Hover);
    assert_eq!(legacy_alias.flip_mode_source_attr, "flip_on_hover");
}

#[test]
fn resolve_part_state_tracks_sources_and_visibility() {
    let root = resolve_part_state(FlipCardPartStateInput {
        slot: FlipCardSlot::Root,
        disabled: false,
        is_flipped: true,
        flip_on_hover: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_custom_id: true,
    });

    assert_eq!(root.state_attr, "flipped");
    assert_eq!(root.flip_mode_attr, "hover");
    assert_eq!(root.class_source_attr, "custom");
    assert_eq!(root.motion_source_attr, "custom");
    assert_eq!(root.id_source_attr, "custom");
    assert_eq!(root.flip_mode_source_attr, "custom");

    let front = resolve_part_state(FlipCardPartStateInput {
        slot: FlipCardSlot::Front,
        disabled: false,
        is_flipped: true,
        flip_on_hover: false,
        has_custom_class_name: false,
        has_custom_motion: false,
        has_custom_id: false,
    });
    assert_eq!(front.visibility_attr, "hidden");

    let back = resolve_part_state(FlipCardPartStateInput {
        slot: FlipCardSlot::Back,
        disabled: false,
        is_flipped: true,
        flip_on_hover: false,
        has_custom_class_name: false,
        has_custom_motion: false,
        has_custom_id: false,
    });
    assert_eq!(back.visibility_attr, "visible");
}

#[test]
fn slot_contracts_are_stable() {
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
