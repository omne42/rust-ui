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
            id: DEFAULT_ID.to_string(),
            id_source_attr: "default",
            has_custom_id: false,
        }
    } else {
        IdState {
            id: trimmed.to_string(),
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
mod tests {
    use super::*;

    #[test]
    fn normalize_value_axis_tracks_control_mode_and_sources() {
        let (value, _set_value) = signal(30.0_f64);
        let (_legacy_value, set_legacy_value) = signal(0.0_f64);
        let on_value_change = Callback::new(|_: f64| {});

        let normalized = normalize_value_axis(ValueAxisInput {
            value: Some(value.into()),
            default_value: Some(15.0),
            on_value_change: Some(on_value_change),
            set_value: Some(set_legacy_value),
            on_change: Some(Callback::new(|_: f64| {})),
        });

        assert_eq!(normalized.control_mode_attr, "controlled");
        assert_eq!(normalized.value_source_attr, "external");
        assert_eq!(normalized.default_value_source_attr, "custom");
        assert_eq!(normalized.value_change_source_attr, "on_value_change");

        let normalized = normalize_value_axis(ValueAxisInput {
            value: None,
            default_value: None,
            on_value_change: None,
            set_value: None,
            on_change: None,
        });

        assert_eq!(normalized.control_mode_attr, "uncontrolled");
        assert_eq!(normalized.value_source_attr, "default_value");
        assert_eq!(normalized.default_value, DEFAULT_MIN);
        assert_eq!(normalized.default_value_source_attr, "default");
        assert_eq!(normalized.value_change_source_attr, "none");
    }

    #[test]
    fn normalize_value_axis_supports_legacy_setter_fallback() {
        let (value, set_value) = signal(10.0_f64);
        let normalized = normalize_value_axis(ValueAxisInput {
            value: Some(value.into()),
            default_value: None,
            on_value_change: None,
            set_value: Some(set_value),
            on_change: None,
        });

        assert_eq!(normalized.value_change_source_attr, "set_value");
        let handler = normalized
            .on_value_change
            .expect("legacy setter should map to on_value_change");
        handler.run(77.0);
        assert_eq!(value.get_untracked(), 77.0);
    }

    #[test]
    fn normalize_accessibility_state_prefers_is_prefixed_input() {
        let state = normalize_accessibility_state(AccessibilityStateInput {
            is_disabled: Some(false),
            disabled: true,
        });
        assert!(!state.is_disabled);
        assert_eq!(state.disabled_source_attr, "is_disabled");

        let state = normalize_accessibility_state(AccessibilityStateInput {
            is_disabled: None,
            disabled: true,
        });
        assert!(state.is_disabled);
        assert_eq!(state.disabled_source_attr, "disabled");
    }

    #[test]
    fn normalize_id_resolves_default_and_custom_sources() {
        let default_id = normalize_id("  ".to_string());
        assert_eq!(default_id.id, DEFAULT_ID);
        assert_eq!(default_id.id_source_attr, "default");
        assert!(!default_id.has_custom_id);

        let custom_id = normalize_id(" docs-slider ".to_string());
        assert_eq!(custom_id.id, "docs-slider");
        assert_eq!(custom_id.id_source_attr, "custom");
        assert!(custom_id.has_custom_id);
    }

    #[test]
    fn resolve_agent_contract_uses_closed_set_markers() {
        let controlled = resolve_agent_contract(true);
        assert_eq!(controlled.schema_attr, "ui.slider.agent-contract.v1");
        assert_eq!(controlled.stream_support_attr, "unsupported");
        assert_eq!(controlled.stream_fallback_attr, "snapshot");
        assert_eq!(controlled.stream_mode_attr, "snapshot");
        assert_eq!(controlled.output_status_attr, "submittable");
        assert_eq!(controlled.intent_attr, "adjust-value");

        let snapshot_only = resolve_agent_contract(false);
        assert_eq!(snapshot_only.output_status_attr, "verified");
    }

    #[test]
    fn resolve_ui_action_is_closed_set_and_priority_ordered() {
        assert_eq!(resolve_ui_action(false, false).as_attr(), "idle");
        assert_eq!(resolve_ui_action(false, true).as_attr(), "focus");
        assert_eq!(resolve_ui_action(true, false).as_attr(), "press");
        assert_eq!(resolve_ui_action(true, true).as_attr(), "press");
    }
}
