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
    SetValue,
    None,
}

impl SearchFieldValueChangeSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::OnValueChange => "on_value_change",
            Self::SetValue => "set_value",
            Self::None => "none",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchFieldValueAxisInput {
    pub is_controlled: bool,
    pub has_default_value: bool,
    pub has_on_value_change: bool,
    pub has_legacy_set_value: bool,
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
    } else if input.has_legacy_set_value {
        SearchFieldValueChangeSource::SetValue
    } else {
        SearchFieldValueChangeSource::None
    };

    SearchFieldValueAxisState {
        is_controlled: input.is_controlled,
        control_mode_attr: control_mode.as_attr(),
        default_value_source_attr: source_attr_from_presence(input.has_default_value),
        value_change_source_attr: value_change_source.as_attr(),
        has_value_change_handler: input.has_on_value_change || input.has_legacy_set_value,
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
mod tests {
    use super::*;

    #[test]
    fn resolve_state_shows_clear_button_only_for_editable_non_empty_values() {
        assert_eq!(
            resolve_state(SearchFieldStateInput {
                is_disabled: false,
                is_read_only: false,
                has_value: false,
            }),
            SearchFieldState {
                show_clear_button: false,
            }
        );

        assert_eq!(
            resolve_state(SearchFieldStateInput {
                is_disabled: false,
                is_read_only: false,
                has_value: true,
            }),
            SearchFieldState {
                show_clear_button: true,
            }
        );

        assert_eq!(
            resolve_state(SearchFieldStateInput {
                is_disabled: true,
                is_read_only: false,
                has_value: true,
            }),
            SearchFieldState {
                show_clear_button: false,
            }
        );

        assert_eq!(
            resolve_state(SearchFieldStateInput {
                is_disabled: false,
                is_read_only: true,
                has_value: true,
            }),
            SearchFieldState {
                show_clear_button: false,
            }
        );
    }

    #[test]
    fn resolve_value_axis_tracks_control_and_source_markers() {
        let state = resolve_value_axis_state(SearchFieldValueAxisInput {
            is_controlled: true,
            has_default_value: true,
            has_on_value_change: true,
            has_legacy_set_value: true,
        });

        assert!(state.is_controlled);
        assert_eq!(state.control_mode_attr, "controlled");
        assert_eq!(state.default_value_source_attr, "custom");
        assert_eq!(state.value_change_source_attr, "on_value_change");
        assert!(state.has_value_change_handler);
    }

    #[test]
    fn resolve_value_axis_falls_back_to_legacy_set_value_marker() {
        let state = resolve_value_axis_state(SearchFieldValueAxisInput {
            is_controlled: false,
            has_default_value: false,
            has_on_value_change: false,
            has_legacy_set_value: true,
        });

        assert!(!state.is_controlled);
        assert_eq!(state.control_mode_attr, "uncontrolled");
        assert_eq!(state.default_value_source_attr, "default");
        assert_eq!(state.value_change_source_attr, "set_value");
        assert!(state.has_value_change_handler);
    }

    #[test]
    fn resolve_value_axis_handles_read_only_callbacks() {
        let state = resolve_value_axis_state(SearchFieldValueAxisInput {
            is_controlled: false,
            has_default_value: false,
            has_on_value_change: false,
            has_legacy_set_value: false,
        });

        assert_eq!(state.value_change_source_attr, "none");
        assert!(!state.has_value_change_handler);
    }

    #[test]
    fn resolve_semantic_state_yields_closed_attr_sets() {
        let ready = resolve_semantic_state(SearchFieldSemanticStateInput {
            is_disabled: false,
            is_invalid: false,
            is_read_only: false,
            is_required: false,
            has_value: false,
        });
        assert_eq!(ready.state_attr, "ready");
        assert_eq!(ready.value_attr, "empty");
        assert_eq!(ready.requirement_attr, "optional");

        let invalid = resolve_semantic_state(SearchFieldSemanticStateInput {
            is_disabled: false,
            is_invalid: true,
            is_read_only: true,
            is_required: true,
            has_value: true,
        });
        assert_eq!(invalid.state_attr, "invalid");
        assert_eq!(invalid.value_attr, "filled");
        assert_eq!(invalid.requirement_attr, "required");

        let disabled = resolve_semantic_state(SearchFieldSemanticStateInput {
            is_disabled: true,
            is_invalid: true,
            is_read_only: true,
            is_required: true,
            has_value: true,
        });
        assert_eq!(disabled.state_attr, "disabled");
    }
}
