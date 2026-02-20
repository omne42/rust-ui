use super::*;

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

#[test]
fn toggle_key_handler_accepts_enter_and_space() {
    assert!(should_toggle_key("Enter", false));
    assert!(should_toggle_key(" ", false));
    assert!(!should_toggle_key("Escape", false));
    assert!(!should_toggle_key("Enter", true));
}
