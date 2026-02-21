use super::*;
use crate::{CarouselItemStatus, CarouselPartStateInput, CarouselSlot};

fn disabled_flags(items: &[CarouselItemResolved]) -> Vec<bool> {
    items.iter().map(|item| item.disabled).collect()
}

#[test]
fn id_base_and_aria_label_defaults_are_stable() {
    assert_eq!(
        normalize_id_base("  featured-carousel  ".to_string()),
        "featured-carousel"
    );
    assert_eq!(normalize_id_base(" ".to_string()), DEFAULT_ID_BASE);

    assert_eq!(resolve_aria_label(None), (DEFAULT_ARIA_LABEL.into(), false));
    assert_eq!(
        resolve_aria_label(Some("  Product slides  ".to_string())),
        ("Product slides".to_string(), true)
    );
}

#[test]
fn a11y_and_i18n_labels_follow_props_then_bundle_fallback() {
    assert_eq!(
        resolve_aria_label_with_fallback(Some("  Product slides  ".to_string()), "Carousel"),
        ("Product slides".to_string(), true)
    );
    assert_eq!(
        resolve_aria_label_with_fallback(None, "  Featured carousel  "),
        ("Featured carousel".to_string(), false)
    );
    assert_eq!(
        resolve_label_with_fallback(Some("  Back  ".to_string()), "Previous"),
        ("Back".to_string(), true)
    );
    assert_eq!(
        resolve_label_with_fallback(None, "  Next slide  "),
        ("Next slide".to_string(), false)
    );
}

#[test]
fn indicator_aria_label_template_supports_placeholder_and_plain_prefix() {
    assert_eq!(
        resolve_indicator_aria_label("Go to {title}", "Overview"),
        "Go to Overview"
    );
    assert_eq!(
        resolve_indicator_aria_label("Slide", "Overview"),
        "Slide Overview"
    );
}

#[test]
fn semantic_marker_values_stay_in_closed_sets() {
    assert_eq!(state_attr(0, false, false), "empty");
    assert_eq!(state_attr(2, false, false), "idle");
    assert_eq!(state_attr(2, false, true), "focused");
    assert_eq!(state_attr(2, true, true), "selected");

    assert_eq!(item_attr(0), "empty");
    assert_eq!(item_attr(1), "populated");

    assert_eq!(selected_attr(false), "absent");
    assert_eq!(selected_attr(true), "present");
    assert_eq!(focus_attr(false), "absent");
    assert_eq!(focus_attr(true), "present");
    assert_eq!(navigation_attr(false), "bounded");
    assert_eq!(navigation_attr(true), "loop");
    assert_eq!(selection_mode_attr(false), "uncontrolled");
    assert_eq!(selection_mode_attr(true), "controlled");

    let default_state = resolve_state(CarouselPartStateInput {
        slot: CarouselSlot::Root,
        item_count: 1,
        selected_index: None,
        focused_index: None,
        has_disabled_items: false,
        orientation: CarouselOrientation::Horizontal,
        loop_navigation: true,
        is_controlled: false,
        has_custom_id_base: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
        has_custom_orientation: false,
        has_custom_loop_navigation: false,
        has_custom_selected_index: false,
        has_custom_default_selected_index: false,
        has_custom_on_selected_index_change: false,
        has_custom_motion: false,
    });
    assert_eq!(default_state.id_source_attr, "default");
    assert_eq!(default_state.aria_label_source_attr, "default");
    assert_eq!(default_state.class_source_attr, "default");
    assert_eq!(default_state.orientation_source_attr, "default");
    assert_eq!(default_state.loop_navigation_source_attr, "default");
    assert_eq!(default_state.selected_index_source_attr, "default");
    assert_eq!(default_state.default_selected_index_source_attr, "default");
    assert_eq!(default_state.selected_index_change_source_attr, "default");
    assert_eq!(default_state.motion_source_attr, "default");

    let custom_state = resolve_state(CarouselPartStateInput {
        slot: CarouselSlot::Root,
        item_count: 1,
        selected_index: Some(0),
        focused_index: Some(0),
        has_disabled_items: false,
        orientation: CarouselOrientation::Vertical,
        loop_navigation: false,
        is_controlled: true,
        has_custom_id_base: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        has_custom_orientation: true,
        has_custom_loop_navigation: true,
        has_custom_selected_index: true,
        has_custom_default_selected_index: true,
        has_custom_on_selected_index_change: true,
        has_custom_motion: true,
    });
    assert_eq!(custom_state.id_source_attr, "custom");
    assert_eq!(custom_state.aria_label_source_attr, "custom");
    assert_eq!(custom_state.class_source_attr, "custom");
    assert_eq!(custom_state.orientation_source_attr, "custom");
    assert_eq!(custom_state.loop_navigation_source_attr, "custom");
    assert_eq!(custom_state.selected_index_source_attr, "custom");
    assert_eq!(custom_state.default_selected_index_source_attr, "custom");
    assert_eq!(custom_state.selected_index_change_source_attr, "custom");
    assert_eq!(custom_state.motion_source_attr, "custom");
}

#[test]
fn resolve_items_normalizes_ids_and_titles() {
    let items = resolve_items(
        "docs-carousel",
        vec![
            CarouselItem::new("Featured", "Featured"),
            CarouselItem::new("Featured", " "),
            CarouselItem::new(" ", "Gallery"),
        ],
    );

    assert_eq!(items.len(), 3);
    assert_eq!(items[0].id, "featured");
    assert_eq!(items[1].id, "featured-2");
    assert_eq!(items[2].id, "slide-3");
    assert_eq!(items[1].title, "Slide 2");
    assert_eq!(items[0].slide_dom_id, "docs-carousel-featured-slide");
    assert_eq!(items[0].dot_dom_id, "docs-carousel-featured-dot");
}

#[test]
fn adjacent_index_handles_loop_and_non_loop() {
    let items = resolve_items(
        "docs-carousel",
        vec![
            CarouselItem::new("a", "A"),
            CarouselItem::new("b", "B").disabled(true),
            CarouselItem::new("c", "C"),
        ],
    );
    let disabled_flags = disabled_flags(&items);

    assert_eq!(
        carousel_primitives::adjacent_enabled_index(&disabled_flags, 0, 1, true),
        Some(2)
    );
    assert_eq!(
        carousel_primitives::adjacent_enabled_index(&disabled_flags, 2, 1, true),
        Some(0)
    );
    assert_eq!(
        carousel_primitives::adjacent_enabled_index(&disabled_flags, 2, 1, false),
        None
    );
    assert_eq!(
        carousel_primitives::adjacent_enabled_index(&disabled_flags, 2, -1, false),
        Some(0)
    );
}

#[test]
fn selected_and_focus_indices_are_sanitized() {
    let items = resolve_items(
        "docs-carousel",
        vec![
            CarouselItem::new("a", "A").disabled(true),
            CarouselItem::new("b", "B"),
        ],
    );
    let disabled_flags = disabled_flags(&items);

    assert_eq!(
        carousel_primitives::sanitize_enabled_index(Some(0), &disabled_flags),
        None
    );
    assert_eq!(
        carousel_primitives::sanitize_enabled_index(Some(1), &disabled_flags),
        Some(1)
    );
    assert_eq!(
        carousel_primitives::sanitize_enabled_index(Some(1), &disabled_flags),
        Some(1)
    );
    assert_eq!(
        carousel_primitives::resolve_initial_selected_index(&disabled_flags, Some(0)),
        Some(1)
    );
    assert_eq!(
        carousel_primitives::resolve_initial_focused_index(&disabled_flags, Some(0)),
        Some(1)
    );
}

#[test]
fn default_selected_index_priority_is_centralized_in_logic() {
    let items = resolve_items(
        "docs-carousel",
        vec![
            CarouselItem::new("a", "A").disabled(true),
            CarouselItem::new("b", "B"),
            CarouselItem::new("c", "C"),
        ],
    );

    assert_eq!(
        resolve_default_selected_index(Some(0), &items),
        Some(1),
        "disabled default index should be normalized to first enabled item"
    );
    assert_eq!(
        resolve_default_selected_index(Some(99), &items),
        Some(1),
        "out-of-range default index should be clamped by logic normalization"
    );
    assert_eq!(
        resolve_default_selected_index(None, &items),
        Some(1),
        "missing default should still resolve via logic fallback rules"
    );
}

#[test]
fn item_state_derivation_is_centralized_and_deterministic() {
    let selected = resolve_item_state_attrs(1, Some(1), Some(1), false);
    assert_eq!(selected.status, CarouselItemStatus::Selected);
    assert_eq!(selected.selected_attr, Some("true"));
    assert_eq!(selected.focused_attr, Some("true"));
    assert_eq!(selected.disabled_attr, None);
    assert!(selected.is_selected);

    let disabled = resolve_item_state_attrs(1, Some(1), Some(1), true);
    assert_eq!(disabled.status, CarouselItemStatus::Disabled);
    assert_eq!(disabled.selected_attr, Some("true"));
    assert_eq!(disabled.focused_attr, Some("true"));
    assert_eq!(disabled.disabled_attr, Some("true"));
    assert!(disabled.is_selected);
}

#[test]
fn item_interaction_gate_is_driven_by_logic() {
    assert!(can_item_receive_selection(false));
    assert!(!can_item_receive_selection(true));
}

#[test]
fn runtime_index_derivation_is_delegated_to_primitives() {
    let items = resolve_items(
        "docs-carousel",
        vec![
            CarouselItem::new("a", "A").disabled(true),
            CarouselItem::new("b", "B"),
            CarouselItem::new("c", "C"),
        ],
    );

    assert_eq!(resolve_selected_index(Some(0), &items), Some(1));
    assert_eq!(resolve_focused_index(Some(0), Some(2), &items), Some(2));
    assert_eq!(step_selected_index(Some(1), 1, true, &items), Some(2));
    assert_eq!(step_selected_index(Some(2), 1, false, &items), None);
    assert_eq!(edge_selected_index(false, &items), Some(1));
    assert_eq!(edge_selected_index(true, &items), Some(2));
    assert!(can_step_selection(Some(1), 1, true, &items));
    assert_eq!(resolve_active_index(Some(0), Some(2), &items), 2);
}

#[test]
fn resolve_state_tracks_source_markers() {
    let state = resolve_state(CarouselPartStateInput {
        slot: CarouselSlot::Root,
        item_count: 3,
        selected_index: Some(1),
        focused_index: Some(1),
        has_disabled_items: true,
        orientation: CarouselOrientation::Vertical,
        loop_navigation: false,
        is_controlled: true,
        has_custom_id_base: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
        has_custom_orientation: true,
        has_custom_loop_navigation: true,
        has_custom_selected_index: true,
        has_custom_default_selected_index: true,
        has_custom_on_selected_index_change: true,
        has_custom_motion: true,
    });

    assert_eq!(state.state_attr, "selected");
    assert_eq!(state.navigation_attr, "bounded");
    assert_eq!(state.selection_mode_attr, "controlled");
    assert_eq!(state.id_source_attr, "custom");
    assert_eq!(state.aria_label_source_attr, "default");
    assert_eq!(state.motion_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(CarouselPartStateInput {
            slot: CarouselSlot::Root,
            item_count: 3,
            selected_index: Some(1),
            focused_index: Some(1),
            has_disabled_items: true,
            orientation: CarouselOrientation::Vertical,
            loop_navigation: true,
            is_controlled: true,
            has_custom_id_base: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            has_custom_orientation: true,
            has_custom_loop_navigation: true,
            has_custom_selected_index: true,
            has_custom_default_selected_index: true,
            has_custom_on_selected_index_change: true,
            has_custom_motion: true,
        }),
    );

    for token in [
        "ui-carousel",
        "ui-carousel--vertical",
        "ui-carousel--selected",
        "ui-carousel--has-disabled-items",
        "ui-carousel--loop",
        "ui-carousel--controlled",
        "ui-carousel--custom-id",
        "ui-carousel--custom-aria-label",
        "ui-carousel--custom-class",
        "ui-carousel--custom-orientation",
        "ui-carousel--custom-loop-navigation",
        "ui-carousel--custom-selected-index",
        "ui-carousel--custom-default-selected-index",
        "ui-carousel--custom-selected-index-change",
        "ui-carousel--custom-motion",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn resolve_state_selection_mode_matches_controlled_switch() {
    let controlled = resolve_state(CarouselPartStateInput {
        slot: CarouselSlot::Root,
        item_count: 3,
        selected_index: Some(1),
        focused_index: Some(1),
        has_disabled_items: false,
        orientation: CarouselOrientation::Horizontal,
        loop_navigation: true,
        is_controlled: true,
        has_custom_id_base: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
        has_custom_orientation: false,
        has_custom_loop_navigation: false,
        has_custom_selected_index: true,
        has_custom_default_selected_index: false,
        has_custom_on_selected_index_change: true,
        has_custom_motion: false,
    });

    let uncontrolled = resolve_state(CarouselPartStateInput {
        slot: CarouselSlot::Root,
        item_count: 3,
        selected_index: Some(1),
        focused_index: Some(1),
        has_disabled_items: false,
        orientation: CarouselOrientation::Horizontal,
        loop_navigation: true,
        is_controlled: false,
        has_custom_id_base: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
        has_custom_orientation: false,
        has_custom_loop_navigation: false,
        has_custom_selected_index: false,
        has_custom_default_selected_index: true,
        has_custom_on_selected_index_change: true,
        has_custom_motion: false,
    });

    assert_eq!(controlled.selection_mode_attr, "controlled");
    assert!(controlled.is_controlled);
    assert!(!controlled.is_uncontrolled);

    assert_eq!(uncontrolled.selection_mode_attr, "uncontrolled");
    assert!(!uncontrolled.is_controlled);
    assert!(uncontrolled.is_uncontrolled);
}
