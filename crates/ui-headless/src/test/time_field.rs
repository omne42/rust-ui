use super::*;

#[test]
fn time_field_handlers_update_value_and_clear() {
    let (value, set_value) = signal(Some("09:30".to_string()));
    let on_value_change = Callback::new(move |next: Option<String>| {
        set_value.set(next);
    });

    let time_field = use_time_field(TimeFieldOptions {
        is_disabled: false,
        value: value.into(),
        on_value_change,
        minute_step: 15,
        aria_label: "Meeting time".to_string(),
        lang: None,
        dir: None,
        hour_aria_label: "Hour".to_string(),
        minute_aria_label: "Minute".to_string(),
        clear_aria_label: "Clear time".to_string(),
    });

    assert_eq!(
        time_field.state.normalized_value.get_untracked(),
        Some("09:30".to_string())
    );
    time_field.handlers.on_hour_input.run("18".to_string());
    assert_eq!(value.get_untracked(), Some("18:30".to_string()));
    time_field.handlers.on_minute_input.run("44".to_string());
    assert_eq!(value.get_untracked(), Some("18:30".to_string()));
    time_field.handlers.on_clear.run(());
    assert_eq!(value.get_untracked(), None);
}

#[test]
fn time_field_handlers_are_noop_when_disabled() {
    let (value, set_value) = signal(Some("09:30".to_string()));
    let on_value_change = Callback::new(move |next: Option<String>| {
        set_value.set(next);
    });

    let time_field = use_time_field(TimeFieldOptions {
        is_disabled: true,
        value: value.into(),
        on_value_change,
        minute_step: 10,
        aria_label: "Meeting time".to_string(),
        lang: None,
        dir: None,
        hour_aria_label: "Hour".to_string(),
        minute_aria_label: "Minute".to_string(),
        clear_aria_label: "Clear time".to_string(),
    });

    time_field.handlers.on_hour_input.run("10".to_string());
    time_field.handlers.on_minute_input.run("50".to_string());
    time_field.handlers.on_clear.run(());
    assert_eq!(value.get_untracked(), Some("09:30".to_string()));
}

#[test]
fn time_field_attrs_expose_locale_and_segment_labels() {
    let (value, _) = signal(None::<String>);
    let time_field = use_time_field(TimeFieldOptions {
        is_disabled: false,
        value: value.into(),
        on_value_change: Callback::new(move |_| {}),
        minute_step: 5,
        aria_label: "Meeting time".to_string(),
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
        hour_aria_label: "Hour".to_string(),
        minute_aria_label: "Minute".to_string(),
        clear_aria_label: "Clear time".to_string(),
    });

    assert_eq!(time_field.attrs.role, "group");
    assert_eq!(time_field.attrs.aria_label, "Meeting time");
    assert_eq!(time_field.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(time_field.attrs.dir, Some("rtl"));
    assert_eq!(time_field.attrs.hour_aria_label, "Hour");
    assert_eq!(time_field.attrs.minute_aria_label, "Minute");
    assert_eq!(time_field.attrs.clear_aria_label, "Clear time");
}
