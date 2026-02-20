use super::*;

#[test]
fn variant_class_names_and_attrs_are_stable() {
    assert_eq!(
        BadgeVariant::Default.class_name(),
        "ui-badge--variant-default"
    );
    assert_eq!(
        BadgeVariant::Accent.class_name(),
        "ui-badge--variant-accent"
    );
    assert_eq!(
        BadgeVariant::Danger.class_name(),
        "ui-badge--variant-danger"
    );
    assert_eq!(
        BadgeVariant::Outline.class_name(),
        "ui-badge--variant-outline"
    );

    assert_eq!(BadgeVariant::Default.as_attr(), "default");
    assert_eq!(BadgeVariant::Accent.as_attr(), "accent");
    assert_eq!(BadgeVariant::Danger.as_attr(), "danger");
    assert_eq!(BadgeVariant::Outline.as_attr(), "outline");
}

#[test]
fn variant_fill_class_and_attrs_are_stable() {
    for variant in [
        BadgeVariant::Default,
        BadgeVariant::Accent,
        BadgeVariant::Danger,
    ] {
        assert_eq!(variant.fill_class(), "ui-badge--fill-solid");
        assert_eq!(variant.fill_attr(), "solid");
    }

    assert_eq!(BadgeVariant::Outline.fill_class(), "ui-badge--fill-outline");
    assert_eq!(BadgeVariant::Outline.fill_attr(), "outline");
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-badge  ".to_string())),
        Some("docs-badge".to_string())
    );
}

#[test]
fn resolve_state_tracks_variant_fill_and_class_source() {
    let solid = resolve_state(BadgeStateInput {
        variant: BadgeVariant::Accent,
        has_custom_class_name: true,
    });

    assert_eq!(solid.variant, BadgeVariant::Accent);
    assert_eq!(solid.variant_class, "ui-badge--variant-accent");
    assert_eq!(solid.variant_attr, "accent");
    assert_eq!(solid.fill_class, "ui-badge--fill-solid");
    assert_eq!(solid.fill_attr, "solid");
    assert!(solid.is_solid);
    assert!(!solid.is_outline);
    assert!(solid.has_custom_class_name);

    let outline = resolve_state(BadgeStateInput {
        variant: BadgeVariant::Outline,
        has_custom_class_name: false,
    });

    assert!(!outline.is_solid);
    assert!(outline.is_outline);
    assert_eq!(outline.fill_attr, "outline");
}
