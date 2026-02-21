pub type A11yDirection = ui_headless::A11yDirection;
pub type FormLabelPosition = crate::logic::FormLabelPosition;
pub type FormLabelAlign = crate::logic::FormLabelAlign;
pub type FormContextValue = crate::logic::FormContextValue;
pub type FormViewState = crate::logic::FormViewState;
pub type FormAgentContractAttrs = crate::logic::FormAgentContractAttrs;

pub const FORM_AGENT_SCHEMA: &str = crate::logic::FORM_AGENT_SCHEMA;
pub const FORM_AGENT_SCHEMA_VERSION: &str = crate::logic::FORM_AGENT_SCHEMA_VERSION;

pub enum FormAgentStreamMode {
    Streaming,
    Snapshot,
}

pub enum FormAgentStreamingPolicy {
    Optional,
    Required,
}

pub enum FormAgentStreamingFallback {
    Snapshot,
}

pub enum FormAgentOutputStatus {
    Draft,
    Verified,
    Submittable,
}

pub fn Form(
    children: leptos::children::Children,
    is_disabled: Option<bool>,
    is_read_only: Option<bool>,
    is_required: Option<bool>,
    label_position: Option<FormLabelPosition>,
    label_align: Option<FormLabelAlign>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
