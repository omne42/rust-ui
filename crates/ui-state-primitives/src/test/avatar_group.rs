use super::*;

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Team avatars  ".to_string())),
        Some("Team avatars".to_string())
    );
}

#[test]
fn normalize_max_visible_falls_back_to_default() {
    assert_eq!(normalize_max_visible(None), DEFAULT_MAX_VISIBLE);
    assert_eq!(normalize_max_visible(Some(3)), 3);
    assert_eq!(normalize_max_visible(Some(0)), 0);
}

#[test]
fn resolve_aria_label_defaults_and_trims() {
    assert_eq!(resolve_aria_label(None), (DEFAULT_ARIA_LABEL.into(), false));
    assert_eq!(
        resolve_aria_label(Some("  Team  ".to_string())),
        ("Team".to_string(), true)
    );
}

#[test]
fn resolve_state_tracks_overflow_and_size_metadata() {
    let state = resolve_state(AvatarGroupStateInput {
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
    assert_eq!(state.visual_state, AvatarGroupVisualState::Overflow);
    assert_eq!(state.visual_state.as_str(), "overflow");
    assert!(!state.is_empty());
    assert!(state.has_items());
    assert!(state.has_overflow());
    assert_eq!(state.aria_label_source, AvatarGroupAriaLabelSource::Custom);
    assert_eq!(state.aria_label_source.as_str(), "custom");
    assert_eq!(state.class_source, AvatarGroupClassSource::Custom);
    assert_eq!(state.class_source.as_str(), "custom");
}

#[test]
fn resolve_render_state_maps_discrete_status_and_sources_to_enums() {
    let overflow = resolve_render_state(AvatarGroupStateInput {
        total_count: 5,
        max_visible: 3,
        size: AvatarSize::Md,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });
    assert_eq!(overflow.visual_state, AvatarGroupVisualState::Overflow);
    assert_eq!(
        overflow.aria_label_source,
        AvatarGroupAriaLabelSource::Custom
    );
    assert_eq!(overflow.class_source, AvatarGroupClassSource::Default);
    assert_eq!(overflow.visual_state.as_str(), "overflow");
    assert_eq!(overflow.aria_label_source.as_str(), "custom");
    assert_eq!(overflow.class_source.as_str(), "default");

    let stable = resolve_render_state(AvatarGroupStateInput {
        total_count: 1,
        max_visible: 3,
        size: AvatarSize::Sm,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });
    assert_eq!(stable.visual_state, AvatarGroupVisualState::Stable);
    assert_eq!(
        stable.aria_label_source,
        AvatarGroupAriaLabelSource::Default
    );
    assert_eq!(stable.class_source, AvatarGroupClassSource::Custom);
    assert!(stable.has_items());

    let empty = resolve_render_state(AvatarGroupStateInput {
        total_count: 0,
        max_visible: 3,
        size: AvatarSize::Lg,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });
    assert_eq!(empty.visual_state, AvatarGroupVisualState::Empty);
    assert!(!empty.has_items());
}
