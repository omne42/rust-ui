use super::*;

#[test]
fn file_type_is_send_sync_friendly() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FileTriggerFile>();
}

#[test]
fn resolve_state_tracks_disabled_and_motion_sources() {
    let default_state = resolve_state(FileTriggerStateInput {
        disabled: false,
        has_custom_motion: false,
    });

    assert!(default_state.is_enabled);
    assert!(!default_state.is_disabled);
    assert_eq!(default_state.state_attr, "ready");
    assert_eq!(default_state.motion_source_attr, "default");
    assert!(!default_state.has_custom_motion);

    let custom_state = resolve_state(FileTriggerStateInput {
        disabled: true,
        has_custom_motion: true,
    });

    assert!(!custom_state.is_enabled);
    assert!(custom_state.is_disabled);
    assert_eq!(custom_state.state_attr, "disabled");
    assert_eq!(custom_state.motion_source_attr, "custom");
    assert!(custom_state.has_custom_motion);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(resolve_state(FileTriggerStateInput {
        disabled: true,
        has_custom_motion: true,
    }));

    for token in [
        "ui-file-trigger",
        "ui-file-trigger--disabled",
        "ui-file-trigger--custom-motion",
    ] {
        assert!(
            class_name.contains(token),
            "FileTrigger class name should include `{token}`."
        );
    }
}
