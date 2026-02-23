use super::*;
use crate::PreviewCardMotion;
use crate::{PreviewCardPartStateInput, PreviewCardSiteLabelSource};

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  preview card  ".to_string())),
        Some("preview card".to_string())
    );
}

#[test]
fn resolution_helpers_track_custom_and_default_sources() {
    assert_eq!(
        resolve_id(None, "generated-id".to_string()),
        ("generated-id".to_string(), false)
    );
    assert_eq!(
        resolve_id(Some(" docs-preview ".to_string()), "fallback".to_string()),
        ("docs-preview".to_string(), true)
    );

    assert_eq!(resolve_title(None), (DEFAULT_TITLE.into(), false));
    assert_eq!(
        resolve_title(Some("  GitHub  ".to_string())),
        ("GitHub".to_string(), true)
    );

    assert_eq!(
        resolve_description(Some("  Commit activity  ".to_string())),
        ("Commit activity".to_string(), true)
    );
    assert_eq!(
        resolve_description(None),
        (DEFAULT_DESCRIPTION.into(), false)
    );

    assert_eq!(resolve_url(None), (DEFAULT_URL.into(), false));
    assert_eq!(
        resolve_url(Some(" https://github.com/adobe/ui-baseline ".to_string())),
        ("https://github.com/adobe/ui-baseline".to_string(), true)
    );
}

#[test]
fn resolve_site_label_supports_custom_derived_and_default_paths() {
    assert_eq!(
        resolve_site_label(Some(" Baseline ".to_string()), "https://example.com"),
        ("Baseline".to_string(), PreviewCardSiteLabelSource::Custom)
    );

    assert_eq!(
        resolve_site_label(None, "https://www.github.com/adobe/ui-baseline"),
        (
            "github.com".to_string(),
            PreviewCardSiteLabelSource::Derived
        )
    );

    assert_eq!(
        resolve_site_label(None, "   "),
        (
            DEFAULT_SITE_LABEL.into(),
            PreviewCardSiteLabelSource::Default
        )
    );
}

#[test]
fn resolve_part_state_tracks_slot_content_and_sources() {
    let root = resolve_part_state(PreviewCardPartStateInput {
        slot: PreviewCardSlot::Root,
        is_disabled: false,
        has_image: true,
        has_custom_class_name: true,
        has_custom_delays: true,
        has_custom_id: true,
        has_custom_title: true,
        has_custom_description: true,
        has_custom_url: true,
        site_label_source: PreviewCardSiteLabelSource::Derived,
        has_custom_motion: true,
    });

    assert_eq!(root.slot_attr, "preview-card");
    assert_eq!(root.content_attr, "media");
    assert_eq!(root.class_source_attr, "custom");
    assert_eq!(root.delay_source_attr, "custom");
    assert_eq!(root.site_label_source, PreviewCardSiteLabelSource::Derived);
    assert_eq!(root.motion_source_attr, "custom");

    let trigger = resolve_part_state(PreviewCardPartStateInput {
        slot: PreviewCardSlot::Trigger,
        is_disabled: false,
        has_image: true,
        has_custom_class_name: false,
        has_custom_delays: false,
        has_custom_id: false,
        has_custom_title: false,
        has_custom_description: false,
        has_custom_url: false,
        site_label_source: PreviewCardSiteLabelSource::Default,
        has_custom_motion: false,
    });

    assert_eq!(trigger.state_attr, "trigger");
    assert_eq!(trigger.content_attr, "trigger");
}

#[test]
fn compose_class_name_includes_custom_and_content_markers() {
    let class_name = compose_class_name(
        Some("docs-preview-card".to_string()),
        resolve_part_state(PreviewCardPartStateInput {
            slot: PreviewCardSlot::Root,
            is_disabled: false,
            has_image: true,
            has_custom_class_name: true,
            has_custom_delays: true,
            has_custom_id: true,
            has_custom_title: true,
            has_custom_description: true,
            has_custom_url: true,
            site_label_source: PreviewCardSiteLabelSource::Derived,
            has_custom_motion: true,
        }),
    );

    for token in [
        "ui-preview-card",
        "ui-preview-card--enabled",
        "ui-preview-card--media",
        "ui-preview-card--custom-class",
        "ui-preview-card--custom-delay",
        "ui-preview-card--custom-motion",
        "ui-preview-card--custom-id",
        "ui-preview-card--custom-title",
        "ui-preview-card--custom-description",
        "ui-preview-card--custom-url",
        "docs-preview-card",
    ] {
        assert!(
            class_name.contains(token),
            "preview card class name should include `{token}`"
        );
    }
}

#[test]
fn misc_helpers_keep_contracts_stable() {
    assert_eq!(state_attr_for_open(true), "open");
    assert_eq!(state_attr_for_open(false), "closed");
    assert_eq!(content_attr(true), "media");
    assert_eq!(content_attr(false), "text");

    assert!(!has_custom_delays(
        DEFAULT_OPEN_DELAY_MS,
        DEFAULT_CLOSE_DELAY_MS
    ));
    assert!(has_custom_delays(
        DEFAULT_OPEN_DELAY_MS + 1,
        DEFAULT_CLOSE_DELAY_MS
    ));

    assert_eq!(
        compose_panel_vars(12.5, 24.0, 180.0),
        "--ui-preview-card-top: 12.5px; --ui-preview-card-left: 24px; --ui-preview-card-anchor-width: 180px;"
    );
}

#[test]
fn runtime_options_resolve_defaults_and_custom_overrides() {
    let defaults = resolve_runtime_options(PreviewCardRuntimeOptionsInput {
        is_disabled: None,
        placement: None,
        open_delay_ms: None,
        close_delay_ms: None,
        motion: None,
    });
    assert_eq!(defaults.is_disabled, DEFAULT_DISABLED);
    assert_eq!(
        defaults.placement,
        ui_headless::PopoverPlacement::BottomStart
    );
    assert_eq!(defaults.open_delay_ms, DEFAULT_OPEN_DELAY_MS);
    assert_eq!(defaults.close_delay_ms, DEFAULT_CLOSE_DELAY_MS);
    assert_eq!(defaults.motion, PreviewCardMotion::default());
    assert!(!defaults.has_custom_delays);
    assert!(!defaults.has_custom_motion);

    let custom_motion = PreviewCardMotion {
        initial_scale: 0.95,
        offset_y_px: 12.0,
        ..PreviewCardMotion::default()
    };
    let custom = resolve_runtime_options(PreviewCardRuntimeOptionsInput {
        is_disabled: Some(true),
        placement: Some(ui_headless::PopoverPlacement::TopEnd),
        open_delay_ms: Some(220),
        close_delay_ms: Some(200),
        motion: Some(custom_motion),
    });
    assert!(custom.is_disabled);
    assert_eq!(custom.placement, ui_headless::PopoverPlacement::TopEnd);
    assert_eq!(custom.open_delay_ms, 220);
    assert_eq!(custom.close_delay_ms, 200);
    assert_eq!(custom.motion, custom_motion);
    assert!(custom.has_custom_delays);
    assert!(custom.has_custom_motion);
}

#[test]
fn state_model_derives_all_slots_from_single_input() {
    let model = resolve_state_model(PreviewCardStateModelInput {
        class_name: Some(" docs-preview-card ".to_string()),
        is_disabled: true,
        has_image: true,
        has_custom_delays: true,
        has_custom_id: true,
        has_custom_title: true,
        has_custom_description: true,
        has_custom_url: true,
        site_label_source: PreviewCardSiteLabelSource::Derived,
        has_custom_motion: true,
    });

    assert_eq!(model.root_state.slot, PreviewCardSlot::Root);
    assert_eq!(model.trigger_state.slot, PreviewCardSlot::Trigger);
    assert_eq!(model.panel_state.slot, PreviewCardSlot::Panel);

    assert_eq!(
        model.root_state.site_label_source,
        PreviewCardSiteLabelSource::Derived
    );
    assert_eq!(
        model.trigger_state.site_label_source,
        PreviewCardSiteLabelSource::Derived
    );
    assert_eq!(
        model.panel_state.site_label_source,
        PreviewCardSiteLabelSource::Derived
    );

    assert!(model.root_class.contains("ui-preview-card--disabled"));
    assert!(model.root_class.contains("docs-preview-card"));
    assert!(model.trigger_class.contains("ui-preview-card__trigger"));
    assert!(model.panel_class.contains("ui-preview-card__panel"));
}

#[test]
fn image_source_normalization_trims_and_drops_blank_values() {
    assert_eq!(resolve_image_src(None), None);
    assert_eq!(resolve_image_src(Some("  ".to_string())), None);
    assert_eq!(
        resolve_image_src(Some(" https://example.com/preview.png ".to_string())),
        Some("https://example.com/preview.png".to_string())
    );
}
