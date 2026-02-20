use super::*;

#[test]
fn variant_maps_to_expected_tone() {
    assert_eq!(AlertVariant::Default.as_tone(), AlertTone::Neutral);
    assert_eq!(AlertVariant::Accent.as_tone(), AlertTone::Info);
    assert_eq!(AlertVariant::Danger.as_tone(), AlertTone::Negative);
}

#[test]
fn resolve_state_prefers_tone_then_variant_then_default() {
    let tone = resolve_state(AlertStateInput {
        tone: Some(AlertTone::Notice),
        variant: Some(AlertVariant::Danger),
        layout: None,
        fill: None,
        has_title: true,
        has_description: true,
        hide_icon: false,
        has_custom_class_name: false,
    });
    assert_eq!(tone.tone, AlertTone::Notice);
    assert_eq!(tone.variant_source_attr, "tone");

    let variant = resolve_state(AlertStateInput {
        tone: None,
        variant: Some(AlertVariant::Danger),
        layout: None,
        fill: None,
        has_title: false,
        has_description: false,
        hide_icon: false,
        has_custom_class_name: false,
    });
    assert_eq!(variant.tone, AlertTone::Negative);
    assert_eq!(variant.variant_source_attr, "variant");

    let default = resolve_state(AlertStateInput {
        tone: None,
        variant: None,
        layout: Some(AlertLayout::Inline),
        fill: None,
        has_title: false,
        has_description: false,
        hide_icon: false,
        has_custom_class_name: false,
    });
    assert_eq!(default.tone, AlertTone::Neutral);
    assert_eq!(default.variant_source_attr, "default");
    assert_eq!(default.layout_attr, "inline");
}

#[test]
fn compose_class_name_includes_layout_tone_fill_and_custom_markers() {
    let class_name = compose_class_name(
        Some("docs-alert-custom".to_string()),
        resolve_state(AlertStateInput {
            tone: Some(AlertTone::Info),
            variant: None,
            layout: Some(AlertLayout::Inline),
            fill: Some(AlertFill::Bold),
            has_title: false,
            has_description: true,
            hide_icon: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-alert",
        "ui-alert--layout-inline",
        "ui-alert--tone-info",
        "ui-alert--fill-bold",
        "ui-alert--custom-class",
        "docs-alert-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn resolve_hide_icon_prefers_is_hide_icon_then_legacy_then_default() {
    assert_eq!(
        resolve_hide_icon(Some(true), Some(false)),
        (true, "is-hide-icon")
    );
    assert_eq!(resolve_hide_icon(None, Some(true)), (true, "hide-icon"));
    assert_eq!(resolve_hide_icon(None, None), (false, "default"));
}
