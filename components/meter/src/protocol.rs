use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/meter`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MeterComponentSchemaVersion {
    #[default]
    V1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MeterComponentSpec {
    #[serde(default)]
    pub schema_version: MeterComponentSchemaVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeterAgentIntent {
    ProgressMeter,
}

impl MeterAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            MeterAgentIntent::ProgressMeter => "progress-meter",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeterAgentAction {
    Render,
}

impl MeterAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            MeterAgentAction::Render => "render",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeterAgentStatePhase {
    Determinate,
    Indeterminate,
}

impl MeterAgentStatePhase {
    pub const fn as_attr(self) -> &'static str {
        match self {
            MeterAgentStatePhase::Determinate => "determinate",
            MeterAgentStatePhase::Indeterminate => "indeterminate",
        }
    }

    pub const fn from_phase(phase: crate::logic::MeterPhase) -> Self {
        match phase {
            crate::logic::MeterPhase::Determinate => MeterAgentStatePhase::Determinate,
            crate::logic::MeterPhase::Indeterminate => MeterAgentStatePhase::Indeterminate,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeterAgentLabelSource {
    Default,
    Custom,
}

impl MeterAgentLabelSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            MeterAgentLabelSource::Default => "default",
            MeterAgentLabelSource::Custom => "custom",
        }
    }

    pub const fn from_has_custom(has_custom: bool) -> Self {
        if has_custom {
            MeterAgentLabelSource::Custom
        } else {
            MeterAgentLabelSource::Default
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeterAgentValueLabelSource {
    Auto,
    Custom,
}

impl MeterAgentValueLabelSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            MeterAgentValueLabelSource::Auto => "auto",
            MeterAgentValueLabelSource::Custom => "custom",
        }
    }

    pub const fn from_has_custom(has_custom: bool) -> Self {
        if has_custom {
            MeterAgentValueLabelSource::Custom
        } else {
            MeterAgentValueLabelSource::Auto
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeterAgentMotionSource {
    Default,
    Custom,
}

impl MeterAgentMotionSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            MeterAgentMotionSource::Default => "default",
            MeterAgentMotionSource::Custom => "custom",
        }
    }

    pub const fn from_has_custom(has_custom: bool) -> Self {
        if has_custom {
            MeterAgentMotionSource::Custom
        } else {
            MeterAgentMotionSource::Default
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeterAgentClassSource {
    Default,
    Custom,
}

impl MeterAgentClassSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            MeterAgentClassSource::Default => "default",
            MeterAgentClassSource::Custom => "custom",
        }
    }

    pub const fn from_has_custom(has_custom: bool) -> Self {
        if has_custom {
            MeterAgentClassSource::Custom
        } else {
            MeterAgentClassSource::Default
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeterAgentStreamMode {
    Snapshot,
}

impl MeterAgentStreamMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            MeterAgentStreamMode::Snapshot => "snapshot",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeterAgentOutputMode {
    Snapshot,
}

impl MeterAgentOutputMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            MeterAgentOutputMode::Snapshot => "snapshot",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeterAgentOutputStatus {
    Validated,
}

impl MeterAgentOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            MeterAgentOutputStatus::Validated => "validated",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeterAgentDataAttrs {
    pub schema: &'static str,
    pub intent: &'static str,
    pub action: &'static str,
    pub stream_mode: &'static str,
    pub output_mode: &'static str,
    pub output_status: &'static str,
    pub state_phase: &'static str,
    pub state_variant: &'static str,
    pub state_size: &'static str,
    pub source_label: &'static str,
    pub source_value_label: &'static str,
    pub source_motion: &'static str,
    pub source_class: &'static str,
}

pub const METER_AGENT_SCHEMA: &str = "ui.meter.agent-contract.v1";

pub fn agent_data_attrs(
    state: crate::logic::MeterState,
    phase: crate::logic::MeterPhase,
) -> MeterAgentDataAttrs {
    MeterAgentDataAttrs {
        schema: METER_AGENT_SCHEMA,
        intent: MeterAgentIntent::ProgressMeter.as_attr(),
        action: MeterAgentAction::Render.as_attr(),
        stream_mode: MeterAgentStreamMode::Snapshot.as_attr(),
        output_mode: MeterAgentOutputMode::Snapshot.as_attr(),
        output_status: MeterAgentOutputStatus::Validated.as_attr(),
        state_phase: MeterAgentStatePhase::from_phase(phase).as_attr(),
        state_variant: state.variant.as_str(),
        state_size: state.size.as_str(),
        source_label: MeterAgentLabelSource::from_has_custom(state.has_custom_aria_label).as_attr(),
        source_value_label: MeterAgentValueLabelSource::from_has_custom(
            state.has_custom_value_label,
        )
        .as_attr(),
        source_motion: MeterAgentMotionSource::from_has_custom(state.has_custom_motion).as_attr(),
        source_class: MeterAgentClassSource::from_has_custom(state.has_custom_class_name).as_attr(),
    }
}

#[cfg(test)]
#[path = "../test/protocol.rs"]
mod tests;
