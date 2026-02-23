use crate::pressable_feedback::{
    DEFAULT_ARIA_LABEL, DEFAULT_IS_BOUNDED, DEFAULT_IS_DISABLED,
    PressableFeedbackDefaultPressedSource, PressableFeedbackEffect,
    PressableFeedbackPressedAxisInput, PressableFeedbackPressedChangeSource,
    PressableFeedbackPressedMode, PressableFeedbackStateContractInput, PressableFeedbackStateInput,
    PressableFeedbackTone, compose_class_name, normalize_aria_label, normalize_flags,
    normalize_optional_text, normalize_state_contract, resolve_pressed_axis_state, resolve_state,
};

#[test]
fn tone_and_effect_contracts_are_stable() {
    assert_eq!(
        PressableFeedbackTone::Default.class_name(),
        "ui-pressable-feedback--tone-default"
    );
    assert_eq!(
        PressableFeedbackTone::Neutral.class_name(),
        "ui-pressable-feedback--tone-neutral"
    );
    assert_eq!(
        PressableFeedbackTone::Accent.class_name(),
        "ui-pressable-feedback--tone-accent"
    );
    assert_eq!(PressableFeedbackTone::Default.as_attr(), "default");

    assert_eq!(
        PressableFeedbackEffect::Scale.class_name(),
        "ui-pressable-feedback--effect-scale"
    );
    assert_eq!(
        PressableFeedbackEffect::Highlight.class_name(),
        "ui-pressable-feedback--effect-highlight"
    );
    assert_eq!(
        PressableFeedbackEffect::Ripple.class_name(),
        "ui-pressable-feedback--effect-ripple"
    );
    assert_eq!(
        PressableFeedbackEffect::HighlightRipple.class_name(),
        "ui-pressable-feedback--effect-highlight-ripple"
    );

    assert!(PressableFeedbackEffect::Highlight.has_highlight());
    assert!(!PressableFeedbackEffect::Highlight.has_ripple());
    assert!(PressableFeedbackEffect::Ripple.has_ripple());
    assert!(!PressableFeedbackEffect::Ripple.has_highlight());
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-pressable-feedback  ".to_string())),
        Some("docs-pressable-feedback".to_string())
    );

    let (label, custom_label) = normalize_aria_label(Some("  Press card  ".to_string()));
    assert_eq!(label, "Press card");
    assert!(custom_label);

    let (label, custom_label) = normalize_aria_label(None);
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom_label);
}

#[test]
fn resolve_state_tracks_sources_and_visibility_flags() {
    let state = resolve_state(PressableFeedbackStateInput {
        tone: PressableFeedbackTone::Accent,
        effect: PressableFeedbackEffect::HighlightRipple,
        is_disabled: false,
        is_pressed: true,
        bounded: false,
        has_highlight: true,
        has_ripple: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
        has_custom_motion: true,
        has_custom_press_handler: true,
    });

    assert_eq!(state.tone_attr, "accent");
    assert_eq!(state.effect_attr, "highlight-ripple");
    assert_eq!(state.state_attr, "pressed");
    assert_eq!(state.boundary_attr, "unbounded");
    assert_eq!(state.highlight_attr, "enabled");
    assert_eq!(state.ripple_attr, "enabled");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert_eq!(state.motion_source_attr, "custom");
    assert!(state.has_custom_press_handler);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-pressable-feedback".to_string()),
        resolve_state(PressableFeedbackStateInput {
            tone: PressableFeedbackTone::Neutral,
            effect: PressableFeedbackEffect::Highlight,
            is_disabled: false,
            is_pressed: false,
            bounded: true,
            has_highlight: true,
            has_ripple: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_motion: false,
            has_custom_press_handler: false,
        }),
    );

    for token in [
        "ui-pressable-feedback",
        "ui-pressable-feedback--tone-neutral",
        "ui-pressable-feedback--effect-highlight",
        "ui-pressable-feedback--boundary-bounded",
        "ui-pressable-feedback--state-idle",
        "ui-pressable-feedback--highlight-enabled",
        "ui-pressable-feedback--custom-class",
        "docs-pressable-feedback",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}

#[test]
fn normalize_flags_uses_primitive_defaults() {
    let flags = normalize_flags(None, None);
    assert_eq!(flags.is_bounded, DEFAULT_IS_BOUNDED);
    assert_eq!(flags.is_disabled, DEFAULT_IS_DISABLED);

    let flags = normalize_flags(Some(false), Some(true));
    assert!(!flags.is_bounded);
    assert!(flags.is_disabled);
}

#[test]
fn normalize_state_contract_aggregates_sources_and_effect_flags() {
    let contract = normalize_state_contract(PressableFeedbackStateContractInput {
        effect: PressableFeedbackEffect::HighlightRipple,
        is_bounded: None,
        is_disabled: Some(true),
        aria_label: Some("  Press tile  ".to_string()),
        class_name: Some("  docs-pressable-feedback  ".to_string()),
        has_custom_motion: true,
        has_custom_press_handler: true,
    });

    assert!(contract.flags.is_bounded);
    assert!(contract.flags.is_disabled);
    assert_eq!(contract.aria_label, "Press tile");
    assert_eq!(
        contract.class_name,
        Some("docs-pressable-feedback".to_string())
    );
    assert!(contract.has_custom_aria_label);
    assert!(contract.has_custom_class_name);
    assert!(contract.has_highlight);
    assert!(contract.has_ripple);
    assert!(contract.has_custom_motion);
    assert!(contract.has_custom_press_handler);
}

#[test]
fn resolve_pressed_axis_state_maps_control_and_source_markers() {
    let controlled = resolve_pressed_axis_state(PressableFeedbackPressedAxisInput {
        has_controlled_value: true,
        default_pressed: Some(true),
        has_on_pressed_change: true,
    });
    assert_eq!(
        controlled.pressed_mode,
        PressableFeedbackPressedMode::Controlled
    );
    assert!(controlled.default_pressed);
    assert_eq!(
        controlled.default_pressed_source,
        PressableFeedbackDefaultPressedSource::Provided
    );
    assert_eq!(
        controlled.pressed_change_source,
        PressableFeedbackPressedChangeSource::Provided
    );

    let uncontrolled = resolve_pressed_axis_state(PressableFeedbackPressedAxisInput {
        has_controlled_value: false,
        default_pressed: None,
        has_on_pressed_change: false,
    });
    assert_eq!(
        uncontrolled.pressed_mode,
        PressableFeedbackPressedMode::Uncontrolled
    );
    assert!(!uncontrolled.default_pressed);
    assert_eq!(
        uncontrolled.default_pressed_source,
        PressableFeedbackDefaultPressedSource::Default
    );
    assert_eq!(
        uncontrolled.pressed_change_source,
        PressableFeedbackPressedChangeSource::None
    );
}
