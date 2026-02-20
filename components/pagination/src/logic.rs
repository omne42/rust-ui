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
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>, default: &str) -> String {
    normalize_optional_text(value).unwrap_or_else(|| default.into())
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
#[path = "../test/logic.rs"]
mod tests;
