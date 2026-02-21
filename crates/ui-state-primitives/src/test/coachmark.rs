use super::*;

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-coachmark ".to_string())),
        Some("docs-coachmark".to_string())
    );
}

#[test]
fn compose_heading_appends_shortcut_keys() {
    assert_eq!(
        compose_heading(
            Some("Keyboard shortcuts".to_string()),
            vec!["Ctrl".to_string(), "K".to_string()],
            None,
        ),
        "Keyboard shortcuts (Ctrl + K)"
    );
    assert_eq!(compose_heading(None, vec![], None), DEFAULT_TITLE);
}

#[test]
fn compose_step_label_requires_multi_step_context() {
    assert_eq!(
        compose_step_label(Some(2), Some(5)),
        Some("2 of 5".to_string())
    );
    assert_eq!(compose_step_label(Some(1), Some(1)), None);
    assert_eq!(compose_step_label(Some(0), Some(8)), None);
}

#[test]
fn resolve_cta_mode_prefers_typed_discrete_states() {
    assert_eq!(resolve_cta_mode(None, None), CoachmarkCtaMode::None);
    assert_eq!(
        resolve_cta_mode(Some("Continue"), None),
        CoachmarkCtaMode::Primary
    );
    assert_eq!(
        resolve_cta_mode(None, Some("Later")),
        CoachmarkCtaMode::Secondary
    );
    assert_eq!(
        resolve_cta_mode(Some("Continue"), Some("Later")),
        CoachmarkCtaMode::Dual
    );
}

#[test]
fn resolve_asset_source_prefers_image_over_variant() {
    assert_eq!(resolve_asset_source(None, None), CoachmarkAssetSource::None);
    assert_eq!(
        resolve_asset_source(Some(crate::asset::AssetVariant::Folder), None),
        CoachmarkAssetSource::Variant
    );
    assert_eq!(
        resolve_asset_source(None, Some("https://example.com/demo.png")),
        CoachmarkAssetSource::Image
    );
    assert_eq!(
        resolve_asset_source(
            Some(crate::asset::AssetVariant::Folder),
            Some("https://example.com/demo.png")
        ),
        CoachmarkAssetSource::Image
    );
}

#[test]
fn resolve_state_tracks_sources_and_markers() {
    let state = resolve_state(CoachmarkStateInput {
        variant_attr: "info",
        placement_attr: "top-end",
        disabled: false,
        is_controlled: true,
        has_footer: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        has_shortcut: true,
        cta_mode: CoachmarkCtaMode::Dual,
        has_actions_slot: true,
        has_step_label: true,
        asset_source: CoachmarkAssetSource::Image,
    });

    assert_eq!(state.variant_attr, "info");
    assert_eq!(state.placement_attr, "top-end");
    assert_eq!(state.open_mode_attr, "controlled");
    assert_eq!(state.footer_attr, "present");
    assert_eq!(state.cta_attr, "dual");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.asset_source_attr, "image");
}

#[test]
fn compose_class_name_exposes_state_markers() {
    let state = resolve_state(CoachmarkStateInput {
        variant_attr: "help",
        placement_attr: "bottom-start",
        disabled: true,
        is_controlled: false,
        has_footer: false,
        has_custom_aria_label: false,
        has_custom_class_name: true,
        has_shortcut: false,
        cta_mode: CoachmarkCtaMode::None,
        has_actions_slot: false,
        has_step_label: false,
        asset_source: CoachmarkAssetSource::None,
    });

    let class_name = compose_class_name(Some("docs-coachmark".to_string()), state);
    for token in [
        "ui-coachmark",
        "ui-coachmark--variant-help",
        "ui-coachmark--placement-bottom-start",
        "ui-coachmark--state-disabled",
        "ui-coachmark--mode-uncontrolled",
        "ui-coachmark--custom-class",
        "docs-coachmark",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
