pub use crate::DirectionMode as Direction;
pub use crate::DirectionProvider as I18nProvider;
pub use crate::Item as Collection;
pub use crate::ListBoxItem as ListBoxLoadMoreItem;
pub use crate::UiRoot as RouterProvider;

pub type Key = String;
pub type Selection = Vec<Key>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

pub fn is_rtl(direction: Direction) -> bool {
    direction == Direction::Rtl
}

pub fn use_locale(direction: Direction) -> &'static str {
    if is_rtl(direction) { "ar-EG" } else { "en-US" }
}

pub fn use_filter(value: &str, query: &str) -> bool {
    let value = value.trim().to_lowercase();
    let query = query.trim().to_lowercase();

    query.is_empty() || value.contains(&query)
}

pub fn get_localization_script(direction: Direction) -> String {
    format!(
        "window.__RUST_UI_DIRECTION__ = '{}';",
        if is_rtl(direction) { "rtl" } else { "ltr" }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_rtl_maps_direction_flag() {
        assert!(!is_rtl(Direction::Ltr));
        assert!(is_rtl(Direction::Rtl));
    }

    #[test]
    fn use_locale_returns_expected_defaults() {
        assert_eq!(use_locale(Direction::Ltr), "en-US");
        assert_eq!(use_locale(Direction::Rtl), "ar-EG");
    }

    #[test]
    fn use_filter_matches_case_insensitive_substrings() {
        assert!(use_filter("Calendar", "len"));
        assert!(use_filter("Calendar", "CAL"));
        assert!(use_filter("Calendar", ""));
        assert!(!use_filter("Calendar", "pick"));
    }

    #[test]
    fn localization_script_tracks_direction() {
        assert!(get_localization_script(Direction::Ltr).contains("'ltr'"));
        assert!(get_localization_script(Direction::Rtl).contains("'rtl'"));
    }
}
