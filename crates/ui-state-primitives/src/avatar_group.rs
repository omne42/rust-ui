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
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub is_empty: bool,
    pub has_items: bool,
    pub has_overflow: bool,
    pub has_custom_aria_label: bool,
    pub aria_label_source_class: &'static str,
    pub aria_label_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub class_source_attr: &'static str,
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

    pub fn class_name(self) -> &'static str {
        match self {
            Self::Stable => "ui-avatar-group--stable",
            Self::Overflow => "ui-avatar-group--overflow",
            Self::Empty => "ui-avatar-group--empty",
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

    pub fn class_name(self) -> &'static str {
        match self {
            Self::Default => "ui-avatar-group--label-source-default",
            Self::Custom => "ui-avatar-group--label-source-custom",
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
    let is_empty = input.total_count == 0;
    let has_overflow = overflow_count > 0;

    let (state_class, state_attr) = if has_overflow {
        ("ui-avatar-group--overflow", "overflow")
    } else if is_empty {
        ("ui-avatar-group--empty", "empty")
    } else {
        ("ui-avatar-group--stable", "stable")
    };

    let (aria_label_source_class, aria_label_source_attr) = if input.has_custom_aria_label {
        ("ui-avatar-group--label-source-custom", "custom")
    } else {
        ("ui-avatar-group--label-source-default", "default")
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    AvatarGroupState {
        total_count: input.total_count,
        visible_count,
        overflow_count,
        max_visible: input.max_visible,
        size: input.size,
        size_attr: input.size.as_str(),
        state_class,
        state_attr,
        is_empty,
        has_items: input.total_count > 0,
        has_overflow,
        has_custom_aria_label: input.has_custom_aria_label,
        aria_label_source_class,
        aria_label_source_attr,
        has_custom_class_name: input.has_custom_class_name,
        class_source_attr,
    }
}

pub fn resolve_render_state(input: AvatarGroupStateInput) -> AvatarGroupRenderState {
    let state = resolve_state(input);

    let visual_state = if state.has_overflow {
        AvatarGroupVisualState::Overflow
    } else if state.is_empty {
        AvatarGroupVisualState::Empty
    } else {
        AvatarGroupVisualState::Stable
    };
    let aria_label_source = if state.has_custom_aria_label {
        AvatarGroupAriaLabelSource::Custom
    } else {
        AvatarGroupAriaLabelSource::Default
    };
    let class_source = if state.has_custom_class_name {
        AvatarGroupClassSource::Custom
    } else {
        AvatarGroupClassSource::Default
    };

    AvatarGroupRenderState {
        total_count: state.total_count,
        visible_count: state.visible_count,
        overflow_count: state.overflow_count,
        max_visible: state.max_visible,
        size: state.size,
        size_attr: state.size_attr,
        visual_state,
        aria_label_source,
        class_source,
    }
}

#[cfg(test)]
#[path = "test/avatar_group.rs"]
mod tests;
