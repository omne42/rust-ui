pub enum EmptyMediaVariant {
    Default,
    Icon,
}

pub enum EmptySlot {
    Root,
    Header,
    Title,
    Description,
    Content,
    Media,
}

pub struct EmptyPartStateInput {
    pub slot: EmptySlot,
    pub media_variant: EmptyMediaVariant,
    pub has_custom_class_name: bool,
}

pub struct EmptyPartState {
    pub slot: EmptySlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub media_variant: EmptyMediaVariant,
    pub media_variant_attr: &'static str,
    pub has_custom_class_name: bool,
    pub class_source_attr: &'static str,
    pub variant_source_attr: &'static str,
}

pub const EMPTY_COMPONENT_SCHEMA_NAME: &str;
pub const EMPTY_COMPONENT_SCHEMA_VERSION: &str;

pub enum EmptyAgentIntent {
    EmptyDisplay,
}

pub enum EmptyAgentAction {
    RenderSnapshot,
}

pub enum EmptyAgentSource {
    Default,
    Custom,
}

pub enum EmptyAgentOutputStatus {
    Draft,
    Verified,
    Submittable,
}

pub enum EmptyAgentStreamSupport {
    Required,
    Optional,
}

pub enum EmptyAgentStreamFallback {
    Snapshot,
}

pub struct EmptyAgentContract {
    pub schema_name: &'static str,
    pub schema_version: &'static str,
    pub intent: EmptyAgentIntent,
    pub action: EmptyAgentAction,
    pub state: &'static str,
    pub source: EmptyAgentSource,
    pub stream_support: EmptyAgentStreamSupport,
    pub stream_fallback: EmptyAgentStreamFallback,
    pub output_status: EmptyAgentOutputStatus,
}

pub fn resolve_agent_contract(state: EmptyPartState) -> EmptyAgentContract;

pub fn Empty(
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<String>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;

pub fn EmptyHeader(
    class_name: Option<String>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;

pub fn EmptyTitle(
    class_name: Option<String>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;

pub fn EmptyDescription(
    class_name: Option<String>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;

pub fn EmptyContent(
    class_name: Option<String>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;

pub fn EmptyMedia(
    variant: Option<EmptyMediaVariant>,
    class_name: Option<String>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
