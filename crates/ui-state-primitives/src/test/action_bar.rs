use super::*;

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n \t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" action-bar ".to_string())),
        Some("action-bar".to_string())
    );
}

#[test]
fn normalize_labels_use_defaults_when_empty() {
    let (label, custom) = normalize_aria_label(Some("  Batch actions  ".to_string()), "Actions");
    assert_eq!(label, "Batch actions");
    assert!(custom);

    let (label, custom) = normalize_aria_label(Some(" \n ".to_string()), "Actions");
    assert_eq!(label, "Actions");
    assert!(!custom);

    let (clear_label, custom) = normalize_clear_label(None, "Clear selection");
    assert_eq!(clear_label, "Clear selection");
    assert!(!custom);
}

#[test]
fn resolve_selection_kind_supports_all_paths() {
    assert_eq!(resolve_selection_kind(0), ActionBarSelectionKind::Empty);
    assert_eq!(resolve_selection_kind(1), ActionBarSelectionKind::Single);
    assert_eq!(resolve_selection_kind(2), ActionBarSelectionKind::Multiple);
}

#[test]
fn resolve_state_tracks_phase_position_and_source_markers() {
    let state = resolve_state(ActionBarStateInput {
        selected_count: 2,
        position: ActionBarPosition::Top,
        force_visible: false,
        is_controlled_selected_count: true,
        has_default_selected_count: true,
        has_selected_count_change_handler: true,
        has_clear_action: true,
        has_custom_label: true,
        has_custom_class_name: false,
        has_custom_selection_text: true,
        has_custom_clear_label: false,
        has_custom_motion: true,
    });

    assert!(state.is_visible);
    assert!(state.is_top);
    assert!(!state.is_bottom);
    assert_eq!(state.phase_attr, "visible");
    assert_eq!(state.position_attr, "top");
    assert_eq!(state.selection_attr, "multiple");
    assert_eq!(state.control_mode_attr, "controlled");
    assert_eq!(state.selected_count_source_attr, "external");
    assert_eq!(state.default_selected_count_source_attr, "provided");
    assert_eq!(state.selected_count_change_source_attr, "provided");
    assert_eq!(state.clear_action_source_attr, "provided");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert_eq!(state.selection_source_attr, "custom");
    assert_eq!(state.clear_label_source_attr, "default");
    assert_eq!(state.motion_source_attr, "custom");
}
