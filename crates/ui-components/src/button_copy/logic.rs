#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonCopyViewState {
    pub is_copyable: bool,
}

pub fn resolve_view_state(text: &str, disabled: bool) -> ButtonCopyViewState {
    let is_copyable = !disabled && !text.trim().is_empty();
    ButtonCopyViewState { is_copyable }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text_is_not_copyable() {
        assert!(!resolve_view_state("", false).is_copyable);
        assert!(!resolve_view_state("   ", false).is_copyable);
    }

    #[test]
    fn disabled_is_not_copyable_even_when_text_present() {
        assert!(!resolve_view_state("hello", true).is_copyable);
    }

    #[test]
    fn enabled_with_text_is_copyable() {
        assert!(resolve_view_state("hello", false).is_copyable);
    }
}
