use super::*;

#[test]
fn variant_and_size_contract_are_stable() {
    assert_eq!(TagVariant::Default.class_name(), "ui-tag--variant-default");
    assert_eq!(TagVariant::Surface.class_name(), "ui-tag--variant-surface");
    assert_eq!(TagVariant::Default.as_attr(), "default");
    assert_eq!(TagVariant::Surface.as_attr(), "surface");

    assert_eq!(TagSize::Sm.class_name(), "ui-tag--size-sm");
    assert_eq!(TagSize::Md.class_name(), "ui-tag--size-md");
    assert_eq!(TagSize::Lg.class_name(), "ui-tag--size-lg");

    assert_eq!(TagSize::Sm.as_attr(), "sm");
    assert_eq!(TagSize::Md.as_attr(), "md");
    assert_eq!(TagSize::Lg.as_attr(), "lg");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  release-ready  ".to_string())),
        Some("release-ready".to_string())
    );

    assert_eq!(
        normalize_remove_aria_label(Some("  Remove framework  ".to_string())),
        ("Remove framework".to_string(), true)
    );
    assert_eq!(
        normalize_remove_aria_label(None),
        (DEFAULT_REMOVE_ARIA_LABEL.into(), false)
    );
}

#[test]
fn interactivity_mode_normalization_is_stable_and_prioritizes_explicit_mode() {
    assert_eq!(
        normalize_interactivity_mode(TagInteractivityModeInput {
            mode: Some(TagInteractivityMode::Removable),
            is_disabled: Some(true),
            is_removable: Some(false),
        }),
        TagInteractivityMode::Removable
    );

    assert_eq!(
        normalize_interactivity_mode(TagInteractivityModeInput {
            mode: None,
            is_disabled: Some(true),
            is_removable: Some(true),
        }),
        TagInteractivityMode::Disabled
    );

    assert_eq!(
        normalize_interactivity_mode(TagInteractivityModeInput {
            mode: None,
            is_disabled: Some(false),
            is_removable: Some(true),
        }),
        TagInteractivityMode::Removable
    );

    assert_eq!(
        normalize_interactivity_mode(TagInteractivityModeInput {
            mode: None,
            is_disabled: None,
            is_removable: None,
        }),
        TagInteractivityMode::Static
    );
}

#[test]
fn resolve_state_tracks_removable_and_source_markers() {
    let removable_state = resolve_state(TagStateInput {
        variant: TagVariant::Surface,
        size: TagSize::Lg,
        disabled: false,
        removable: true,
        has_remove_handler: true,
        has_custom_remove_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(removable_state.state_attr, "removable");
    assert!(removable_state.is_enabled);
    assert!(removable_state.is_removable);
    assert!(!removable_state.is_static);
    assert_eq!(removable_state.remove_label_source_attr, "custom");
    assert_eq!(removable_state.class_source_attr, "custom");

    let disabled_state = resolve_state(TagStateInput {
        variant: TagVariant::Default,
        size: TagSize::Md,
        disabled: true,
        removable: true,
        has_remove_handler: true,
        has_custom_remove_aria_label: false,
        has_custom_class_name: false,
    });

    assert_eq!(disabled_state.state_attr, "disabled");
    assert!(disabled_state.is_disabled);
    assert!(!disabled_state.is_enabled);
}
