use super::*;

#[test]
fn attr_and_class_mapping_are_stable() {
    assert_eq!(FlipDirection::Top.as_attr(), "top");
    assert_eq!(FlipDirection::Bottom.as_attr(), "bottom");
    assert_eq!(FlipDirection::Left.as_attr(), "left");
    assert_eq!(FlipDirection::Right.as_attr(), "right");

    assert_eq!(FlipDirection::Top.class_name(), "ui-flip-button--from-top");
    assert_eq!(
        FlipDirection::Bottom.class_name(),
        "ui-flip-button--from-bottom"
    );
    assert_eq!(
        FlipDirection::Left.class_name(),
        "ui-flip-button--from-left"
    );
    assert_eq!(
        FlipDirection::Right.class_name(),
        "ui-flip-button--from-right"
    );
}

#[test]
fn normalize_input_centralizes_defaults_and_sources() {
    let normalized = normalize_input(FlipButtonInputNormalizationInput {
        from: None,
        motion: None,
        class_name: Some("  ".to_string()),
    });

    assert_eq!(normalized.direction, FlipDirection::Top);
    assert_eq!(normalized.motion, FlipButtonMotion::default());
    assert!(!normalized.has_custom_motion);
    assert!(!normalized.has_custom_class_name);
    assert_eq!(normalized.class_name, None);

    let custom = normalize_input(FlipButtonInputNormalizationInput {
        from: Some(FlipDirection::Right),
        motion: Some(FlipButtonMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 320.0,
                damping: 18.0,
                mass: 1.0,
                precision: 0.001,
            },
        }),
        class_name: Some(" custom ".to_string()),
    });

    assert_eq!(custom.direction, FlipDirection::Right);
    assert!(custom.has_custom_motion);
    assert!(custom.has_custom_class_name);
    assert_eq!(custom.class_name, Some("custom".to_string()));
}

#[test]
fn resolve_state_tracks_interaction_and_source_metadata() {
    let active = resolve_state(FlipButtonStateInput {
        direction: FlipDirection::Left,
        is_hovered: true,
        is_focus_within: false,
        has_custom_class_name: true,
        has_custom_motion: true,
    });

    assert!(active.is_active);
    assert!(!active.is_inactive);
    assert!(active.is_hovered);
    assert!(!active.is_focus_within);
    assert_eq!(active.direction, FlipDirection::Left);
    assert_eq!(active.direction_attr, "left");
    assert_eq!(active.direction_class, "ui-flip-button--from-left");
    assert_eq!(active.state_attr, "active");
    assert_eq!(active.state_class, "ui-flip-button--state-active");
    assert_eq!(active.hover_attr, "hovered");
    assert_eq!(active.hover_class, "ui-flip-button--hovered");
    assert_eq!(active.focus_within_attr, "inactive");
    assert_eq!(active.focus_within_class, "ui-flip-button--no-focus-within");
    assert_eq!(active.class_source_attr, "custom");
    assert_eq!(active.motion_source_attr, "custom");
    assert!(active.has_custom_class_name);
    assert!(active.has_custom_motion);

    let inactive = resolve_state(FlipButtonStateInput {
        direction: FlipDirection::Bottom,
        is_hovered: false,
        is_focus_within: false,
        has_custom_class_name: false,
        has_custom_motion: false,
    });

    assert!(!inactive.is_active);
    assert!(inactive.is_inactive);
    assert!(!inactive.is_hovered);
    assert!(!inactive.is_focus_within);
    assert_eq!(inactive.direction_attr, "bottom");
    assert_eq!(inactive.state_attr, "inactive");
    assert_eq!(inactive.hover_attr, "resting");
    assert_eq!(inactive.focus_within_attr, "inactive");
    assert_eq!(inactive.class_source_attr, "default");
    assert_eq!(inactive.motion_source_attr, "default");
    assert!(!inactive.has_custom_class_name);
    assert!(!inactive.has_custom_motion);
}

#[test]
fn compose_class_name_includes_state_markers_and_custom_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(FlipButtonStateInput {
            direction: FlipDirection::Right,
            is_hovered: true,
            is_focus_within: true,
            has_custom_class_name: true,
            has_custom_motion: true,
        }),
    );

    for token in [
        "ui-flip-button",
        "ui-flip-button--from-right",
        "ui-flip-button--state-active",
        "ui-flip-button--hovered",
        "ui-flip-button--focus-within",
        "ui-flip-button--custom-class",
        "ui-flip-button--custom-motion",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn resolve_agent_contract_reuses_button_agent_schema_contract() {
    let contract = resolve_agent_contract(resolve_state(FlipButtonStateInput {
        direction: FlipDirection::Top,
        is_hovered: false,
        is_focus_within: false,
        has_custom_class_name: false,
        has_custom_motion: false,
    }));

    assert_eq!(contract.schema_name, "ui.button.agent-contract");
    assert_eq!(contract.schema_version.as_str(), "1");
    assert_eq!(contract.intent.as_str(), "trigger");
    assert_eq!(contract.state.as_str(), "ready");
    assert!(contract.capabilities.can_press);
    assert!(contract.capabilities.can_focus);
    assert!(contract.capabilities.can_hover);
    assert!(!contract.capabilities.can_popup_trigger);
}
