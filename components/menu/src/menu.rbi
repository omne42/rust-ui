pub use crate::MenuItemKind;

pub const MENU_AGENT_SCHEMA: &str;

pub struct MenuItemSpec {
    pub label: String,
    pub kind: MenuItemKind,
    pub is_disabled: bool,
}

impl MenuItemSpec {
    pub fn action(label: impl Into<String>) -> Self;
    pub fn with_kind(self, kind: MenuItemKind) -> Self;
    pub fn with_disabled(self, is_disabled: bool) -> Self;
}

pub struct MenuMotion {
    pub highlight: ui_visual_primitive::active_highlight::ActiveHighlightMotion,
}

pub struct MenuState {
    pub is_empty: bool,
    pub has_items: bool,
    pub has_checked_items: bool,
    pub has_disabled_items: bool,
}

pub enum MenuAgentSchemaVersion {
    V1,
}

impl MenuAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str;
}

pub enum MenuAgentIntent {
    MenuInteraction,
}

impl MenuAgentIntent {
    pub const fn as_str(self) -> &'static str;
}

pub enum MenuAgentAction {
    NavigateSelect,
}

impl MenuAgentAction {
    pub const fn as_str(self) -> &'static str;
}

pub enum MenuAgentState {
    Disabled,
    Empty,
    Ready,
    ReadyChecked,
}

impl MenuAgentState {
    pub const fn as_str(self) -> &'static str;
}

pub enum MenuAgentSource {
    StatePrimitives,
}

impl MenuAgentSource {
    pub const fn as_str(self) -> &'static str;
}

pub enum MenuAgentOutputStatus {
    Verified,
}

impl MenuAgentOutputStatus {
    pub const fn as_str(self) -> &'static str;
}

pub enum MenuAgentStreamSupport {
    Unsupported,
}

impl MenuAgentStreamSupport {
    pub const fn as_str(self) -> &'static str;
}

pub enum MenuAgentStreamFallback {
    Snapshot,
}

impl MenuAgentStreamFallback {
    pub const fn as_str(self) -> &'static str;
}

pub enum MenuAgentStreamMode {
    Streaming,
    Snapshot,
}

impl MenuAgentStreamMode {
    pub const fn as_str(self) -> &'static str;
}

pub struct MenuAgentContract {
    pub schema_name: &'static str,
    pub schema_version: MenuAgentSchemaVersion,
    pub intent: MenuAgentIntent,
    pub action: MenuAgentAction,
    pub state: MenuAgentState,
    pub source: MenuAgentSource,
    pub output_status: MenuAgentOutputStatus,
    pub stream_support: MenuAgentStreamSupport,
    pub stream_fallback: MenuAgentStreamFallback,
    pub stream_mode: MenuAgentStreamMode,
    pub state_source: &'static str,
    pub motion_source: &'static str,
    pub items_source: &'static str,
    pub config_policy: &'static str,
}

pub struct MenuAgentContractInput {
    pub render_state: MenuState,
    pub is_disabled: bool,
    pub motion_source: &'static str,
    pub items_source: &'static str,
}

pub fn sanitize_motion(motion: crate::menu::MenuMotion) -> crate::menu::MenuMotion;
pub fn resolve_agent_contract(input: MenuAgentContractInput) -> MenuAgentContract;

pub fn attach_motion(
    container_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    highlight_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    active_index: leptos::prelude::ReadSignal<usize>,
    option_id: leptos::prelude::Callback<usize, String>,
    motion: crate::menu::MenuMotion,
);

pub fn Menu(
    id_base: String,
    items: std::sync::Arc<[String]>,
    on_action: leptos::prelude::Callback<usize>,
    item_specs: Vec<MenuItemSpec>,
    id: Option<String>,
    aria_label: Option<String>,
    aria_labelledby: Option<String>,
    is_disabled: Option<bool>,
    disabled: bool,
    disabled_indices: Vec<usize>,
    item_kinds: Vec<MenuItemKind>,
    default_index: usize,
    motion: MenuMotion,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
