pub use ui_components::DirectionMode as Direction;
pub use ui_components::DirectionProvider as I18nProvider;
pub use ui_components::Item as Collection;
pub use ui_components::ListBoxItem as ListBoxLoadMoreItem;
pub use ui_components::UiRoot as RouterProvider;

pub type Key = String;
pub type Selection = Vec<Key>;

pub const DEFAULT_LOCALE_LTR: &str = "en-US";
pub const DEFAULT_LOCALE_RTL: &str = "ar-EG";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

pub fn is_rtl(direction: Direction) -> bool {
    direction == Direction::Rtl
}

pub fn use_locale(direction: Direction) -> &'static str {
    if is_rtl(direction) {
        DEFAULT_LOCALE_RTL
    } else {
        DEFAULT_LOCALE_LTR
    }
}

pub fn resolve_locale(direction: Direction, locale: Option<&str>) -> String {
    let locale = locale
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);

    locale.unwrap_or_else(|| use_locale(direction).to_string())
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

pub fn direction_data_attr(direction: Direction) -> &'static str {
    if is_rtl(direction) { "rtl" } else { "ltr" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_defaults_match_direction() {
        assert_eq!(use_locale(Direction::Ltr), DEFAULT_LOCALE_LTR);
        assert_eq!(use_locale(Direction::Rtl), DEFAULT_LOCALE_RTL);
    }

    #[test]
    fn resolve_locale_supports_custom_injection() {
        assert_eq!(resolve_locale(Direction::Ltr, Some("  fr-FR  ")), "fr-FR");
        assert_eq!(
            resolve_locale(Direction::Rtl, Some("   ")),
            DEFAULT_LOCALE_RTL
        );
        assert_eq!(resolve_locale(Direction::Ltr, None), DEFAULT_LOCALE_LTR);
    }

    #[test]
    fn filter_matches_substrings_case_insensitive() {
        assert!(use_filter("Calendar", "len"));
        assert!(use_filter("Calendar", "CAL"));
        assert!(use_filter("Calendar", ""));
        assert!(!use_filter("Calendar", "pick"));
    }

    #[test]
    fn direction_attrs_are_stable() {
        assert_eq!(direction_data_attr(Direction::Ltr), "ltr");
        assert_eq!(direction_data_attr(Direction::Rtl), "rtl");
    }
}
