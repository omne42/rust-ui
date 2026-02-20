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
    let (label, custom) =
        normalize_aria_label(Some("  Batch actions  ".to_string()), DEFAULT_ARIA_LABEL);
    assert_eq!(label, "Batch actions");
    assert!(custom);

    let (label, custom) = normalize_aria_label(Some(" \n ".to_string()), DEFAULT_ARIA_LABEL);
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom);

    let (clear_label, custom) = normalize_clear_label(None, DEFAULT_CLEAR_LABEL);
    assert_eq!(clear_label, DEFAULT_CLEAR_LABEL);
    assert!(!custom);
}

#[test]
fn resolve_selection_text_supports_default_and_custom_paths() {
    let strings = ActionBarStrings::default();
    assert_eq!(
        resolve_selection_text(0, None, &strings),
        DEFAULT_SELECTION_EMPTY_LABEL
    );
    assert_eq!(
        resolve_selection_text(1, None, &strings),
        DEFAULT_SELECTION_SINGLE_LABEL
    );
    assert_eq!(
        resolve_selection_text(3, None, &strings),
        "3 items selected"
    );

    assert_eq!(
        resolve_selection_text(24, Some("24 rows selected".to_string()), &strings),
        "24 rows selected"
    );
}

#[test]
fn normalize_default_selected_count_falls_back_to_zero() {
    assert_eq!(normalize_default_selected_count(None), 0);
    assert_eq!(normalize_default_selected_count(Some(7)), 7);
}

#[test]
fn resolve_view_state_maps_typed_input_into_state_primitive_input() {
    let state = resolve_view_state(ActionBarViewStateInput {
        selected_count: 2,
        position: ActionBarPosition::Top,
        is_force_visible: true,
        is_controlled_selected_count: true,
        has_default_selected_count: true,
        has_selected_count_change_handler: true,
        has_clear_action: true,
        has_custom_label: false,
        has_custom_class_name: true,
        has_custom_selection_text: false,
        has_custom_clear_label: true,
        has_custom_motion: false,
    });

    assert!(state.is_visible);
    assert_eq!(state.position_attr, "top");
    assert_eq!(state.selection_attr, "multiple");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.clear_label_source_attr, "custom");
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

#[test]
fn compose_class_name_includes_state_and_custom_markers() {
    let state = resolve_state(ActionBarStateInput {
        selected_count: 0,
        position: ActionBarPosition::Bottom,
        force_visible: false,
        is_controlled_selected_count: false,
        has_default_selected_count: false,
        has_selected_count_change_handler: false,
        has_clear_action: false,
        has_custom_label: false,
        has_custom_class_name: true,
        has_custom_selection_text: false,
        has_custom_clear_label: true,
        has_custom_motion: true,
    });

    let class_name = compose_class_name(Some("docs-action-bar".to_string()), state);
    assert!(class_name.contains("ui-action-bar"));
    assert!(class_name.contains("ui-action-bar--state-hidden"));
    assert!(class_name.contains("ui-action-bar--position-bottom"));
    assert!(class_name.contains("ui-action-bar--selection-empty"));
    assert!(class_name.contains("ui-action-bar--custom-class"));
    assert!(class_name.contains("ui-action-bar--clear-label-custom"));
    assert!(class_name.contains("ui-action-bar--motion-custom"));
    assert!(class_name.contains("docs-action-bar"));
}
