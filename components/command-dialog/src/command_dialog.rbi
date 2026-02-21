pub enum CommandDialogSlot {
    Root,
    Modal,
    Command,
}

impl CommandDialogSlot {
    pub fn as_attr(self) -> &'static str;
    pub fn base_class(self) -> &'static str;
}

pub const DEFAULT_ID_BASE: &str;
pub const DEFAULT_TITLE: &str;
pub const DEFAULT_CLOSE_ON_ACTION: bool;
pub const DEFAULT_DISABLED: bool;
pub const DEFAULT_DEFAULT_OPEN: bool;

pub enum CommandDialogAgentStreamMode {
    Streaming,
    Snapshot,
}

impl CommandDialogAgentStreamMode {
    pub const fn as_str(self) -> &'static str;
}

pub enum CommandDialogAgentStreamSupport {
    Required,
    Optional,
}

impl CommandDialogAgentStreamSupport {
    pub const fn as_str(self) -> &'static str;
}

pub enum CommandDialogAgentOutputStatus {
    Draft,
    Verified,
    CommitReady,
}

impl CommandDialogAgentOutputStatus {
    pub const fn as_str(self) -> &'static str;
}

pub struct CommandDialogMotion {
    pub command: crate::command::CommandMotion,
    pub overlay: crate::overlay::OverlayMotion,
}

pub fn sanitize_motion(
    motion: crate::command_dialog::motion::CommandDialogMotion,
) -> crate::command_dialog::motion::CommandDialogMotion;

pub fn attach_motion(
    command: crate::command::CommandMotion,
    overlay: crate::overlay::OverlayMotion,
) -> crate::command_dialog::motion::CommandDialogMotion;

pub fn CommandDialog(
    groups: std::sync::Arc<[crate::command::CommandGroup]>,
    open: Option<leptos::prelude::Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<leptos::prelude::Callback<bool>>,
    on_action: Option<leptos::prelude::Callback<String>>,
    close_on_action: bool,
    id_base: Option<String>,
    title: Option<String>,
    description: Option<String>,
    is_disabled: Option<bool>,
    disabled: bool,
    command_motion: crate::command::CommandMotion,
    overlay_motion: crate::overlay::OverlayMotion,
    placeholder: Option<String>,
    empty_label: Option<String>,
    aria_label: Option<String>,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
