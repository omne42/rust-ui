use super::*;

#[test]
fn uncontrolled_open_state_uses_default_and_can_toggle() {
    let mut state = use_collapsible_open_state(CollapsibleOpenStateOptions {
        open: None,
        default_open: Some(true),
    });

    assert!(state.is_open());
    assert!(state.default_open());
    assert!(!state.is_controlled());

    state.toggle();
    assert!(!state.is_open());
}

#[test]
fn controlled_open_state_requires_sync_from_external_source() {
    let mut state = use_collapsible_open_state(CollapsibleOpenStateOptions {
        open: Some(true),
        default_open: Some(false),
    });

    assert!(state.is_open());
    assert!(!state.default_open());
    assert!(state.is_controlled());

    state.set_open(false);
    assert!(
        state.is_open(),
        "controlled state should not mutate internal value until synchronized"
    );

    state.sync_controlled(Some(false));
    assert!(!state.is_open());
}

#[test]
fn open_state_switch_between_controlled_and_uncontrolled_is_predictable() {
    let mut state = use_collapsible_open_state(CollapsibleOpenStateOptions {
        open: Some(true),
        default_open: Some(false),
    });

    assert!(state.is_controlled());
    assert!(state.is_open());

    state.set_open(false);
    assert!(
        state.is_open(),
        "controlled state should ignore local writes before external sync"
    );

    state.sync_controlled(None);
    assert!(
        !state.is_controlled(),
        "dropping external value should switch to uncontrolled mode"
    );
    assert!(
        state.is_open(),
        "switching to uncontrolled mode should keep last synchronized value"
    );

    state.set_open(false);
    assert!(
        !state.is_open(),
        "uncontrolled mode should allow local writes after the switch"
    );

    state.sync_controlled(Some(true));
    assert!(state.is_controlled());
    assert!(
        state.is_open(),
        "switching back to controlled mode should follow external value immediately"
    );

    state.set_open(false);
    assert!(
        state.is_open(),
        "controlled mode should continue blocking local writes after switching back"
    );
}

#[test]
fn normalize_id_base_sanitizes_whitespace_and_symbols() {
    assert_eq!(normalize_id_base("  My Panel  ".to_string()), "my-panel");
    assert_eq!(
        normalize_id_base("Settings/Panel#1".to_string()),
        "settings-panel-1"
    );
    assert_eq!(normalize_id_base("   ".to_string()), DEFAULT_ID_BASE);
}

#[test]
fn resolve_title_and_aria_label_fall_back_to_defaults() {
    assert_eq!(resolve_title("  ".to_string()), DEFAULT_TITLE);
    assert_eq!(
        resolve_title("  Advanced Options  ".to_string()),
        "Advanced Options"
    );

    let (aria_label, custom) = resolve_aria_label("Advanced Options", None);
    assert_eq!(aria_label, "Advanced Options");
    assert!(!custom);

    let (aria_label, custom) =
        resolve_aria_label("Advanced Options", Some("  Settings panel  ".to_string()));
    assert_eq!(aria_label, "Settings panel");
    assert!(custom);
}

#[test]
fn resolve_state_tracks_open_mode_sources_and_motion() {
    let state = resolve_state(CollapsibleStateInput {
        status: CollapsibleStatus::Open,
        open_mode: CollapsibleOpenMode::Controlled,
        label_source: CollapsibleLabelSource::Custom,
        class_source: CollapsibleClassSource::Default,
        motion_source: CollapsibleMotionSource::Custom,
        open_value_source: CollapsibleOpenValueSource::External,
        open_change_source: CollapsibleOpenChangeSource::Interaction,
    });

    assert_eq!(state.state_attr, "open");
    assert_eq!(state.open_mode_attr, "controlled");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.open_value_source_attr, "external");
    assert_eq!(state.open_change_source_attr, "interaction");
    assert!(state.is_open);
    assert!(!state.is_closed);
    assert_eq!(state.status, CollapsibleStatus::Open);
    assert_eq!(state.open_mode, CollapsibleOpenMode::Controlled);
    assert_eq!(state.label_source, CollapsibleLabelSource::Custom);
    assert_eq!(state.class_source, CollapsibleClassSource::Default);
    assert_eq!(state.motion_source, CollapsibleMotionSource::Custom);
    assert_eq!(
        state.open_value_source,
        CollapsibleOpenValueSource::External
    );
    assert_eq!(
        state.open_change_source,
        CollapsibleOpenChangeSource::Interaction
    );
}

#[test]
fn discrete_state_enums_lock_mutually_exclusive_axes() {
    assert_eq!(
        CollapsibleStatus::from_parts(true, false),
        CollapsibleStatus::Open
    );
    assert_eq!(
        CollapsibleStatus::from_parts(false, false),
        CollapsibleStatus::Closed
    );
    assert_eq!(
        CollapsibleStatus::from_parts(true, true),
        CollapsibleStatus::Disabled
    );
    assert_eq!(
        CollapsibleOpenMode::from_is_controlled(true),
        CollapsibleOpenMode::Controlled
    );
    assert_eq!(
        CollapsibleOpenMode::from_is_controlled(false),
        CollapsibleOpenMode::Uncontrolled
    );
    assert_eq!(
        CollapsibleOpenValueSource::from_input(Some(true), Some(false)),
        CollapsibleOpenValueSource::External
    );
    assert_eq!(
        CollapsibleOpenValueSource::from_input(None, Some(false)),
        CollapsibleOpenValueSource::Default
    );
    assert_eq!(
        CollapsibleOpenValueSource::from_input(None, None),
        CollapsibleOpenValueSource::Primitive
    );
    assert_eq!(CollapsibleOpenChangeSource::Initial.as_attr(), "initial");
    assert_eq!(
        CollapsibleOpenChangeSource::Interaction.as_attr(),
        "interaction"
    );
    assert_eq!(
        CollapsibleOpenChangeSource::ExternalSync.as_attr(),
        "external-sync"
    );
}
