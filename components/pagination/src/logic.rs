pub use ui_state_primitives::pagination::{
    PaginationItem, normalize_aria_label, normalize_optional_text, resolve_default_page,
    resolve_direct_page_target, resolve_next_page_target, resolve_pagination_range,
    resolve_pagination_view_state, resolve_prev_page_target, should_sync_uncontrolled_page,
};

#[cfg(test)]
pub use ui_state_primitives::pagination::{
    DEFAULT_PAGE, PaginationPageControlMode, normalize_default_page, resolve_effective_page,
    resolve_page_control_mode, resolve_pagination_state,
};

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
