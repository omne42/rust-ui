use crate::toggle_button::{
    DEFAULT_TOGGLE_GROUP_ARIA_LABEL, ToggleButtonGroupOrientation, ToggleGroupItem,
    ToggleGroupOrientation, ToggleGroupSelectionMode, ToggleGroupStateInput,
    collect_toggle_group_item_ids, normalize_toggle_button_group_aria_label,
    normalize_toggle_group_aria_label, normalize_toggle_group_items,
    resolve_toggle_button_group_state, resolve_toggle_button_state, resolve_toggle_group_state,
    sanitize_toggle_group_selected_ids, toggle_toggle_group_selected_id,
};
use std::collections::BTreeSet;

#[test]
fn normalize_toggle_group_aria_label_trims_or_falls_back() {
    let (label, explicit) = normalize_toggle_group_aria_label(Some("  View mode  ".to_string()));
    assert_eq!(label, "View mode");
    assert!(explicit);

    let (label, explicit) = normalize_toggle_group_aria_label(Some("   ".to_string()));
    assert_eq!(label, DEFAULT_TOGGLE_GROUP_ARIA_LABEL);
    assert!(!explicit);
}

#[test]
fn normalize_toggle_group_items_applies_id_and_label_fallbacks() {
    let items = normalize_toggle_group_items(vec![
        ToggleGroupItem::new("  ", " First "),
        ToggleGroupItem::new("second", "   "),
    ]);

    assert_eq!(items[0].id, "toggle-1");
    assert_eq!(items[0].label, "First");
    assert_eq!(items[1].id, "second");
    assert_eq!(items[1].label, "second");
}

#[test]
fn sanitize_toggle_group_selected_ids_filters_invalid_and_single_mode() {
    let items = normalize_toggle_group_items(vec![
        ToggleGroupItem::new("bold", "Bold"),
        ToggleGroupItem::new("italic", "Italic").disabled(true),
        ToggleGroupItem::new("underline", "Underline"),
    ]);
    let item_ids = collect_toggle_group_item_ids(&items);

    let selected = BTreeSet::from([
        "bold".to_string(),
        "italic".to_string(),
        "missing".to_string(),
        "underline".to_string(),
    ]);

    let multiple = sanitize_toggle_group_selected_ids(
        selected.clone(),
        &item_ids,
        &items,
        ToggleGroupSelectionMode::Multiple,
    );
    assert_eq!(
        multiple,
        BTreeSet::from(["bold".to_string(), "underline".to_string()])
    );

    let single = sanitize_toggle_group_selected_ids(
        selected,
        &item_ids,
        &items,
        ToggleGroupSelectionMode::Single,
    );
    assert_eq!(single, BTreeSet::from(["bold".to_string()]));
}

#[test]
fn toggle_toggle_group_selected_id_respects_mode_and_disabled_items() {
    let items = normalize_toggle_group_items(vec![
        ToggleGroupItem::new("bold", "Bold"),
        ToggleGroupItem::new("italic", "Italic").disabled(true),
        ToggleGroupItem::new("underline", "Underline"),
    ]);
    let item_ids = collect_toggle_group_item_ids(&items);

    let selected = toggle_toggle_group_selected_id(
        BTreeSet::from(["bold".to_string()]),
        "underline",
        &item_ids,
        &items,
        ToggleGroupSelectionMode::Single,
        true,
    );
    assert_eq!(selected, BTreeSet::from(["underline".to_string()]));

    let selected = toggle_toggle_group_selected_id(
        BTreeSet::from(["bold".to_string()]),
        "italic",
        &item_ids,
        &items,
        ToggleGroupSelectionMode::Multiple,
        true,
    );
    assert_eq!(selected, BTreeSet::from(["bold".to_string()]));
}

#[test]
fn resolve_toggle_group_state_tracks_orientation_selection_and_sources() {
    let state = resolve_toggle_group_state(ToggleGroupStateInput {
        orientation: ToggleGroupOrientation::Vertical,
        selection_mode: ToggleGroupSelectionMode::Single,
        disabled: false,
        attached: true,
        item_count: 3,
        selected_count: 1,
        disabled_item_count: 1,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.orientation_attr, "vertical");
    assert_eq!(state.selection_mode_attr, "single");
    assert!(state.has_selection);
    assert!(!state.is_empty);
    assert!(state.has_disabled_items);
    assert_eq!(state.data_state_attr, "selected");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn resolve_toggle_button_state_masks_interactions_when_disabled() {
    let enabled = resolve_toggle_button_state(true, false, true, true, true, true);
    assert!(enabled.is_selected);
    assert!(enabled.is_pressed);
    assert!(enabled.is_hovered);
    assert!(enabled.is_focused);
    assert!(enabled.is_focus_visible);
    assert_eq!(enabled.data_state(), "selected");

    let disabled = resolve_toggle_button_state(false, true, true, true, true, true);
    assert!(disabled.is_disabled);
    assert!(!disabled.is_enabled);
    assert!(!disabled.is_pressed);
    assert!(!disabled.is_hovered);
    assert!(!disabled.is_focused);
    assert!(!disabled.is_focus_visible);
    assert_eq!(disabled.data_state(), "unselected");
}

#[test]
fn toggle_button_group_accessible_name_and_layout_state_are_stable() {
    let (label, explicit) =
        normalize_toggle_button_group_aria_label(Some("  Toggle actions  ".to_string()));
    assert_eq!(label, "Toggle actions");
    assert!(explicit);

    let (label, explicit) = normalize_toggle_button_group_aria_label(None);
    assert_eq!(label, DEFAULT_TOGGLE_GROUP_ARIA_LABEL);
    assert!(!explicit);

    let vertical =
        resolve_toggle_button_group_state(ToggleButtonGroupOrientation::Vertical, true, true);
    assert!(!vertical.is_horizontal);
    assert!(vertical.is_vertical);
    assert!(vertical.is_attached);
    assert!(!vertical.is_detached);
    assert!(vertical.has_explicit_label);
    assert!(!vertical.has_fallback_label);
}
