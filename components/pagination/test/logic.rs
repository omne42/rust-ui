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
