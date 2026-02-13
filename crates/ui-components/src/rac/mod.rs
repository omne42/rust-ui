mod logic;
mod styles;
mod view;

pub use crate::DirectionMode as Direction;
pub use crate::DirectionProvider as I18nProvider;
pub use crate::Item as Collection;
pub use crate::ListBoxItem as ListBoxLoadMoreItem;
pub use crate::UiRoot as RouterProvider;
pub use logic::{DEFAULT_LOCALE_LTR, DEFAULT_LOCALE_RTL, resolve_locale};
pub use styles::direction_data_attr;

pub type Key = String;
pub type Selection = Vec<Key>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

pub fn is_rtl(direction: Direction) -> bool {
    view::is_rtl(direction)
}

pub fn use_locale(direction: Direction) -> &'static str {
    view::use_locale(direction)
}

pub fn use_filter(value: &str, query: &str) -> bool {
    view::use_filter(value, query)
}

pub fn get_localization_script(direction: Direction) -> String {
    view::get_localization_script(direction)
}
