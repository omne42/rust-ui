use super::*;

#[test]
fn variant_and_size_contracts_are_stable() {
    assert_eq!(
        CloseButtonVariant::Default.class_name(),
        "ui-close-button--variant-default"
    );
    assert_eq!(
        CloseButtonVariant::OverBackground.class_name(),
        "ui-close-button--variant-over-background"
    );
    assert_eq!(CloseButtonVariant::Default.as_attr(), "default");
    assert_eq!(
        CloseButtonVariant::OverBackground.as_attr(),
        "over-background"
    );

    assert_eq!(CloseButtonSize::Sm.class_name(), "ui-close-button--size-sm");
    assert_eq!(CloseButtonSize::Md.class_name(), "ui-close-button--size-md");
    assert_eq!(CloseButtonSize::Lg.class_name(), "ui-close-button--size-lg");
    assert_eq!(CloseButtonSize::Xl.class_name(), "ui-close-button--size-xl");

    assert_eq!(CloseButtonSize::Sm.as_attr(), "sm");
    assert_eq!(CloseButtonSize::Md.as_attr(), "md");
    assert_eq!(CloseButtonSize::Lg.as_attr(), "lg");
    assert_eq!(CloseButtonSize::Xl.as_attr(), "xl");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-close-button  ".to_string())),
        Some("docs-close-button".to_string())
    );

    let (aria_label, custom) = normalize_aria_label(Some("  Dismiss panel  ".to_string()), "Close");
    assert_eq!(aria_label, "Dismiss panel");
    assert!(custom);

    let (aria_label, custom) = normalize_aria_label(None, "Close");
    assert_eq!(aria_label, "Close");
    assert!(!custom);
}

#[test]
fn resolve_state_tracks_variant_size_and_sources() {
    let state = resolve_state(CloseButtonStateInput {
        variant: CloseButtonVariant::OverBackground,
        size: CloseButtonSize::Lg,
        disabled: false,
        has_custom_aria_label: true,
        has_custom_class_name: false,
        has_custom_press_handler: true,
    });

    assert_eq!(state.variant_attr, "over-background");
    assert_eq!(state.size_attr, "lg");
    assert!(!state.is_disabled);
    assert_eq!(state.data_state_attr, "ready");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}
