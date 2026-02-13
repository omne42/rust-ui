use super::Direction;

pub const DEFAULT_LOCALE_LTR: &str = "en-US";
pub const DEFAULT_LOCALE_RTL: &str = "ar-EG";

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
        assert_eq!(use_locale(Direction::Ltr), DEFAULT_LOCALE_LTR);
        assert_eq!(use_locale(Direction::Rtl), DEFAULT_LOCALE_RTL);
    }

    #[test]
    fn resolve_locale_supports_custom_injection() {
        assert_eq!(
            resolve_locale(Direction::Ltr, Some("  fr-FR  ")),
            "fr-FR".to_string()
        );
        assert_eq!(
            resolve_locale(Direction::Rtl, Some("   ")),
            DEFAULT_LOCALE_RTL.to_string()
        );
        assert_eq!(
            resolve_locale(Direction::Ltr, None),
            DEFAULT_LOCALE_LTR.to_string()
        );
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
