use super::*;
use ui_headless::A11yDirection;
use ui_state_primitives::collapsible::CollapsibleStateInput;

#[test]
fn compose_class_name_includes_state_mode_and_custom_markers() {
    let state = resolve_state(CollapsibleStateInput {
        status: CollapsibleStatus::Disabled,
        open_mode: CollapsibleOpenMode::Uncontrolled,
        label_source: CollapsibleLabelSource::Title,
        class_source: CollapsibleClassSource::Custom,
        motion_source: CollapsibleMotionSource::Custom,
        open_value_source: CollapsibleOpenValueSource::Default,
        open_change_source: CollapsibleOpenChangeSource::Initial,
    });

    let class_name = compose_class_name(Some("docs-collapsible".to_string()), state);

    for token in [
        "ui-collapsible",
        "ui-collapsible--state-disabled",
        "ui-collapsible--mode-uncontrolled",
        "ui-collapsible--custom-motion",
        "ui-collapsible--custom-class",
        "docs-collapsible",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn normalize_discrete_axes_maps_boolean_inputs_to_enums() {
    assert_eq!(normalize_status(true, false), CollapsibleStatus::Open);
    assert_eq!(normalize_status(false, false), CollapsibleStatus::Closed);
    assert_eq!(normalize_status(true, true), CollapsibleStatus::Disabled);

    assert_eq!(normalize_open_mode(true), CollapsibleOpenMode::Controlled);
    assert_eq!(
        normalize_open_mode(false),
        CollapsibleOpenMode::Uncontrolled
    );

    assert_eq!(normalize_label_source(true), CollapsibleLabelSource::Custom);
    assert_eq!(normalize_label_source(false), CollapsibleLabelSource::Title);

    assert_eq!(normalize_class_source(true), CollapsibleClassSource::Custom);
    assert_eq!(
        normalize_class_source(false),
        CollapsibleClassSource::Default
    );

    assert_eq!(
        normalize_motion_source(true),
        CollapsibleMotionSource::Custom
    );
    assert_eq!(
        normalize_motion_source(false),
        CollapsibleMotionSource::Default
    );

    assert_eq!(
        normalize_open_value_source(Some(true), Some(false)),
        CollapsibleOpenValueSource::External
    );
    assert_eq!(
        normalize_open_value_source(None, Some(true)),
        CollapsibleOpenValueSource::Default
    );
    assert_eq!(
        normalize_open_value_source(None, None),
        CollapsibleOpenValueSource::Primitive
    );
    assert_eq!(
        normalize_open_change_source(true),
        CollapsibleOpenChangeSource::Interaction
    );
    assert_eq!(
        normalize_open_change_source(false),
        CollapsibleOpenChangeSource::ExternalSync
    );
}

#[test]
fn normalize_is_disabled_prefers_canonical_prop_with_alias_fallback() {
    assert!(
        normalize_is_disabled(Some(true), false),
        "is_disabled should take precedence over disabled alias when true",
    );
    assert!(
        normalize_is_disabled(None, true),
        "disabled alias should be used when is_disabled is not provided",
    );
    assert!(
        !normalize_is_disabled(Some(false), true),
        "is_disabled should take precedence over disabled alias when false",
    );
}

#[test]
fn normalize_dir_accepts_ltr_and_rtl_and_rejects_unknown_values() {
    assert_eq!(
        normalize_dir(Some("ltr".to_string())),
        Some(A11yDirection::Ltr),
    );
    assert_eq!(
        normalize_dir(Some("RTL".to_string())),
        Some(A11yDirection::Rtl),
    );
    assert_eq!(
        normalize_dir(Some("  rtl  ".to_string())),
        Some(A11yDirection::Rtl),
    );
    assert_eq!(normalize_dir(Some("auto".to_string())), None);
    assert_eq!(normalize_dir(None), None);
}

#[test]
fn open_state_switch_between_controlled_and_uncontrolled_is_stable() {
    let mut state = use_collapsible_open_state(CollapsibleOpenStateOptions {
        open: Some(true),
        default_open: Some(false),
    });

    assert!(state.is_controlled());
    assert!(state.is_open());

    state.set_open(false);
    assert!(
        state.is_open(),
        "controlled mode should ignore local writes until external sync"
    );

    state.sync_controlled(None);
    assert!(
        !state.is_controlled(),
        "removing external value should switch to uncontrolled mode"
    );
    state.set_open(false);
    assert!(
        !state.is_open(),
        "uncontrolled mode should allow local writes after switching"
    );
}

#[test]
fn normalize_open_state_options_prioritizes_open_then_default_then_primitive_fallback() {
    let mut controlled =
        use_collapsible_open_state(normalize_open_state_options(Some(true), Some(false)));
    assert!(controlled.is_controlled());
    assert!(
        controlled.is_open(),
        "controlled value should win over default_open during initialization"
    );
    controlled.set_open(false);
    assert!(
        controlled.is_open(),
        "controlled value should remain source of truth after local write attempts"
    );

    let uncontrolled = use_collapsible_open_state(normalize_open_state_options(None, Some(true)));
    assert!(!uncontrolled.is_controlled());
    assert!(
        uncontrolled.is_open(),
        "default_open should seed uncontrolled initialization when open is absent"
    );

    let fallback = use_collapsible_open_state(normalize_open_state_options(None, None));
    assert!(!fallback.is_controlled());
    assert!(
        !fallback.is_open(),
        "primitive fallback should initialize to false when open/default_open are both absent"
    );
}

#[test]
fn compute_next_open_toggles_current_open_state() {
    assert!(!compute_next_open(true));
    assert!(compute_next_open(false));
}

#[test]
fn should_emit_open_change_rejects_same_value_transitions() {
    assert!(should_emit_open_change(false, true));
    assert!(!should_emit_open_change(true, true));
}

#[test]
fn apply_open_change_uses_primitive_controlled_semantics() {
    let mut controlled =
        use_collapsible_open_state(normalize_open_state_options(Some(true), Some(false)));
    apply_open_change(&mut controlled, Some(true), false);
    assert!(
        controlled.is_open(),
        "controlled mode should keep external value as source of truth"
    );

    let mut uncontrolled =
        use_collapsible_open_state(normalize_open_state_options(None, Some(true)));
    apply_open_change(&mut uncontrolled, None, false);
    assert!(
        !uncontrolled.is_open(),
        "uncontrolled mode should accept internal updates"
    );
}
