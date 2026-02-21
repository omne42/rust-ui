use super::*;

#[test]
fn dropped_file_is_send_sync_friendly() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DroppedFile>();
}

#[test]
fn resolve_is_disabled_prefers_primary_is_prefix() {
    assert_eq!(
        classify_disabled_input(Some(true), Some(false)),
        DisabledInput::IsDisabled(true)
    );
    assert_eq!(
        classify_disabled_input(Some(false), Some(true)),
        DisabledInput::IsDisabled(false)
    );
    assert_eq!(
        resolve_is_disabled(classify_disabled_input(Some(true), Some(false))),
        (true, DisabledSource::IsDisabled)
    );
    assert_eq!(
        resolve_is_disabled(classify_disabled_input(Some(false), Some(true))),
        (false, DisabledSource::IsDisabled)
    );
}

#[test]
fn resolve_is_disabled_supports_legacy_alias_and_default() {
    assert_eq!(
        classify_disabled_input(None, Some(true)),
        DisabledInput::DisabledAlias(true)
    );
    assert_eq!(
        classify_disabled_input(None, Some(false)),
        DisabledInput::DisabledAlias(false)
    );
    assert_eq!(classify_disabled_input(None, None), DisabledInput::Default);

    assert_eq!(
        resolve_is_disabled(classify_disabled_input(None, Some(true))),
        (true, DisabledSource::DisabledAlias)
    );
    assert_eq!(
        resolve_is_disabled(classify_disabled_input(None, Some(false))),
        (false, DisabledSource::DisabledAlias)
    );
    assert_eq!(
        resolve_is_disabled(classify_disabled_input(None, None)),
        (false, DisabledSource::Default)
    );
}

#[test]
fn resolve_props_keeps_default_source_in_logic() {
    let resolved = resolve_props(DropZonePropsInput {
        disabled_input: classify_disabled_input(None, None),
        motion: None,
    });

    assert!(!resolved.is_disabled);
    assert_eq!(resolved.disabled_source, DisabledSource::Default);
    assert_eq!(resolved.motion, DropZoneMotion::default());
    assert_eq!(resolved.motion_source, MotionSource::Default);
}

#[test]
fn resolve_props_applies_priority_before_view_consumes_state() {
    let custom_motion = DropZoneMotion {
        hover_scale: 1.2,
        ..DropZoneMotion::default()
    };
    let resolved = resolve_props(DropZonePropsInput {
        disabled_input: classify_disabled_input(Some(true), Some(false)),
        motion: Some(custom_motion),
    });

    assert!(resolved.is_disabled);
    assert_eq!(resolved.disabled_source, DisabledSource::IsDisabled);
    assert_eq!(resolved.motion, custom_motion);
    assert_eq!(resolved.motion_source, MotionSource::Custom);
}

#[test]
fn reduce_drag_interaction_derives_drop_target_state_in_logic() {
    let entered = reduce_drag_interaction(DragDepth::default(), DragInteractionAction::Enter);
    assert!(entered.is_drop_target);

    let left = reduce_drag_interaction(entered.depth, DragInteractionAction::Leave);
    assert!(!left.is_drop_target);

    let dropped = reduce_drag_interaction(entered.depth, DragInteractionAction::Drop);
    assert!(!dropped.is_drop_target);
}

#[test]
fn reduce_drag_lifecycle_converges_with_explicit_drag_end_action() {
    assert_eq!(
        reduce_drag_lifecycle(DragLifecyclePhase::Idle, DragLifecycleAction::DragStart),
        DragLifecyclePhase::Dragging
    );
    assert_eq!(
        reduce_drag_lifecycle(DragLifecyclePhase::Dragging, DragLifecycleAction::DragEnd),
        DragLifecyclePhase::Idle
    );
    assert_eq!(DragLifecyclePhase::Idle.as_attr(), "idle");
    assert_eq!(DragLifecyclePhase::Dragging.as_attr(), "dragging");
}

#[test]
fn bool_data_attr_maps_render_markers() {
    assert_eq!(bool_data_attr(true), Some("true"));
    assert_eq!(bool_data_attr(false), None);
}

#[test]
fn resolve_aria_label_source_is_closed_enum_mapping() {
    assert_eq!(resolve_aria_label_source(false), AriaLabelSource::Default);
    assert_eq!(resolve_aria_label_source(true), AriaLabelSource::Custom);
    assert_eq!(AriaLabelSource::Default.as_attr(), "default");
    assert_eq!(AriaLabelSource::Custom.as_attr(), "custom");
}
