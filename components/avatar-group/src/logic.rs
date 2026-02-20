pub use ui_avatar::AvatarSize;
pub use ui_state_primitives::avatar_group::{AvatarGroupRenderState, AvatarGroupStateInput};

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

pub fn normalize_avatar_group_optional_text(value: Option<String>) -> Option<String> {
    ui_state_primitives::avatar_group::normalize_optional_text(value)
}

pub fn normalize_avatar_group_max_visible(value: Option<usize>) -> usize {
    ui_state_primitives::avatar_group::normalize_max_visible(value)
}

pub fn resolve_avatar_group_aria_label(value: Option<String>) -> (String, bool) {
    ui_state_primitives::avatar_group::resolve_aria_label(value)
}

pub fn resolve_avatar_group_render_state(input: AvatarGroupStateInput) -> AvatarGroupRenderState {
    ui_state_primitives::avatar_group::resolve_render_state(input)
}

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
#[path = "../test/logic.rs"]
mod tests;
