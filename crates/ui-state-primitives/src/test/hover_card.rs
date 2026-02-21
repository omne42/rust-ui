use super::*;
use std::borrow::Cow;

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  hover-card  ".to_string())),
        Some("hover-card".to_string())
    );
}

#[test]
fn resolve_id_uses_custom_or_generated_paths() {
    assert_eq!(
        resolve_id(
            Some(" docs-hover-card ".to_string()),
            Cow::Borrowed("ui-hover-card-1")
        ),
        ("docs-hover-card".to_string(), true)
    );
    assert_eq!(
        resolve_id(Some("   ".to_string()), Cow::Borrowed("ui-hover-card-2")),
        ("ui-hover-card-2".to_string(), false)
    );
    assert_eq!(
        resolve_id(None, Cow::Borrowed("ui-hover-card-3")),
        ("ui-hover-card-3".to_string(), false)
    );
}

#[test]
fn delay_source_detection_matches_default_contract() {
    assert!(!has_custom_delays(
        DEFAULT_OPEN_DELAY_MS,
        DEFAULT_CLOSE_DELAY_MS
    ));
    assert!(has_custom_delays(
        DEFAULT_OPEN_DELAY_MS + 1,
        DEFAULT_CLOSE_DELAY_MS
    ));
    assert!(has_custom_delays(
        DEFAULT_OPEN_DELAY_MS,
        DEFAULT_CLOSE_DELAY_MS + 1
    ));
}

#[test]
fn resolve_state_tracks_slot_and_source_markers() {
    let root = resolve_state(HoverCardPartStateInput {
        slot: HoverCardSlot::Root,
        open: true,
        disabled: false,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_custom_delays: true,
        has_custom_id: true,
    });

    assert_eq!(root.slot_attr, "hover-card");
    assert_eq!(root.base_class, "ui-hover-card");
    assert_eq!(root.state_attr, "open");
    assert_eq!(root.class_source_attr, "custom");
    assert_eq!(root.motion_source_attr, "custom");
    assert_eq!(root.delay_source_attr, "custom");
    assert_eq!(root.id_source_attr, "custom");

    let trigger = resolve_state(HoverCardPartStateInput {
        slot: HoverCardSlot::Trigger,
        open: false,
        disabled: true,
        has_custom_class_name: false,
        has_custom_motion: false,
        has_custom_delays: false,
        has_custom_id: false,
    });
    assert_eq!(trigger.state_attr, "trigger");
    assert_eq!(trigger.motion_source_attr, "default");
}

#[test]
fn compose_class_name_includes_custom_markers() {
    let class_name = compose_class_name(
        Some("docs-hover-card".to_string()),
        resolve_state(HoverCardPartStateInput {
            slot: HoverCardSlot::Root,
            open: false,
            disabled: false,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_delays: true,
            has_custom_id: true,
        }),
    );

    for token in [
        "ui-hover-card",
        "ui-hover-card--custom-motion",
        "ui-hover-card--custom-delay",
        "ui-hover-card--custom-id",
        "ui-hover-card--custom-class",
        "docs-hover-card",
    ] {
        assert!(
            class_name.contains(token),
            "hover card class name should include `{token}`"
        );
    }
}

#[test]
fn compose_panel_vars_generates_css_variables_only() {
    let vars = compose_panel_vars(12.5, 24.0, 220.0);
    assert_eq!(
        vars,
        "--ui-hover-card-top: 12.5px; --ui-hover-card-left: 24px; --ui-hover-card-anchor-width: 220px;"
    );
}
