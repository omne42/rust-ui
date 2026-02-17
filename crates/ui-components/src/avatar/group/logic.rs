#[cfg(test)]
use ui_state_primitives::avatar_group::{
    AvatarGroupAriaLabelSource, AvatarGroupClassSource, AvatarGroupVisualState, resolve_state,
};
pub use ui_state_primitives::avatar_group::{
    AvatarGroupRenderState, AvatarGroupStateInput, normalize_max_visible, normalize_optional_text,
    resolve_aria_label, resolve_render_state,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AvatarGroupNormalizedInput {
    pub max_visible: usize,
    pub aria_label: String,
    pub has_custom_aria_label: bool,
    pub class_name: Option<String>,
    pub has_custom_class_name: bool,
    pub lang: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AvatarGroupItemFields {
    pub name: String,
    pub src: String,
    pub alt: String,
    pub has_name: bool,
    pub has_src: bool,
    pub has_alt: bool,
}

pub fn normalize_group_input(
    max: Option<usize>,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    default_aria_label: &str,
) -> AvatarGroupNormalizedInput {
    let max_visible = normalize_max_visible(max);
    let (aria_label, has_custom_aria_label) =
        resolve_aria_label_with_fallback(aria_label, default_aria_label);
    let class_name = normalize_optional_text(class_name);

    AvatarGroupNormalizedInput {
        max_visible,
        aria_label,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
        class_name,
        lang: normalize_optional_text(lang),
    }
}

pub fn resolve_aria_label_with_fallback(value: Option<String>, fallback: &str) -> (String, bool) {
    let (label, explicit) = resolve_aria_label(value);
    if explicit {
        (label, true)
    } else {
        (fallback.to_string(), false)
    }
}

pub fn normalize_item_fields(
    name: Option<String>,
    src: Option<String>,
    alt: Option<String>,
) -> AvatarGroupItemFields {
    let name = normalize_optional_text(name);
    let src = normalize_optional_text(src);
    let alt = normalize_optional_text(alt);

    AvatarGroupItemFields {
        has_name: name.is_some(),
        has_src: src.is_some(),
        has_alt: alt.is_some(),
        name: name.unwrap_or_default(),
        src: src.unwrap_or_default(),
        alt: alt.unwrap_or_default(),
    }
}

pub fn compose_class_name(
    base_class_name: Option<String>,
    state: AvatarGroupRenderState,
) -> String {
    let mut classes = vec![
        "ui-avatar-group".to_string(),
        format!("ui-avatar-group--size-{}", state.size_attr),
        state.visual_state.class_name().to_string(),
        state.aria_label_source.class_name().to_string(),
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
    use crate::avatar::AvatarSize;

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
        assert_eq!(normalize_max_visible(None), 4);
        assert_eq!(normalize_max_visible(Some(3)), 3);
        assert_eq!(normalize_max_visible(Some(0)), 0);
    }

    #[test]
    fn resolve_aria_label_defaults_and_trims() {
        assert_eq!(
            resolve_aria_label(None),
            ("Avatar group".to_string(), false)
        );
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
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_render_state(AvatarGroupStateInput {
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
    fn resolve_aria_label_with_fallback_prefers_explicit_label() {
        assert_eq!(
            resolve_aria_label_with_fallback(Some(" Team ".to_string()), "Fallback"),
            ("Team".to_string(), true)
        );
        assert_eq!(
            resolve_aria_label_with_fallback(Some("  ".to_string()), "Fallback"),
            ("Fallback".to_string(), false)
        );
    }

    #[test]
    fn normalize_group_input_centralizes_defaults() {
        let normalized = normalize_group_input(
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
    fn normalize_item_fields_centralizes_string_defaults() {
        let fields =
            normalize_item_fields(Some(" Ada ".to_string()), Some("   ".to_string()), None);
        assert!(fields.has_name);
        assert!(!fields.has_src);
        assert!(!fields.has_alt);
        assert_eq!(fields.name, "Ada");
        assert_eq!(fields.src, "");
        assert_eq!(fields.alt, "");
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
}
