use super::*;

#[test]
fn label_and_placeholder_defaults_are_stable() {
    assert_eq!(DEFAULT_ARIA_LABEL, "Date field");
    assert_eq!(DEFAULT_LABEL, "Date");
    assert_eq!(DEFAULT_PLACEHOLDER, "yyyy-mm-dd");
    assert_eq!(DEFAULT_YEAR_ARIA_LABEL, "Year");
    assert_eq!(DEFAULT_MONTH_ARIA_LABEL, "Month");
    assert_eq!(DEFAULT_DAY_ARIA_LABEL, "Day");
    assert_eq!(DEFAULT_CLEAR_LABEL, "Clear");
    assert_eq!(DEFAULT_CLEAR_ARIA_LABEL, "Clear date");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  2024-03-08  ".to_string())),
        Some("2024-03-08".to_string())
    );

    assert_eq!(
        normalize_label(Some("  Booking date  ".to_string()), DEFAULT_LABEL),
        ("Booking date".to_string(), true)
    );
    assert_eq!(
        normalize_placeholder(Some("  ".to_string()), DEFAULT_PLACEHOLDER),
        (DEFAULT_PLACEHOLDER.to_string(), false)
    );
    assert_eq!(
        normalize_clear_aria_label(None, DEFAULT_CLEAR_ARIA_LABEL),
        (DEFAULT_CLEAR_ARIA_LABEL.to_string(), false)
    );
}

#[test]
fn resolve_ids_uses_normalized_base() {
    let ids = resolve_ids("  booking-date  ");
    assert_eq!(ids.root_id, "booking-date");
    assert_eq!(ids.label_id, "booking-date-label");
    assert_eq!(ids.year_id, "booking-date-year");
    assert_eq!(ids.month_id, "booking-date-month");
    assert_eq!(ids.day_id, "booking-date-day");

    let default_ids = resolve_ids("   ");
    assert_eq!(default_ids.root_id, "ui-date-field");
}

#[test]
fn date_math_and_parse_contract_are_stable() {
    assert!(is_leap_year(2024));
    assert!(!is_leap_year(2023));
    assert_eq!(days_in_month(2024, 2), 29);
    assert_eq!(days_in_month(2023, 2), 28);
    assert_eq!(normalize_year(0), 1);
    assert_eq!(normalize_year(12_345), 9999);
    assert_eq!(normalize_month(0), 1);
    assert_eq!(normalize_month(99), 12);
    assert_eq!(normalize_day(2024, 2, 99), 29);
    assert_eq!(normalize_day(2023, 2, 99), 28);

    assert_eq!(format_date_value(2024, 2, 9), "2024-02-09");
    assert_eq!(parse_date_value("2024-02-29"), Some((2024, 2, 29)));
    assert_eq!(parse_date_value("2023-02-29"), None);
    assert_eq!(parse_date_value("invalid"), None);
}

#[test]
fn resolve_and_update_parts_are_normalized() {
    assert_eq!(
        normalize_date_value(Some(" 2024-2-9 ".to_string())),
        Some("2024-02-09".to_string())
    );
    assert_eq!(normalize_date_value(Some("2024-13-01".to_string())), None);

    assert_eq!(
        resolve_date_parts(Some("2024-03-10".to_string())),
        (2024, 3, 10, true)
    );
    assert_eq!(resolve_date_parts(None), (1970, 1, 1, false));

    assert_eq!(
        resolve_input_placeholders("yyyy - mm - dd"),
        ("yyyy".to_string(), "mm".to_string(), "dd".to_string())
    );
    assert_eq!(
        resolve_input_placeholders("bad"),
        ("yyyy".to_string(), "mm".to_string(), "dd".to_string())
    );

    assert_eq!(
        update_year_from_input(Some("2024-03-30".to_string()), "2025"),
        Some("2025-03-30".to_string())
    );
    assert_eq!(
        update_month_from_input(Some("2024-03-30".to_string()), "2"),
        Some("2024-02-29".to_string())
    );
    assert_eq!(
        update_day_from_input(Some("2024-02-01".to_string()), "31"),
        Some("2024-02-29".to_string())
    );
}
