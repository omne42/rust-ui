use super::*;
use crate::{CarouselPartStateInput, CarouselSlot};

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
            is_controlled: false,
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
