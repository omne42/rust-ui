#[derive(Clone, Debug, Default, PartialEq)]
pub struct IllustratedMessageViewState {
    pub show_illustration: bool,
    pub show_title: bool,
    pub show_description: bool,
    pub show_actions: bool,
}

pub fn resolve_view_state(
    has_illustration: bool,
    title: Option<&str>,
    description: Option<&str>,
    has_actions: bool,
) -> IllustratedMessageViewState {
    let show_title = title.is_some_and(|v| !v.trim().is_empty());
    let show_description = description.is_some_and(|v| !v.trim().is_empty());

    IllustratedMessageViewState {
        show_illustration: has_illustration,
        show_title,
        show_description,
        show_actions: has_actions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_flags() {
        let state = resolve_view_state(true, Some("Hello"), Some("World"), true);
        assert!(state.show_illustration);
        assert!(state.show_title);
        assert!(state.show_description);
        assert!(state.show_actions);

        let state = resolve_view_state(false, Some(" "), None, false);
        assert!(!state.show_illustration);
        assert!(!state.show_title);
        assert!(!state.show_description);
        assert!(!state.show_actions);
    }
}
