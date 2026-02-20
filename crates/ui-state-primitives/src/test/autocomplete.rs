use super::*;

fn items() -> Vec<String> {
    vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Apricot".to_string(),
    ]
}

#[test]
fn normalize_label_trims_and_defaults() {
    assert_eq!(normalize_label("  City  ".to_string()), "City");
    assert_eq!(normalize_label("   ".to_string()), DEFAULT_LABEL);
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Pick a city  ".to_string())),
        Some("Pick a city".to_string())
    );
}

#[test]
fn normalize_id_base_falls_back_when_blank() {
    assert_eq!(
        normalize_id_base("  city-autocomplete  ".to_string()),
        "city-autocomplete"
    );
    assert_eq!(normalize_id_base("   ".to_string()), DEFAULT_ID_BASE);
}

#[test]
fn resolve_placeholder_uses_fallback() {
    assert_eq!(
        resolve_placeholder(Some("  Search  ".to_string())),
        "Search"
    );
    assert_eq!(
        resolve_placeholder(Some("   ".to_string())),
        DEFAULT_PLACEHOLDER
    );
    assert_eq!(resolve_placeholder(None), DEFAULT_PLACEHOLDER);
}

#[test]
fn resolve_empty_message_uses_fallback() {
    assert_eq!(
        resolve_empty_message(Some("  Nothing found  ".to_string())),
        "Nothing found"
    );
    assert_eq!(
        resolve_empty_message(Some("   ".to_string())),
        DEFAULT_EMPTY_MESSAGE
    );
    assert_eq!(resolve_empty_message(None), DEFAULT_EMPTY_MESSAGE);
}

#[test]
fn disabled_indices_are_deduped_and_clamped_to_item_count() {
    assert_eq!(normalize_disabled_indices(vec![2, 1, 1, 9], 3), vec![1, 2]);
    assert_eq!(normalize_disabled_indices(vec![4], 0), Vec::<usize>::new());
}

#[test]
fn resolve_state_tracks_component_flags() {
    let state = resolve_state(AutocompleteStateInput {
        item_count: 5,
        disabled_option_count: 2,
        is_disabled: false,
        has_custom_label: true,
        has_custom_description: true,
        has_custom_error: true,
        has_custom_placeholder: true,
        has_custom_id_base: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        is_controlled: true,
    });

    assert_eq!(state.item_count, 5);
    assert_eq!(state.disabled_option_count, 2);
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert!(!state.is_disabled);
    assert!(state.is_enabled);
    assert!(state.has_description);
    assert!(state.has_error);
    assert!(state.has_disabled_options);
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.description_source_attr, "custom");
    assert_eq!(state.error_source_attr, "custom");
    assert_eq!(state.placeholder_source_attr, "custom");
    assert_eq!(state.id_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert!(state.has_custom_label);
    assert!(state.has_custom_class_name);
    assert!(state.has_custom_motion);
    assert!(state.is_controlled);
    assert!(!state.is_uncontrolled);
}

#[test]
fn filter_returns_all_when_not_typed_or_query_empty() {
    let items = items();
    assert_eq!(filter_indices(&items, "ap", false), vec![0, 1, 2]);
    assert_eq!(filter_indices(&items, " ", true), vec![0, 1, 2]);
}

#[test]
fn filter_is_case_insensitive_contains() {
    let items = items();
    assert_eq!(filter_indices(&items, "ap", true), vec![0, 2]);
    assert_eq!(filter_indices(&items, "BAN", true), vec![1]);
}

#[test]
fn selected_index_maps_to_filtered_position() {
    let filtered = vec![2, 0];
    assert_eq!(map_selected_to_filtered(Some(0), &filtered), Some(1));
    assert_eq!(map_selected_to_filtered(Some(2), &filtered), Some(0));
    assert_eq!(map_selected_to_filtered(Some(1), &filtered), None);
    assert_eq!(map_selected_to_filtered(None, &filtered), None);
}

#[test]
fn filtered_to_original_maps_by_lookup() {
    let filtered = vec![2, 0];
    assert_eq!(map_filtered_to_original(0, &filtered), Some(2));
    assert_eq!(map_filtered_to_original(1, &filtered), Some(0));
    assert_eq!(map_filtered_to_original(2, &filtered), None);
}
