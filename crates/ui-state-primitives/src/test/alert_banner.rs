use super::*;

#[test]
fn variant_maps_to_expected_tone() {
    assert_eq!(
        AlertBannerVariant::Default.as_tone(),
        AlertBannerTone::Neutral
    );
    assert_eq!(AlertBannerVariant::Accent.as_tone(), AlertBannerTone::Info);
    assert_eq!(
        AlertBannerVariant::Danger.as_tone(),
        AlertBannerTone::Negative
    );
}

#[test]
fn fill_attr_values_are_closed_set() {
    assert_eq!(AlertBannerFill::Border.attr_value(), "border");
    assert_eq!(AlertBannerFill::Subtle.attr_value(), "subtle");
    assert_eq!(AlertBannerFill::Bold.attr_value(), "bold");
}

#[test]
fn resolve_tone_prefers_tone_then_variant_then_default() {
    let explicit = resolve_tone(
        Some(AlertBannerTone::Notice),
        Some(AlertBannerVariant::Danger),
    );
    assert_eq!(
        explicit,
        (AlertBannerTone::Notice, AlertBannerToneSource::Tone)
    );

    let from_variant = resolve_tone(None, Some(AlertBannerVariant::Danger));
    assert_eq!(
        from_variant,
        (AlertBannerTone::Negative, AlertBannerToneSource::Variant)
    );

    let defaulted = resolve_tone(None, None);
    assert_eq!(
        defaulted,
        (AlertBannerTone::Neutral, AlertBannerToneSource::Default)
    );
}

#[test]
fn normalize_fill_defaults_to_border() {
    assert_eq!(normalize_fill(None), AlertBannerFill::Border);
    assert_eq!(
        normalize_fill(Some(AlertBannerFill::Bold)),
        AlertBannerFill::Bold
    );
}

#[test]
fn resolve_hide_icon_prefers_is_hide_icon_then_legacy_then_default() {
    assert_eq!(
        resolve_hide_icon(Some(true), Some(false)),
        AlertBannerHideIcon {
            value: true,
            source: AlertBannerHideIconSource::IsHideIcon,
        }
    );
    assert_eq!(
        resolve_hide_icon(None, Some(true)),
        AlertBannerHideIcon {
            value: true,
            source: AlertBannerHideIconSource::HideIcon,
        }
    );
    assert_eq!(
        resolve_hide_icon(None, None),
        AlertBannerHideIcon {
            value: false,
            source: AlertBannerHideIconSource::Default,
        }
    );
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
