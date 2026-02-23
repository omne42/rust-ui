use super::*;

#[test]
fn small_totals_return_all_pages() {
    let items = resolve_pagination_range(5, 3, 1, 1);
    assert_eq!(
        items,
        vec![
            PaginationItem::Page(1),
            PaginationItem::Page(2),
            PaginationItem::Page(3),
            PaginationItem::Page(4),
            PaginationItem::Page(5)
        ]
    );
}

#[test]
fn adds_dots_when_middle_is_large() {
    let items = resolve_pagination_range(20, 10, 1, 1);
    assert_eq!(
        items,
        vec![
            PaginationItem::Page(1),
            PaginationItem::Dots,
            PaginationItem::Page(9),
            PaginationItem::Page(10),
            PaginationItem::Page(11),
            PaginationItem::Dots,
            PaginationItem::Page(20),
        ]
    );
}

#[test]
fn clamps_out_of_range_pages() {
    let items = resolve_pagination_range(10, 999, 1, 1);
    assert!(items.contains(&PaginationItem::Page(10)));
}

#[test]
fn resolve_state_clamps_page_and_disabled_flags() {
    let state = resolve_pagination_state(12, 99, false);
    assert_eq!(state.current_page, 12);
    assert_eq!(state.effective_total_pages, 12);
    assert!(!state.is_empty);
    assert!(!state.is_prev_disabled);
    assert!(state.is_next_disabled);
}

#[test]
fn resolve_state_handles_zero_total_pages() {
    let state = resolve_pagination_state(0, 0, false);
    assert_eq!(state.current_page, 1);
    assert_eq!(state.effective_total_pages, 1);
    assert!(state.is_empty);
    assert!(state.is_prev_disabled);
    assert!(state.is_next_disabled);
}

#[test]
fn resolve_state_global_disabled_overrides_navigation() {
    let state = resolve_pagination_state(8, 4, true);
    assert!(state.is_prev_disabled);
    assert!(state.is_next_disabled);
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  aria  ".to_string())),
        Some("aria".to_string())
    );

    assert_eq!(normalize_aria_label(None, "Default"), "Default");
    assert_eq!(
        normalize_aria_label(Some("  Pagination  ".to_string()), "Default"),
        "Pagination"
    );
}

#[test]
fn page_control_mode_and_default_page_are_normalized() {
    assert_eq!(DEFAULT_PAGE, 1);
    assert_eq!(normalize_default_page(0), 1);
    assert_eq!(normalize_default_page(7), 7);
    assert_eq!(resolve_default_page(None), 1);
    assert_eq!(resolve_default_page(Some(0)), 1);
    assert_eq!(resolve_default_page(Some(5)), 5);
    assert_eq!(
        resolve_page_control_mode(Some(9)),
        PaginationPageControlMode::Controlled
    );
    assert_eq!(
        resolve_page_control_mode(None),
        PaginationPageControlMode::Uncontrolled
    );
    assert_eq!(resolve_effective_page(Some(11), 3), 11);
    assert_eq!(resolve_effective_page(None, 3), 3);
}

#[test]
fn navigation_targets_are_derived_in_logic() {
    let controlled = resolve_pagination_view_state(12, Some(4), 1, false);
    assert_eq!(
        controlled.control_mode,
        PaginationPageControlMode::Controlled
    );
    assert_eq!(controlled.state.current_page, 4);
    assert_eq!(resolve_prev_page_target(controlled), Some(3));
    assert_eq!(resolve_next_page_target(controlled), Some(5));
    assert_eq!(resolve_direct_page_target(controlled, 4), None);
    assert_eq!(resolve_direct_page_target(controlled, 99), Some(12));

    let first_page = resolve_pagination_view_state(12, Some(1), 1, false);
    assert_eq!(resolve_prev_page_target(first_page), None);

    let last_page = resolve_pagination_view_state(12, Some(12), 1, false);
    assert_eq!(resolve_next_page_target(last_page), None);

    let disabled = resolve_pagination_view_state(12, Some(4), 1, true);
    assert_eq!(resolve_prev_page_target(disabled), None);
    assert_eq!(resolve_next_page_target(disabled), None);
    assert_eq!(resolve_direct_page_target(disabled, 6), None);

    let uncontrolled = resolve_pagination_view_state(12, None, 5, false);
    assert_eq!(
        uncontrolled.control_mode,
        PaginationPageControlMode::Uncontrolled
    );
    assert!(should_sync_uncontrolled_page(uncontrolled.control_mode));
}
