use std::borrow::Cow;

pub use ui_state_primitives::field_label::{
    DEFAULT_ARIA_LABEL, DEFAULT_REQUIRED_INDICATOR, DEFAULT_TEXT, FieldLabelState,
    FieldLabelStateInput, FieldLabelTone, resolve_state,
};

pub const FIELD_LABEL_AGENT_SCHEMA: &str = "field_label.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldLabelAgentIntent {
    Label,
}

impl FieldLabelAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Label => "label",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldLabelAgentAction {
    SnapshotRender,
}

impl FieldLabelAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::SnapshotRender => "snapshot_render",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldLabelAgentStreaming {
    Optional,
}

impl FieldLabelAgentStreaming {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldLabelAgentFallback {
    Snapshot,
}

impl FieldLabelAgentFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldLabelAgentOutputState {
    Verified,
}

impl FieldLabelAgentOutputState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedFieldLabelProps {
    pub text: String,
    pub required_indicator: String,
    pub aria_label: String,
    pub for_id: Option<String>,
    pub class_name: Option<String>,
    pub has_for_id: bool,
    pub has_custom_text: bool,
    pub has_custom_indicator: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldLabelLogicInput {
    pub tone: FieldLabelTone,
    pub is_required: bool,
    pub is_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldLabelViewModel {
    pub text: String,
    pub required_indicator: String,
    pub aria_label: String,
    pub for_id: Option<String>,
    pub class_name: Option<String>,
    pub state: FieldLabelState,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    ui_state_primitives::field_label::normalize_optional_text(value)
}

pub fn normalize_text(value: Option<String>) -> (String, bool) {
    ui_state_primitives::field_label::normalize_text(value)
}

pub fn normalize_required_indicator(value: Option<String>) -> (String, bool) {
    ui_state_primitives::field_label::normalize_required_indicator(value)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    ui_state_primitives::field_label::normalize_aria_label(value)
}

pub fn normalize_props(
    text: Option<String>,
    required_indicator: Option<String>,
    aria_label: Option<String>,
    for_id: Option<String>,
    class_name: Option<String>,
) -> NormalizedFieldLabelProps {
    let (text, has_custom_text) = normalize_text(text);
    let (required_indicator, has_custom_indicator) =
        normalize_required_indicator(required_indicator);
    let (aria_label, has_custom_aria_label) = normalize_aria_label(aria_label);

    let for_id = normalize_optional_text(for_id);
    let class_name = normalize_optional_text(class_name);

    NormalizedFieldLabelProps {
        text,
        required_indicator,
        aria_label,
        has_for_id: for_id.is_some(),
        has_custom_text,
        has_custom_indicator,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
        for_id,
        class_name,
    }
}

pub fn derive_view_model(
    input: FieldLabelLogicInput,
    text: Option<String>,
    required_indicator: Option<String>,
    aria_label: Option<String>,
    for_id: Option<String>,
    class_name: Option<String>,
) -> FieldLabelViewModel {
    let normalized = normalize_props(text, required_indicator, aria_label, for_id, class_name);

    let state = resolve_state(FieldLabelStateInput {
        tone: input.tone,
        required: input.is_required,
        disabled: input.is_disabled,
        has_for_id: normalized.has_for_id,
        has_custom_text: normalized.has_custom_text,
        has_custom_indicator: normalized.has_custom_indicator,
        has_custom_aria_label: normalized.has_custom_aria_label,
        has_custom_class_name: normalized.has_custom_class_name,
    });

    FieldLabelViewModel {
        text: normalized.text,
        required_indicator: normalized.required_indicator,
        aria_label: normalized.aria_label,
        for_id: normalized.for_id,
        class_name: normalized.class_name,
        state,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: FieldLabelState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-field-label"),
        Cow::Borrowed(state.tone_class),
    ];

    if state.is_required {
        classes.push(Cow::Borrowed("ui-field-label--required"));
    }

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-field-label--disabled"));
    }

    if state.has_for_id {
        classes.push(Cow::Borrowed("ui-field-label--for"));
    }

    if state.has_custom_text {
        classes.push(Cow::Borrowed("ui-field-label--text-custom"));
    }

    if state.has_custom_indicator {
        classes.push(Cow::Borrowed("ui-field-label--indicator-custom"));
    }

    if state.has_custom_aria_label {
        classes.push(Cow::Borrowed("ui-field-label--aria-custom"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-field-label--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
