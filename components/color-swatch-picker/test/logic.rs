use super::*;

#[test]
fn normalize_items_filters_invalid_and_duplicate_colors() {
    let items = vec![
        ColorSwatchPickerItem::named("#ff0000", "Red"),
        ColorSwatchPickerItem::named("#FF0000", "Duplicate red"),
        ColorSwatchPickerItem::new("javascript:alert(1)"),
        ColorSwatchPickerItem::new("#00ff00"),
    ];

    let normalized = normalize_items(items);
    assert_eq!(normalized.len(), 2);
    assert_eq!(normalized[0].color, "#ff0000");
    assert_eq!(normalized[1].color, "#00ff00");
}

#[test]
fn selected_resolution_maps_color_and_index() {
    let items = normalize_items(vec![
        ColorSwatchPickerItem::new("#111111"),
        ColorSwatchPickerItem::new("#222222"),
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
    let named = ColorSwatchPickerItem::named("#ff0000", "Fire truck red");
    assert_eq!(resolve_option_label(&named, 0), "Fire truck red");

    let unnamed = ColorSwatchPickerItem::new("#00ff00");
    assert_eq!(resolve_option_label(&unnamed, 1), "Color 2 (#00ff00)");
}

#[test]
fn resolve_state_tracks_selection_and_sources() {
    let state = resolve_state(ColorSwatchPickerStateInput {
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

    let class_name = compose_class_name(Some("docs-picker".to_string()), state);
    assert!(class_name.contains("ui-color-swatch-picker"));
    assert!(class_name.contains("ui-color-swatch-picker--custom-class"));
    assert!(class_name.contains("docs-picker"));
}
