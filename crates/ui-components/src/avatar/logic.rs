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
        state.size_class.into(),
        state.label_source_class.into(),
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

#[cfg(feature = "component-avatar_group")]
pub use ui_state_primitives::avatar_group::{AvatarGroupRenderState, AvatarGroupStateInput};

#[cfg(feature = "component-avatar_group")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AvatarGroupNormalizedInput {
    pub max_visible: usize,
    pub aria_label: String,
    pub has_custom_aria_label: bool,
    pub class_name: Option<String>,
    pub has_custom_class_name: bool,
    pub lang: Option<String>,
}

#[cfg(feature = "component-avatar_group")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AvatarGroupItemFields {
    pub name: String,
    pub src: String,
    pub alt: String,
    pub has_name: bool,
    pub has_src: bool,
    pub has_alt: bool,
}

#[cfg(feature = "component-avatar_group")]
pub fn normalize_avatar_group_optional_text(value: Option<String>) -> Option<String> {
    ui_state_primitives::avatar_group::normalize_optional_text(value)
}

#[cfg(feature = "component-avatar_group")]
pub fn normalize_avatar_group_max_visible(value: Option<usize>) -> usize {
    ui_state_primitives::avatar_group::normalize_max_visible(value)
}

#[cfg(feature = "component-avatar_group")]
pub fn resolve_avatar_group_aria_label(value: Option<String>) -> (String, bool) {
    ui_state_primitives::avatar_group::resolve_aria_label(value)
}

#[cfg(feature = "component-avatar_group")]
pub fn resolve_avatar_group_render_state(input: AvatarGroupStateInput) -> AvatarGroupRenderState {
    ui_state_primitives::avatar_group::resolve_render_state(input)
}

#[cfg(feature = "component-avatar_group")]
pub fn normalize_avatar_group_input(
    max: Option<usize>,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    default_aria_label: &str,
) -> AvatarGroupNormalizedInput {
    let max_visible = normalize_avatar_group_max_visible(max);
    let (aria_label, has_custom_aria_label) =
        resolve_avatar_group_aria_label_with_fallback(aria_label, default_aria_label);
    let class_name = normalize_avatar_group_optional_text(class_name);

    AvatarGroupNormalizedInput {
        max_visible,
        aria_label,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
        class_name,
        lang: normalize_avatar_group_optional_text(lang),
    }
}

#[cfg(feature = "component-avatar_group")]
pub fn resolve_avatar_group_aria_label_with_fallback(
    value: Option<String>,
    fallback: &str,
) -> (String, bool) {
    let (label, explicit) = resolve_avatar_group_aria_label(value);
    if explicit {
        (label, true)
    } else {
        (fallback.into(), false)
    }
}

#[cfg(feature = "component-avatar_group")]
pub fn normalize_avatar_group_item_fields(
    name: Option<String>,
    src: Option<String>,
    alt: Option<String>,
) -> AvatarGroupItemFields {
    let name = normalize_avatar_group_optional_text(name);
    let src = normalize_avatar_group_optional_text(src);
    let alt = normalize_avatar_group_optional_text(alt);

    AvatarGroupItemFields {
        has_name: name.is_some(),
        has_src: src.is_some(),
        has_alt: alt.is_some(),
        name: name.unwrap_or_default(),
        src: src.unwrap_or_default(),
        alt: alt.unwrap_or_default(),
    }
}

#[cfg(feature = "component-avatar_group")]
pub fn compose_avatar_group_class_name(
    base_class_name: Option<String>,
    state: AvatarGroupRenderState,
) -> String {
    let mut classes = vec![
        "ui-avatar-group".to_string(),
        format!("ui-avatar-group--size-{}", state.size_attr),
        state.visual_state.class_name().into(),
        state.aria_label_source.class_name().into(),
    ];

    if state.class_source.is_custom() {
        classes.push("ui-avatar-group--custom-class".to_string());
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

#[cfg(all(test, feature = "component-avatar_group"))]
mod avatar_group_tests {
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
}
