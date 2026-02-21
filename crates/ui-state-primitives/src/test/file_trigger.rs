use super::*;

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
fn resolve_props_uses_single_default_source_and_alias_precedence() {
    let defaults = resolve_props(FileTriggerPropsInput::default());
    assert_eq!(
        defaults,
        FileTriggerProps {
            is_disabled: false,
            selection_mode: FileTriggerSelectionMode::SingleFile,
        }
    );

    let from_legacy_aliases = resolve_props(FileTriggerPropsInput {
        disabled: Some(true),
        multiple: Some(true),
        accept_directory: Some(true),
        ..FileTriggerPropsInput::default()
    });
    assert_eq!(
        from_legacy_aliases,
        FileTriggerProps {
            is_disabled: true,
            selection_mode: FileTriggerSelectionMode::Directory,
        }
    );

    let is_prefix_wins = resolve_props(FileTriggerPropsInput {
        is_disabled: Some(false),
        disabled: Some(true),
        is_multiple: Some(false),
        multiple: Some(true),
        is_accept_directory: Some(false),
        accept_directory: Some(true),
    });
    assert_eq!(
        is_prefix_wins,
        FileTriggerProps {
            is_disabled: false,
            selection_mode: FileTriggerSelectionMode::SingleFile,
        }
    );

    let multiple_mode = resolve_props(FileTriggerPropsInput {
        is_multiple: Some(true),
        ..FileTriggerPropsInput::default()
    });
    assert_eq!(
        multiple_mode.selection_mode,
        FileTriggerSelectionMode::MultipleFiles
    );
}

#[test]
fn resolve_render_state_composes_props_and_state() {
    let render_state = resolve_render_state(FileTriggerRenderStateInput {
        props: FileTriggerPropsInput {
            is_disabled: Some(true),
            disabled: Some(false),
            is_multiple: Some(true),
            multiple: Some(false),
            is_accept_directory: Some(true),
            accept_directory: Some(false),
        },
        has_custom_motion: true,
    });

    assert_eq!(
        render_state.props,
        FileTriggerProps {
            is_disabled: true,
            selection_mode: FileTriggerSelectionMode::Directory,
        }
    );
    assert_eq!(render_state.state.state_attr, "disabled");
    assert_eq!(render_state.state.motion_source_attr, "custom");
    assert!(render_state.state.has_custom_motion);
}
