use super::*;

#[test]
fn normalize_id_base_trims_or_falls_back() {
    assert_eq!(
        normalize_id_base("  docs-dropdown  ".to_string()),
        "docs-dropdown"
    );
    assert_eq!(normalize_id_base("   ".to_string()), DEFAULT_ID_BASE);
}

#[test]
fn normalize_aria_label_trims_or_falls_back() {
    assert_eq!(
        normalize_aria_label(Some("  Actions menu  ".to_string())),
        ("Actions menu".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(Some("\n\t".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn normalize_disabled_indices_dedupes_and_clamps() {
    assert_eq!(normalize_disabled_indices(vec![2, 1, 1, 9], 3), vec![1, 2]);
    assert_eq!(normalize_disabled_indices(vec![4], 0), Vec::<usize>::new());
}

#[test]
fn focus_strategy_for_open_key_maps_arrow_keys() {
    assert_eq!(
        focus_strategy_for_open_key("ArrowDown"),
        Some(DropdownOpenFocusStrategy::First)
    );
    assert_eq!(
        focus_strategy_for_open_key("ArrowUp"),
        Some(DropdownOpenFocusStrategy::Last)
    );
    assert_eq!(focus_strategy_for_open_key("Enter"), None);
}

#[test]
fn resolve_state_tracks_state_and_sources() {
    let state = resolve_state(DropdownStateInput {
        item_count: 3,
        disabled: false,
        close_on_action: false,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        is_controlled: true,
        has_disabled_items: true,
        has_item_kinds: true,
    });

    assert_eq!(state.data_state_attr, "persistent");
    assert!(state.has_items);
    assert!(!state.is_empty);
    assert!(state.keep_open_on_action);
    assert!(state.is_controlled);
    assert!(!state.is_uncontrolled);
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}
