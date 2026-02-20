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
