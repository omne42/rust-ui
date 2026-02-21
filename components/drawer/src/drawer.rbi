pub use crate::logic::DrawerPlacement;
pub use crate::motion::DrawerMotion;
pub use ui_headless::{A11yDirection, OnPress};

pub const DEFAULT_ID_BASE: &str;
pub const DEFAULT_TITLE: &str;
pub const DEFAULT_OPEN: bool;
pub const DEFAULT_SHOW_CLOSE_BUTTON: bool;
pub const DEFAULT_PLACEMENT: crate::logic::DrawerPlacement;
pub const DEFAULT_CLOSE_LABEL: &str;
pub const DRAWER_AGENT_SCHEMA: &str;

pub enum DrawerSlot {
    Root,
    Header,
    Title,
    Description,
    Body,
    Footer,
    Close,
}

impl DrawerSlot {
    pub fn as_attr(self) -> &'static str;
    pub fn base_class(self) -> &'static str;
}

pub struct DrawerPartStateInput {
    pub slot: DrawerSlot,
    pub placement: DrawerPlacement,
    pub has_description: bool,
    pub has_footer: bool,
    pub show_close_button: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

pub struct DrawerPartState {
    pub slot: DrawerSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub placement_attr: &'static str,
    pub placement_class: &'static str,
    pub placement_source_attr: &'static str,
    pub description_attr: &'static str,
    pub description_source_attr: &'static str,
    pub footer_attr: &'static str,
    pub footer_source_attr: &'static str,
    pub close_button_attr: &'static str,
    pub close_source_attr: &'static str,
    pub id_source_attr: &'static str,
    pub title_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub exit_source_attr: &'static str,
    pub show_description: bool,
    pub show_footer: bool,
    pub show_close_button: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

pub enum DrawerOpenMode {
    Controlled,
    Uncontrolled,
}

pub enum DrawerOpenValueSource {
    External,
    Default,
    PrimitiveDefault,
}

impl DrawerOpenValueSource {
    pub fn as_attr(self) -> &'static str;
}

pub enum DrawerOpenActionSource {
    Programmatic,
    Interaction,
}

impl DrawerOpenActionSource {
    pub fn as_attr(self) -> &'static str;
}

pub enum DrawerAgentSchemaVersion {
    V1,
}

impl DrawerAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str;
}

pub enum DrawerAgentIntent {
    OverlayDrawer,
}

impl DrawerAgentIntent {
    pub const fn as_str(self) -> &'static str;
}

pub enum DrawerAgentAction {
    Open,
    Close,
}

impl DrawerAgentAction {
    pub const fn as_str(self) -> &'static str;
}

pub enum DrawerAgentState {
    Open,
    Closed,
}

impl DrawerAgentState {
    pub const fn as_str(self) -> &'static str;
}

pub enum DrawerAgentSource {
    Controlled,
    Uncontrolled,
}

impl DrawerAgentSource {
    pub const fn as_str(self) -> &'static str;
}

pub enum DrawerAgentConfigPolicy {
    Whitelist,
}

impl DrawerAgentConfigPolicy {
    pub const fn as_str(self) -> &'static str;
}

pub enum DrawerAgentOutputStatus {
    Verified,
}

impl DrawerAgentOutputStatus {
    pub const fn as_str(self) -> &'static str;
}

pub struct DrawerAgentCapabilities {
    pub has_description: bool,
    pub has_footer: bool,
    pub can_open: bool,
    pub can_close: bool,
}

pub struct DrawerAgentContractInput {
    pub is_open: bool,
    pub open_mode: DrawerOpenMode,
    pub has_description: bool,
    pub has_footer: bool,
}

pub struct DrawerAgentContract {
    pub schema_name: &'static str,
    pub schema_version: DrawerAgentSchemaVersion,
    pub intent: DrawerAgentIntent,
    pub action: DrawerAgentAction,
    pub state: DrawerAgentState,
    pub source: DrawerAgentSource,
    pub config_policy: DrawerAgentConfigPolicy,
    pub output_status: DrawerAgentOutputStatus,
    pub capabilities: DrawerAgentCapabilities,
}

pub fn resolve_agent_contract(input: DrawerAgentContractInput) -> DrawerAgentContract;

pub enum DrawerComponentSchemaVersion {
    V1,
}

pub struct DrawerComponentSpec {
    pub schema_version: DrawerComponentSchemaVersion,
}

pub fn Drawer(
    is_open: Option<leptos::prelude::Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<leptos::prelude::Callback<bool>>,
    on_close: Option<ui_headless::OnPress>,
    id_base: String,
    title: String,
    children: leptos::children::ChildrenFn,
    description: Option<String>,
    footer: Option<leptos::children::ViewFn>,
    placement: Option<crate::logic::DrawerPlacement>,
    motion: crate::DrawerMotion,
    is_close_button_visible: Option<bool>,
    close_label: Option<&'static str>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    on_exit_complete: Option<leptos::prelude::Callback<()>>,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
