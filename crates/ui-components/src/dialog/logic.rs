#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DialogSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl DialogSize {
    pub fn class_name(self) -> &'static str {
        match self {
            DialogSize::Sm => "ui-dialog--size-sm",
            DialogSize::Md => "ui-dialog--size-md",
            DialogSize::Lg => "ui-dialog--size-lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogViewState {
    pub show_description: bool,
    pub show_footer: bool,
    pub show_close_button: bool,
}

pub fn resolve_view_state(
    description: Option<&str>,
    has_footer: bool,
    show_close_button: bool,
) -> DialogViewState {
    DialogViewState {
        show_description: description.is_some_and(|v| !v.trim().is_empty()),
        show_footer: has_footer,
        show_close_button,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn description_requires_non_empty_text() {
        let state = resolve_view_state(Some(" "), false, true);
        assert!(!state.show_description);

        let state = resolve_view_state(Some("Hello"), false, true);
        assert!(state.show_description);
    }
}
