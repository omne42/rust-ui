mod logic;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_CLOSE_ON_ACTION, DEFAULT_DEFAULT_OPEN, DEFAULT_DISABLED, DEFAULT_ID_BASE, DEFAULT_TITLE,
};
pub use view::CommandDialog;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandDialogSlot {
    Root,
    Modal,
    Command,
}

impl CommandDialogSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            CommandDialogSlot::Root => "command-dialog",
            CommandDialogSlot::Modal => "command-dialog-modal",
            CommandDialogSlot::Command => "command-dialog-command",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            CommandDialogSlot::Root => "ui-command-dialog",
            CommandDialogSlot::Modal => "ui-command-dialog__modal",
            CommandDialogSlot::Command => "ui-command-dialog__command",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandDialogPartStateInput {
    pub slot: CommandDialogSlot,
    pub is_open: bool,
    pub has_description: bool,
    pub close_on_action: bool,
    pub disabled: bool,
    pub is_controlled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_empty_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_on_action: bool,
    pub has_custom_on_open_change: bool,
    pub has_custom_default_open: bool,
    pub has_custom_close_on_action: bool,
    pub has_custom_disabled: bool,
    pub has_custom_command_motion: bool,
    pub has_custom_overlay_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandDialogPartState {
    pub slot: CommandDialogSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub description_attr: &'static str,
    pub close_on_action_attr: &'static str,
    pub disabled_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub open_attr: Option<&'static str>,
    pub is_open: bool,
    pub has_description: bool,
    pub close_on_action: bool,
    pub disabled: bool,
    pub enabled: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_empty_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_on_action: bool,
    pub has_custom_on_open_change: bool,
    pub has_custom_default_open: bool,
    pub has_custom_close_on_action: bool,
    pub has_custom_disabled: bool,
    pub has_custom_command_motion: bool,
    pub has_custom_overlay_motion: bool,
    pub id_source_attr: &'static str,
    pub title_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub empty_label_source_attr: &'static str,
    pub aria_label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub action_source_attr: &'static str,
    pub open_change_source_attr: &'static str,
    pub default_open_source_attr: &'static str,
    pub close_on_action_source_attr: &'static str,
    pub disabled_source_attr: &'static str,
    pub command_motion_source_attr: &'static str,
    pub overlay_motion_source_attr: &'static str,
}
