pub use crate::button::normalize_optional_text;

pub const DEFAULT_ARIA_LABEL: &str = "Avatar";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AvatarSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl AvatarSize {
    pub fn class_name(self) -> &'static str {
        match self {
            AvatarSize::Sm => "ui-avatar--sm",
            AvatarSize::Md => "ui-avatar--md",
            AvatarSize::Lg => "ui-avatar--lg",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            AvatarSize::Sm => "sm",
            AvatarSize::Md => "md",
            AvatarSize::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarLabelSource {
    Alt,
    Name,
    Fallback,
}

impl AvatarLabelSource {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Alt => "ui-avatar--label-alt",
            Self::Name => "ui-avatar--label-name",
            Self::Fallback => "ui-avatar--label-fallback",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Alt => "alt",
            Self::Name => "name",
            Self::Fallback => "fallback",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AvatarStateInput {
    pub size: AvatarSize,
    pub has_name: bool,
    pub has_src: bool,
    pub has_alt: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AvatarState {
    pub size: AvatarSize,
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub has_name: bool,
    pub has_src: bool,
    pub has_alt: bool,
    pub has_custom_class_name: bool,
    pub label_source: AvatarLabelSource,
    pub label_source_class: &'static str,
    pub label_source_attr: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AvatarAccessibility {
    pub aria_label: String,
    pub img_alt: String,
    pub title: Option<String>,
    pub label_source: AvatarLabelSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AvatarImageRenderInput {
    pub has_src: bool,
    pub has_img_error: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarRenderMode {
    Image,
    Fallback,
}

impl AvatarRenderMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Fallback => "fallback",
        }
    }

    pub fn image_attr(self) -> Option<&'static str> {
        matches!(self, Self::Image).then_some("true")
    }

    pub fn fallback_attr(self) -> Option<&'static str> {
        matches!(self, Self::Fallback).then_some("true")
    }

    pub fn shows_image(self) -> bool {
        matches!(self, Self::Image)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AvatarImageRenderState {
    pub mode: AvatarRenderMode,
}

fn initials_from_name(name: &str) -> Option<String> {
    let name = name.trim();
    if name.is_empty() {
        return None;
    }

    let mut words = name.split_whitespace().filter(|w| !w.is_empty());
    let first = words.next()?;
    let last = words.next_back().unwrap_or(first);

    let first_char = first.chars().next()?;
    let last_char = (last != first).then(|| last.chars().next()).flatten();

    let mut initials = String::new();
    initials.push(first_char);
    if let Some(last_char) = last_char {
        initials.push(last_char);
    }

    Some(initials.to_uppercase())
}

pub fn resolve_initials(name: Option<&str>) -> String {
    name.and_then(initials_from_name)
        .unwrap_or_else(|| "?".to_string())
}

pub fn resolve_accessibility(name: Option<&str>, alt: Option<&str>) -> AvatarAccessibility {
    let title = name.map(str::to_string);

    if let Some(alt) = alt {
        return AvatarAccessibility {
            aria_label: alt.into(),
            img_alt: alt.into(),
            title,
            label_source: AvatarLabelSource::Alt,
        };
    }

    if let Some(name) = name {
        return AvatarAccessibility {
            aria_label: name.into(),
            img_alt: name.into(),
            title,
            label_source: AvatarLabelSource::Name,
        };
    }

    AvatarAccessibility {
        aria_label: DEFAULT_ARIA_LABEL.into(),
        img_alt: String::new(),
        title,
        label_source: AvatarLabelSource::Fallback,
    }
}

pub fn resolve_state(input: AvatarStateInput) -> AvatarState {
    let label_source = if input.has_alt {
        AvatarLabelSource::Alt
    } else if input.has_name {
        AvatarLabelSource::Name
    } else {
        AvatarLabelSource::Fallback
    };

    AvatarState {
        size: input.size,
        size_class: input.size.class_name(),
        size_attr: input.size.as_str(),
        has_name: input.has_name,
        has_src: input.has_src,
        has_alt: input.has_alt,
        has_custom_class_name: input.has_custom_class_name,
        label_source,
        label_source_class: label_source.class_name(),
        label_source_attr: label_source.as_str(),
    }
}

pub fn resolve_image_render_state(input: AvatarImageRenderInput) -> AvatarImageRenderState {
    AvatarImageRenderState {
        mode: if input.has_src && !input.has_img_error {
            AvatarRenderMode::Image
        } else {
            AvatarRenderMode::Fallback
        },
    }
}

#[cfg(test)]
#[path = "test/avatar.rs"]
mod tests;
