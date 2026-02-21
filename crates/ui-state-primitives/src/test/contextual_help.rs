use super::*;

#[test]
fn resolve_open_config_keeps_controlled_default_single_source() {
    let controlled = resolve_open_config(ContextualHelpOpenConfigInput {
        has_custom_open: true,
        default_open: Some(true),
        has_custom_on_open_change: true,
    });
    assert_eq!(controlled.default_open, None);
    assert!(controlled.has_custom_open);
    assert!(controlled.has_custom_default_open);
    assert!(controlled.has_custom_on_open_change);
    assert!(controlled.is_controlled);

    let uncontrolled = resolve_open_config(ContextualHelpOpenConfigInput {
        has_custom_open: false,
        default_open: Some(true),
        has_custom_on_open_change: false,
    });
    assert_eq!(uncontrolled.default_open, Some(true));
    assert!(!uncontrolled.has_custom_open);
    assert!(uncontrolled.has_custom_default_open);
    assert!(!uncontrolled.has_custom_on_open_change);
    assert!(!uncontrolled.is_controlled);
}

#[test]
fn interaction_intent_maps_to_source_and_sets_pending_flag() {
    let trigger =
        resolve_open_interaction_intent(ContextualHelpOpenInteractionIntent::TriggerPress);
    assert_eq!(
        trigger.next_source,
        ContextualHelpOpenInteractionSource::TriggerPress
    );
    assert!(trigger.has_pending_local_open_change);

    let dismiss =
        resolve_open_interaction_intent(ContextualHelpOpenInteractionIntent::DismissPress);
    assert_eq!(
        dismiss.next_source,
        ContextualHelpOpenInteractionSource::DismissPress
    );
    assert!(dismiss.has_pending_local_open_change);
}

#[test]
fn interaction_sync_preserves_local_source_and_tags_external_syncs() {
    let local_change = resolve_open_interaction_sync(ContextualHelpOpenInteractionSyncInput {
        previous_open: false,
        current_open: true,
        current_source: ContextualHelpOpenInteractionSource::TriggerPress,
        has_pending_local_open_change: true,
    });
    assert!(local_change.next_previous_open);
    assert_eq!(
        local_change.next_source,
        ContextualHelpOpenInteractionSource::TriggerPress
    );
    assert!(!local_change.has_pending_local_open_change);

    let external_change = resolve_open_interaction_sync(ContextualHelpOpenInteractionSyncInput {
        previous_open: true,
        current_open: false,
        current_source: ContextualHelpOpenInteractionSource::DismissPress,
        has_pending_local_open_change: false,
    });
    assert!(!external_change.next_previous_open);
    assert_eq!(
        external_change.next_source,
        ContextualHelpOpenInteractionSource::ExternalSync
    );
    assert!(!external_change.has_pending_local_open_change);

    let no_change = resolve_open_interaction_sync(ContextualHelpOpenInteractionSyncInput {
        previous_open: false,
        current_open: false,
        current_source: ContextualHelpOpenInteractionSource::Initial,
        has_pending_local_open_change: true,
    });
    assert!(!no_change.next_previous_open);
    assert_eq!(
        no_change.next_source,
        ContextualHelpOpenInteractionSource::Initial
    );
    assert!(no_change.has_pending_local_open_change);
}

#[test]
fn interaction_source_attr_values_are_stable() {
    assert_eq!(
        ContextualHelpOpenInteractionSource::Initial.as_attr(),
        "initial"
    );
    assert_eq!(
        ContextualHelpOpenInteractionSource::TriggerPress.as_attr(),
        "trigger-press"
    );
    assert_eq!(
        ContextualHelpOpenInteractionSource::DismissPress.as_attr(),
        "dismiss-press"
    );
    assert_eq!(
        ContextualHelpOpenInteractionSource::ExternalSync.as_attr(),
        "external-sync"
    );
}
