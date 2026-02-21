use super::*;

#[test]
fn normalize_id_base_and_aria_label_default_when_blank() {
    assert_eq!(
        normalize_id_base("  featured-carousel ".to_string()),
        "featured-carousel"
    );
    assert_eq!(normalize_id_base(" ".to_string()), DEFAULT_ID_BASE);

    assert_eq!(resolve_aria_label(None), (DEFAULT_ARIA_LABEL.into(), false));
    assert_eq!(
        resolve_aria_label(Some("  Feature carousel ".to_string())),
        ("Feature carousel".to_string(), true)
    );
}

#[test]
fn resolve_items_normalizes_ids_titles_and_dom_ids() {
    let items = resolve_items(
        "docs-carousel",
        vec![
            CarouselItemInput {
                id: "Featured".to_string(),
                title: "Featured".to_string(),
                description: None,
                disabled: false,
            },
            CarouselItemInput {
                id: "Featured".to_string(),
                title: " ".to_string(),
                description: Some(" Product update ".to_string()),
                disabled: false,
            },
            CarouselItemInput {
                id: " ".to_string(),
                title: "Gallery".to_string(),
                description: None,
                disabled: true,
            },
        ],
    );

    assert_eq!(items.len(), 3);
    assert_eq!(items[0].id, "featured");
    assert_eq!(items[1].id, "featured-2");
    assert_eq!(items[2].id, "slide-3");
    assert_eq!(items[1].title, "Slide 2");
    assert_eq!(items[0].slide_dom_id, "docs-carousel-featured-slide");
    assert_eq!(items[0].dot_dom_id, "docs-carousel-featured-dot");
    assert_eq!(items[1].description.as_deref(), Some("Product update"));
    assert!(items[2].disabled);
}

#[test]
fn resolve_state_core_tracks_state_and_source_markers() {
    let state = resolve_state_core(CarouselStateCoreInput {
        item_count: 3,
        selected_index: Some(1),
        focused_index: Some(1),
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
fn sanitize_index_clamps_out_of_bounds() {
    assert_eq!(sanitize_index(Some(0), 3), Some(0));
    assert_eq!(sanitize_index(Some(3), 3), None);
    assert_eq!(sanitize_index(None, 3), None);
}

#[test]
fn default_selected_index_resolves_to_enabled_fallback() {
    let disabled_flags = [true, false, false];
    assert_eq!(
        resolve_default_selected_index(Some(0), &disabled_flags),
        Some(1)
    );
    assert_eq!(
        resolve_default_selected_index(Some(2), &disabled_flags),
        Some(2)
    );
    assert_eq!(
        resolve_default_selected_index(Some(99), &disabled_flags),
        Some(1)
    );
    assert_eq!(
        resolve_default_selected_index(None, &disabled_flags),
        Some(1)
    );
}

#[test]
fn item_state_attrs_follow_priority_rules() {
    let selected = resolve_item_state_attrs(1, Some(1), Some(1), false);
    assert_eq!(selected.status, CarouselItemStatus::Selected);
    assert_eq!(selected.status.as_attr(), "selected");
    assert_eq!(selected.selected_attr, Some("true"));
    assert_eq!(selected.focused_attr, Some("true"));
    assert_eq!(selected.disabled_attr, None);
    assert!(selected.is_selected);

    let disabled = resolve_item_state_attrs(1, Some(1), Some(1), true);
    assert_eq!(disabled.status, CarouselItemStatus::Disabled);
    assert_eq!(disabled.status.as_attr(), "disabled");
    assert_eq!(disabled.selected_attr, Some("true"));
    assert_eq!(disabled.focused_attr, Some("true"));
    assert_eq!(disabled.disabled_attr, Some("true"));
    assert!(disabled.is_selected);
}

#[test]
fn item_selection_gate_respects_disabled_flag() {
    assert!(can_item_receive_selection(false));
    assert!(!can_item_receive_selection(true));
}

#[test]
fn sanitize_enabled_index_rejects_disabled_and_out_of_bounds() {
    let disabled_flags = [false, true, false];
    assert_eq!(sanitize_enabled_index(Some(0), &disabled_flags), Some(0));
    assert_eq!(sanitize_enabled_index(Some(1), &disabled_flags), None);
    assert_eq!(sanitize_enabled_index(Some(9), &disabled_flags), None);
}

#[test]
fn enabled_index_helpers_find_edges() {
    let disabled_flags = [true, false, false, true];
    assert_eq!(first_enabled_index(&disabled_flags), Some(1));
    assert_eq!(last_enabled_index(&disabled_flags), Some(2));
}

#[test]
fn adjacent_enabled_index_supports_loop_and_bounded_navigation() {
    let disabled_flags = [false, true, false];
    assert_eq!(adjacent_enabled_index(&disabled_flags, 0, 1, true), Some(2));
    assert_eq!(adjacent_enabled_index(&disabled_flags, 2, 1, true), Some(0));
    assert_eq!(adjacent_enabled_index(&disabled_flags, 2, 1, false), None);
    assert_eq!(
        adjacent_enabled_index(&disabled_flags, 2, -1, false),
        Some(0)
    );
}

#[test]
fn adjacent_enabled_index_returns_none_when_no_other_enabled_item_exists() {
    let disabled_flags = [true, false, true];
    assert_eq!(adjacent_enabled_index(&disabled_flags, 1, 1, true), None);
    assert_eq!(adjacent_enabled_index(&disabled_flags, 1, -1, true), None);
    assert_eq!(adjacent_enabled_index(&disabled_flags, 1, 0, true), None);
}

#[test]
fn initial_indices_fall_back_to_first_enabled_item() {
    let disabled_flags = [true, false, false];
    assert_eq!(
        resolve_initial_selected_index(&disabled_flags, Some(0)),
        Some(1)
    );
    assert_eq!(
        resolve_initial_focused_index(&disabled_flags, Some(0)),
        Some(1)
    );
}

#[test]
fn resolved_indices_keep_selection_and_focus_stable() {
    let disabled_flags = [true, false, false];

    assert_eq!(resolve_selected_index(&disabled_flags, Some(0)), Some(1));
    assert_eq!(resolve_selected_index(&disabled_flags, Some(2)), Some(2));

    assert_eq!(
        resolve_focused_index(&disabled_flags, Some(0), Some(2)),
        Some(2)
    );
    assert_eq!(
        resolve_focused_index(&disabled_flags, Some(1), Some(2)),
        Some(1)
    );
}

#[test]
fn step_and_edge_navigation_respect_disabled_and_direction() {
    let disabled_flags = [true, false, true, false];

    assert_eq!(step_selected_index(&disabled_flags, None, 1, true), Some(3));
    assert_eq!(
        step_selected_index(&disabled_flags, Some(3), -1, false),
        Some(1)
    );
    assert_eq!(
        step_selected_index(&disabled_flags, Some(3), 1, false),
        None
    );
    assert_eq!(step_selected_index(&disabled_flags, Some(1), 0, true), None);

    assert_eq!(edge_selected_index(&disabled_flags, false), Some(1));
    assert_eq!(edge_selected_index(&disabled_flags, true), Some(3));
}

#[test]
fn can_step_and_active_index_follow_priority_rules() {
    let disabled_flags = [true, false, true];

    assert!(!can_step_selection(&disabled_flags, Some(1), -1, true));
    assert!(!can_step_selection(&disabled_flags, Some(1), 1, false));

    assert_eq!(resolve_active_index(&disabled_flags, Some(0), Some(1)), 1);
    assert_eq!(resolve_active_index(&disabled_flags, Some(1), Some(0)), 1);
    assert_eq!(resolve_active_index(&[true, true], None, None), 0);
}
