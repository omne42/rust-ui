use crate::avatar::AvatarSize;

pub use crate::button::normalize_optional_text;

pub const DEFAULT_ARIA_LABEL: &str = "Avatar group";
pub const DEFAULT_MAX_VISIBLE: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AvatarGroupStateInput {
    pub total_count: usize,
    pub max_visible: usize,
    pub size: AvatarSize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AvatarGroupState {
    pub total_count: usize,
    pub visible_count: usize,
    pub overflow_count: usize,
    pub max_visible: usize,
    pub size: AvatarSize,
    pub size_attr: &'static str,
    pub visual_state: AvatarGroupVisualState,
    pub aria_label_source: AvatarGroupAriaLabelSource,
    pub class_source: AvatarGroupClassSource,
}

impl AvatarGroupState {
    pub fn is_empty(self) -> bool {
        self.visual_state.is_empty()
    }

    pub fn has_items(self) -> bool {
        self.total_count > 0
    }

    pub fn has_overflow(self) -> bool {
        self.visual_state.has_overflow()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarGroupVisualState {
    Stable,
    Overflow,
    Empty,
}

impl AvatarGroupVisualState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Overflow => "overflow",
            Self::Empty => "empty",
        }
    }

    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn has_overflow(self) -> bool {
        matches!(self, Self::Overflow)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarGroupAriaLabelSource {
    Default,
    Custom,
}

impl AvatarGroupAriaLabelSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }

    pub fn is_custom(self) -> bool {
        matches!(self, Self::Custom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarGroupClassSource {
    Default,
    Custom,
}

impl AvatarGroupClassSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }

    pub fn is_custom(self) -> bool {
        matches!(self, Self::Custom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AvatarGroupRenderState {
    pub total_count: usize,
    pub visible_count: usize,
    pub overflow_count: usize,
    pub max_visible: usize,
    pub size: AvatarSize,
    pub size_attr: &'static str,
    pub visual_state: AvatarGroupVisualState,
    pub aria_label_source: AvatarGroupAriaLabelSource,
    pub class_source: AvatarGroupClassSource,
}

impl AvatarGroupRenderState {
    pub fn has_items(self) -> bool {
        self.total_count > 0
    }
}

pub fn normalize_max_visible(value: Option<usize>) -> usize {
    value.unwrap_or(DEFAULT_MAX_VISIBLE)
}

pub fn resolve_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_state(input: AvatarGroupStateInput) -> AvatarGroupState {
    let visible_count = input.total_count.min(input.max_visible);
    let overflow_count = input.total_count.saturating_sub(visible_count);
    let visual_state = if overflow_count > 0 {
        AvatarGroupVisualState::Overflow
    } else if input.total_count == 0 {
        AvatarGroupVisualState::Empty
    } else {
        AvatarGroupVisualState::Stable
    };

    let aria_label_source = if input.has_custom_aria_label {
        AvatarGroupAriaLabelSource::Custom
    } else {
        AvatarGroupAriaLabelSource::Default
    };

    let class_source = if input.has_custom_class_name {
        AvatarGroupClassSource::Custom
    } else {
        AvatarGroupClassSource::Default
    };

    AvatarGroupState {
        total_count: input.total_count,
        visible_count,
        overflow_count,
        max_visible: input.max_visible,
        size: input.size,
        size_attr: input.size.as_str(),
        visual_state,
        aria_label_source,
        class_source,
    }
}

pub fn resolve_render_state(input: AvatarGroupStateInput) -> AvatarGroupRenderState {
    let state = resolve_state(input);

    AvatarGroupRenderState {
        total_count: state.total_count,
        visible_count: state.visible_count,
        overflow_count: state.overflow_count,
        max_visible: state.max_visible,
        size: state.size,
        size_attr: state.size_attr,
        visual_state: state.visual_state,
        aria_label_source: state.aria_label_source,
        class_source: state.class_source,
    }
}

#[cfg(test)]
#[path = "test/avatar_group.rs"]
mod tests;
