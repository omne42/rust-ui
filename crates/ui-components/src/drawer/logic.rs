pub type DrawerPlacement = crate::sheet::SheetPlacement;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawerViewState {
    pub show_description: bool,
    pub show_footer: bool,
    pub show_close_button: bool,
}

pub fn resolve_view_state(
    description: Option<&str>,
    has_footer: bool,
    show_close_button: bool,
) -> DrawerViewState {
    DrawerViewState {
        show_description: description.is_some_and(|v| !v.trim().is_empty()),
        show_footer: has_footer,
        show_close_button,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn footer_flag_tracks_presence() {
        let state = resolve_view_state(None, false, true);
        assert!(!state.show_footer);
        let state = resolve_view_state(None, true, true);
        assert!(state.show_footer);
    }
}
