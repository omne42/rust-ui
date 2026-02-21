pub use ui_state_primitives::avatar::{
    AvatarImageRenderInput, AvatarLabelSource, AvatarRenderMode, AvatarSize, AvatarState,
    AvatarStateInput, normalize_optional_text, resolve_accessibility, resolve_image_render_state,
    resolve_initials, resolve_state,
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

pub const AVATAR_AGENT_SCHEMA: &str = "ui.avatar.agent.v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarAgentIntent {
    DisplayIdentity,
}

impl AvatarAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisplayIdentity => "display-identity",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarAgentAction {
    ImageFallbackOnError,
    PassiveFallback,
}

impl AvatarAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImageFallbackOnError => "image-fallback-on-error",
            Self::PassiveFallback => "passive-fallback",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarAgentSource {
    Name,
    Alt,
    Fallback,
}

impl AvatarAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::Alt => "alt",
            Self::Fallback => "fallback",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AvatarAgentContract {
    pub schema: &'static str,
    pub intent: AvatarAgentIntent,
    pub action: AvatarAgentAction,
    pub source: AvatarAgentSource,
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

pub fn resolve_aria_label(
    label_source: AvatarLabelSource,
    normalized_aria_label: String,
    fallback_aria_label: String,
) -> String {
    if label_source == AvatarLabelSource::Fallback {
        fallback_aria_label
    } else {
        normalized_aria_label
    }
}

pub fn resolve_agent_contract(
    label_source: AvatarLabelSource,
    render_mode: AvatarRenderMode,
) -> AvatarAgentContract {
    let action = if render_mode.shows_image() {
        AvatarAgentAction::ImageFallbackOnError
    } else {
        AvatarAgentAction::PassiveFallback
    };

    let source = match label_source {
        AvatarLabelSource::Name => AvatarAgentSource::Name,
        AvatarLabelSource::Alt => AvatarAgentSource::Alt,
        AvatarLabelSource::Fallback => AvatarAgentSource::Fallback,
    };

    AvatarAgentContract {
        schema: AVATAR_AGENT_SCHEMA,
        intent: AvatarAgentIntent::DisplayIdentity,
        action,
        source,
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

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
