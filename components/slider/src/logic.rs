use leptos::prelude::*;
use ui_state_primitives::slider as slider_state;

pub use slider_state::{
    DEFAULT_LABEL, DEFAULT_MAX, DEFAULT_MIN, DEFAULT_STEP, SliderStateInput, compose_class_name,
    normalize_optional_text, resolve_label, resolve_state,
};

pub const DEFAULT_ID: &str = "ui-slider";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderControlMode {
    Controlled,
    Uncontrolled,
}

impl SliderControlMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderValueSource {
    External,
    DefaultValue,
}

impl SliderValueSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::External => "external",
            Self::DefaultValue => "default_value",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderValueChangeSource {
    OnValueChange,
    SetValue,
    OnChange,
    None,
}

impl SliderValueChangeSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::OnValueChange => "on_value_change",
            Self::SetValue => "set_value",
            Self::OnChange => "on_change",
            Self::None => "none",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderDisabledSource {
    IsDisabled,
    Disabled,
    Default,
}

impl SliderDisabledSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::IsDisabled => "is_disabled",
            Self::Disabled => "disabled",
            Self::Default => "default",
        }
    }
}

pub struct ValueAxisInput {
    pub value: Option<Signal<f64>>,
    pub default_value: Option<f64>,
    pub on_value_change: Option<Callback<f64>>,
    pub set_value: Option<WriteSignal<f64>>,
    pub on_change: Option<Callback<f64>>,
}

pub struct ValueAxisState {
    pub value: Option<Signal<f64>>,
    pub default_value: f64,
    pub on_value_change: Option<Callback<f64>>,
    pub control_mode_attr: &'static str,
    pub value_source_attr: &'static str,
    pub default_value_source_attr: &'static str,
    pub value_change_source_attr: &'static str,
}

pub fn normalize_default_value(default_value: Option<f64>) -> f64 {
    default_value.unwrap_or(DEFAULT_MIN)
}

pub fn normalize_on_value_change_handler(
    on_value_change: Option<Callback<f64>>,
    set_value: Option<WriteSignal<f64>>,
    on_change: Option<Callback<f64>>,
) -> Option<Callback<f64>> {
    on_value_change
        .or_else(|| set_value.map(|set_value| Callback::new(move |next| set_value.set(next))))
        .or(on_change)
}

pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState {
    let has_value = input.value.is_some();
    let has_default_value = input.default_value.is_some();
    let has_on_value_change = input.on_value_change.is_some();
    let has_set_value = input.set_value.is_some();
    let has_on_change = input.on_change.is_some();

    let control_mode = if has_value {
        SliderControlMode::Controlled
    } else {
        SliderControlMode::Uncontrolled
    };
    let value_source = if has_value {
        SliderValueSource::External
    } else {
        SliderValueSource::DefaultValue
    };
    let value_change_source = if has_on_value_change {
        SliderValueChangeSource::OnValueChange
    } else if has_set_value {
        SliderValueChangeSource::SetValue
    } else if has_on_change {
        SliderValueChangeSource::OnChange
    } else {
        SliderValueChangeSource::None
    };
    let on_value_change =
        normalize_on_value_change_handler(input.on_value_change, input.set_value, input.on_change);
    ValueAxisState {
        value: input.value,
        default_value: normalize_default_value(input.default_value),
        on_value_change,
        control_mode_attr: control_mode.as_attr(),
        value_source_attr: value_source.as_attr(),
        default_value_source_attr: slider_state::source_attr_from_presence(has_default_value),
        value_change_source_attr: value_change_source.as_attr(),
    }
}

pub struct AccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
}

pub struct AccessibilityState {
    pub is_disabled: bool,
    pub disabled_source_attr: &'static str,
}

pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState {
    let disabled_source = if input.is_disabled.is_some() {
        SliderDisabledSource::IsDisabled
    } else if input.disabled {
        SliderDisabledSource::Disabled
    } else {
        SliderDisabledSource::Default
    };

    AccessibilityState {
        is_disabled: input.is_disabled.unwrap_or(input.disabled),
        disabled_source_attr: disabled_source.as_attr(),
    }
}

pub struct IdState {
    pub id: String,
    pub id_source_attr: &'static str,
    pub has_custom_id: bool,
}

pub fn normalize_id(id: String) -> IdState {
    let trimmed = id.trim();
    if trimmed.is_empty() {
        IdState {
            id: DEFAULT_ID.into(),
            id_source_attr: "default",
            has_custom_id: false,
        }
    } else {
        IdState {
            id: trimmed.into(),
            id_source_attr: "custom",
            has_custom_id: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderAgentSchema {
    V1,
}

impl SliderAgentSchema {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "ui.slider.agent-contract.v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderStreamSupport {
    Unsupported,
}

impl SliderStreamSupport {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderStreamFallback {
    Snapshot,
}

impl SliderStreamFallback {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderStreamMode {
    Snapshot,
}

impl SliderStreamMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderOutputStatus {
    Verified,
    Submittable,
}

impl SliderOutputStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderIntent {
    AdjustValue,
}

impl SliderIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::AdjustValue => "adjust-value",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderUiAction {
    Idle,
    Focus,
    Press,
}

impl SliderUiAction {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Focus => "focus",
            Self::Press => "press",
        }
    }
}

pub fn resolve_ui_action(is_pressed: bool, is_focused: bool) -> SliderUiAction {
    if is_pressed {
        SliderUiAction::Press
    } else if is_focused {
        SliderUiAction::Focus
    } else {
        SliderUiAction::Idle
    }
}

pub struct SliderAgentContract {
    pub schema_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub stream_mode_attr: &'static str,
    pub output_status_attr: &'static str,
    pub intent_attr: &'static str,
}

pub fn resolve_agent_contract(has_value_change_handler: bool) -> SliderAgentContract {
    let output_status = if has_value_change_handler {
        SliderOutputStatus::Submittable
    } else {
        SliderOutputStatus::Verified
    };

    SliderAgentContract {
        schema_attr: SliderAgentSchema::V1.as_attr(),
        stream_support_attr: SliderStreamSupport::Unsupported.as_attr(),
        stream_fallback_attr: SliderStreamFallback::Snapshot.as_attr(),
        stream_mode_attr: SliderStreamMode::Snapshot.as_attr(),
        output_status_attr: output_status.as_attr(),
        intent_attr: SliderIntent::AdjustValue.as_attr(),
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
