use leptos::prelude::*;

pub use ui_logic_calendar::time_field::{
    DEFAULT_ARIA_LABEL, DEFAULT_CLEAR_ARIA_LABEL, DEFAULT_CLEAR_LABEL, DEFAULT_HOUR_ARIA_LABEL,
    DEFAULT_LABEL, DEFAULT_MINUTE_ARIA_LABEL, DEFAULT_PLACEHOLDER, TimeFieldIds, TimeFieldState,
    TimeFieldStateInput, TimeFieldTone, compose_class_name, normalize_aria_label,
    normalize_clear_aria_label, normalize_clear_label, normalize_hour_aria_label, normalize_label,
    normalize_minute_aria_label, normalize_minute_step, normalize_optional_text,
    normalize_placeholder, normalize_time_value, resolve_ids, resolve_input_placeholders,
    resolve_state,
};

pub struct DisabledStateInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
}

pub fn normalize_disabled_state(input: DisabledStateInput) -> bool {
    input.is_disabled.unwrap_or(input.disabled)
}

pub struct ValueStateInput {
    pub value: Option<Signal<Option<String>>>,
    pub default_value: Option<String>,
    pub on_value_change: Option<Callback<Option<String>>>,
    pub minute_step: u8,
}

pub struct ValueState {
    pub value: Option<Signal<Option<String>>>,
    pub default_value: Option<String>,
    pub on_value_change: Option<Callback<Option<String>>>,
    pub is_controlled: bool,
    pub has_default_value: bool,
    pub has_value_change_handler: bool,
}

pub fn normalize_value_state(input: ValueStateInput) -> ValueState {
    let has_default_value = input.default_value.is_some();
    let has_value_change_handler = input.on_value_change.is_some();
    let minute_step = normalize_minute_step(input.minute_step);
    let default_value = normalize_time_value(input.default_value, minute_step);
    let is_controlled = input.value.is_some();

    ValueState {
        value: input.value,
        default_value,
        on_value_change: input.on_value_change,
        is_controlled,
        has_default_value,
        has_value_change_handler,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeFieldAgentSchemaVersion {
    V1,
}

impl TimeFieldAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeFieldAgentIntent {
    TimeInput,
}

impl TimeFieldAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TimeInput => "time-input",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeFieldAgentAction {
    Initialize,
    EditHour,
    EditMinute,
    Clear,
}

impl TimeFieldAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::EditHour => "edit-hour",
            Self::EditMinute => "edit-minute",
            Self::Clear => "clear",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeFieldAgentStateAxis {
    Empty,
    Filled,
    Disabled,
}

impl TimeFieldAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Filled => "filled",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeFieldAgentSource {
    Init,
    HourInput,
    MinuteInput,
    ClearPress,
}

impl TimeFieldAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::HourInput => "hour-input",
            Self::MinuteInput => "minute-input",
            Self::ClearPress => "clear-press",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeFieldAgentOutputStatus {
    Verified,
    Submittable,
}

impl TimeFieldAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeFieldAgentStreamSupport {
    Unsupported,
}

impl TimeFieldAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeFieldAgentStreamFallback {
    FullSnapshot,
}

impl TimeFieldAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSnapshot => "full-snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeFieldAgentCapabilities {
    pub can_edit: bool,
    pub can_clear: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeFieldAgentContract {
    pub schema_name: &'static str,
    pub schema_version: TimeFieldAgentSchemaVersion,
    pub intent: TimeFieldAgentIntent,
    pub action: TimeFieldAgentAction,
    pub state: TimeFieldAgentStateAxis,
    pub source: TimeFieldAgentSource,
    pub output_status: TimeFieldAgentOutputStatus,
    pub stream_support: TimeFieldAgentStreamSupport,
    pub stream_fallback: TimeFieldAgentStreamFallback,
    pub capabilities: TimeFieldAgentCapabilities,
}

pub fn resolve_agent_action(source: TimeFieldAgentSource) -> TimeFieldAgentAction {
    match source {
        TimeFieldAgentSource::Init => TimeFieldAgentAction::Initialize,
        TimeFieldAgentSource::HourInput => TimeFieldAgentAction::EditHour,
        TimeFieldAgentSource::MinuteInput => TimeFieldAgentAction::EditMinute,
        TimeFieldAgentSource::ClearPress => TimeFieldAgentAction::Clear,
    }
}

pub fn resolve_agent_state_axis(state: TimeFieldState) -> TimeFieldAgentStateAxis {
    if state.is_disabled {
        TimeFieldAgentStateAxis::Disabled
    } else if state.has_value {
        TimeFieldAgentStateAxis::Filled
    } else {
        TimeFieldAgentStateAxis::Empty
    }
}

pub fn resolve_agent_output_status(source: TimeFieldAgentSource) -> TimeFieldAgentOutputStatus {
    match source {
        TimeFieldAgentSource::Init => TimeFieldAgentOutputStatus::Verified,
        TimeFieldAgentSource::HourInput
        | TimeFieldAgentSource::MinuteInput
        | TimeFieldAgentSource::ClearPress => TimeFieldAgentOutputStatus::Submittable,
    }
}

pub fn resolve_agent_contract(
    state: TimeFieldState,
    source: TimeFieldAgentSource,
) -> TimeFieldAgentContract {
    TimeFieldAgentContract {
        schema_name: "ui.time-field.agent-contract",
        schema_version: TimeFieldAgentSchemaVersion::V1,
        intent: TimeFieldAgentIntent::TimeInput,
        action: resolve_agent_action(source),
        state: resolve_agent_state_axis(state),
        source,
        output_status: resolve_agent_output_status(source),
        stream_support: TimeFieldAgentStreamSupport::Unsupported,
        stream_fallback: TimeFieldAgentStreamFallback::FullSnapshot,
        capabilities: TimeFieldAgentCapabilities {
            can_edit: !state.is_disabled,
            can_clear: !state.is_disabled && state.has_value,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_disabled_state_prefers_is_disabled_alias() {
        assert!(normalize_disabled_state(DisabledStateInput {
            is_disabled: Some(true),
            disabled: false,
        }));
        assert!(!normalize_disabled_state(DisabledStateInput {
            is_disabled: None,
            disabled: false,
        }));
        assert!(normalize_disabled_state(DisabledStateInput {
            is_disabled: None,
            disabled: true,
        }));
    }

    #[test]
    fn normalize_value_state_centralizes_default_value_normalization() {
        let normalized = normalize_value_state(ValueStateInput {
            value: None,
            default_value: Some(" 9:17 ".to_string()),
            on_value_change: None,
            minute_step: 5,
        });

        assert_eq!(normalized.default_value, Some("09:15".to_string()));
        assert!(!normalized.is_controlled);
        assert!(normalized.has_default_value);
        assert!(!normalized.has_value_change_handler);
    }

    #[test]
    fn normalize_value_state_marks_controlled_axis_when_value_signal_exists() {
        let (value, _) = signal(Some("09:30".to_string()));
        let normalized = normalize_value_state(ValueStateInput {
            value: Some(value.into()),
            default_value: Some("08:00".to_string()),
            on_value_change: None,
            minute_step: 15,
        });

        assert!(normalized.is_controlled);
        assert_eq!(normalized.default_value, Some("08:00".to_string()));
        assert!(normalized.has_default_value);
        assert!(!normalized.has_value_change_handler);
    }

    #[test]
    fn normalize_value_state_clamps_invalid_step_and_value_into_testable_state() {
        let normalized = normalize_value_state(ValueStateInput {
            value: None,
            default_value: Some("10:59".to_string()),
            on_value_change: None,
            minute_step: 0,
        });

        assert_eq!(normalized.default_value, Some("10:59".to_string()));

        let normalized = normalize_value_state(ValueStateInput {
            value: None,
            default_value: Some("10:59".to_string()),
            on_value_change: None,
            minute_step: 60,
        });
        assert_eq!(normalized.default_value, Some("10:30".to_string()));
    }

    #[test]
    fn resolve_agent_contract_tracks_state_source_and_capabilities() {
        let state = resolve_state(TimeFieldStateInput {
            tone: TimeFieldTone::Default,
            disabled: false,
            is_controlled: true,
            has_default_value: false,
            has_value_change_handler: true,
            has_value: true,
            minute_step: 15,
            has_custom_label: false,
            has_custom_placeholder: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
            has_custom_motion: false,
        });

        let contract = resolve_agent_contract(state, TimeFieldAgentSource::HourInput);
        assert_eq!(contract.schema_name, "ui.time-field.agent-contract");
        assert_eq!(contract.schema_version.as_str(), "1");
        assert_eq!(contract.intent.as_str(), "time-input");
        assert_eq!(contract.action.as_str(), "edit-hour");
        assert_eq!(contract.state.as_str(), "filled");
        assert_eq!(contract.source.as_str(), "hour-input");
        assert_eq!(contract.output_status.as_str(), "submittable");
        assert_eq!(contract.stream_support.as_str(), "unsupported");
        assert_eq!(contract.stream_fallback.as_str(), "full-snapshot");
        assert!(contract.capabilities.can_edit);
        assert!(contract.capabilities.can_clear);
    }

    #[test]
    fn resolve_agent_contract_disabled_maps_to_disabled_state_axis() {
        let state = resolve_state(TimeFieldStateInput {
            tone: TimeFieldTone::Default,
            disabled: true,
            is_controlled: false,
            has_default_value: false,
            has_value_change_handler: false,
            has_value: true,
            minute_step: 5,
            has_custom_label: false,
            has_custom_placeholder: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
            has_custom_motion: false,
        });

        let contract = resolve_agent_contract(state, TimeFieldAgentSource::Init);
        assert_eq!(contract.state.as_str(), "disabled");
        assert_eq!(contract.output_status.as_str(), "verified");
        assert!(!contract.capabilities.can_edit);
        assert!(!contract.capabilities.can_clear);
    }
}
