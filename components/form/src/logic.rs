use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FormLabelPosition {
    #[default]
    Top,
    Left,
}

impl FormLabelPosition {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormLabelPosition::Top => "top",
            FormLabelPosition::Left => "left",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FormLabelAlign {
    #[default]
    Start,
    End,
}

impl FormLabelAlign {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormLabelAlign::Start => "start",
            FormLabelAlign::End => "end",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormContextValue {
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub label_position: FormLabelPosition,
    pub label_align: FormLabelAlign,
}

pub fn use_form_context() -> Option<FormContextValue> {
    use_context::<FormContextValue>()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormViewState {
    pub data_disabled: Option<&'static str>,
    pub data_read_only: Option<&'static str>,
    pub data_required: Option<&'static str>,
    pub label_position: &'static str,
    pub label_align: &'static str,
    pub aria_disabled: Option<&'static str>,
    pub state_source: &'static str,
}

pub const FORM_AGENT_SCHEMA: &str = "ui.form.agent-contract.v1";
pub const FORM_AGENT_SCHEMA_VERSION: &str = "v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormAgentIntent {
    FormContainer,
}

impl FormAgentIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormAgentIntent::FormContainer => "form-container",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormAgentAction {
    Render,
}

impl FormAgentAction {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormAgentAction::Render => "render",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormAgentStreamMode {
    Streaming,
    Snapshot,
}

impl FormAgentStreamMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormAgentStreamMode::Streaming => "streaming",
            FormAgentStreamMode::Snapshot => "snapshot",
        }
    }
}
const _: [FormAgentStreamMode; 2] = [
    FormAgentStreamMode::Streaming,
    FormAgentStreamMode::Snapshot,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormAgentStreamingPolicy {
    Optional,
    Required,
}

impl FormAgentStreamingPolicy {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormAgentStreamingPolicy::Optional => "optional",
            FormAgentStreamingPolicy::Required => "required",
        }
    }
}
const _: [FormAgentStreamingPolicy; 2] = [
    FormAgentStreamingPolicy::Optional,
    FormAgentStreamingPolicy::Required,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormAgentStreamingFallback {
    Snapshot,
}

impl FormAgentStreamingFallback {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormAgentStreamingFallback::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormAgentOutputStatus {
    Draft,
    Verified,
    Submittable,
}

impl FormAgentOutputStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormAgentOutputStatus::Draft => "draft",
            FormAgentOutputStatus::Verified => "verified",
            FormAgentOutputStatus::Submittable => "submittable",
        }
    }
}
const _: [FormAgentOutputStatus; 3] = [
    FormAgentOutputStatus::Draft,
    FormAgentOutputStatus::Verified,
    FormAgentOutputStatus::Submittable,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormAgentSource {
    LogicResolved,
}

impl FormAgentSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormAgentSource::LogicResolved => "logic-resolved",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormAgentContractAttrs {
    pub schema: &'static str,
    pub schema_version: &'static str,
    pub intent: &'static str,
    pub action: &'static str,
    pub stream_mode: &'static str,
    pub streaming_policy: &'static str,
    pub streaming_fallback: &'static str,
    pub output_status: &'static str,
    pub state_disabled: &'static str,
    pub state_read_only: &'static str,
    pub state_required: &'static str,
    pub source: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormResolvedProps {
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub label_position: FormLabelPosition,
    pub label_align: FormLabelAlign,
    pub class_name: String,
}

fn resolve_class_name(class_name: Option<String>) -> String {
    let base_class = "ui-form";
    class_name
        .and_then(|value| {
            let trimmed = value.trim();
            (!trimmed.is_empty()).then(|| format!("{base_class} {trimmed}"))
        })
        .unwrap_or_else(|| base_class.to_string())
}

pub fn resolve_props(
    is_disabled: Option<bool>,
    is_read_only: Option<bool>,
    is_required: Option<bool>,
    label_position: Option<FormLabelPosition>,
    label_align: Option<FormLabelAlign>,
    class_name: Option<String>,
) -> FormResolvedProps {
    FormResolvedProps {
        disabled: is_disabled.unwrap_or(false),
        read_only: is_read_only.unwrap_or(false),
        required: is_required.unwrap_or(false),
        label_position: label_position.unwrap_or_default(),
        label_align: label_align.unwrap_or_default(),
        class_name: resolve_class_name(class_name),
    }
}

fn bool_attr(value: bool) -> Option<&'static str> {
    value.then_some("true")
}

fn bool_token_from_attr(value: Option<&'static str>) -> &'static str {
    if value.is_some() { "true" } else { "false" }
}

pub fn resolve_view_state(resolved: &FormResolvedProps) -> FormViewState {
    FormViewState {
        data_disabled: bool_attr(resolved.disabled),
        data_read_only: bool_attr(resolved.read_only),
        data_required: bool_attr(resolved.required),
        label_position: resolved.label_position.as_attr(),
        label_align: resolved.label_align.as_attr(),
        aria_disabled: bool_attr(resolved.disabled),
        state_source: "logic.rs::resolve_view_state",
    }
}

pub fn resolve_agent_contract_attrs(view_state: &FormViewState) -> FormAgentContractAttrs {
    FormAgentContractAttrs {
        schema: FORM_AGENT_SCHEMA,
        schema_version: FORM_AGENT_SCHEMA_VERSION,
        intent: FormAgentIntent::FormContainer.as_attr(),
        action: FormAgentAction::Render.as_attr(),
        stream_mode: FormAgentStreamMode::Snapshot.as_attr(),
        streaming_policy: FormAgentStreamingPolicy::Optional.as_attr(),
        streaming_fallback: FormAgentStreamingFallback::Snapshot.as_attr(),
        output_status: FormAgentOutputStatus::Verified.as_attr(),
        state_disabled: bool_token_from_attr(view_state.data_disabled),
        state_read_only: bool_token_from_attr(view_state.data_read_only),
        state_required: bool_token_from_attr(view_state.data_required),
        source: FormAgentSource::LogicResolved.as_attr(),
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
