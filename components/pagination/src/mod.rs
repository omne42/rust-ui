mod i18n;
mod logic;
mod motion;
pub mod styles;
mod view;

pub use i18n::PaginationStrings;
pub use logic::{PaginationItem, resolve_pagination_range};
pub use motion::PaginationMotion;
pub use view::Pagination;
