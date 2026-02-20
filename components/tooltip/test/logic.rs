use super::*;

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" tooltip ".to_string())),
        Some("tooltip".to_string())
    );
}

#[test]
fn resolve_id_uses_custom_or_generated_paths() {
    assert_eq!(
        resolve_id(Some("docs-tooltip".to_string()), "ui-tooltip-1".to_string()),
        ("docs-tooltip".to_string(), true)
    );
    assert_eq!(
        resolve_id(Some("   ".to_string()), "ui-tooltip-2".to_string()),
        ("ui-tooltip-2".to_string(), false)
    );
    assert_eq!(
        resolve_id(None, "ui-tooltip-3".to_string()),
        ("ui-tooltip-3".to_string(), false)
    );
}

#[test]
fn trigger_and_press_behavior_attrs_match_contract() {
    assert_eq!(trigger_attr(TooltipTriggerMode::Hover), "hover");
    assert_eq!(trigger_attr(TooltipTriggerMode::Focus), "focus");
    assert_eq!(press_behavior_attr(true), "close");
    assert_eq!(press_behavior_attr(false), "persist");
}

#[test]
fn has_custom_delays_detects_non_default_values() {
    assert!(!has_custom_delays(DEFAULT_DELAY_MS, DEFAULT_CLOSE_DELAY_MS));
    assert!(has_custom_delays(
        DEFAULT_DELAY_MS + 1,
        DEFAULT_CLOSE_DELAY_MS
    ));
    assert!(has_custom_delays(
        DEFAULT_DELAY_MS,
        DEFAULT_CLOSE_DELAY_MS + 1
    ));
}

#[test]
fn resolve_state_tracks_source_markers_and_slot_attrs() {
    let state = resolve_state(TooltipPartStateInput {
        slot: TooltipSlot::Root,
        open: true,
        disabled: false,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_custom_delays: true,
        has_custom_trigger_mode: true,
        has_custom_press_behavior: true,
        has_custom_id: true,
        trigger_attr: "focus",
        press_behavior_attr: "persist",
    });

    assert_eq!(state.slot_attr, "tooltip");
    assert_eq!(state.base_class, "ui-tooltip");
    assert_eq!(state.state_attr, "open");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.delay_source_attr, "custom");
    assert_eq!(state.trigger_source_attr, "custom");
    assert_eq!(state.press_source_attr, "custom");
    assert_eq!(state.id_source_attr, "custom");
    assert_eq!(state.trigger_attr, "focus");
    assert_eq!(state.press_behavior_attr, "persist");
}

#[test]
fn compose_class_name_includes_custom_markers() {
    let class_name = compose_class_name(
        Some("docs-tooltip".to_string()),
        resolve_state(TooltipPartStateInput {
            slot: TooltipSlot::Root,
            open: false,
            disabled: false,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_delays: true,
            has_custom_trigger_mode: true,
            has_custom_press_behavior: true,
            has_custom_id: true,
            trigger_attr: "focus",
            press_behavior_attr: "persist",
        }),
    );

    for token in [
        "ui-tooltip",
        "ui-tooltip--custom-motion",
        "ui-tooltip--custom-delay",
        "ui-tooltip--custom-trigger",
        "ui-tooltip--custom-press",
        "ui-tooltip--custom-id",
        "ui-tooltip--custom-class",
        "docs-tooltip",
    ] {
        assert!(
            class_name.contains(token),
            "tooltip class name should include `{token}`"
        );
    }
}

#[test]
fn compose_panel_vars_formats_css_custom_properties() {
    assert_eq!(
        compose_panel_vars(18.5, 42.0),
        "--ui-tooltip-top: 18.5px; --ui-tooltip-left: 42px;"
    );
}

#[test]
fn normalize_accessibility_state_prefers_is_prefixed_prop() {
    let normalized = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: Some(false),
        disabled: true,
    });
    assert!(!normalized.is_disabled);

    let normalized = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: None,
        disabled: true,
    });
    assert!(normalized.is_disabled);
}

#[test]
fn normalize_open_state_prefers_is_open_and_tracks_control_mode() {
    let (prefixed, _set_prefixed) = signal(true);
    let (legacy, _set_legacy) = signal(false);
    let on_open_change = Callback::new(|_: bool| {});

    let normalized = normalize_open_state(OpenStateInput {
        is_open: Some(prefixed.into()),
        open: Some(legacy.into()),
        default_open: Some(false),
        on_open_change: Some(on_open_change),
    });
    assert_eq!(
        normalized.open.map(|signal| signal.get_untracked()),
        Some(true)
    );
    assert_eq!(normalized.default_open, Some(false));
    assert!(normalized.on_open_change.is_some());
    assert!(normalized.is_controlled);
    assert!(normalized.has_custom_open);
    assert!(normalized.has_custom_default_open);
    assert!(normalized.has_custom_on_open_change);
    assert_eq!(normalized.open_mode_attr, "controlled");
    assert_eq!(normalized.open_source_attr, "custom");
    assert_eq!(normalized.default_open_source_attr, "provided");
    assert_eq!(normalized.open_change_source_attr, "provided");

    let normalized = normalize_open_state(OpenStateInput {
        is_open: None,
        open: None,
        default_open: Some(true),
        on_open_change: None,
    });
    assert_eq!(normalized.open, None);
    assert_eq!(normalized.default_open, Some(true));
    assert!(!normalized.is_controlled);
    assert!(!normalized.has_custom_open);
    assert!(normalized.has_custom_default_open);
    assert!(!normalized.has_custom_on_open_change);
    assert_eq!(normalized.open_mode_attr, "uncontrolled");
    assert_eq!(normalized.open_source_attr, "default");
    assert_eq!(normalized.default_open_source_attr, "provided");
    assert_eq!(normalized.open_change_source_attr, "none");
}
