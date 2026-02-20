use super::*;

#[test]
fn resolve_checkbox_group_ids_builds_legend_id() {
    assert_eq!(
        resolve_checkbox_group_ids("prefs"),
        CheckboxGroupIds {
            legend_id: "prefs-label".to_string(),
        }
    );
}

#[test]
fn resolve_checkbox_group_state_tracks_optional_without_messages() {
    let state = resolve_checkbox_group_state(false, false, false, false, false);

    assert!(!state.is_disabled);
    assert!(state.is_enabled);
    assert!(!state.is_invalid);
    assert!(state.is_valid);
    assert!(!state.is_required);
    assert!(state.is_optional);
    assert!(!state.has_description);
    assert!(!state.has_error);
    assert!(!state.shows_error);
    assert!(!state.has_messages);
}

#[test]
fn resolve_checkbox_group_state_tracks_invalid_required_and_messages() {
    let state = resolve_checkbox_group_state(true, true, true, true, true);

    assert!(state.is_disabled);
    assert!(!state.is_enabled);
    assert!(state.is_invalid);
    assert!(!state.is_valid);
    assert!(state.is_required);
    assert!(!state.is_optional);
    assert!(state.has_description);
    assert!(state.has_error);
    assert!(state.shows_error);
    assert!(state.has_messages);
}

#[test]
fn normalize_checkbox_group_label_trims_and_defaults() {
    assert_eq!(
        normalize_checkbox_group_label("  Fruits  ".to_string()),
        "Fruits".to_string()
    );
    assert_eq!(
        normalize_checkbox_group_label("   ".to_string()),
        "Options".to_string()
    );
}

#[test]
fn normalize_checkbox_group_optional_text_filters_blank_values() {
    assert_eq!(normalize_checkbox_group_optional_text(None), None);
    assert_eq!(
        normalize_checkbox_group_optional_text(Some("  ".to_string())),
        None
    );
    assert_eq!(
        normalize_checkbox_group_optional_text(Some("  Pick at least one  ".to_string())),
        Some("Pick at least one".to_string())
    );
}
