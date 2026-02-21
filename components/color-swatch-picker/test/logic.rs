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
fn id_base_default_is_resolved_in_logic() {
    assert_eq!(normalize_id_base(None), DEFAULT_ID_BASE);
    assert_eq!(
        normalize_id_base(Some("  docs-picker-id  ".to_string())),
        "docs-picker-id".to_string()
    );
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

#[test]
fn view_state_helpers_centralize_disabled_and_tabindex_derivation() {
    let items = vec![
        ColorSwatchPickerItem::new("#111111"),
        ColorSwatchPickerItem::new("#222222").disabled(true),
        ColorSwatchPickerItem::new("#333333"),
    ];

    assert_eq!(count_disabled_items(&items), 1);
    assert!(!is_item_disabled_at(false, &items, 0));
    assert!(is_item_disabled_at(false, &items, 1));
    assert!(is_item_disabled_at(false, &items, 99));
    assert!(is_item_disabled_at(true, &items, 0));

    assert!(!resolve_option_disabled(false, false));
    assert!(resolve_option_disabled(false, true));
    assert!(resolve_option_disabled(true, false));

    assert_eq!(resolve_option_tabindex(false, 2, 2), 0);
    assert_eq!(resolve_option_tabindex(false, 2, 1), -1);
    assert_eq!(resolve_option_tabindex(true, 2, 2), -1);
}

#[test]
fn resolve_component_state_derives_render_state_from_typed_inputs() {
    let items = vec![
        ColorSwatchPickerItem::new("#111111"),
        ColorSwatchPickerItem::new("#222222").disabled(true),
        ColorSwatchPickerItem::new("#333333"),
    ];

    let state = resolve_component_state(true, &items, Some(2), true, false);
    assert!(state.is_disabled);
    assert_eq!(state.item_count, 3);
    assert_eq!(state.selected_index, Some(2));
    assert_eq!(state.disabled_item_count, 1);
    assert_eq!(state.data_state_attr, "disabled");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}
