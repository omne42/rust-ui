use super::*;

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
    assert_eq!(fallback.aria_label, "Avatar");
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
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(AvatarStateInput {
            size: AvatarSize::Md,
            has_name: true,
            has_src: true,
            has_alt: true,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-avatar",
        "ui-avatar--md",
        "ui-avatar--has-name",
        "ui-avatar--has-src",
        "ui-avatar--has-alt",
        "ui-avatar--label-alt",
        "ui-avatar--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn normalize_input_centralizes_optional_defaults() {
    let normalized = normalize_input(
        Some("  Ada Lovelace ".to_string()),
        Some("   ".to_string()),
        Some(" profile ".to_string()),
        Some(" custom ".to_string()),
    );

    assert_eq!(normalized.name.as_deref(), Some("Ada Lovelace"));
    assert_eq!(normalized.src, None);
    assert_eq!(normalized.alt.as_deref(), Some("profile"));
    assert_eq!(normalized.class_name.as_deref(), Some("custom"));
    assert!(normalized.has_name);
    assert!(!normalized.has_src);
    assert!(normalized.has_alt);
    assert!(normalized.has_custom_class_name);
    assert_eq!(normalized.image_src, "");
}

#[test]
fn normalize_lang_filters_blank_value() {
    assert_eq!(
        normalize_lang(Some("  zh-CN ".to_string())),
        Some("zh-CN".to_string())
    );
    assert_eq!(normalize_lang(Some("   ".to_string())), None);
    assert_eq!(normalize_lang(None), None);
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

#[test]
fn resolve_aria_label_uses_i18n_fallback_only_for_fallback_label_source() {
    assert_eq!(
        resolve_aria_label(
            AvatarLabelSource::Fallback,
            "Avatar".to_string(),
            "头像".to_string()
        ),
        "头像"
    );

    assert_eq!(
        resolve_aria_label(
            AvatarLabelSource::Name,
            "Ada Lovelace".to_string(),
            "头像".to_string()
        ),
        "Ada Lovelace"
    );
}

#[test]
fn resolve_agent_contract_maps_intent_action_source_without_string_concat() {
    let image = resolve_agent_contract(AvatarLabelSource::Alt, AvatarRenderMode::Image);
    assert_eq!(image.schema, AVATAR_AGENT_SCHEMA);
    assert_eq!(image.intent, AvatarAgentIntent::DisplayIdentity);
    assert_eq!(image.action, AvatarAgentAction::ImageFallbackOnError);
    assert_eq!(image.source, AvatarAgentSource::Alt);
    assert_eq!(image.intent.as_str(), "display-identity");
    assert_eq!(image.action.as_str(), "image-fallback-on-error");
    assert_eq!(image.source.as_str(), "alt");

    let fallback = resolve_agent_contract(AvatarLabelSource::Fallback, AvatarRenderMode::Fallback);
    assert_eq!(fallback.schema, AVATAR_AGENT_SCHEMA);
    assert_eq!(fallback.intent, AvatarAgentIntent::DisplayIdentity);
    assert_eq!(fallback.action, AvatarAgentAction::PassiveFallback);
    assert_eq!(fallback.source, AvatarAgentSource::Fallback);
    assert_eq!(fallback.source.as_str(), "fallback");
}
