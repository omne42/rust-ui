use super::*;

#[test]
fn resolve_state_tracks_empty_and_disabled() {
    let state = resolve_state(SelectStateInput {
        disabled: true,
        item_count: 0,
        selected_index: Some(0),
        disabled_option_count: 0,
        is_open: false,
        has_custom_class_name: false,
        has_custom_motion: false,
    });

    assert!(state.is_empty);
    assert!(state.trigger_disabled);
    assert!(!state.has_selection);
    assert_eq!(state.selected_index, None);
}

#[test]
fn resolve_state_normalizes_selection_and_markers() {
    let state = resolve_state(SelectStateInput {
        disabled: false,
        item_count: 3,
        selected_index: Some(1),
        disabled_option_count: 2,
        is_open: true,
        has_custom_class_name: true,
        has_custom_motion: true,
    });

    assert!(state.has_items);
    assert!(state.has_selection);
    assert_eq!(state.selected_index, Some(1));
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
}

#[test]
fn compose_class_name_collects_state_classes() {
    let class_name = compose_class_name(
        Some("docs-select".to_string()),
        resolve_state(SelectStateInput {
            disabled: true,
            item_count: 0,
            selected_index: None,
            disabled_option_count: 0,
            is_open: true,
            has_custom_class_name: true,
            has_custom_motion: true,
        }),
    );

    for token in [
        "ui-select",
        "ui-select--open",
        "ui-select--disabled",
        "ui-select--empty",
        "ui-select--custom-motion",
        "ui-select--custom-class",
        "docs-select",
    ] {
        assert!(class_name.contains(token), "class should include `{token}`");
    }
}

#[test]
fn resolve_disabled_option_count_ignores_out_of_range_indices() {
    let disabled = HashSet::from([0_usize, 2, 8]);
    assert_eq!(resolve_disabled_option_count(&disabled, 3), 2);
    assert_eq!(resolve_disabled_option_count(&disabled, 2), 1);
}

#[test]
fn resolve_horizontal_nav_target_skips_disabled_items() {
    let disabled = HashSet::from([1_usize, 3]);

    assert_eq!(
        resolve_horizontal_nav_target(Some(0), SelectHorizontalNav::Next, 5, &disabled),
        Some(2)
    );
    assert_eq!(
        resolve_horizontal_nav_target(Some(4), SelectHorizontalNav::Previous, 5, &disabled),
        Some(2)
    );
    assert_eq!(
        resolve_horizontal_nav_target(None, SelectHorizontalNav::Next, 5, &disabled),
        Some(0)
    );
}

#[test]
fn typeahead_char_accepts_single_ascii_alphanumeric_only() {
    assert_eq!(typeahead_char("A"), Some('a'));
    assert_eq!(typeahead_char("7"), Some('7'));
    assert_eq!(typeahead_char("中"), None);
    assert_eq!(typeahead_char("ab"), None);
}

#[test]
fn find_typeahead_match_wraps_and_skips_disabled_items() {
    let items = vec![
        "Apple".to_string(),
        "Apricot".to_string(),
        "Banana".to_string(),
        "Blueberry".to_string(),
    ];
    let disabled = HashSet::from([1_usize]);

    assert_eq!(find_typeahead_match("ap", 0, &items, &disabled), Some(0));
    assert_eq!(find_typeahead_match("bl", 2, &items, &disabled), Some(3));
    assert_eq!(find_typeahead_match("ap", 1, &items, &disabled), Some(0));
    assert_eq!(find_typeahead_match("zz", 0, &items, &disabled), None);
}
