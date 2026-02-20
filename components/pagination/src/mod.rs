mod i18n;
mod logic;
pub mod styles;
mod view;

pub use i18n::PaginationStrings;
pub use logic::{PaginationItem, resolve_pagination_range};
pub use view::Pagination;
