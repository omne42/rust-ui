pub type FieldLabelTone = ui_state_primitives::field_label::FieldLabelTone;
pub const FIELD_LABEL_AGENT_SCHEMA: &str = "field_label.v1";

pub enum FieldLabelAgentIntent {
    Label,
}

pub enum FieldLabelAgentAction {
    SnapshotRender,
}

pub enum FieldLabelAgentStreaming {
    Optional,
}

pub enum FieldLabelAgentFallback {
    Snapshot,
}

pub enum FieldLabelAgentOutputState {
    Verified,
}

pub fn FieldLabel(
    text: Option<String>,
    for_id: Option<String>,
    is_required: bool,
    is_disabled: bool,
    tone: FieldLabelTone,
    required_indicator: Option<String>,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
