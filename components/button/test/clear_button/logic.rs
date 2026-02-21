use super::*;

#[test]
fn variant_contract_is_stable() {
    assert_eq!(
        ClearButtonVariant::Default.class_name(),
        "ui-clear-button--variant-default"
    );
    assert_eq!(
        ClearButtonVariant::OverBackground.class_name(),
        "ui-clear-button--variant-over-background"
    );

    assert_eq!(ClearButtonVariant::Default.as_attr(), "default");
    assert_eq!(
        ClearButtonVariant::OverBackground.as_attr(),
        "over-background"
    );
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-clear-button  ".to_string())),
        Some("docs-clear-button".to_string())
    );

    let (aria_label, custom) =
        normalize_aria_label(Some("  Clear query  ".to_string()), DEFAULT_ARIA_LABEL);
    assert_eq!(aria_label, "Clear query");
    assert!(custom);

    let (aria_label, custom) = normalize_aria_label(None, DEFAULT_ARIA_LABEL);
    assert_eq!(aria_label, DEFAULT_ARIA_LABEL);
    assert!(!custom);
}

#[test]
fn resolve_state_tracks_variant_focus_mode_and_sources() {
    let state = resolve_state(ClearButtonStateInput {
        variant: ClearButtonVariant::OverBackground,
        inset: true,
        disabled: false,
        focus_mode: ClearButtonFocusMode::Prevent,
        has_custom_aria_label: true,
        has_custom_class_name: false,
        has_custom_press_handler: true,
    });

    assert_eq!(state.variant_attr, "over-background");
    assert!(state.is_inset);
    assert!(!state.is_disabled);
    assert!(state.prevent_focus);
    assert!(!state.exclude_from_tab_order);
    assert_eq!(state.data_state_attr, "prevent-focus");
    assert_eq!(state.focus_mode_attr, "prevent");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-clear-button-custom".to_string()),
        resolve_state(ClearButtonStateInput {
            variant: ClearButtonVariant::Default,
            inset: false,
            disabled: true,
            focus_mode: ClearButtonFocusMode::ExcludeTab,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_press_handler: true,
        }),
    );

    for token in [
        "ui-clear-button",
        "ui-clear-button--variant-default",
        "ui-clear-button--disabled",
        "ui-clear-button--exclude-tab",
        "ui-clear-button--custom-handler",
        "ui-clear-button--custom-class",
        "docs-clear-button-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}
