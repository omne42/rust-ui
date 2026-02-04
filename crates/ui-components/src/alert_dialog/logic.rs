#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertDialogVariant {
    #[default]
    Default,
    Destructive,
}

impl AlertDialogVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            AlertDialogVariant::Default => "ui-alert-dialog--variant-default",
            AlertDialogVariant::Destructive => "ui-alert-dialog--variant-destructive",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertDialogViewState {
    pub show_description: bool,
    pub show_cancel: bool,
}

pub fn resolve_view_state(description: Option<&str>, cancel_label: &str) -> AlertDialogViewState {
    AlertDialogViewState {
        show_description: description.is_some_and(|v| !v.trim().is_empty()),
        show_cancel: !cancel_label.trim().is_empty(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancel_is_visible_when_label_has_content() {
        let state = resolve_view_state(Some("x"), "Cancel");
        assert!(state.show_cancel);
        let state = resolve_view_state(Some("x"), " ");
        assert!(!state.show_cancel);
    }
}
