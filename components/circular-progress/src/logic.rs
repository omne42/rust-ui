use std::borrow::Cow;

pub use ui_state_primitives::circular_progress::{
    CircularProgressState, CircularProgressStateInput, DEFAULT_ARIA_LABEL, normalize_optional_text,
    resolve_aria_label, resolve_state,
};

pub const CIRCULAR_PROGRESS_AGENT_SCHEMA: &str = "ui.circular-progress.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CircularProgressAgentSchemaVersion {
    V1,
}

impl CircularProgressAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CircularProgressAgentIntent {
    ProgressIndeterminate,
}

impl CircularProgressAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProgressIndeterminate => "progress.indeterminate",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CircularProgressAgentAction {
    Render,
}

impl CircularProgressAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Render => "render",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CircularProgressAgentState {
    Indeterminate,
}

impl CircularProgressAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Indeterminate => "indeterminate",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CircularProgressAgentSource {
    StatePrimitives,
}

impl CircularProgressAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatePrimitives => "state-primitives",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CircularProgressAgentContract {
    pub schema_name: &'static str,
    pub schema_version: CircularProgressAgentSchemaVersion,
    pub intent: CircularProgressAgentIntent,
    pub action: CircularProgressAgentAction,
    pub state: CircularProgressAgentState,
    pub source: CircularProgressAgentSource,
    pub size_source: &'static str,
    pub thickness_source: &'static str,
    pub label_source: &'static str,
    pub class_source: &'static str,
}

#[derive(Debug)]
pub struct CircularProgressLogicInput<'a> {
    pub aria_label: Option<String>,
    pub size_px: Option<f64>,
    pub thickness_px: Option<f64>,
    pub class_name: Option<String>,
    pub lang: Option<String>,
    pub default_aria_label: &'a str,
}

#[derive(Debug)]
pub struct CircularProgressLogicOutput {
    pub state: CircularProgressState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub class: String,
    pub style_vars: Option<String>,
    pub agent_contract: CircularProgressAgentContract,
}

fn resolve_default_aria_label(default_aria_label: &str) -> &str {
    let normalized = default_aria_label.trim();
    if normalized.is_empty() {
        DEFAULT_ARIA_LABEL
    } else {
        normalized
    }
}

pub fn resolve_agent_contract(state: &CircularProgressState) -> CircularProgressAgentContract {
    CircularProgressAgentContract {
        schema_name: CIRCULAR_PROGRESS_AGENT_SCHEMA,
        schema_version: CircularProgressAgentSchemaVersion::V1,
        intent: CircularProgressAgentIntent::ProgressIndeterminate,
        action: CircularProgressAgentAction::Render,
        state: CircularProgressAgentState::Indeterminate,
        source: CircularProgressAgentSource::StatePrimitives,
        size_source: state.size_source_attr,
        thickness_source: state.thickness_source_attr,
        label_source: state.label_source_attr,
        class_source: state.class_source_attr,
    }
}

pub fn resolve_component_contract(
    input: CircularProgressLogicInput<'_>,
) -> CircularProgressLogicOutput {
    let lang = normalize_optional_text(input.lang);
    let class_name = normalize_optional_text(input.class_name);
    let default_aria_label = resolve_default_aria_label(input.default_aria_label);
    let (aria_label, has_custom_aria_label) =
        resolve_aria_label(input.aria_label, default_aria_label);

    let state = resolve_state(CircularProgressStateInput {
        size_px: input.size_px,
        thickness_px: input.thickness_px,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class = compose_class_name(class_name, &state);
    let style_vars = compose_style_vars(&state);
    let agent_contract = resolve_agent_contract(&state);

    CircularProgressLogicOutput {
        state,
        aria_label,
        lang,
        class,
        style_vars,
        agent_contract,
    }
}

pub fn compose_style_vars(state: &CircularProgressState) -> Option<String> {
    let mut vars = Vec::new();

    if let Some(size_px) = state.size_px {
        vars.push(format!("--ui-cp-size: {size_px}px;"));
    }

    if let Some(thickness_px) = state.thickness_px {
        vars.push(format!("--ui-cp-thickness: {thickness_px}px;"));
    }

    (!vars.is_empty()).then(|| vars.join(" "))
}

pub fn compose_class_name(
    base_class_name: Option<String>,
    state: &CircularProgressState,
) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-circular-progress"),
        Cow::Borrowed("ui-circular-progress--state-indeterminate"),
    ];

    if state.has_custom_size {
        classes.push(Cow::Borrowed("ui-circular-progress--size-custom"));
    }

    if state.has_custom_thickness {
        classes.push(Cow::Borrowed("ui-circular-progress--thickness-custom"));
    }

    if state.has_custom_aria_label {
        classes.push(Cow::Borrowed("ui-circular-progress--label-custom"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-circular-progress--custom-class"));
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
