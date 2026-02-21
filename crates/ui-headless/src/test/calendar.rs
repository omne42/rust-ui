use super::*;

#[test]
fn use_calendar_root_normalizes_locale_and_group_semantics() {
    let root = use_calendar_root(CalendarRootOptions {
        aria_label: "Calendar".to_string(),
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(root.attrs.role, "group");
    assert_eq!(root.attrs.aria_label, "Calendar");
    assert_eq!(root.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(root.attrs.dir, Some("rtl"));
}

#[test]
fn use_calendar_day_exposes_typed_attrs_handlers_and_state() {
    let (pressed_count, set_pressed_count) = signal(0_u8);
    let contract = use_calendar_day(
        CalendarDayA11yInput {
            year: 2026,
            month: 2,
            day: 14,
            in_current_month: true,
            is_selected: true,
        },
        CalendarDayOptions {
            on_press: Some(Callback::new(move |_| {
                set_pressed_count.update(|count| *count += 1);
            })),
        },
    );

    assert_eq!(contract.attrs.aria_selected, Some("true"));
    assert_eq!(contract.attrs.aria_disabled, None);
    assert_eq!(contract.attrs.aria_label, "2026-02-14");
    assert!(!contract.attrs.disabled);
    assert!(contract.state.is_pressable);
    assert!(contract.state.is_selected);
    assert_eq!(contract.state.month_source, "current");

    contract.handlers.press.press.on_click.run(());
    assert_eq!(pressed_count.get_untracked(), 1);
}

#[test]
fn use_calendar_day_disables_outside_month_cells() {
    let (pressed_count, set_pressed_count) = signal(0_u8);
    let contract = use_calendar_day(
        CalendarDayA11yInput {
            year: 2026,
            month: 1,
            day: 31,
            in_current_month: false,
            is_selected: false,
        },
        CalendarDayOptions {
            on_press: Some(Callback::new(move |_| {
                set_pressed_count.update(|count| *count += 1);
            })),
        },
    );

    assert_eq!(contract.attrs.aria_selected, None);
    assert_eq!(contract.attrs.aria_disabled, Some("true"));
    assert!(contract.attrs.disabled);
    assert!(!contract.state.is_pressable);
    assert_eq!(contract.state.month_source, "outside");

    contract.handlers.press.press.on_click.run(());
    assert_eq!(pressed_count.get_untracked(), 0);
}
