use super::*;

#[test]
fn normalize_items_filters_invalid_and_duplicate_colors() {
    let items = vec![
        SwatchPickerItem::named("#ff0000", "Red"),
        SwatchPickerItem::named("#FF0000", "Duplicate red"),
        SwatchPickerItem::new("javascript:alert(1)"),
        SwatchPickerItem::new("#00ff00"),
    ];

    let normalized = normalize_items(items);
    assert_eq!(normalized.len(), 2);
    assert_eq!(normalized[0].color, "#ff0000");
    assert_eq!(normalized[1].color, "#00ff00");
}

#[test]
fn selected_resolution_maps_color_and_index() {
    let items = normalize_items(vec![
        SwatchPickerItem::new("#111111"),
        SwatchPickerItem::new("#222222"),
    ]);

    assert_eq!(
        resolve_selected_index(&items, Some("#222222".to_string())),
        Some(1)
    );
    assert_eq!(
        resolve_selected_color(&items, Some(0)),
        Some("#111111".to_string())
    );
    assert_eq!(resolve_selected_color(&items, Some(999)), None);
}

#[test]
fn label_resolution_prefers_color_name() {
    let named = SwatchPickerItem::named("#ff0000", "Fire truck red");
    assert_eq!(resolve_option_label(&named, 0), "Fire truck red");

    let unnamed = SwatchPickerItem::new("#00ff00");
    assert_eq!(resolve_option_label(&unnamed, 1), "Color 2 (#00ff00)");
}

#[test]
fn aria_label_normalization_uses_default_when_missing() {
    assert_eq!(
        normalize_aria_label(Some("  Palette  ".to_string())),
        ("Palette".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.to_string(), false)
    );
}

#[test]
fn resolve_state_tracks_selection_and_sources() {
    let state = resolve_state(SwatchPickerStateInput {
        disabled: false,
        item_count: 3,
        selected_index: Some(1),
        disabled_item_count: 1,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert!(state.has_items);
    assert!(state.has_selection);
    assert_eq!(state.data_state_attr, "selected");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");

    let empty = resolve_state(SwatchPickerStateInput {
        disabled: false,
        item_count: 0,
        selected_index: None,
        disabled_item_count: 0,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });
    assert_eq!(empty.data_state_attr, "empty");

    let disabled = resolve_state(SwatchPickerStateInput {
        disabled: true,
        item_count: 4,
        selected_index: Some(0),
        disabled_item_count: 2,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });
    assert_eq!(disabled.data_state_attr, "disabled");
    assert_eq!(disabled.aria_source_attr, "default");
    assert_eq!(disabled.class_source_attr, "default");
}
