#[cfg(test)]
use ui_state_primitives::avatar::AvatarRenderMode;
pub use ui_state_primitives::avatar::{
    AvatarImageRenderInput, AvatarLabelSource, AvatarSize, AvatarState, AvatarStateInput,
    normalize_optional_text, resolve_accessibility, resolve_image_render_state, resolve_initials,
    resolve_state,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AvatarNormalizedInput {
    pub name: Option<String>,
    pub src: Option<String>,
    pub alt: Option<String>,
    pub class_name: Option<String>,
    pub has_name: bool,
    pub has_src: bool,
    pub has_alt: bool,
    pub has_custom_class_name: bool,
    pub image_src: String,
}

pub fn normalize_lang(value: Option<String>) -> Option<String> {
    normalize_optional_text(value)
}

pub fn normalize_input(
    name: Option<String>,
    src: Option<String>,
    alt: Option<String>,
    class_name: Option<String>,
) -> AvatarNormalizedInput {
    let name = normalize_optional_text(name);
    let src = normalize_optional_text(src);
    let alt = normalize_optional_text(alt);
    let class_name = normalize_optional_text(class_name);
    let image_src = src.clone().unwrap_or_default();

    AvatarNormalizedInput {
        has_name: name.is_some(),
        has_src: src.is_some(),
        has_alt: alt.is_some(),
        has_custom_class_name: class_name.is_some(),
        name,
        src,
        alt,
        class_name,
        image_src,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: AvatarState) -> String {
    let mut classes = vec![
        "ui-avatar".to_string(),
        state.size_class.to_string(),
        state.label_source_class.to_string(),
    ];

    if state.has_name {
        classes.push("ui-avatar--has-name".to_string());
    }
    if state.has_src {
        classes.push("ui-avatar--has-src".to_string());
    }
    if state.has_alt {
        classes.push("ui-avatar--has-alt".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-avatar--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
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
}
