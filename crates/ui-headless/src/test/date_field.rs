use super::*;

fn resolve_parts_from_value(value: Option<String>) -> (i32, u8, u8, bool) {
    let Some(value) = value else {
        return (1970, 1, 1, false);
    };

    let mut iter = value.split('-');
    let year = iter.next().and_then(|part| part.parse::<i32>().ok());
    let month = iter.next().and_then(|part| part.parse::<u8>().ok());
    let day = iter.next().and_then(|part| part.parse::<u8>().ok());

    match (year, month, day) {
        (Some(year), Some(month), Some(day)) => (year, month, day, true),
        _ => (1970, 1, 1, false),
    }
}

#[test]
fn date_field_handlers_dispatch_in_enabled_mode() {
    let (year_calls, set_year_calls) = signal(0usize);
    let (month_calls, set_month_calls) = signal(0usize);
    let (day_calls, set_day_calls) = signal(0usize);
    let (clear_calls, set_clear_calls) = signal(0usize);
    let (value, _) = signal(Some("2026-07-22".to_string()));

    let date_field = use_date_field(DateFieldOptions {
        is_disabled: false,
        value: value.into(),
        resolve_parts: Callback::new(resolve_parts_from_value),
        on_year_input: Callback::new(move |_| set_year_calls.update(|count| *count += 1)),
        on_month_input: Callback::new(move |_| set_month_calls.update(|count| *count += 1)),
        on_day_input: Callback::new(move |_| set_day_calls.update(|count| *count += 1)),
        on_clear: Callback::new(move |_| set_clear_calls.update(|count| *count += 1)),
        aria_label: "Invoice date".to_string(),
        aria_labelledby: Some("date-field-label".to_string()),
        lang: None,
        dir: None,
        year_aria_label: "Year".to_string(),
        month_aria_label: "Month".to_string(),
        day_aria_label: "Day".to_string(),
        clear_aria_label: "Clear date".to_string(),
    });

    date_field.handlers.on_year_input.run("2025".to_string());
    date_field.handlers.on_month_input.run("12".to_string());
    date_field.handlers.on_day_input.run("31".to_string());
    date_field.handlers.on_clear.run(());

    assert_eq!(year_calls.get_untracked(), 1);
    assert_eq!(month_calls.get_untracked(), 1);
    assert_eq!(day_calls.get_untracked(), 1);
    assert_eq!(clear_calls.get_untracked(), 1);
}

#[test]
fn date_field_handlers_are_noop_when_disabled() {
    let (year_calls, set_year_calls) = signal(0usize);
    let (clear_calls, set_clear_calls) = signal(0usize);
    let (value, _) = signal(Some("2026-07-22".to_string()));

    let date_field = use_date_field(DateFieldOptions {
        is_disabled: true,
        value: value.into(),
        resolve_parts: Callback::new(resolve_parts_from_value),
        on_year_input: Callback::new(move |_| set_year_calls.update(|count| *count += 1)),
        on_month_input: Callback::new(move |_| {}),
        on_day_input: Callback::new(move |_| {}),
        on_clear: Callback::new(move |_| set_clear_calls.update(|count| *count += 1)),
        aria_label: "Invoice date".to_string(),
        aria_labelledby: None,
        lang: None,
        dir: None,
        year_aria_label: "Year".to_string(),
        month_aria_label: "Month".to_string(),
        day_aria_label: "Day".to_string(),
        clear_aria_label: "Clear date".to_string(),
    });

    date_field.handlers.on_year_input.run("2025".to_string());
    date_field.handlers.on_clear.run(());

    assert_eq!(year_calls.get_untracked(), 0);
    assert_eq!(clear_calls.get_untracked(), 0);
}

#[test]
fn date_field_attrs_and_state_expose_locale_and_segments() {
    let (value, _) = signal(Some("2026-07-22".to_string()));

    let date_field = use_date_field(DateFieldOptions {
        is_disabled: false,
        value: value.into(),
        resolve_parts: Callback::new(resolve_parts_from_value),
        on_year_input: Callback::new(move |_| {}),
        on_month_input: Callback::new(move |_| {}),
        on_day_input: Callback::new(move |_| {}),
        on_clear: Callback::new(move |_| {}),
        aria_label: "Invoice date".to_string(),
        aria_labelledby: Some("date-field-label".to_string()),
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
        year_aria_label: "Year".to_string(),
        month_aria_label: "Month".to_string(),
        day_aria_label: "Day".to_string(),
        clear_aria_label: "Clear date".to_string(),
    });

    assert_eq!(date_field.attrs.role, "group");
    assert_eq!(date_field.attrs.aria_label, "Invoice date");
    assert_eq!(
        date_field.attrs.aria_labelledby.as_deref(),
        Some("date-field-label")
    );
    assert_eq!(date_field.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(date_field.attrs.dir, Some("rtl"));
    assert_eq!(date_field.attrs.year_aria_label, "Year");
    assert_eq!(date_field.attrs.month_aria_label, "Month");
    assert_eq!(date_field.attrs.day_aria_label, "Day");
    assert_eq!(date_field.attrs.clear_aria_label, "Clear date");

    assert_eq!(date_field.state.parts.get_untracked(), (2026, 7, 22, true));
    assert!(date_field.state.has_value.get_untracked());
}
