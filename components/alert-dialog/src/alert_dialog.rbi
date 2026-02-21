pub enum AlertDialogVariant {
    Default,
    Confirmation,
    Destructive,
    Warning,
    Error,
}

impl AlertDialogVariant {
    pub fn class_name(self) -> &'static str;
    pub fn data_attr(self) -> &'static str;
}

pub enum AlertDialogAutoFocusButton {
    None,
    Cancel,
    Secondary,
    Confirm,
}

impl AlertDialogAutoFocusButton {
    pub fn as_attr(self) -> &'static str;
}

pub const DEFAULT_ID_BASE: &str;
pub const DEFAULT_TITLE: &str;
pub const DEFAULT_CONFIRM_LABEL: &str;
pub const DEFAULT_CANCEL_LABEL: &str;
pub const DEFAULT_AUTO_FOCUS_BUTTON: AlertDialogAutoFocusButton;
pub const DEFAULT_CONFIRM_DISABLED: bool;
pub const DEFAULT_SECONDARY_DISABLED: bool;
pub const ALERT_DIALOG_AGENT_SCHEMA: &str;

pub enum AlertDialogAgentSchemaVersion {
    V1,
}

impl AlertDialogAgentSchemaVersion {
    pub fn as_str(self) -> &'static str;
}

pub enum AlertDialogAgentIntent {
    ConfirmationDialog,
}

impl AlertDialogAgentIntent {
    pub fn as_str(self) -> &'static str;
}

pub enum AlertDialogAgentAction {
    ConfirmOnly,
    ConfirmCancel,
    ConfirmSecondary,
    ConfirmCancelSecondary,
}

impl AlertDialogAgentAction {
    pub fn as_str(self) -> &'static str;
}

pub enum AlertDialogAgentState {
    Open,
    Closed,
}

impl AlertDialogAgentState {
    pub fn as_str(self) -> &'static str;
}

pub enum AlertDialogAgentSource {
    Default,
    Customized,
}

impl AlertDialogAgentSource {
    pub fn as_str(self) -> &'static str;
}

pub enum AlertDialogAgentConfigPolicy {
    Whitelist,
}

impl AlertDialogAgentConfigPolicy {
    pub fn as_str(self) -> &'static str;
}

pub enum AlertDialogAgentOutputStatus {
    Draft,
    Verified,
    CommitReady,
}

impl AlertDialogAgentOutputStatus {
    pub fn as_str(self) -> &'static str;
}

pub struct AlertDialogAgentCapabilities {
    pub has_description: bool,
    pub has_cancel: bool,
    pub has_secondary: bool,
    pub can_confirm: bool,
    pub can_dismiss: bool,
}

pub struct AlertDialogAgentContractInput {
    pub is_open: bool,
    pub root_state: AlertDialogPartState,
}

pub struct AlertDialogAgentContract {
    pub schema_name: &'static str,
    pub schema_version: AlertDialogAgentSchemaVersion,
    pub intent: AlertDialogAgentIntent,
    pub action: AlertDialogAgentAction,
    pub state: AlertDialogAgentState,
    pub source: AlertDialogAgentSource,
    pub config_policy: AlertDialogAgentConfigPolicy,
    pub output_status: AlertDialogAgentOutputStatus,
    pub capabilities: AlertDialogAgentCapabilities,
    pub variant_source: &'static str,
    pub title_source: &'static str,
    pub description_source: &'static str,
    pub cancel_source: &'static str,
    pub secondary_source: &'static str,
    pub confirm_source: &'static str,
    pub auto_focus_source: &'static str,
    pub motion_source: &'static str,
}

pub fn resolve_agent_contract(input: AlertDialogAgentContractInput) -> AlertDialogAgentContract;

pub struct AlertDialogMotion {
    pub overlay: crate::overlay::OverlayMotion,
}

pub enum AlertDialogComponentSchemaVersion {
    V1,
}

pub struct AlertDialogComponentSpec {
    pub schema_version: AlertDialogComponentSchemaVersion,
}

pub fn AlertDialog(
    open: leptos::prelude::Signal<bool>,
    id_base: String,
    title: String,
    on_close: ui_headless::OnPress,
    confirm_label: String,
    on_confirm: ui_headless::OnPress,
    description: Option<String>,
    cancel_label: Option<String>,
    secondary_label: Option<String>,
    on_secondary: Option<ui_headless::OnPress>,
    on_cancel: Option<ui_headless::OnPress>,
    is_confirm_disabled: Option<bool>,
    confirm_disabled: Option<bool>,
    is_secondary_disabled: Option<bool>,
    secondary_disabled: Option<bool>,
    auto_focus_button: AlertDialogAutoFocusButton,
    variant: AlertDialogVariant,
    motion: AlertDialogMotion,
    on_exit_complete: Option<leptos::prelude::Callback<()>>,
    lang: Option<String>,
    dir: Option<ui_headless::a11y::A11yDirection>,
) -> impl leptos::prelude::IntoView;
