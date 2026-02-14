#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaginationItem {
    Page(usize),
    Dots,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PaginationState {
    pub current_page: usize,
    pub effective_total_pages: usize,
    pub is_empty: bool,
    pub is_prev_disabled: bool,
    pub is_next_disabled: bool,
}

pub fn resolve_pagination_state(
    total_pages: usize,
    current_page: usize,
    disabled: bool,
) -> PaginationState {
    let effective_total_pages = total_pages.max(1);
    let current_page = current_page.clamp(1, effective_total_pages);

    PaginationState {
        current_page,
        effective_total_pages,
        is_empty: total_pages == 0,
        is_prev_disabled: disabled || current_page <= 1,
        is_next_disabled: disabled || current_page >= effective_total_pages,
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>, default: &str) -> String {
    normalize_optional_text(value).unwrap_or_else(|| default.to_string())
}

pub fn resolve_pagination_range(
    total_pages: usize,
    current_page: usize,
    siblings: usize,
    boundaries: usize,
) -> Vec<PaginationItem> {
    if total_pages == 0 {
        return Vec::new();
    }

    let current_page = current_page.clamp(1, total_pages);
    let total_numbers = boundaries.saturating_mul(2) + siblings.saturating_mul(2) + 3;
    let total_blocks = total_numbers + 2;

    if total_pages <= total_numbers {
        return (1..=total_pages).map(PaginationItem::Page).collect();
    }

    let start_page = (current_page.saturating_sub(siblings)).max(boundaries + 2);
    let end_page = (current_page + siblings).min(total_pages.saturating_sub(boundaries + 1));

    let show_left_dots = start_page > boundaries + 2;
    let show_right_dots = end_page + 1 < total_pages.saturating_sub(boundaries);

    let mut range: Vec<PaginationItem> = Vec::new();

    for i in 1..=boundaries {
        range.push(PaginationItem::Page(i));
    }

    if show_left_dots {
        range.push(PaginationItem::Dots);
    } else {
        for i in (boundaries + 1)..start_page {
            range.push(PaginationItem::Page(i));
        }
    }

    for i in start_page..=end_page {
        range.push(PaginationItem::Page(i));
    }

    if show_right_dots {
        range.push(PaginationItem::Dots);
    } else {
        for i in (end_page + 1)..=(total_pages.saturating_sub(boundaries)) {
            range.push(PaginationItem::Page(i));
        }
    }

    for i in (total_pages.saturating_sub(boundaries) + 1)..=total_pages {
        range.push(PaginationItem::Page(i));
    }

    if range.len() > total_blocks {
        range.truncate(total_blocks);
    }

    range
}

#[cfg(test)]
mod tests {
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
}
