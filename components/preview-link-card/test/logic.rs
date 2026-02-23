use super::*;

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
        (
            "Baseline".to_string(),
            PreviewLinkCardSiteLabelSourceAttr::Custom
        )
    );

    assert_eq!(
        resolve_site_label(None, "https://www.github.com/adobe/ui-baseline"),
        (
            "github.com".to_string(),
            PreviewLinkCardSiteLabelSourceAttr::Derived
        )
    );

    assert_eq!(
        resolve_site_label(None, "   "),
        (
            DEFAULT_SITE_LABEL.into(),
            PreviewLinkCardSiteLabelSourceAttr::Default
        )
    );
}

#[test]
fn resolve_part_state_tracks_slot_content_and_sources() {
    let root = resolve_part_state(PreviewLinkCardPartStateInput {
        slot: PreviewLinkCardSlot::Root,
        disabled: false,
        has_image: true,
        has_custom_class_name: true,
        has_custom_delays: true,
        has_custom_id: true,
        has_custom_title: true,
        has_custom_description: true,
        has_custom_url: true,
        site_label_source_attr: PreviewLinkCardSiteLabelSourceAttr::Derived,
        has_custom_motion: true,
    });

    assert_eq!(root.slot_attr, "preview-link-card");
    assert_eq!(root.content_attr, PreviewLinkCardContentAttr::Media);
    assert_eq!(root.class_source_attr, PreviewLinkCardSourceAttr::Custom);
    assert_eq!(root.delay_source_attr, PreviewLinkCardSourceAttr::Custom);
    assert_eq!(
        root.site_label_source_attr,
        PreviewLinkCardSiteLabelSourceAttr::Derived
    );
    assert_eq!(root.motion_source_attr, PreviewLinkCardSourceAttr::Custom);

    let trigger = resolve_part_state(PreviewLinkCardPartStateInput {
        slot: PreviewLinkCardSlot::Trigger,
        disabled: false,
        has_image: true,
        has_custom_class_name: false,
        has_custom_delays: false,
        has_custom_id: false,
        has_custom_title: false,
        has_custom_description: false,
        has_custom_url: false,
        site_label_source_attr: PreviewLinkCardSiteLabelSourceAttr::Default,
        has_custom_motion: false,
    });

    assert_eq!(trigger.state_attr, PreviewLinkCardStateAttr::Trigger);
    assert_eq!(trigger.content_attr, PreviewLinkCardContentAttr::Trigger);
}

#[test]
fn compose_class_name_includes_custom_and_content_markers() {
    let class_name = compose_class_name(
        Some("docs-preview-link-card".to_string()),
        resolve_part_state(PreviewLinkCardPartStateInput {
            slot: PreviewLinkCardSlot::Root,
            disabled: false,
            has_image: true,
            has_custom_class_name: true,
            has_custom_delays: true,
            has_custom_id: true,
            has_custom_title: true,
            has_custom_description: true,
            has_custom_url: true,
            site_label_source_attr: PreviewLinkCardSiteLabelSourceAttr::Derived,
            has_custom_motion: true,
        }),
    );

    for token in [
        "ui-preview-link-card",
        "ui-preview-link-card--enabled",
        "ui-preview-link-card--media",
        "ui-preview-link-card--custom-class",
        "ui-preview-link-card--custom-delay",
        "ui-preview-link-card--custom-motion",
        "ui-preview-link-card--custom-id",
        "ui-preview-link-card--custom-title",
        "ui-preview-link-card--custom-description",
        "ui-preview-link-card--custom-url",
        "docs-preview-link-card",
    ] {
        assert!(
            class_name.contains(token),
            "preview card class name should include `{token}`"
        );
    }
}

#[test]
fn misc_helpers_keep_contracts_stable() {
    assert_eq!(state_attr_for_open(true), PreviewLinkCardStateAttr::Open);
    assert_eq!(state_attr_for_open(false), PreviewLinkCardStateAttr::Closed);
    assert_eq!(content_attr(true), PreviewLinkCardContentAttr::Media);
    assert_eq!(content_attr(false), PreviewLinkCardContentAttr::Text);

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
        "--ui-preview-link-card-top: 12.5px; --ui-preview-link-card-left: 24px; --ui-preview-link-card-anchor-width: 180px;"
    );
}

#[test]
fn resolve_open_state_markers_provides_renderable_state_contracts() {
    let open = resolve_open_state_markers(OpenStateMarkersInput { is_open: true });
    assert_eq!(open.state_attr, PreviewLinkCardStateAttr::Open);
    assert_eq!(open.open_attr, Some("true"));
    assert_eq!(open.closed_attr, None);

    let closed = resolve_open_state_markers(OpenStateMarkersInput { is_open: false });
    assert_eq!(closed.state_attr, PreviewLinkCardStateAttr::Closed);
    assert_eq!(closed.open_attr, None);
    assert_eq!(closed.closed_attr, Some("true"));
}

#[test]
fn normalize_delays_keeps_default_priority_inside_logic() {
    let defaults = normalize_delays(DelayInput {
        open_delay_ms: None,
        close_delay_ms: None,
    });
    assert_eq!(defaults.open_delay_ms, DEFAULT_OPEN_DELAY_MS);
    assert_eq!(defaults.close_delay_ms, DEFAULT_CLOSE_DELAY_MS);
    assert!(!defaults.has_custom_delays);

    let custom_open_only = normalize_delays(DelayInput {
        open_delay_ms: Some(DEFAULT_OPEN_DELAY_MS + 10),
        close_delay_ms: None,
    });
    assert_eq!(custom_open_only.open_delay_ms, DEFAULT_OPEN_DELAY_MS + 10);
    assert_eq!(custom_open_only.close_delay_ms, DEFAULT_CLOSE_DELAY_MS);
    assert!(custom_open_only.has_custom_delays);
}

#[test]
fn open_state_source_markers_are_closed_and_observable() {
    let controlled = resolve_open_state_source_markers(OpenStateSourceMarkersInput {
        is_controlled: true,
        has_open_prop: true,
        has_default_open: false,
        has_on_open_change: true,
    });
    assert_eq!(
        controlled.open_mode_attr,
        PreviewLinkCardOpenModeAttr::Controlled
    );
    assert_eq!(
        controlled.open_source_attr,
        PreviewLinkCardSourceAttr::Custom
    );
    assert_eq!(
        controlled.default_open_source_attr,
        PreviewLinkCardSourceAttr::Default
    );
    assert_eq!(
        controlled.open_change_source_attr,
        PreviewLinkCardSourceAttr::Custom
    );

    let uncontrolled = resolve_open_state_source_markers(OpenStateSourceMarkersInput {
        is_controlled: false,
        has_open_prop: false,
        has_default_open: true,
        has_on_open_change: false,
    });
    assert_eq!(
        uncontrolled.open_mode_attr,
        PreviewLinkCardOpenModeAttr::Uncontrolled
    );
    assert_eq!(
        uncontrolled.open_source_attr,
        PreviewLinkCardSourceAttr::Default
    );
    assert_eq!(
        uncontrolled.default_open_source_attr,
        PreviewLinkCardSourceAttr::Custom
    );
    assert_eq!(
        uncontrolled.open_change_source_attr,
        PreviewLinkCardSourceAttr::Default
    );
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

#[test]
fn normalize_open_state_prefers_is_open_and_supports_controlled_uncontrolled_modes() {
    let controlled_prefixed_raw = RwSignal::new(true);
    let controlled_prefixed = Signal::derive(move || controlled_prefixed_raw.get());
    let controlled_legacy_raw = RwSignal::new(false);
    let controlled_legacy = Signal::derive(move || controlled_legacy_raw.get());
    let on_open_change = Callback::new(|_: bool| {});

    let controlled = normalize_open_state(OpenStateInput {
        is_open: Some(controlled_prefixed),
        open: Some(controlled_legacy),
        default_open: Some(false),
        on_open_change: Some(on_open_change),
    });

    assert!(controlled.is_controlled);
    assert!(controlled.open.is_some());
    assert_eq!(controlled.default_open, Some(false));
    assert!(controlled.on_open_change.is_some());
    assert!(
        controlled
            .open
            .map(|signal| signal.get_untracked())
            .unwrap_or(false)
    );

    let uncontrolled = normalize_open_state(OpenStateInput {
        is_open: None,
        open: None,
        default_open: Some(true),
        on_open_change: None,
    });

    assert!(!uncontrolled.is_controlled);
    assert!(uncontrolled.open.is_none());
    assert_eq!(uncontrolled.default_open, Some(true));
    assert!(uncontrolled.on_open_change.is_none());
}
