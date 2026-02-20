use super::*;

#[test]
fn normalize_avatar_group_optional_text_filters_blank_values() {
    assert_eq!(normalize_avatar_group_optional_text(None), None);
    assert_eq!(
        normalize_avatar_group_optional_text(Some("   ".to_string())),
        None
    );
    assert_eq!(
        normalize_avatar_group_optional_text(Some("  Team avatars  ".to_string())),
        Some("Team avatars".to_string())
    );
}

#[test]
fn normalize_avatar_group_max_visible_falls_back_to_default() {
    assert_eq!(normalize_avatar_group_max_visible(None), 4);
    assert_eq!(normalize_avatar_group_max_visible(Some(3)), 3);
    assert_eq!(normalize_avatar_group_max_visible(Some(0)), 0);
}

#[test]
fn resolve_avatar_group_aria_label_defaults_and_trims() {
    assert_eq!(
        resolve_avatar_group_aria_label(None),
        ("Avatar group".to_string(), false)
    );
    assert_eq!(
        resolve_avatar_group_aria_label(Some("  Team  ".to_string())),
        ("Team".to_string(), true)
    );
}

#[test]
fn resolve_avatar_group_state_tracks_overflow_and_size_metadata() {
    let state = ui_state_primitives::avatar_group::resolve_state(AvatarGroupStateInput {
        total_count: 6,
        max_visible: 4,
        size: AvatarSize::Lg,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.total_count, 6);
    assert_eq!(state.visible_count, 4);
    assert_eq!(state.overflow_count, 2);
    assert_eq!(state.max_visible, 4);
    assert_eq!(state.size, AvatarSize::Lg);
    assert_eq!(state.size_attr, "lg");
    assert_eq!(state.state_class, "ui-avatar-group--overflow");
    assert_eq!(state.state_attr, "overflow");
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert!(state.has_overflow);
    assert!(state.has_custom_aria_label);
    assert_eq!(
        state.aria_label_source_class,
        "ui-avatar-group--label-source-custom"
    );
    assert_eq!(state.aria_label_source_attr, "custom");
    assert!(state.has_custom_class_name);
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_avatar_group_class_name_includes_state_markers() {
    let class_name = compose_avatar_group_class_name(
        Some("custom".to_string()),
        resolve_avatar_group_render_state(AvatarGroupStateInput {
            total_count: 2,
            max_visible: 4,
            size: AvatarSize::Sm,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-avatar-group",
        "ui-avatar-group--size-sm",
        "ui-avatar-group--stable",
        "ui-avatar-group--label-source-custom",
        "ui-avatar-group--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn resolve_avatar_group_aria_label_with_fallback_prefers_explicit_label() {
    assert_eq!(
        resolve_avatar_group_aria_label_with_fallback(Some(" Team ".to_string()), "Fallback"),
        ("Team".to_string(), true)
    );
    assert_eq!(
        resolve_avatar_group_aria_label_with_fallback(Some("  ".to_string()), "Fallback"),
        ("Fallback".to_string(), false)
    );
}

#[test]
fn normalize_avatar_group_input_centralizes_defaults() {
    let normalized = normalize_avatar_group_input(
        None,
        Some("  ".to_string()),
        Some(" custom ".to_string()),
        Some(" en-US ".to_string()),
        "Avatar group",
    );

    assert_eq!(normalized.max_visible, 4);
    assert_eq!(normalized.aria_label, "Avatar group");
    assert!(!normalized.has_custom_aria_label);
    assert_eq!(normalized.class_name.as_deref(), Some("custom"));
    assert!(normalized.has_custom_class_name);
    assert_eq!(normalized.lang.as_deref(), Some("en-US"));
}

#[test]
fn normalize_avatar_group_item_fields_centralizes_string_defaults() {
    let fields = normalize_avatar_group_item_fields(
        Some(" Ada ".to_string()),
        Some("   ".to_string()),
        None,
    );
    assert!(fields.has_name);
    assert!(!fields.has_src);
    assert!(!fields.has_alt);
    assert_eq!(fields.name, "Ada");
    assert_eq!(fields.src, "");
    assert_eq!(fields.alt, "");
}

#[test]
fn resolve_avatar_group_render_state_maps_discrete_status_and_sources_to_enums() {
    let overflow = resolve_avatar_group_render_state(AvatarGroupStateInput {
        total_count: 5,
        max_visible: 3,
        size: AvatarSize::Md,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });
    assert_eq!(
        overflow.visual_state,
        ui_state_primitives::avatar_group::AvatarGroupVisualState::Overflow
    );
    assert_eq!(
        overflow.aria_label_source,
        ui_state_primitives::avatar_group::AvatarGroupAriaLabelSource::Custom
    );
    assert_eq!(
        overflow.class_source,
        ui_state_primitives::avatar_group::AvatarGroupClassSource::Default
    );
    assert_eq!(overflow.visual_state.as_str(), "overflow");
    assert_eq!(overflow.aria_label_source.as_str(), "custom");
    assert_eq!(overflow.class_source.as_str(), "default");

    let stable = resolve_avatar_group_render_state(AvatarGroupStateInput {
        total_count: 1,
        max_visible: 3,
        size: AvatarSize::Sm,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });
    assert_eq!(
        stable.visual_state,
        ui_state_primitives::avatar_group::AvatarGroupVisualState::Stable
    );
    assert_eq!(
        stable.aria_label_source,
        ui_state_primitives::avatar_group::AvatarGroupAriaLabelSource::Default
    );
    assert_eq!(
        stable.class_source,
        ui_state_primitives::avatar_group::AvatarGroupClassSource::Custom
    );
    assert!(stable.has_items());

    let empty = resolve_avatar_group_render_state(AvatarGroupStateInput {
        total_count: 0,
        max_visible: 3,
        size: AvatarSize::Lg,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });
    assert_eq!(
        empty.visual_state,
        ui_state_primitives::avatar_group::AvatarGroupVisualState::Empty
    );
    assert!(!empty.has_items());
}
