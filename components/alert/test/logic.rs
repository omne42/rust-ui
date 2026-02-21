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

#[test]
fn resolve_state_maps_live_region_via_headless_priority() {
    let negative = resolve_state(AlertStateInput {
        tone: Some(AlertTone::Negative),
        variant: None,
        layout: None,
        fill: None,
        has_title: false,
        has_description: false,
        hide_icon: false,
        has_custom_class_name: false,
    });
    assert_eq!(negative.role_attr, "alert");
    assert_eq!(negative.live_attr, "assertive");

    let info = resolve_state(AlertStateInput {
        tone: Some(AlertTone::Info),
        variant: None,
        layout: None,
        fill: None,
        has_title: false,
        has_description: false,
        hide_icon: false,
        has_custom_class_name: false,
    });
    assert_eq!(info.role_attr, "status");
    assert_eq!(info.live_attr, "polite");
}

#[test]
fn resolve_icon_label_prefers_custom_then_tone_default_then_empty() {
    let custom = resolve_icon_label(Some("  custom icon label  ".to_string()), AlertTone::Info);
    assert_eq!(
        custom,
        (
            "custom icon label".to_string(),
            AlertIconLabelSource::Custom
        )
    );

    let tone_default = resolve_icon_label(None, AlertTone::Info);
    assert_eq!(
        tone_default,
        ("Info".to_string(), AlertIconLabelSource::ToneDefault)
    );

    let none = resolve_icon_label(None, AlertTone::Neutral);
    assert_eq!(none, (String::new(), AlertIconLabelSource::None));
}

#[test]
fn agent_contract_fields_are_enum_typed_and_closed() {
    assert_eq!(AlertAgentSchema::V1.as_attr(), "alert.v1");
    assert_eq!(AlertAgentIntent::StatusRegion.as_attr(), "status-region");
    assert_eq!(AlertAgentAction::Announce.as_attr(), "announce");
    assert_eq!(AlertAgentState::Snapshot.as_attr(), "snapshot");
    assert_eq!(AlertStreamingPolicy::Optional.as_attr(), "optional");
    assert_eq!(AlertStreamingPolicy::Required.as_attr(), "required");
    assert_eq!(AlertStreamingFallback::Snapshot.as_attr(), "snapshot");
    assert_eq!(AlertOutputStatus::Draft.as_attr(), "draft");
    assert_eq!(AlertOutputStatus::Verified.as_attr(), "verified");
    assert_eq!(AlertOutputStatus::Committable.as_attr(), "committable");

    assert_eq!(resolve_agent_source("tone"), AlertAgentSource::Tone);
    assert_eq!(resolve_agent_source("variant"), AlertAgentSource::Variant);
    assert_eq!(resolve_agent_source("default"), AlertAgentSource::Default);
    assert_eq!(
        resolve_agent_source("unexpected"),
        AlertAgentSource::Default
    );

    assert_eq!(resolve_motion_source(true), AlertMotionSource::Default);
    assert_eq!(resolve_motion_source(false), AlertMotionSource::Custom);
}
