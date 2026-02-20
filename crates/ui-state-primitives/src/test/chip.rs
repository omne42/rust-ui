use super::*;

#[test]
fn variant_and_size_mappings_are_stable() {
    assert_eq!(
        ChipVariant::Default.class_name(),
        "ui-chip--variant-default"
    );
    assert_eq!(ChipVariant::Accent.class_name(), "ui-chip--variant-accent");
    assert_eq!(ChipVariant::Danger.class_name(), "ui-chip--variant-danger");
    assert_eq!(
        ChipVariant::Outline.class_name(),
        "ui-chip--variant-outline"
    );

    assert_eq!(ChipVariant::Default.as_str(), "default");
    assert_eq!(ChipVariant::Accent.as_str(), "accent");
    assert_eq!(ChipVariant::Danger.as_str(), "danger");
    assert_eq!(ChipVariant::Outline.as_str(), "outline");

    assert_eq!(ChipSize::Sm.class_name(), "ui-chip--size-sm");
    assert_eq!(ChipSize::Md.class_name(), "ui-chip--size-md");
    assert_eq!(ChipSize::Lg.class_name(), "ui-chip--size-lg");

    assert_eq!(ChipSize::Sm.as_str(), "sm");
    assert_eq!(ChipSize::Md.as_str(), "md");
    assert_eq!(ChipSize::Lg.as_str(), "lg");
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  custom-chip  ".to_string())),
        Some("custom-chip".to_string())
    );
}

#[test]
fn resolve_dismiss_aria_label_defaults_and_trims() {
    assert_eq!(
        resolve_dismiss_aria_label(None),
        (DEFAULT_DISMISS_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        resolve_dismiss_aria_label(Some(" Dismiss assignee ".to_string())),
        ("Dismiss assignee".to_string(), true)
    );
}

#[test]
fn resolve_state_tracks_variant_size_and_sources() {
    let removable = resolve_state(ChipStateInput {
        variant: ChipVariant::Danger,
        size: ChipSize::Lg,
        disabled: false,
        has_dismiss_action: true,
        has_custom_dismiss_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(removable.variant, ChipVariant::Danger);
    assert_eq!(removable.size, ChipSize::Lg);
    assert_eq!(removable.variant_class, "ui-chip--variant-danger");
    assert_eq!(removable.size_class, "ui-chip--size-lg");
    assert_eq!(removable.variant_attr, "danger");
    assert_eq!(removable.size_attr, "lg");
    assert_eq!(removable.state_class, "ui-chip--removable");
    assert_eq!(removable.state_attr, "removable");
    assert!(!removable.is_disabled);
    assert!(removable.is_enabled);
    assert!(removable.has_dismiss_action);
    assert!(!removable.is_static);
    assert!(removable.has_custom_dismiss_aria_label);
    assert_eq!(
        removable.dismiss_label_source_class,
        "ui-chip--dismiss-label-custom"
    );
    assert_eq!(removable.dismiss_label_source_attr, "custom");
    assert!(removable.has_custom_class_name);
    assert_eq!(removable.class_source_attr, "custom");

    let disabled = resolve_state(ChipStateInput {
        variant: ChipVariant::Default,
        size: ChipSize::Md,
        disabled: true,
        has_dismiss_action: true,
        has_custom_dismiss_aria_label: false,
        has_custom_class_name: false,
    });

    assert_eq!(disabled.state_class, "ui-chip--disabled");
    assert_eq!(disabled.state_attr, "disabled");
    assert_eq!(
        disabled.dismiss_label_source_class,
        "ui-chip--dismiss-label-default"
    );
    assert_eq!(disabled.dismiss_label_source_attr, "default");
    assert_eq!(disabled.class_source_attr, "default");
}
