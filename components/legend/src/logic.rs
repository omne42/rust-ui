pub use ui_state_primitives::legend::{
    DEFAULT_REQUIRED_INDICATOR, DEFAULT_TEXT, LegendState, LegendStateInput, LegendTone,
    normalize_optional_text, normalize_required_indicator, normalize_text, resolve_state,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendRequiredSource {
    IsRequired,
    Required,
    Default,
}

impl LegendRequiredSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::IsRequired => "is_required",
            Self::Required => "required",
            Self::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendDisabledSource {
    IsDisabled,
    Disabled,
    Default,
}

impl LegendDisabledSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::IsDisabled => "is_disabled",
            Self::Disabled => "disabled",
            Self::Default => "default",
        }
    }
}

pub struct RequiredState {
    pub is_required: bool,
    pub required_source_attr: &'static str,
}

pub fn normalize_required_state(is_required: Option<bool>, required: bool) -> RequiredState {
    let source = if is_required.is_some() {
        LegendRequiredSource::IsRequired
    } else if required {
        LegendRequiredSource::Required
    } else {
        LegendRequiredSource::Default
    };

    RequiredState {
        is_required: is_required.unwrap_or(required),
        required_source_attr: source.as_attr(),
    }
}

pub struct AccessibilityState {
    pub is_disabled: bool,
    pub disabled_source_attr: &'static str,
}

pub fn normalize_accessibility_state(
    is_disabled: Option<bool>,
    disabled: bool,
) -> AccessibilityState {
    let source = if is_disabled.is_some() {
        LegendDisabledSource::IsDisabled
    } else if disabled {
        LegendDisabledSource::Disabled
    } else {
        LegendDisabledSource::Default
    };

    AccessibilityState {
        is_disabled: is_disabled.unwrap_or(disabled),
        disabled_source_attr: source.as_attr(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendAgentSchema {
    V1,
}

impl LegendAgentSchema {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "ui.legend.agent-contract.v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendAgentSchemaVersion {
    V1,
}

impl LegendAgentSchemaVersion {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendStreamSupport {
    Unsupported,
}

impl LegendStreamSupport {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendStreamFallback {
    Snapshot,
}

impl LegendStreamFallback {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendStreamMode {
    Snapshot,
}

impl LegendStreamMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendOutputStatus {
    Verified,
}

impl LegendOutputStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendIntent {
    DescribeFieldset,
}

impl LegendIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::DescribeFieldset => "describe-fieldset",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendUiAction {
    Idle,
}

impl LegendUiAction {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Idle => "idle",
        }
    }
}

pub struct LegendAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub stream_mode_attr: &'static str,
    pub output_status_attr: &'static str,
    pub intent_attr: &'static str,
}

pub fn resolve_agent_contract() -> LegendAgentContract {
    LegendAgentContract {
        schema_attr: LegendAgentSchema::V1.as_attr(),
        schema_version_attr: LegendAgentSchemaVersion::V1.as_attr(),
        stream_support_attr: LegendStreamSupport::Unsupported.as_attr(),
        stream_fallback_attr: LegendStreamFallback::Snapshot.as_attr(),
        stream_mode_attr: LegendStreamMode::Snapshot.as_attr(),
        output_status_attr: LegendOutputStatus::Verified.as_attr(),
        intent_attr: LegendIntent::DescribeFieldset.as_attr(),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: LegendState) -> String {
    let mut classes = vec!["ui-legend".to_string(), state.tone_class.into()];

    if state.is_required {
        classes.push("ui-legend--required".to_string());
    }

    if state.is_disabled {
        classes.push("ui-legend--disabled".to_string());
    }

    if state.has_custom_text {
        classes.push("ui-legend--text-custom".to_string());
    }

    if state.has_custom_indicator {
        classes.push("ui-legend--indicator-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-legend--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
