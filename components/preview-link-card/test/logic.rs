use super::*;
use crate::preview_link_card::PreviewLinkCardPartStateInput;

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
        ("Baseline".to_string(), "custom")
    );

    assert_eq!(
        resolve_site_label(None, "https://www.github.com/adobe/ui-baseline"),
        ("github.com".to_string(), "derived")
    );

    assert_eq!(
        resolve_site_label(None, "   "),
        (DEFAULT_SITE_LABEL.into(), "default")
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
        site_label_source_attr: "derived",
        has_custom_motion: true,
    });

    assert_eq!(root.slot_attr, "preview-link-card");
    assert_eq!(root.content_attr, "media");
    assert_eq!(root.class_source_attr, "custom");
    assert_eq!(root.delay_source_attr, "custom");
    assert_eq!(root.site_label_source_attr, "derived");
    assert_eq!(root.motion_source_attr, "custom");

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
        site_label_source_attr: "default",
        has_custom_motion: false,
    });

    assert_eq!(trigger.state_attr, "trigger");
    assert_eq!(trigger.content_attr, "trigger");
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
            site_label_source_attr: "derived",
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
        "--ui-preview-link-card-top: 12.5px; --ui-preview-link-card-left: 24px; --ui-preview-link-card-anchor-width: 180px;"
    );

    assert!(should_handle_escape("Escape", true, false));
    assert!(!should_handle_escape("Escape", false, false));
    assert!(!should_handle_escape("Escape", true, true));
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
