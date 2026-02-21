mod logic;
pub mod motion;
pub mod protocol;
pub mod styles;
mod view;

pub use logic::{
    AlertDialogAutoFocusButton, AlertDialogVariant, DEFAULT_AUTO_FOCUS_BUTTON,
    DEFAULT_CANCEL_LABEL, DEFAULT_CONFIRM_DISABLED, DEFAULT_CONFIRM_LABEL, DEFAULT_ID_BASE,
    DEFAULT_SECONDARY_DISABLED, DEFAULT_TITLE,
};
pub use motion::AlertDialogMotion;
pub use view::AlertDialog;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertDialogSlot {
    Root,
    Header,
    HeaderText,
    TypeIcon,
    Title,
    Description,
    Footer,
    CancelAction,
    SecondaryAction,
    ConfirmAction,
}

impl AlertDialogSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            AlertDialogSlot::Root => "alert-dialog",
            AlertDialogSlot::Header => "alert-dialog-header",
            AlertDialogSlot::HeaderText => "alert-dialog-header-text",
            AlertDialogSlot::TypeIcon => "alert-dialog-type-icon",
            AlertDialogSlot::Title => "alert-dialog-title",
            AlertDialogSlot::Description => "alert-dialog-description",
            AlertDialogSlot::Footer => "alert-dialog-footer",
            AlertDialogSlot::CancelAction => "alert-dialog-cancel",
            AlertDialogSlot::SecondaryAction => "alert-dialog-secondary",
            AlertDialogSlot::ConfirmAction => "alert-dialog-confirm",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            AlertDialogSlot::Root => "ui-alert-dialog",
            AlertDialogSlot::Header => "ui-alert-dialog__header",
            AlertDialogSlot::HeaderText => "ui-alert-dialog__header-text",
            AlertDialogSlot::TypeIcon => "ui-alert-dialog__type-icon",
            AlertDialogSlot::Title => "ui-alert-dialog__title",
            AlertDialogSlot::Description => "ui-alert-dialog__description",
            AlertDialogSlot::Footer => "ui-alert-dialog__footer",
            AlertDialogSlot::CancelAction => {
                "ui-alert-dialog__action ui-alert-dialog__action--cancel"
            }
            AlertDialogSlot::SecondaryAction => {
                "ui-alert-dialog__action ui-alert-dialog__action--secondary"
            }
            AlertDialogSlot::ConfirmAction => {
                "ui-alert-dialog__action ui-alert-dialog__action--confirm"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertDialogPartStateInput {
    pub slot: AlertDialogSlot,
    pub is_open: bool,
    pub variant: AlertDialogVariant,
    pub auto_focus_button: AlertDialogAutoFocusButton,
    pub show_description: bool,
    pub show_cancel: bool,
    pub show_secondary: bool,
    pub confirm_disabled: bool,
    pub secondary_disabled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_confirm_label: bool,
    pub has_custom_cancel_label: bool,
    pub has_custom_secondary_label: bool,
    pub has_custom_on_cancel: bool,
    pub has_custom_on_secondary: bool,
    pub has_custom_auto_focus_button: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertDialogPartState {
    pub slot: AlertDialogSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub variant: AlertDialogVariant,
    pub variant_attr: &'static str,
    pub variant_class: &'static str,
    pub description_attr: &'static str,
    pub cancel_attr: &'static str,
    pub secondary_attr: &'static str,
    pub confirm_disabled_attr: &'static str,
    pub secondary_disabled_attr: &'static str,
    pub auto_focus_attr: &'static str,
    pub show_description: bool,
    pub show_cancel: bool,
    pub show_secondary: bool,
    pub show_type_icon: bool,
    pub confirm_disabled: bool,
    pub secondary_disabled: bool,
    pub has_custom_variant: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_confirm_label: bool,
    pub has_custom_cancel_label: bool,
    pub has_custom_secondary_label: bool,
    pub has_custom_on_cancel: bool,
    pub has_custom_on_secondary: bool,
    pub has_custom_auto_focus_button: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
    pub variant_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub cancel_source_attr: &'static str,
    pub secondary_source_attr: &'static str,
    pub confirm_source_attr: &'static str,
    pub id_source_attr: &'static str,
    pub title_source_attr: &'static str,
    pub auto_focus_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}

#[cfg(all(test, not(feature = "component-alert_dialog")))]
#[path = "test/semantics.rs"]
mod semantics_tests;
