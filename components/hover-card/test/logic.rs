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
fn normalize_delay_state_uses_logic_defaults() {
    let delay_state = normalize_delay_state(DelayStateInput {
        open_delay_ms: None,
        close_delay_ms: None,
    });

    assert_eq!(delay_state.open_delay_ms, DEFAULT_OPEN_DELAY_MS);
    assert_eq!(delay_state.close_delay_ms, DEFAULT_CLOSE_DELAY_MS);
    assert!(!delay_state.has_custom_delays);
}

#[test]
fn normalize_delay_state_prefers_explicit_values() {
    let delay_state = normalize_delay_state(DelayStateInput {
        open_delay_ms: Some(DEFAULT_OPEN_DELAY_MS + 24),
        close_delay_ms: None,
    });

    assert_eq!(delay_state.open_delay_ms, DEFAULT_OPEN_DELAY_MS + 24);
    assert_eq!(delay_state.close_delay_ms, DEFAULT_CLOSE_DELAY_MS);
    assert!(delay_state.has_custom_delays);
}

#[test]
fn custom_motion_detection_uses_motion_contract() {
    assert!(!is_custom_motion(HoverCardMotion::default()));
    assert!(is_custom_motion(HoverCardMotion {
        initial_scale: HoverCardMotion::default().initial_scale + 0.02,
        ..HoverCardMotion::default()
    }));
}

#[test]
fn is_disabled_naming_compatibility_prefers_canonical_prop() {
    assert!(!resolve_is_disabled(None, None));
    assert!(resolve_is_disabled(Some(true), None));
    assert!(resolve_is_disabled(None, Some(true)));
    assert!(!resolve_is_disabled(Some(false), Some(true)));
}

#[test]
fn open_state_is_uncontrolled_by_default() {
    let open_state = normalize_open_state(OpenStateInput {
        is_open: None,
        open: None,
        default_open: None,
        on_open_change: None,
    });

    assert!(open_state.open.is_none());
    assert!(!open_state.is_controlled);
    assert!(open_state.default_open.is_none());
    assert!(open_state.on_open_change.is_none());
}

#[test]
fn open_state_normalization_prefers_is_open_and_marks_controlled_source() {
    let is_open_signal: Signal<bool> = RwSignal::new(true).into();
    let open_signal_alias: Signal<bool> = RwSignal::new(false).into();

    let open_state = normalize_open_state(OpenStateInput {
        is_open: Some(is_open_signal),
        open: Some(open_signal_alias),
        default_open: Some(false),
        on_open_change: None,
    });

    assert!(open_state.is_controlled);
    let normalized = open_state
        .open
        .expect("normalized open signal should exist in controlled mode");
    assert!(normalized.get_untracked());
    assert_eq!(open_mode_attr(open_state.is_controlled), "controlled");
    assert_eq!(open_value_source_attr(open_state.is_controlled), "external");
}

#[test]
fn machine_readable_state_attr_set_is_closed_and_enumerable() {
    assert_eq!(state_attr_for_open(true), "open");
    assert_eq!(state_attr_for_open(false), "closed");
    assert_eq!(open_mode_attr(true), "controlled");
    assert_eq!(open_mode_attr(false), "uncontrolled");
    assert_eq!(open_value_source_attr(true), "external");
    assert_eq!(open_value_source_attr(false), "default");
    assert_eq!(open_intent_source_attr(), "interaction");
}

#[test]
fn resolve_part_state_tracks_slot_and_source_markers() {
    let root = resolve_part_state(HoverCardPartStateInput {
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

    let trigger = resolve_part_state(HoverCardPartStateInput {
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
        resolve_part_state(HoverCardPartStateInput {
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
fn normalize_part_states_centralizes_slot_state_derivation() {
    let part_states = normalize_part_states(PartStatesInput {
        class_name: Some("docs-hover-card".to_string()),
        is_open: true,
        is_disabled: false,
        motion: HoverCardMotion {
            initial_scale: HoverCardMotion::default().initial_scale + 0.02,
            ..HoverCardMotion::default()
        },
        has_custom_delays: true,
        has_custom_id: true,
    });

    assert_eq!(part_states.root_state.slot, HoverCardSlot::Root);
    assert_eq!(part_states.trigger_state.slot, HoverCardSlot::Trigger);
    assert_eq!(part_states.panel_state.slot, HoverCardSlot::Panel);
    assert_eq!(part_states.root_state.state_attr, "open");
    assert_eq!(part_states.trigger_state.state_attr, "trigger");
    assert_eq!(part_states.panel_state.state_attr, "panel");
    assert_eq!(part_states.root_state.motion_source_attr, "custom");
    assert_eq!(part_states.root_state.delay_source_attr, "custom");

    for token in [
        "ui-hover-card",
        "ui-hover-card--custom-motion",
        "ui-hover-card--custom-delay",
        "ui-hover-card--custom-id",
        "ui-hover-card--custom-class",
        "docs-hover-card",
    ] {
        assert!(
            part_states.root_class.contains(token),
            "normalized root class should include `{token}`"
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
