pub use crate::button::normalize_optional_text;

pub const DEFAULT_CLEAR_BUTTON_ARIA_LABEL: &str = "Clear search";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchFieldStateInput {
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub has_value: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchFieldState {
    pub show_clear_button: bool,
}

pub fn resolve_state(input: SearchFieldStateInput) -> SearchFieldState {
    SearchFieldState {
        show_clear_button: !input.is_disabled && !input.is_read_only && input.has_value,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchFieldControlMode {
    Controlled,
    Uncontrolled,
}

impl SearchFieldControlMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchFieldValueChangeSource {
    OnValueChange,
    None,
}

impl SearchFieldValueChangeSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::OnValueChange => "on_value_change",
            Self::None => "none",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchFieldValueAxisInput {
    pub is_controlled: bool,
    pub has_default_value: bool,
    pub has_on_value_change: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchFieldValueAxisState {
    pub is_controlled: bool,
    pub control_mode_attr: &'static str,
    pub default_value_source_attr: &'static str,
    pub value_change_source_attr: &'static str,
    pub has_value_change_handler: bool,
}

pub fn source_attr_from_presence(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_value_axis_state(input: SearchFieldValueAxisInput) -> SearchFieldValueAxisState {
    let control_mode = if input.is_controlled {
        SearchFieldControlMode::Controlled
    } else {
        SearchFieldControlMode::Uncontrolled
    };

    let value_change_source = if input.has_on_value_change {
        SearchFieldValueChangeSource::OnValueChange
    } else {
        SearchFieldValueChangeSource::None
    };

    SearchFieldValueAxisState {
        is_controlled: input.is_controlled,
        control_mode_attr: control_mode.as_attr(),
        default_value_source_attr: source_attr_from_presence(input.has_default_value),
        value_change_source_attr: value_change_source.as_attr(),
        has_value_change_handler: input.has_on_value_change,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchFieldSemanticStateKind {
    Ready,
    ReadOnly,
    Invalid,
    Disabled,
}

impl SearchFieldSemanticStateKind {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::ReadOnly => "readonly",
            Self::Invalid => "invalid",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchFieldValueKind {
    Empty,
    Filled,
}

impl SearchFieldValueKind {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Filled => "filled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchFieldRequirementKind {
    Optional,
    Required,
}

impl SearchFieldRequirementKind {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
            Self::Required => "required",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchFieldSemanticStateInput {
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub is_required: bool,
    pub has_value: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchFieldSemanticState {
    pub state_attr: &'static str,
    pub value_attr: &'static str,
    pub requirement_attr: &'static str,
}

pub fn resolve_semantic_state(input: SearchFieldSemanticStateInput) -> SearchFieldSemanticState {
    let state = if input.is_disabled {
        SearchFieldSemanticStateKind::Disabled
    } else if input.is_invalid {
        SearchFieldSemanticStateKind::Invalid
    } else if input.is_read_only {
        SearchFieldSemanticStateKind::ReadOnly
    } else {
        SearchFieldSemanticStateKind::Ready
    };

    let value = if input.has_value {
        SearchFieldValueKind::Filled
    } else {
        SearchFieldValueKind::Empty
    };

    let requirement = if input.is_required {
        SearchFieldRequirementKind::Required
    } else {
        SearchFieldRequirementKind::Optional
    };

    SearchFieldSemanticState {
        state_attr: state.as_attr(),
        value_attr: value.as_attr(),
        requirement_attr: requirement.as_attr(),
    }
}

#[cfg(test)]
#[path = "test/search_field.rs"]
mod tests;
