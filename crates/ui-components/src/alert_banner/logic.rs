pub use ui_state_primitives::alert_banner::{
    AlertBannerFill, AlertBannerTone, normalize_optional_text, resolve_view_state,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_primitives_are_reexported_from_ui_state_primitives() {
        assert_eq!(
            normalize_optional_text(Some("  docs-alert-banner  ".to_string())),
            Some("docs-alert-banner".to_string())
        );

        let state = resolve_view_state(AlertBannerTone::Info, Some("  "), Some("ok"), false);
        assert!(!state.show_title);
        assert!(state.show_description);
        assert!(state.show_icon);
    }
}
