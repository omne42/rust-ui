use super::{AlertDialogPartState, AlertDialogPartStateInput, AlertDialogSlot};
use std::borrow::Cow;
use ui_state_primitives::alert_dialog as alert_dialog_state;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertDialogVariant {
    #[default]
    Default,
    Confirmation,
    Destructive,
    Warning,
    Error,
}

impl AlertDialogVariant {
    pub fn class_name(self) -> &'static str {
        to_primitive_variant(self).class_name()
    }

    pub fn data_attr(self) -> &'static str {
        to_primitive_variant(self).data_attr()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertDialogAutoFocusButton {
    #[default]
    None,
    Cancel,
    Secondary,
    Confirm,
}

impl AlertDialogAutoFocusButton {
    pub fn as_attr(self) -> &'static str {
        to_primitive_auto_focus_button(self).as_attr()
    }
}

fn to_primitive_variant(variant: AlertDialogVariant) -> alert_dialog_state::AlertDialogVariant {
    match variant {
        AlertDialogVariant::Default => alert_dialog_state::AlertDialogVariant::Default,
        AlertDialogVariant::Confirmation => alert_dialog_state::AlertDialogVariant::Confirmation,
        AlertDialogVariant::Destructive => alert_dialog_state::AlertDialogVariant::Destructive,
        AlertDialogVariant::Warning => alert_dialog_state::AlertDialogVariant::Warning,
        AlertDialogVariant::Error => alert_dialog_state::AlertDialogVariant::Error,
    }
}

fn to_primitive_auto_focus_button(
    auto_focus_button: AlertDialogAutoFocusButton,
) -> alert_dialog_state::AlertDialogAutoFocusButton {
    match auto_focus_button {
        AlertDialogAutoFocusButton::None => alert_dialog_state::AlertDialogAutoFocusButton::None,
        AlertDialogAutoFocusButton::Cancel => {
            alert_dialog_state::AlertDialogAutoFocusButton::Cancel
        }
        AlertDialogAutoFocusButton::Secondary => {
            alert_dialog_state::AlertDialogAutoFocusButton::Secondary
        }
        AlertDialogAutoFocusButton::Confirm => {
            alert_dialog_state::AlertDialogAutoFocusButton::Confirm
        }
    }
}

pub const DEFAULT_ID_BASE: &str = alert_dialog_state::DEFAULT_ID_BASE;
pub const DEFAULT_TITLE: &str = alert_dialog_state::DEFAULT_TITLE;
pub const DEFAULT_CONFIRM_LABEL: &str = alert_dialog_state::DEFAULT_CONFIRM_LABEL;
pub const DEFAULT_CANCEL_LABEL: &str = alert_dialog_state::DEFAULT_CANCEL_LABEL;
pub const DEFAULT_AUTO_FOCUS_BUTTON: AlertDialogAutoFocusButton = AlertDialogAutoFocusButton::None;
pub const DEFAULT_CONFIRM_DISABLED: bool = alert_dialog_state::DEFAULT_CONFIRM_DISABLED;
pub const DEFAULT_SECONDARY_DISABLED: bool = alert_dialog_state::DEFAULT_SECONDARY_DISABLED;
pub const ALERT_DIALOG_AGENT_SCHEMA: &str = "ui.alert-dialog.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertDialogAgentSchemaVersion {
    V1,
}

impl AlertDialogAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertDialogAgentIntent {
    ConfirmationDialog,
}

impl AlertDialogAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConfirmationDialog => "confirmation.dialog",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertDialogAgentAction {
    Only,
    WithCancel,
    WithSecondary,
    WithCancelSecondary,
}

impl AlertDialogAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Only => "confirm",
            Self::WithCancel => "confirm.cancel",
            Self::WithSecondary => "confirm.secondary",
            Self::WithCancelSecondary => "confirm.cancel.secondary",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertDialogAgentState {
    Open,
    Closed,
}

impl AlertDialogAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertDialogAgentSource {
    Default,
    Customized,
}

impl AlertDialogAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Customized => "customized",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertDialogAgentConfigPolicy {
    Whitelist,
}

impl AlertDialogAgentConfigPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Whitelist => "whitelist",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertDialogAgentOutputStatus {
    Draft,
    Verified,
    CommitReady,
}

impl AlertDialogAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::CommitReady => "commit-ready",
        }
    }
}
const _: [AlertDialogAgentOutputStatus; 3] = [
    AlertDialogAgentOutputStatus::Draft,
    AlertDialogAgentOutputStatus::Verified,
    AlertDialogAgentOutputStatus::CommitReady,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertDialogAgentCapabilities {
    pub has_description: bool,
    pub has_cancel: bool,
    pub has_secondary: bool,
    pub can_confirm: bool,
    pub can_dismiss: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertDialogAgentContractInput {
    pub is_open: bool,
    pub root_state: AlertDialogPartState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

fn is_customized_source(state: AlertDialogPartState) -> bool {
    state.has_custom_variant
        || state.has_custom_id_base
        || state.has_custom_title
        || state.has_custom_description
        || state.has_custom_confirm_label
        || state.has_custom_cancel_label
        || state.has_custom_secondary_label
        || state.has_custom_on_cancel
        || state.has_custom_on_secondary
        || state.has_custom_auto_focus_button
        || state.has_custom_motion
        || state.has_on_exit_complete
}

pub fn resolve_agent_contract(input: AlertDialogAgentContractInput) -> AlertDialogAgentContract {
    let action = match (
        input.root_state.show_cancel,
        input.root_state.show_secondary,
    ) {
        (false, false) => AlertDialogAgentAction::Only,
        (true, false) => AlertDialogAgentAction::WithCancel,
        (false, true) => AlertDialogAgentAction::WithSecondary,
        (true, true) => AlertDialogAgentAction::WithCancelSecondary,
    };

    AlertDialogAgentContract {
        schema_name: ALERT_DIALOG_AGENT_SCHEMA,
        schema_version: AlertDialogAgentSchemaVersion::V1,
        intent: AlertDialogAgentIntent::ConfirmationDialog,
        action,
        state: if input.is_open {
            AlertDialogAgentState::Open
        } else {
            AlertDialogAgentState::Closed
        },
        source: if is_customized_source(input.root_state) {
            AlertDialogAgentSource::Customized
        } else {
            AlertDialogAgentSource::Default
        },
        config_policy: AlertDialogAgentConfigPolicy::Whitelist,
        output_status: AlertDialogAgentOutputStatus::Verified,
        capabilities: AlertDialogAgentCapabilities {
            has_description: input.root_state.show_description,
            has_cancel: input.root_state.show_cancel,
            has_secondary: input.root_state.show_secondary,
            can_confirm: !input.root_state.confirm_disabled,
            can_dismiss: input.root_state.show_cancel || input.root_state.show_secondary,
        },
        variant_source: input.root_state.variant_source_attr,
        title_source: input.root_state.title_source_attr,
        description_source: input.root_state.description_source_attr,
        cancel_source: input.root_state.cancel_source_attr,
        secondary_source: input.root_state.secondary_source_attr,
        confirm_source: input.root_state.confirm_source_attr,
        auto_focus_source: input.root_state.auto_focus_source_attr,
        motion_source: input.root_state.motion_source_attr,
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    alert_dialog_state::normalize_optional_text(value)
}

pub fn normalize_required_text(value: String, fallback: &'static str) -> String {
    alert_dialog_state::normalize_required_text(value, fallback)
}

pub fn normalize_id_base(value: String) -> String {
    alert_dialog_state::normalize_id_base(value)
}

pub fn normalize_cancel_label(value: Option<String>) -> String {
    alert_dialog_state::normalize_cancel_label(value)
}

pub fn normalize_secondary_label(value: Option<String>) -> Option<String> {
    alert_dialog_state::normalize_secondary_label(value)
}

pub fn resolve_disabled_flag(
    is_disabled: Option<bool>,
    legacy_disabled: Option<bool>,
    default_value: bool,
) -> bool {
    alert_dialog_state::resolve_disabled_flag(is_disabled, legacy_disabled, default_value)
}

pub fn resolve_state(input: AlertDialogPartStateInput) -> AlertDialogPartState {
    let core =
        alert_dialog_state::resolve_state_core(alert_dialog_state::AlertDialogStateCoreInput {
            is_open: input.is_open,
            variant: to_primitive_variant(input.variant),
            auto_focus_button: to_primitive_auto_focus_button(input.auto_focus_button),
            show_description: input.show_description,
            show_cancel: input.show_cancel,
            show_secondary: input.show_secondary,
            confirm_disabled: input.confirm_disabled,
            secondary_disabled: input.secondary_disabled,
            has_custom_id_base: input.has_custom_id_base,
            has_custom_title: input.has_custom_title,
            has_custom_description: input.has_custom_description,
            has_custom_confirm_label: input.has_custom_confirm_label,
            has_custom_cancel_label: input.has_custom_cancel_label,
            has_custom_secondary_label: input.has_custom_secondary_label,
            has_custom_on_cancel: input.has_custom_on_cancel,
            has_custom_on_secondary: input.has_custom_on_secondary,
            has_custom_auto_focus_button: input.has_custom_auto_focus_button,
            has_custom_motion: input.has_custom_motion,
            has_on_exit_complete: input.has_on_exit_complete,
        });

    AlertDialogPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: core.state_attr,
        variant: input.variant,
        variant_attr: core.variant_attr,
        variant_class: core.variant_class,
        description_attr: core.description_attr,
        cancel_attr: core.cancel_attr,
        secondary_attr: core.secondary_attr,
        confirm_disabled_attr: core.confirm_disabled_attr,
        secondary_disabled_attr: core.secondary_disabled_attr,
        auto_focus_attr: core.auto_focus_attr,
        show_description: core.show_description,
        show_cancel: core.show_cancel,
        show_secondary: core.show_secondary,
        show_type_icon: core.show_type_icon,
        confirm_disabled: core.confirm_disabled,
        secondary_disabled: core.secondary_disabled,
        has_custom_variant: core.has_custom_variant,
        has_custom_id_base: core.has_custom_id_base,
        has_custom_title: core.has_custom_title,
        has_custom_description: core.has_custom_description,
        has_custom_confirm_label: core.has_custom_confirm_label,
        has_custom_cancel_label: core.has_custom_cancel_label,
        has_custom_secondary_label: core.has_custom_secondary_label,
        has_custom_on_cancel: core.has_custom_on_cancel,
        has_custom_on_secondary: core.has_custom_on_secondary,
        has_custom_auto_focus_button: core.has_custom_auto_focus_button,
        has_custom_motion: core.has_custom_motion,
        has_on_exit_complete: core.has_on_exit_complete,
        variant_source_attr: core.variant_source_attr,
        description_source_attr: core.description_source_attr,
        cancel_source_attr: core.cancel_source_attr,
        secondary_source_attr: core.secondary_source_attr,
        confirm_source_attr: core.confirm_source_attr,
        id_source_attr: core.id_source_attr,
        title_source_attr: core.title_source_attr,
        auto_focus_source_attr: core.auto_focus_source_attr,
        motion_source_attr: core.motion_source_attr,
        exit_source_attr: core.exit_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: AlertDialogPartState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(state.base_class)];

    if matches!(state.slot, AlertDialogSlot::Root) {
        classes.push(Cow::Borrowed(state.variant_class));
        classes.push(Cow::Borrowed(if state.state_attr == "open" {
            "ui-alert-dialog--open"
        } else {
            "ui-alert-dialog--closed"
        }));
        classes.push(Cow::Borrowed(if state.show_description {
            "ui-alert-dialog--with-description"
        } else {
            "ui-alert-dialog--title-only"
        }));
        classes.push(Cow::Borrowed(if state.show_cancel {
            "ui-alert-dialog--cancel-shown"
        } else {
            "ui-alert-dialog--cancel-hidden"
        }));
        classes.push(Cow::Borrowed(if state.show_secondary {
            "ui-alert-dialog--secondary-shown"
        } else {
            "ui-alert-dialog--secondary-hidden"
        }));

        if state.confirm_disabled {
            classes.push(Cow::Borrowed("ui-alert-dialog--confirm-disabled"));
        }

        if state.secondary_disabled {
            classes.push(Cow::Borrowed("ui-alert-dialog--secondary-disabled"));
        }

        if state.show_type_icon {
            classes.push(Cow::Borrowed("ui-alert-dialog--with-type-icon"));
        }

        if state.has_custom_variant {
            classes.push(Cow::Borrowed("ui-alert-dialog--custom-variant"));
        }

        if state.has_custom_id_base {
            classes.push(Cow::Borrowed("ui-alert-dialog--custom-id"));
        }

        if state.has_custom_title {
            classes.push(Cow::Borrowed("ui-alert-dialog--custom-title"));
        }

        if state.has_custom_description {
            classes.push(Cow::Borrowed("ui-alert-dialog--custom-description"));
        }

        if state.cancel_source_attr == "custom" {
            classes.push(Cow::Borrowed("ui-alert-dialog--custom-cancel"));
        }

        if state.secondary_source_attr == "custom" {
            classes.push(Cow::Borrowed("ui-alert-dialog--custom-secondary"));
        }

        if state.confirm_source_attr == "custom" {
            classes.push(Cow::Borrowed("ui-alert-dialog--custom-confirm"));
        }

        if state.auto_focus_source_attr == "custom" {
            classes.push(Cow::Borrowed("ui-alert-dialog--custom-auto-focus"));
        }

        if state.has_custom_motion {
            classes.push(Cow::Borrowed("ui-alert-dialog--custom-motion"));
        }

        if state.has_on_exit_complete {
            classes.push(Cow::Borrowed("ui-alert-dialog--custom-exit"));
        }

        if let Some(base_class_name) = normalize_optional_text(base_class_name) {
            classes.push(Cow::Owned(base_class_name));
        }
    } else if let Some(base_class_name) = normalize_optional_text(base_class_name) {
        classes.push(Cow::Owned(base_class_name));
    }

    classes
        .iter()
        .map(Cow::as_ref)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "test/logic.rs"]
mod tests;
