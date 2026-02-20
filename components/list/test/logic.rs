use super::*;

#[test]
fn resolve_accessible_name_prefers_explicit_aria_label() {
    assert_eq!(
        resolve_accessible_name(
            Some("  Fruit options  ".to_string()),
            Some("trigger-id".to_string())
        ),
        ListAccessibleName {
            aria_label: Some("Fruit options".to_string()),
            aria_labelledby: None,
        }
    );
}

#[test]
fn resolve_accessible_name_uses_labelledby_when_label_missing() {
    assert_eq!(
        resolve_accessible_name(None, Some("  trigger-id  ".to_string())),
        ListAccessibleName {
            aria_label: None,
            aria_labelledby: Some("trigger-id".to_string()),
        }
    );
}

#[test]
fn resolve_accessible_name_defaults_when_none_provided() {
    assert_eq!(
        resolve_accessible_name(None, None),
        ListAccessibleName {
            aria_label: Some("Listbox".to_string()),
            aria_labelledby: None,
        }
    );
}

#[test]
fn resolve_accessible_name_ignores_blank_inputs() {
    assert_eq!(
        resolve_accessible_name(Some("  ".to_string()), Some("".to_string())),
        ListAccessibleName {
            aria_label: Some("Listbox".to_string()),
            aria_labelledby: None,
        }
    );
}

#[test]
fn resolve_state_tracks_item_and_selection_flags() {
    let state = resolve_state(4, Some(2), true);
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert!(state.has_selection);
    assert!(state.has_disabled_options);
}

#[test]
fn resolve_state_treats_out_of_range_selection_as_empty_selection() {
    let state = resolve_state(2, Some(9), false);
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert!(!state.has_selection);
    assert!(!state.has_disabled_options);
}

#[test]
fn resolve_state_handles_empty_listbox() {
    let state = resolve_state(0, None, false);
    assert!(state.is_empty);
    assert!(!state.has_items);
    assert!(!state.has_selection);
    assert!(!state.has_disabled_options);
}
