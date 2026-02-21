use std::borrow::Cow;

pub use ui_state_primitives::color_loupe::{
    ColorLoupeState, ColorLoupeStateInput, DEFAULT_ARIA_LABEL, DEFAULT_COLOR,
    DEFAULT_POSITION_PERCENT, normalize_aria_label, normalize_optional_text, resolve_state,
    sanitize_color,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorLoupeLogicInput {
    pub is_open: bool,
    pub is_disabled: bool,
    pub has_color: bool,
    pub x_percent: Option<f32>,
    pub y_percent: Option<f32>,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorLoupeOutputState {
    Draft,
    Verified,
    Committable,
}

impl ColorLoupeOutputState {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::Committable => "committable",
        }
    }
}

pub fn normalize_output_state(value: Option<ColorLoupeOutputState>) -> ColorLoupeOutputState {
    match value {
        Some(value) => value,
        None => ColorLoupeOutputState::Verified,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentContractIntent {
    Snapshot,
}

impl AgentContractIntent {
    fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentContractAction {
    Render,
}

impl AgentContractAction {
    fn as_attr(self) -> &'static str {
        match self {
            Self::Render => "render",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentContractState {
    Idle,
    Color,
    Open,
    Disabled,
}

impl AgentContractState {
    fn from_attr(value: &'static str) -> Self {
        match value {
            "idle" => Self::Idle,
            "color" => Self::Color,
            "open" => Self::Open,
            "disabled" => Self::Disabled,
            _ => Self::Idle,
        }
    }

    fn as_attr(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Color => "color",
            Self::Open => "open",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentContractSource {
    Default,
    Custom,
}

impl AgentContractSource {
    fn from_attr(value: &'static str) -> Self {
        match value {
            "custom" => Self::Custom,
            _ => Self::Default,
        }
    }

    fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentContractBucket {
    Start,
    Center,
    End,
}

impl AgentContractBucket {
    fn from_attr(value: &'static str) -> Self {
        match value {
            "start" => Self::Start,
            "end" => Self::End,
            _ => Self::Center,
        }
    }

    fn as_attr(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorLoupeAgentContract {
    pub intent: AgentContractIntent,
    pub action: AgentContractAction,
    pub state: AgentContractState,
    pub output_state: ColorLoupeOutputState,
    pub aria_source: AgentContractSource,
    pub class_source: AgentContractSource,
    pub x_bucket: AgentContractBucket,
    pub y_bucket: AgentContractBucket,
}

pub fn normalize_position_percent(value: Option<f32>) -> f32 {
    match value {
        Some(value) => value,
        None => DEFAULT_POSITION_PERCENT,
    }
}

pub fn resolve_component_state(input: ColorLoupeLogicInput) -> ColorLoupeState {
    resolve_state(ColorLoupeStateInput {
        open: input.is_open,
        disabled: input.is_disabled,
        has_color: input.has_color,
        x_percent: normalize_position_percent(input.x_percent),
        y_percent: normalize_position_percent(input.y_percent),
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
    })
}

pub fn compose_class_name(base_class_name: Option<String>, state: ColorLoupeState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-color-loupe"),
        Cow::Borrowed(state.x_bucket_class),
        Cow::Borrowed(state.y_bucket_class),
    ];

    if state.is_open {
        classes.push(Cow::Borrowed("ui-color-loupe--open"));
    }

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-color-loupe--disabled"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-color-loupe--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .iter()
        .map(Cow::as_ref)
        .collect::<Vec<&str>>()
        .join(" ")
}

pub fn resolve_agent_contract(
    state: ColorLoupeState,
    output_state: ColorLoupeOutputState,
) -> ColorLoupeAgentContract {
    ColorLoupeAgentContract {
        intent: AgentContractIntent::Snapshot,
        action: AgentContractAction::Render,
        state: AgentContractState::from_attr(state.data_state_attr),
        output_state,
        aria_source: AgentContractSource::from_attr(state.aria_source_attr),
        class_source: AgentContractSource::from_attr(state.class_source_attr),
        x_bucket: AgentContractBucket::from_attr(state.x_bucket_attr),
        y_bucket: AgentContractBucket::from_attr(state.y_bucket_attr),
    }
}

pub fn agent_contract_schema_attr(
    state: ColorLoupeState,
    output_state: ColorLoupeOutputState,
) -> String {
    let contract = resolve_agent_contract(state, output_state);
    format!(
        "v=1;intent={};action={};state={};output_state={};source=aria:{},class:{};x_bucket={};y_bucket={}",
        contract.intent.as_attr(),
        contract.action.as_attr(),
        contract.state.as_attr(),
        contract.output_state.as_attr(),
        contract.aria_source.as_attr(),
        contract.class_source.as_attr(),
        contract.x_bucket.as_attr(),
        contract.y_bucket.as_attr(),
    )
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
