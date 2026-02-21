pub use crate::CommandMotion;
pub use ui_state_primitives::command::{
    CommandFilterState, CommandGroup, CommandItem, FilteredCommandGroup, FilteredCommandItem,
};

pub const DEFAULT_ID_BASE: &str;
pub const DEFAULT_PLACEHOLDER: &str;
pub const DEFAULT_EMPTY_LABEL: &str;
pub const DEFAULT_ARIA_LABEL: &str;
pub const DEFAULT_DISABLED: bool;
pub const COMMAND_AGENT_SCHEMA: &str;

pub enum CommandSlot {
    Root,
    InputWrap,
    Input,
    List,
    Options,
    Group,
    GroupHeading,
    GroupItems,
    Item,
    ItemLabel,
    Shortcut,
    Empty,
    Highlight,
}

impl CommandSlot {
    pub fn as_attr(self) -> &'static str;
    pub fn base_class(self) -> &'static str;
}

pub enum CommandAgentSchemaVersion {
    V1,
}

impl CommandAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str;
}

pub enum CommandAgentIntent {
    CommandDiscovery,
}

impl CommandAgentIntent {
    pub const fn as_str(self) -> &'static str;
}

pub enum CommandAgentAction {
    FilterNavigateSelect,
}

impl CommandAgentAction {
    pub const fn as_str(self) -> &'static str;
}

pub enum CommandAgentState {
    Idle,
    QueryResults,
    QueryEmpty,
    Empty,
    Disabled,
}

impl CommandAgentState {
    pub const fn as_str(self) -> &'static str;
}

pub enum CommandAgentSource {
    Controlled,
    Uncontrolled,
}

impl CommandAgentSource {
    pub const fn as_str(self) -> &'static str;
}

pub enum CommandAgentConfigPolicy {
    Whitelist,
}

impl CommandAgentConfigPolicy {
    pub const fn as_str(self) -> &'static str;
}

pub struct CommandAgentContractInput {
    pub state_attr: crate::CommandRootStateAttr,
    pub query_control_attr: crate::CommandQueryControlAttr,
}

pub struct CommandAgentContract {
    pub schema_name: &'static str,
    pub schema_version: CommandAgentSchemaVersion,
    pub intent: CommandAgentIntent,
    pub action: CommandAgentAction,
    pub state: CommandAgentState,
    pub source: CommandAgentSource,
    pub config_policy: CommandAgentConfigPolicy,
}

pub fn resolve_agent_contract(
    input: CommandAgentContractInput,
) -> CommandAgentContract;

pub fn sanitize_motion(motion: crate::motion::CommandMotion) -> crate::motion::CommandMotion;

pub fn Command(
    id_base: String,
    groups: std::sync::Arc<[crate::CommandGroup]>,
    query: Option<leptos::prelude::Signal<String>>,
    default_query: Option<String>,
    on_query_change: Option<leptos::prelude::Callback<String>>,
    on_action: Option<leptos::prelude::Callback<String>>,
    is_disabled: bool,
    motion: crate::CommandMotion,
    placeholder: Option<String>,
    empty_label: Option<String>,
    aria_label: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
