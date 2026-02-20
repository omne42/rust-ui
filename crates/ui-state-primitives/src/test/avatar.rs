use super::*;

#[test]
fn size_class_names_are_stable() {
    assert_eq!(AvatarSize::Sm.class_name(), "ui-avatar--sm");
    assert_eq!(AvatarSize::Md.class_name(), "ui-avatar--md");
    assert_eq!(AvatarSize::Lg.class_name(), "ui-avatar--lg");
}

#[test]
fn label_source_classes_and_attrs_are_stable() {
    assert_eq!(AvatarLabelSource::Alt.class_name(), "ui-avatar--label-alt");
    assert_eq!(
        AvatarLabelSource::Name.class_name(),
        "ui-avatar--label-name"
    );
    assert_eq!(
        AvatarLabelSource::Fallback.class_name(),
        "ui-avatar--label-fallback"
    );

    assert_eq!(AvatarLabelSource::Alt.as_str(), "alt");
    assert_eq!(AvatarLabelSource::Name.as_str(), "name");
    assert_eq!(AvatarLabelSource::Fallback.as_str(), "fallback");
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Ada Lovelace  ".to_string())),
        Some("Ada Lovelace".to_string())
    );
}

#[test]
fn resolve_initials_uses_first_and_last_words() {
    assert_eq!(resolve_initials(Some("Ada Lovelace")), "AL");
    assert_eq!(resolve_initials(Some("grace")), "G");
    assert_eq!(resolve_initials(Some("   ")), "?");
    assert_eq!(resolve_initials(None), "?");
}

#[test]
fn resolve_accessibility_prefers_alt_then_name_then_fallback() {
    let alt = resolve_accessibility(Some("Ada Lovelace"), Some("Profile photo"));
    assert_eq!(alt.aria_label, "Profile photo");
    assert_eq!(alt.img_alt, "Profile photo");
    assert_eq!(alt.label_source, AvatarLabelSource::Alt);

    let name = resolve_accessibility(Some("Ada Lovelace"), None);
    assert_eq!(name.aria_label, "Ada Lovelace");
    assert_eq!(name.img_alt, "Ada Lovelace");
    assert_eq!(name.label_source, AvatarLabelSource::Name);

    let fallback = resolve_accessibility(None, None);
    assert_eq!(fallback.aria_label, DEFAULT_ARIA_LABEL);
    assert_eq!(fallback.img_alt, "");
    assert_eq!(fallback.label_source, AvatarLabelSource::Fallback);
}

#[test]
fn resolve_state_tracks_size_source_and_flags() {
    let state = resolve_state(AvatarStateInput {
        size: AvatarSize::Lg,
        has_name: true,
        has_src: true,
        has_alt: false,
        has_custom_class_name: true,
    });

    assert_eq!(state.size, AvatarSize::Lg);
    assert_eq!(state.size_class, "ui-avatar--lg");
    assert_eq!(state.size_attr, "lg");
    assert!(state.has_name);
    assert!(state.has_src);
    assert!(!state.has_alt);
    assert!(state.has_custom_class_name);
    assert_eq!(state.label_source, AvatarLabelSource::Name);
    assert_eq!(state.label_source_class, "ui-avatar--label-name");
    assert_eq!(state.label_source_attr, "name");
}

#[test]
fn resolve_image_render_state_tracks_image_and_fallback_markers() {
    let image = resolve_image_render_state(AvatarImageRenderInput {
        has_src: true,
        has_img_error: false,
    });
    assert_eq!(image.mode, AvatarRenderMode::Image);
    assert!(image.mode.shows_image());
    assert_eq!(image.mode.as_str(), "image");
    assert_eq!(image.mode.image_attr(), Some("true"));
    assert_eq!(image.mode.fallback_attr(), None);

    let fallback = resolve_image_render_state(AvatarImageRenderInput {
        has_src: true,
        has_img_error: true,
    });
    assert_eq!(fallback.mode, AvatarRenderMode::Fallback);
    assert!(!fallback.mode.shows_image());
    assert_eq!(fallback.mode.as_str(), "fallback");
    assert_eq!(fallback.mode.image_attr(), None);
    assert_eq!(fallback.mode.fallback_attr(), Some("true"));
}
