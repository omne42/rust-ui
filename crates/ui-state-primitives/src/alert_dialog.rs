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
        match self {
            AlertDialogVariant::Default => "ui-alert-dialog--variant-default",
            AlertDialogVariant::Confirmation => "ui-alert-dialog--variant-confirmation",
            AlertDialogVariant::Destructive => "ui-alert-dialog--variant-destructive",
            AlertDialogVariant::Warning => "ui-alert-dialog--variant-warning",
            AlertDialogVariant::Error => "ui-alert-dialog--variant-error",
        }
    }

    pub fn data_attr(self) -> &'static str {
        match self {
            AlertDialogVariant::Default => "default",
            AlertDialogVariant::Confirmation => "confirmation",
            AlertDialogVariant::Destructive => "destructive",
            AlertDialogVariant::Warning => "warning",
            AlertDialogVariant::Error => "error",
        }
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
        match self {
            AlertDialogAutoFocusButton::None => "none",
            AlertDialogAutoFocusButton::Cancel => "cancel",
            AlertDialogAutoFocusButton::Secondary => "secondary",
            AlertDialogAutoFocusButton::Confirm => "confirm",
        }
    }
}

pub const DEFAULT_ID_BASE: &str = "ui-alert-dialog";
pub const DEFAULT_TITLE: &str = "Alert";
pub const DEFAULT_CONFIRM_LABEL: &str = "Confirm";
pub const DEFAULT_CANCEL_LABEL: &str = "Cancel";
pub const DEFAULT_CONFIRM_DISABLED: bool = false;
pub const DEFAULT_SECONDARY_DISABLED: bool = false;

pub fn state_attr(is_open: bool) -> &'static str {
    if is_open { "open" } else { "closed" }
}

pub fn description_attr(show_description: bool) -> &'static str {
    if show_description {
        "present"
    } else {
        "absent"
    }
}

pub fn action_visibility_attr(show: bool) -> &'static str {
    if show { "shown" } else { "hidden" }
}

pub fn disabled_attr(disabled: bool) -> &'static str {
    if disabled { "true" } else { "false" }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_required_text(value: String, fallback: &'static str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        fallback.into()
    } else {
        trimmed.into()
    }
}

pub fn normalize_id_base(value: String) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        DEFAULT_ID_BASE.into()
    } else {
        trimmed.into()
    }
}

pub fn normalize_cancel_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_CANCEL_LABEL.into())
}

pub fn normalize_secondary_label(value: Option<String>) -> Option<String> {
    normalize_optional_text(value)
}

pub fn resolve_disabled_flag(
    is_disabled: Option<bool>,
    legacy_disabled: Option<bool>,
    default_value: bool,
) -> bool {
    is_disabled.or(legacy_disabled).unwrap_or(default_value)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertDialogStateCoreInput {
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
pub struct AlertDialogStateCore {
    pub state_attr: &'static str,
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

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state_core(input: AlertDialogStateCoreInput) -> AlertDialogStateCore {
    let has_custom_variant = input.variant != AlertDialogVariant::Default;
    let has_custom_cancel = input.has_custom_cancel_label || input.has_custom_on_cancel;
    let has_custom_secondary = input.has_custom_secondary_label
        || input.has_custom_on_secondary
        || input.secondary_disabled != DEFAULT_SECONDARY_DISABLED;
    let has_custom_confirm =
        input.has_custom_confirm_label || input.confirm_disabled != DEFAULT_CONFIRM_DISABLED;

    AlertDialogStateCore {
        state_attr: state_attr(input.is_open),
        variant_attr: input.variant.data_attr(),
        variant_class: input.variant.class_name(),
        description_attr: description_attr(input.show_description),
        cancel_attr: action_visibility_attr(input.show_cancel),
        secondary_attr: action_visibility_attr(input.show_secondary),
        confirm_disabled_attr: disabled_attr(input.confirm_disabled),
        secondary_disabled_attr: disabled_attr(input.secondary_disabled),
        auto_focus_attr: input.auto_focus_button.as_attr(),
        show_description: input.show_description,
        show_cancel: input.show_cancel,
        show_secondary: input.show_secondary,
        show_type_icon: matches!(
            input.variant,
            AlertDialogVariant::Warning | AlertDialogVariant::Error
        ),
        confirm_disabled: input.confirm_disabled,
        secondary_disabled: input.secondary_disabled,
        has_custom_variant,
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
        variant_source_attr: source_attr(has_custom_variant),
        description_source_attr: source_attr(input.has_custom_description),
        cancel_source_attr: source_attr(has_custom_cancel),
        secondary_source_attr: source_attr(has_custom_secondary),
        confirm_source_attr: source_attr(has_custom_confirm),
        id_source_attr: source_attr(input.has_custom_id_base),
        title_source_attr: source_attr(input.has_custom_title),
        auto_focus_source_attr: source_attr(input.has_custom_auto_focus_button),
        motion_source_attr: source_attr(input.has_custom_motion),
        exit_source_attr: source_attr(input.has_on_exit_complete),
    }
}

#[cfg(test)]
#[path = "test/alert_dialog.rs"]
mod tests;
