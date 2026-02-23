use super::*;

#[test]
fn orientation_and_size_contracts_are_stable() {
    assert_eq!(
        StepListOrientation::Horizontal.class_name(),
        "ui-step-list--orientation-horizontal"
    );
    assert_eq!(StepListOrientation::Vertical.as_attr(), "vertical");

    assert_eq!(StepListSize::S.class_name(), "ui-step-list--size-s");
    assert_eq!(StepListSize::M.as_attr(), "m");
    assert_eq!(StepListSize::L.class_name(), "ui-step-list--size-l");
    assert_eq!(StepListSize::Xl.as_attr(), "xl");
}

#[test]
fn normalize_items_filters_duplicate_ids_and_sanitizes_content() {
    let items = vec![
        StepListItem::new(" Setup ", " Setup "),
        StepListItem::new("setup", "Duplicate"),
        StepListItem::new("", "").described("  Upload documents  "),
        StepListItem::new("final review", "Final review").disabled(true),
    ];

    let normalized = normalize_items(items);
    assert_eq!(normalized.len(), 3);

    assert_eq!(normalized[0].id, "setup");
    assert_eq!(normalized[0].label, "Setup");

    assert_eq!(normalized[1].id, "step-3");
    assert_eq!(normalized[1].label, "Step 3");
    assert_eq!(
        normalized[1].description,
        Some("Upload documents".to_string())
    );

    assert_eq!(normalized[2].id, "final-review");
    assert!(normalized[2].disabled);
}

#[test]
fn resolve_selected_and_completed_indices_are_bounded() {
    let items = normalize_items(vec![
        StepListItem::new("a", "A"),
        StepListItem::new("b", "B").disabled(true),
        StepListItem::new("c", "C"),
    ]);

    assert_eq!(resolve_selected_index(&items, Some(0)), Some(0));
    assert_eq!(resolve_selected_index(&items, Some(1)), None);
    assert_eq!(resolve_selected_index(&items, Some(9)), None);

    let completed = normalize_completed_indices(items.len(), vec![0, 2, 5]);
    assert_eq!(completed.len(), 2);
    assert!(completed.contains(&0));
    assert!(completed.contains(&2));
}

#[test]
fn completed_and_disabled_counters_are_derived_in_primitives() {
    let items = normalize_items(vec![
        StepListItem::new("a", "A"),
        StepListItem::new("b", "B").disabled(true),
        StepListItem::new("c", "C"),
        StepListItem::new("d", "D"),
    ]);
    let completed = normalize_completed_indices(items.len(), vec![3, 9]);

    assert!(is_completed_step(0, Some(2), &completed));
    assert!(is_completed_step(1, Some(2), &completed));
    assert!(!is_completed_step(2, Some(2), &completed));
    assert!(is_completed_step(3, Some(2), &completed));

    assert_eq!(count_completed_steps(&items, Some(2), &completed), 2);
    assert_eq!(count_disabled_steps(&items), 1);
}

#[test]
fn navigation_helpers_skip_disabled_items() {
    let items = normalize_items(vec![
        StepListItem::new("a", "A"),
        StepListItem::new("b", "B").disabled(true),
        StepListItem::new("c", "C"),
        StepListItem::new("d", "D"),
    ]);

    assert_eq!(first_enabled_index(&items), Some(0));
    assert_eq!(last_enabled_index(&items), Some(3));
    assert_eq!(next_enabled_index(&items, 0, 1), Some(2));
    assert_eq!(next_enabled_index(&items, 3, -1), Some(2));
    assert_eq!(next_enabled_index(&items, 0, -1), None);
}

#[test]
fn resolve_state_and_item_state_track_contracts() {
    let state = resolve_state(StepListStateInput {
        orientation: StepListOrientation::Vertical,
        size: StepListSize::L,
        emphasized: true,
        disabled: false,
        item_count: 4,
        selected_index: Some(2),
        completed_count: 2,
        disabled_count: 1,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.orientation_attr, "vertical");
    assert_eq!(state.size_attr, "l");
    assert!(state.is_emphasized);
    assert_eq!(state.data_state_attr, "active");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");

    let item_state = resolve_item_state(StepListItemStateInput {
        index: 1,
        selected_index: Some(2),
        completed: true,
        disabled: false,
    });

    assert_eq!(item_state.marker_number, 2);
    assert!(item_state.is_completed);
    assert_eq!(item_state.status_attr, "completed");
    assert_eq!(item_state.status_class, "ui-step-list__item--completed");
    assert!(item_state.is_selectable);

    let disabled = resolve_item_state(StepListItemStateInput {
        index: 2,
        selected_index: Some(2),
        completed: true,
        disabled: true,
    });
    assert_eq!(disabled.status_attr, "disabled");
    assert!(!disabled.is_selectable);
}
