use std::borrow::Cow;

pub use ui_state_primitives::legend::{
    AccessibilityState, DEFAULT_REQUIRED_INDICATOR, DEFAULT_TEXT, LegendState, LegendStateInput,
    LegendTone, RequiredState, normalize_optional_text, normalize_required_indicator,
    normalize_text, resolve_state,
};

const DEFAULT_IS_REQUIRED: bool = false;
const DEFAULT_IS_DISABLED: bool = false;

pub fn normalize_required_state(is_required: Option<bool>) -> RequiredState {
    ui_state_primitives::legend::normalize_required_state(is_required, DEFAULT_IS_REQUIRED)
}

pub fn normalize_accessibility_state(is_disabled: Option<bool>) -> AccessibilityState {
    ui_state_primitives::legend::normalize_accessibility_state(is_disabled, DEFAULT_IS_DISABLED)
}

pub struct LegendNormalizeInput {
    pub tone: LegendTone,
    pub is_required: Option<bool>,
    pub is_disabled: Option<bool>,
    pub text: Option<String>,
    pub required_indicator: Option<String>,
    pub class_name: Option<String>,
}

pub struct LegendResolvedModel {
    pub state: LegendState,
    pub required_state: RequiredState,
    pub accessibility_state: AccessibilityState,
    pub text: String,
    pub required_indicator: String,
    pub class_name: Option<String>,
}

pub fn normalize_component_state(input: LegendNormalizeInput) -> LegendResolvedModel {
    let required_state = normalize_required_state(input.is_required);
    let accessibility_state = normalize_accessibility_state(input.is_disabled);
    let (text, has_custom_text) = normalize_text(input.text);
    let (required_indicator, has_custom_indicator) =
        normalize_required_indicator(input.required_indicator);
    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();
    let state = resolve_state(LegendStateInput {
        tone: input.tone,
        is_required: required_state.is_required,
        is_disabled: accessibility_state.is_disabled,
        has_custom_text,
        has_custom_indicator,
        has_custom_class_name,
    });

    LegendResolvedModel {
        state,
        required_state,
        accessibility_state,
        text,
        required_indicator,
        class_name,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendUiSource {
    Component,
}

impl LegendUiSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Component => "component",
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
    let mut classes: Vec<Cow<'static, str>> =
        vec![Cow::Borrowed("ui-legend"), Cow::Borrowed(state.tone_class)];

    if state.is_required {
        classes.push(Cow::Borrowed("ui-legend--required"));
    }

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-legend--disabled"));
    }

    if state.has_custom_text {
        classes.push(Cow::Borrowed("ui-legend--text-custom"));
    }

    if state.has_custom_indicator {
        classes.push(Cow::Borrowed("ui-legend--indicator-custom"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-legend--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .iter()
        .map(Cow::as_ref)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
