use super::*;
use crate::{FlipCardPartStateInput, FlipCardSlot};

#[test]
fn helpers_keep_state_and_mode_contracts_stable() {
    assert_eq!(state_attr(true), "flipped");
    assert_eq!(state_attr(false), "default");
    assert_eq!(flip_mode_attr(true), "hover");
    assert_eq!(flip_mode_attr(false), "toggle");

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
fn normalize_flipped_axis_centralizes_default_priority_and_sources() {
    let normalized = normalize_flipped_axis(FlipCardFlippedAxisInput {
        is_flipped: Some(Signal::derive(|| true)),
        default_is_flipped: Some(true),
        default_flipped: Some(false),
        on_is_flipped_change: Some(Callback::new(|_: bool| {})),
    });

    assert!(normalized.flipped_is_controlled);
    assert_eq!(normalized.flipped_control_mode_attr, "controlled");
    assert_eq!(normalized.flipped_prop_source_attr, "is_flipped");
    assert_eq!(normalized.flipped_default_source_attr, "default_is_flipped");
    assert_eq!(
        normalized.flipped_change_source_attr,
        "on_is_flipped_change"
    );
    assert!(normalized.default_is_flipped);

    let legacy_default = normalize_flipped_axis(FlipCardFlippedAxisInput {
        is_flipped: None,
        default_is_flipped: None,
        default_flipped: Some(true),
        on_is_flipped_change: None,
    });
    assert!(!legacy_default.flipped_is_controlled);
    assert_eq!(legacy_default.flipped_control_mode_attr, "uncontrolled");
    assert_eq!(
        legacy_default.flipped_default_source_attr,
        "default_flipped"
    );
    assert_eq!(legacy_default.flipped_change_source_attr, "none");
    assert!(legacy_default.default_is_flipped);
}

#[test]
fn normalize_behavior_flags_centralizes_bool_defaults_and_alias_priority() {
    let custom = normalize_behavior_flags(FlipCardBehaviorFlagsInput {
        is_disabled: Some(true),
        disabled: Some(false),
        flip_mode: Some(FlipCardFlipMode::Toggle),
        is_flip_on_hover: None,
        flip_on_hover: Some(true),
    });
    assert!(custom.is_disabled);
    assert_eq!(custom.flip_mode, FlipCardFlipMode::Toggle);
    assert_eq!(custom.disabled_source_attr, "is_disabled");
    assert_eq!(custom.flip_mode_source_attr, "flip_mode");

    let fallback = normalize_behavior_flags(FlipCardBehaviorFlagsInput {
        is_disabled: None,
        disabled: None,
        flip_mode: None,
        is_flip_on_hover: None,
        flip_on_hover: None,
    });
    assert!(!fallback.is_disabled);
    assert_eq!(fallback.flip_mode, FlipCardFlipMode::Toggle);
    assert_eq!(fallback.disabled_source_attr, "none");
    assert_eq!(fallback.flip_mode_source_attr, "none");
}

#[test]
fn normalize_behavior_flags_maps_bool_aliases_to_enum_mode() {
    let primary_bool = normalize_behavior_flags(FlipCardBehaviorFlagsInput {
        is_disabled: None,
        disabled: None,
        flip_mode: None,
        is_flip_on_hover: Some(true),
        flip_on_hover: Some(false),
    });
    assert_eq!(primary_bool.flip_mode, FlipCardFlipMode::Hover);
    assert_eq!(primary_bool.flip_mode_source_attr, "is_flip_on_hover");

    let legacy_bool = normalize_behavior_flags(FlipCardBehaviorFlagsInput {
        is_disabled: None,
        disabled: None,
        flip_mode: None,
        is_flip_on_hover: None,
        flip_on_hover: Some(true),
    });
    assert_eq!(legacy_bool.flip_mode, FlipCardFlipMode::Hover);
    assert_eq!(legacy_bool.flip_mode_source_attr, "flip_on_hover");
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
fn derive_render_state_centralizes_slot_states_and_semantic_markers() {
    let derived = derive_render_state(FlipCardDerivedRenderStateInput {
        is_disabled: true,
        is_flipped: true,
        is_hovered: true,
        flip_mode: FlipCardFlipMode::Hover,
        flip_mode_source_attr: "flip_mode",
        has_custom_class_name: true,
        has_custom_motion: true,
        has_custom_id: false,
        flipped_is_controlled: true,
        flipped_control_mode_attr: "controlled",
        flipped_prop_source_attr: "is_flipped",
        flipped_default_source_attr: "default_is_flipped",
        flipped_change_source_attr: "on_is_flipped_change",
    });

    assert_eq!(derived.root.slot, FlipCardSlot::Root);
    assert_eq!(derived.front.slot, FlipCardSlot::Front);
    assert_eq!(derived.back.slot, FlipCardSlot::Back);
    assert_eq!(derived.root.flip_mode_attr, "hover");
    assert_eq!(derived.front.visibility_attr, "hidden");
    assert_eq!(derived.back.visibility_attr, "visible");

    assert_eq!(derived.root_markers.flipped_control_mode_attr, "controlled");
    assert_eq!(derived.root_markers.flipped_prop_source_attr, "is_flipped");
    assert_eq!(
        derived.root_markers.flipped_default_source_attr,
        "default_is_flipped"
    );
    assert_eq!(
        derived.root_markers.flipped_change_source_attr,
        "on_is_flipped_change"
    );
    assert_eq!(derived.root_markers.flipped_attr, Some("true"));
    assert_eq!(derived.root_markers.default_attr, None);
    assert_eq!(derived.root_markers.hovered_attr, Some("true"));
    assert_eq!(derived.root_markers.disabled_attr, Some("true"));
    assert_eq!(derived.root_markers.enabled_attr, None);
    assert_eq!(derived.root_markers.flip_mode_source_attr, "flip_mode");
    assert_eq!(derived.root_markers.custom_class_attr, Some("true"));
    assert_eq!(derived.root_markers.custom_motion_attr, Some("true"));
    assert_eq!(derived.root_markers.custom_id_attr, None);
    assert_eq!(derived.front_markers.visible_attr, None);
    assert_eq!(derived.front_markers.hidden_attr, Some("true"));
    assert_eq!(derived.back_markers.visible_attr, Some("true"));
    assert_eq!(derived.back_markers.hidden_attr, None);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-flip-card".to_string()),
        resolve_part_state(FlipCardPartStateInput {
            slot: FlipCardSlot::Root,
            disabled: false,
            is_flipped: true,
            flip_on_hover: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_id: true,
        }),
    );

    for token in [
        "ui-flip-card",
        "ui-flip-card--enabled",
        "ui-flip-card--flipped",
        "ui-flip-card--hover",
        "ui-flip-card--custom-class",
        "ui-flip-card--custom-motion",
        "ui-flip-card--custom-id",
        "docs-flip-card",
    ] {
        assert!(
            class_name.contains(token),
            "flip card class name should include `{token}`"
        );
    }
}
