use std::sync::Arc;

pub use ui_state_primitives::chart::{
    ChartDomain, ChartKind, ChartPoint, ChartState, ChartStateInput, DEFAULT_ARIA_LABEL,
    DEFAULT_ID_BASE, bar_width, clamp_active_index, compose_class_name, default_active_index,
    normalize_aria_label, normalize_id_base, normalize_optional_text, normalize_points, point_x,
    point_y, polyline_points, resolve_state, value_domain,
};

#[derive(Clone, Debug)]
pub struct ChartNormalizedInput {
    pub id_base: String,
    pub class_name: Option<String>,
    pub aria_label: String,
    pub points: Arc<[ChartPoint]>,
    pub point_count: usize,
    pub domain: ChartDomain,
    pub default_active_index: usize,
}

#[derive(Clone, Debug)]
pub struct ChartInputBoundary {
    pub id_base: Option<String>,
    pub class_name: Option<String>,
    pub aria_label: Option<String>,
    pub points: Vec<ChartPoint>,
    pub default_active_index: Option<usize>,
}

pub fn normalize_input_boundary(input: ChartInputBoundary) -> ChartNormalizedInput {
    let id_base = normalize_id_base(input.id_base);
    let class_name = normalize_optional_text(input.class_name);
    let aria_label = normalize_aria_label(input.aria_label);

    let points: Arc<[ChartPoint]> = normalize_points(input.points).into();
    let point_count = points.len();
    let domain = value_domain(points.as_ref());
    let default_active_index = default_active_index(point_count, input.default_active_index);

    ChartNormalizedInput {
        id_base,
        class_name,
        aria_label,
        points,
        point_count,
        domain,
        default_active_index,
    }
}

pub fn resolve_id_base(id_base: Option<String>, generated_id_base: String) -> String {
    normalize_optional_text(id_base).unwrap_or(generated_id_base)
}

pub fn resolve_aria_label_with_fallback(
    aria_label: Option<String>,
    i18n_aria_label: Option<String>,
) -> String {
    normalize_aria_label(aria_label.or(i18n_aria_label))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChartActiveValueSource {
    #[default]
    Default,
    External,
    Interaction,
}

impl ChartActiveValueSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::External => "external",
            Self::Interaction => "interaction",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChartInteractionSource {
    #[default]
    None,
    Focus,
    Pointer,
    Keyboard,
}

impl ChartInteractionSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Focus => "focus",
            Self::Pointer => "pointer",
            Self::Keyboard => "keyboard",
        }
    }
}

pub const fn initial_active_value_source(is_controlled: bool) -> ChartActiveValueSource {
    if is_controlled {
        ChartActiveValueSource::External
    } else {
        ChartActiveValueSource::Default
    }
}

pub const fn interaction_active_value_source(is_controlled: bool) -> ChartActiveValueSource {
    if is_controlled {
        ChartActiveValueSource::External
    } else {
        ChartActiveValueSource::Interaction
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChartMotionSource {
    #[default]
    Default,
    Custom,
}

impl ChartMotionSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

pub const fn resolve_motion_source(has_custom_motion: bool) -> ChartMotionSource {
    if has_custom_motion {
        ChartMotionSource::Custom
    } else {
        ChartMotionSource::Default
    }
}

pub const CHART_AGENT_SCHEMA: &str = "ui.chart.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartAgentSchemaVersion {
    V1,
}

impl ChartAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartAgentIntent {
    ChartInteraction,
}

impl ChartAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChartInteraction => "chart.interaction",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartAgentAction {
    NavigateActivate,
}

impl ChartAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NavigateActivate => "navigate.activate",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartAgentKind {
    Bar,
    Line,
}

impl ChartAgentKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bar => "bar",
            Self::Line => "line",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartAgentState {
    DisabledEmpty,
    Disabled,
    Empty,
    Ready,
}

impl ChartAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisabledEmpty => "disabled-empty",
            Self::Disabled => "disabled",
            Self::Empty => "empty",
            Self::Ready => "ready",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartAgentSource {
    StatePrimitives,
}

impl ChartAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatePrimitives => "state-primitives",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartAgentStreamSupport {
    Optional,
}

impl ChartAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartAgentStreamFallback {
    Snapshot,
}

impl ChartAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartAgentOutputStatus {
    Verified,
}

impl ChartAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChartAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ChartAgentSchemaVersion,
    pub intent: ChartAgentIntent,
    pub action: ChartAgentAction,
    pub kind: ChartAgentKind,
    pub state: ChartAgentState,
    pub source: ChartAgentSource,
    pub stream_support: ChartAgentStreamSupport,
    pub stream_fallback: ChartAgentStreamFallback,
    pub output_status: ChartAgentOutputStatus,
    pub active_value_source: ChartActiveValueSource,
    pub interaction_source: ChartInteractionSource,
    pub motion_source: ChartMotionSource,
    pub config_policy: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChartAgentContractInput {
    pub state: ChartState,
    pub active_value_source: ChartActiveValueSource,
    pub interaction_source: ChartInteractionSource,
    pub motion_source: ChartMotionSource,
}

pub fn resolve_agent_contract(input: ChartAgentContractInput) -> ChartAgentContract {
    ChartAgentContract {
        schema_name: CHART_AGENT_SCHEMA,
        schema_version: ChartAgentSchemaVersion::V1,
        intent: ChartAgentIntent::ChartInteraction,
        action: ChartAgentAction::NavigateActivate,
        kind: match input.state.kind {
            ChartKind::Bar => ChartAgentKind::Bar,
            ChartKind::Line => ChartAgentKind::Line,
        },
        state: match input.state.state_attr {
            "disabled-empty" => ChartAgentState::DisabledEmpty,
            "disabled" => ChartAgentState::Disabled,
            "empty" => ChartAgentState::Empty,
            _ => ChartAgentState::Ready,
        },
        source: ChartAgentSource::StatePrimitives,
        stream_support: ChartAgentStreamSupport::Optional,
        stream_fallback: ChartAgentStreamFallback::Snapshot,
        output_status: ChartAgentOutputStatus::Verified,
        active_value_source: input.active_value_source,
        interaction_source: input.interaction_source,
        motion_source: input.motion_source,
        config_policy: "whitelist-only",
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChartStateBoundary {
    pub kind: ChartKind,
    pub point_count: usize,
    pub active_index: usize,
    pub is_disabled: bool,
    pub is_show_grid: bool,
    pub is_controlled: bool,
    pub has_custom_class_name: bool,
}

pub fn derive_state_from_boundary(input: ChartStateBoundary) -> ChartState {
    resolve_state(ChartStateInput {
        kind: input.kind,
        point_count: input.point_count,
        active_index: input.active_index,
        disabled: input.is_disabled,
        show_grid: input.is_show_grid,
        is_controlled: input.is_controlled,
        has_custom_class_name: input.has_custom_class_name,
    })
}

pub fn normalize_interaction_index(
    index: usize,
    point_count: usize,
    is_disabled: bool,
) -> Option<usize> {
    if is_disabled || point_count == 0 {
        return None;
    }

    Some(clamp_active_index(index, point_count))
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
