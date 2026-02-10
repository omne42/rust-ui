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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertDialogViewState {
    pub show_description: bool,
    pub show_cancel: bool,
    pub show_secondary: bool,
}

pub fn resolve_view_state(
    description: Option<&str>,
    cancel_label: &str,
    secondary_label: Option<&str>,
) -> AlertDialogViewState {
    AlertDialogViewState {
        show_description: description.is_some_and(|v| !v.trim().is_empty()),
        show_cancel: !cancel_label.trim().is_empty(),
        show_secondary: secondary_label.is_some_and(|v| !v.trim().is_empty()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancel_is_visible_when_label_has_content() {
        let state = resolve_view_state(Some("x"), "Cancel", None);
        assert!(state.show_cancel);
        let state = resolve_view_state(Some("x"), " ", None);
        assert!(!state.show_cancel);
    }

    #[test]
    fn secondary_is_visible_when_label_is_present_and_non_empty() {
        let state = resolve_view_state(Some("x"), "Cancel", Some("Save draft"));
        assert!(state.show_secondary);
        let state = resolve_view_state(Some("x"), "Cancel", Some(" "));
        assert!(!state.show_secondary);
        let state = resolve_view_state(Some("x"), "Cancel", None);
        assert!(!state.show_secondary);
    }
}
