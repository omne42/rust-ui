pub use crate::logic::DialogSize;
pub use crate::motion::DialogMotion;
pub use ui_headless::A11yDirection;

pub const DEFAULT_ID_BASE: &str;
pub const DEFAULT_TITLE: &str;
pub const DEFAULT_CLOSE_LABEL: &str;
pub const DEFAULT_SHOW_CLOSE_BUTTON: bool;
pub const DEFAULT_SIZE: crate::dialog::DialogSize;
pub const DIALOG_AGENT_SCHEMA: &str;

pub enum DialogSlot {
    Root,
    Header,
    Title,
    Description,
    Body,
    Footer,
    Close,
}

impl DialogSlot {
    pub fn as_attr(self) -> &'static str;
    pub fn base_class(self) -> &'static str;
}

pub enum DialogAgentSchemaVersion {
    V1,
}

impl DialogAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str;
}

pub enum DialogAgentIntent {
    OverlayInteraction,
}

impl DialogAgentIntent {
    pub const fn as_str(self) -> &'static str;
}

pub enum DialogAgentAction {
    OpenClose,
}

impl DialogAgentAction {
    pub const fn as_str(self) -> &'static str;
}

pub enum DialogAgentState {
    Open,
    Closed,
}

impl DialogAgentState {
    pub const fn as_str(self) -> &'static str;
}

pub enum DialogAgentSource {
    Controlled,
    Uncontrolled,
}

impl DialogAgentSource {
    pub const fn as_str(self) -> &'static str;
}

pub enum DialogAgentConfigPolicy {
    Whitelist,
}

impl DialogAgentConfigPolicy {
    pub const fn as_str(self) -> &'static str;
}

pub enum DialogAgentStreamMode {
    Streaming,
    Snapshot,
}

impl DialogAgentStreamMode {
    pub const fn as_str(self) -> &'static str;
}

pub enum DialogAgentStreamSupport {
    Required,
    Optional,
}

impl DialogAgentStreamSupport {
    pub const fn as_str(self) -> &'static str;
}

pub enum DialogAgentOutputStatus {
    Draft,
    Verified,
    CommitReady,
}

impl DialogAgentOutputStatus {
    pub const fn as_str(self) -> &'static str;
}

pub struct DialogAgentContract {
    pub schema_name: &'static str,
    pub schema_version: DialogAgentSchemaVersion,
    pub intent: DialogAgentIntent,
    pub action: DialogAgentAction,
    pub state: DialogAgentState,
    pub source: DialogAgentSource,
    pub config_policy: DialogAgentConfigPolicy,
    pub open_change_source: &'static str,
    pub stream_support: DialogAgentStreamSupport,
    pub stream_mode: DialogAgentStreamMode,
    pub stream_fallback: DialogAgentStreamMode,
    pub output_status: DialogAgentOutputStatus,
}

pub struct DialogAgentContractInput {
    pub is_open: bool,
    pub source: DialogAgentSource,
    pub open_change_source: &'static str,
}

pub fn dialog_agent_source_from_open_mode(mode: ui_state_primitives::dialog::DialogOpenMode) -> DialogAgentSource;

pub fn resolve_agent_contract(input: DialogAgentContractInput) -> DialogAgentContract;

pub fn sanitize_motion(motion: crate::motion::DialogMotion) -> crate::motion::DialogMotion;

pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    finish_exit: leptos::prelude::Callback<()>,
    motion: crate::motion::DialogMotion,
);

pub fn Dialog(
    is_open: Option<leptos::prelude::Signal<bool>>,
    open: Option<leptos::prelude::Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<leptos::prelude::Callback<bool>>,
    on_close: Option<crate::OnPress>,
    id_base: String,
    title: String,
    children: leptos::children::ChildrenFn,
    description: Option<String>,
    footer: Option<leptos::children::ViewFn>,
    size: crate::dialog::DialogSize,
    is_close_button_visible: bool,
    show_close_button: Option<bool>,
    close_label: &'static str,
    motion: crate::dialog::DialogMotion,
    on_exit_complete: Option<leptos::prelude::Callback<()>>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
