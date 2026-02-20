use super::*;

#[test]
fn resolve_accessible_name_prefers_explicit_aria_label() {
    assert_eq!(
        resolve_accessible_name(
            Some("  File actions  ".to_string()),
            Some("trigger-id".to_string())
        ),
        MenuAccessibleName {
            aria_label: Some("File actions".to_string()),
            aria_labelledby: None,
        }
    );
}

#[test]
fn resolve_accessible_name_uses_labelledby_when_label_missing() {
    assert_eq!(
        resolve_accessible_name(None, Some("  trigger-id  ".to_string())),
        MenuAccessibleName {
            aria_label: None,
            aria_labelledby: Some("trigger-id".to_string()),
        }
    );
}

#[test]
fn resolve_accessible_name_defaults_when_none_provided() {
    assert_eq!(
        resolve_accessible_name(None, None),
        MenuAccessibleName {
            aria_label: Some("Menu".to_string()),
            aria_labelledby: None,
        }
    );
}

#[test]
fn resolve_accessible_name_ignores_blank_inputs() {
    assert_eq!(
        resolve_accessible_name(Some("  ".to_string()), Some("".to_string())),
        MenuAccessibleName {
            aria_label: Some("Menu".to_string()),
            aria_labelledby: None,
        }
    );
}

#[test]
fn resolve_state_tracks_item_checked_and_disabled_flags() {
    let state = resolve_state(3, true, true);
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert!(state.has_checked_items);
    assert!(state.has_disabled_items);
}

#[test]
fn resolve_state_handles_empty_menu() {
    let state = resolve_state(0, false, false);
    assert!(state.is_empty);
    assert!(!state.has_items);
    assert!(!state.has_checked_items);
    assert!(!state.has_disabled_items);
}
