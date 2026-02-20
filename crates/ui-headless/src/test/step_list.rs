use super::*;

#[test]
fn step_list_root_attrs_expose_locale_and_role() {
    let attrs = step_list_root_a11y_attrs(
        "Setup steps".to_string(),
        Some(" zh-CN ".to_string()),
        Some(A11yDirection::Rtl),
    );
    assert_eq!(attrs.role, "list");
    assert_eq!(attrs.aria_label, "Setup steps");
    assert_eq!(attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(attrs.dir, Some("rtl"));
}

#[test]
fn step_list_item_contract_sets_tabindex_and_aria() {
    let contract = step_list_item_contract(StepListItemA11yInput {
        index: 1,
        selected_index: Some(1),
        first_enabled_index: Some(0),
        is_current: true,
        is_disabled: false,
        is_selectable: false,
    });
    assert_eq!(contract.attrs.aria_current, Some("step"));
    assert_eq!(contract.attrs.aria_disabled, None);
    assert_eq!(contract.attrs.tabindex, 0);
    assert!(!contract.state.is_selectable);

    let disabled = step_list_item_contract(StepListItemA11yInput {
        index: 2,
        selected_index: Some(1),
        first_enabled_index: Some(0),
        is_current: false,
        is_disabled: true,
        is_selectable: false,
    });
    assert_eq!(disabled.attrs.aria_current, None);
    assert_eq!(disabled.attrs.aria_disabled, Some("true"));
    assert_eq!(disabled.attrs.tabindex, -1);
}

#[test]
fn keyboard_navigation_resolves_orientation_aware_targets() {
    let items = vec![
        StepListItem::new("a", "A"),
        StepListItem::new("b", "B").disabled(true),
        StepListItem::new("c", "C"),
    ];

    assert_eq!(
        resolve_step_list_next_index(&items, StepListOrientation::Horizontal, 0, "ArrowRight"),
        Some(2)
    );
    assert_eq!(
        resolve_step_list_next_index(&items, StepListOrientation::Horizontal, 2, "ArrowLeft"),
        Some(0)
    );
    assert_eq!(
        resolve_step_list_next_index(&items, StepListOrientation::Vertical, 0, "ArrowDown"),
        Some(2)
    );
    assert_eq!(
        resolve_step_list_next_index(&items, StepListOrientation::Vertical, 2, "ArrowUp"),
        Some(0)
    );
    assert_eq!(
        resolve_step_list_next_index(&items, StepListOrientation::Horizontal, 2, "Home"),
        Some(0)
    );
    assert_eq!(
        resolve_step_list_next_index(&items, StepListOrientation::Horizontal, 0, "End"),
        Some(2)
    );
    assert_eq!(
        resolve_step_list_next_index(&items, StepListOrientation::Horizontal, 0, "Enter"),
        None
    );
}
