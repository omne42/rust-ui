pub enum ModalSlot {
    Root,
    Title,
    Description,
    Body,
}

impl ModalSlot {
    pub fn as_attr(self) -> &'static str;
    pub fn base_class(self) -> &'static str;
}

pub enum ModalDescriptionState {
    WithDescription,
    TitleOnly,
}

impl ModalDescriptionState {
    pub fn as_state_attr(self) -> &'static str;
    pub fn as_description_attr(self) -> &'static str;
    pub fn shows_description(self) -> bool;
}

pub struct ModalPartStateInput {
    pub slot: ModalSlot,
    pub description_state: ModalDescriptionState,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

pub struct ModalPartState {
    pub slot: ModalSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub description_state: ModalDescriptionState,
    pub state_attr: &'static str,
    pub description_attr: &'static str,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
    pub id_source_attr: &'static str,
    pub title_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}

pub const DEFAULT_ID_BASE: &str;
pub const DEFAULT_TITLE: &str;
pub const DEFAULT_OPEN: bool;
pub const MODAL_AGENT_SCHEMA: &str;

pub enum ModalAgentSchemaVersion {
    V1,
}

impl ModalAgentSchemaVersion {
    pub fn as_str(self) -> &'static str;
}

pub enum ModalAgentIntent {
    OverlayDialog,
}

impl ModalAgentIntent {
    pub fn as_str(self) -> &'static str;
}

pub enum ModalAgentAction {
    Open,
    Close,
}

impl ModalAgentAction {
    pub fn as_str(self) -> &'static str;
}

pub enum ModalAgentState {
    Open,
    Closed,
}

impl ModalAgentState {
    pub fn as_str(self) -> &'static str;
}

pub enum ModalAgentSource {
    Controlled,
    Uncontrolled,
}

impl ModalAgentSource {
    pub fn as_str(self) -> &'static str;
}

pub enum ModalAgentConfigPolicy {
    Whitelist,
}

impl ModalAgentConfigPolicy {
    pub fn as_str(self) -> &'static str;
}

pub enum ModalAgentOutputStatus {
    Verified,
}

impl ModalAgentOutputStatus {
    pub fn as_str(self) -> &'static str;
}

pub struct ModalAgentCapabilities {
    pub has_description: bool,
    pub can_open: bool,
    pub can_close: bool,
}

pub struct ModalAgentContractInput {
    pub is_open: bool,
    pub open_mode: ModalOpenMode,
    pub has_description: bool,
}

pub struct ModalAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ModalAgentSchemaVersion,
    pub intent: ModalAgentIntent,
    pub action: ModalAgentAction,
    pub state: ModalAgentState,
    pub source: ModalAgentSource,
    pub config_policy: ModalAgentConfigPolicy,
    pub output_status: ModalAgentOutputStatus,
    pub capabilities: ModalAgentCapabilities,
}

pub fn resolve_agent_contract(input: ModalAgentContractInput) -> ModalAgentContract;

pub enum ModalOpenMode {
    Controlled,
    Uncontrolled,
}

impl ModalOpenMode {
    pub fn as_attr(self) -> &'static str;
}

pub enum ModalOpenSource {
    Controlled,
    Default,
    ImplicitDefault,
}

impl ModalOpenSource {
    pub fn as_attr(self) -> &'static str;
}

pub enum ModalOpenChangeSource {
    Custom,
    None,
}

impl ModalOpenChangeSource {
    pub fn as_attr(self) -> &'static str;
}

pub enum ModalOpenPropSource {
    IsOpen,
    None,
}

impl ModalOpenPropSource {
    pub fn as_attr(self) -> &'static str;
}

pub struct ModalOpenContract {
    pub mode: ModalOpenMode,
    pub open_source: ModalOpenSource,
    pub open_change_source: ModalOpenChangeSource,
    pub open_prop_source: ModalOpenPropSource,
    pub has_custom_default_open: bool,
    pub has_custom_on_open_change: bool,
}

pub const MODAL_MOTION_CONTRACT_STIFFNESS: f64;
pub const MODAL_MOTION_CONTRACT_DAMPING: f64;
pub const MODAL_MOTION_CONTRACT_MASS: f64;
pub const MODAL_MOTION_CONTRACT_PRECISION: f64;
pub const MODAL_MOTION_CONTRACT_INITIAL_SCALE: f64;
pub const MODAL_MOTION_CONTRACT_INITIAL_Y_PX: f64;

pub fn default_motion_contract() -> crate::overlay::OverlayMotion;
pub fn normalize_motion(motion: crate::overlay::OverlayMotion) -> crate::overlay::OverlayMotion;
pub fn is_custom_motion(motion: crate::overlay::OverlayMotion) -> bool;

pub enum ModalComponentSchemaVersion {
    V1,
}

pub struct ModalComponentSpec {
    pub schema_version: ModalComponentSchemaVersion,
}

pub fn Modal(
    is_open: Option<leptos::prelude::Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<leptos::prelude::Callback<bool>>,
    id_base: String,
    title: String,
    on_close: crate::OnPress,
    description: Option<String>,
    motion: crate::overlay::OverlayMotion,
    on_exit_complete: Option<leptos::prelude::Callback<()>>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    class_name: Option<String>,
    children: leptos::children::ChildrenFn,
) -> impl leptos::prelude::IntoView;
