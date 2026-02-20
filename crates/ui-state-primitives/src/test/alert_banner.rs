use super::*;

#[test]
fn negative_tone_uses_alert_live_region() {
    assert_eq!(AlertBannerTone::Negative.role(), "alert");
    assert_eq!(AlertBannerTone::Negative.aria_live(), "assertive");
}

#[test]
fn non_negative_tones_use_status_live_region() {
    assert_eq!(AlertBannerTone::Neutral.role(), "status");
    assert_eq!(AlertBannerTone::Neutral.aria_live(), "polite");
    assert_eq!(AlertBannerTone::Info.role(), "status");
    assert_eq!(AlertBannerTone::Positive.role(), "status");
    assert_eq!(AlertBannerTone::Notice.role(), "status");
}

#[test]
fn neutral_defaults_to_no_icon() {
    let state = resolve_view_state(AlertBannerTone::Neutral, None, None, false);
    assert!(!state.show_icon);
}

#[test]
fn hide_icon_forces_icon_off() {
    let state = resolve_view_state(AlertBannerTone::Info, None, None, true);
    assert!(!state.show_icon);
}

#[test]
fn title_and_description_flags_respect_trimmed_content() {
    let state = resolve_view_state(AlertBannerTone::Info, Some("  "), Some("ok"), false);
    assert!(!state.show_title);
    assert!(state.show_description);
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-alert-banner ".to_string())),
        Some("docs-alert-banner".to_string())
    );
}
