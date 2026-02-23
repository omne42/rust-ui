use super::*;

pub(crate) fn date_range_picker() -> AnyView {
    let (workbench_start_day, set_workbench_start_day) = signal(Some(8_u8));
    let (workbench_end_day, set_workbench_end_day) = signal(Some(19_u8));
    let workbench_start_day_signal: Signal<Option<u8>> = workbench_start_day.into();
    let workbench_end_day_signal: Signal<Option<u8>> = workbench_end_day.into();
    let (on_start_day_change_runs, set_on_start_day_change_runs) = signal(0_u32);
    let (on_end_day_change_runs, set_on_end_day_change_runs) = signal(0_u32);
    let on_start_day_change = Callback::new(move |next: Option<u8>| {
        set_workbench_start_day.set(next);
        set_on_start_day_change_runs.update(|count| *count += 1);
    });
    let on_end_day_change = Callback::new(move |next: Option<u8>| {
        set_workbench_end_day.set(next);
        set_on_end_day_change_runs.update(|count| *count += 1);
    });

    let (workbench_strong_tone, set_workbench_strong_tone) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_monday, set_workbench_monday) = signal(true);
    let (workbench_show_outside_days, set_workbench_show_outside_days) = signal(true);
    let (workbench_custom_text, set_workbench_custom_text) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<DateRangePicker
  id_base="docs-date-range-picker-hello".to_string()
  start_year=2026
  start_month=8
  end_year=2026
  end_month=8
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let tone = if workbench_strong_tone.get() {
            "DateRangePickerTone::Strong"
        } else {
            "DateRangePickerTone::Default"
        };
        let first_weekday = if workbench_monday.get() {
            "CalendarFirstWeekday::Monday"
        } else {
            "CalendarFirstWeekday::Sunday"
        };
        let class_name = if workbench_custom_class.get() {
            "docs-date-range-picker-custom"
        } else {
            ""
        };
        let start_label = if workbench_custom_text.get() {
            "Ship from"
        } else {
            "Start date"
        };
        let end_label = if workbench_custom_text.get() {
            "Ship to"
        } else {
            "End date"
        };

        [
            "<DateRangePicker".to_string(),
            "  id_base=\"docs-date-range-picker-workbench\".to_string()".to_string(),
            "  start_year=2026".to_string(),
            "  start_month=8".to_string(),
            "  end_year=2026".to_string(),
            "  end_month=8".to_string(),
            format!("  tone={tone}"),
            format!("  disabled={}", bool_word(workbench_disabled.get())),
            format!(
                "  start_day=Signal::derive(|| {:?})",
                workbench_start_day.get()
            ),
            "  default_start_day=8".to_string(),
            "  on_start_day_change=on_start_day_change".to_string(),
            format!("  end_day=Signal::derive(|| {:?})", workbench_end_day.get()),
            "  default_end_day=19".to_string(),
            "  on_end_day_change=on_end_day_change".to_string(),
            format!("  first_weekday={first_weekday}"),
            format!(
                "  show_outside_days={}",
                bool_word(workbench_show_outside_days.get())
            ),
            format!("  start_label={}", rust_string_literal(start_label)),
            format!("  end_label={}", rust_string_literal(end_label)),
            "  start_placeholder=\"Start day\".to_string()".to_string(),
            "  end_placeholder=\"End day\".to_string()".to_string(),
            "  start_aria_label=\"Start date picker\".to_string()".to_string(),
            "  end_aria_label=\"End date picker\".to_string()".to_string(),
            "  invalid_range_message=\"End date must be after start date\".to_string()".to_string(),
            "  aria_label=\"Release window\".to_string()".to_string(),
            format!("  class_name={}", rust_string_literal(class_name)),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let actual_config = Signal::derive(move || {
        let tone = if workbench_strong_tone.get() {
            DateRangePickerTone::Strong
        } else {
            DateRangePickerTone::Default
        };
        let first_weekday = if workbench_monday.get() {
            CalendarFirstWeekday::Monday
        } else {
            CalendarFirstWeekday::Sunday
        };
        let start_label = if workbench_custom_text.get() {
            "Ship from"
        } else {
            "Start date"
        };
        let end_label = if workbench_custom_text.get() {
            "Ship to"
        } else {
            "End date"
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-date-range-picker-custom")
        } else {
            None
        };
        let start = workbench_start_day.get();
        let end = workbench_end_day.get();
        let start_text = start.map_or_else(|| "None".to_string(), |it| format!("Some({it})"));
        let end_text = end.map_or_else(|| "None".to_string(), |it| format!("Some({it})"));

        format!(
            "DateRangePickerActualConfig {{\n  id_base: \"docs-date-range-picker-workbench\",\n  start_year: 2026,\n  start_month: 8,\n  end_year: 2026,\n  end_month: 8,\n  tone: {tone:?},\n  disabled: {},\n  start_day: {start_text},\n  default_start_day: Some(8),\n  on_start_day_change: \"runs={}\",\n  end_day: {end_text},\n  default_end_day: Some(19),\n  on_end_day_change: \"runs={}\",\n  first_weekday: {first_weekday:?},\n  show_outside_days: {},\n  start_label: {},\n  end_label: {},\n  start_placeholder: \"Start day\",\n  end_placeholder: \"End day\",\n  start_aria_label: \"Start date picker\",\n  end_aria_label: \"End date picker\",\n  invalid_range_message: \"End date must be after start date\",\n  aria_label: \"Release window\",\n  class_name: {class_name:?},\n}}",
            bool_word(workbench_disabled.get()),
            on_start_day_change_runs.get(),
            on_end_day_change_runs.get(),
            bool_word(workbench_show_outside_days.get()),
            rust_string_literal(start_label),
            rust_string_literal(end_label),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<DateRangePicker id_base="range-default".to_string() start_year=2026 start_month=8 end_year=2026 end_month=8 default_start_day=8 default_end_day=19 />
<DateRangePicker id_base="range-strong".to_string() start_year=2026 start_month=8 end_year=2026 end_month=8 tone=DateRangePickerTone::Strong default_start_day=22 default_end_day=10 />
<DateRangePicker id_base="range-disabled".to_string() start_year=2026 start_month=8 end_year=2026 end_month=8 disabled=true default_start_day=5 default_end_day=12 />"#.to_string()
    });

    view! {
        <ComponentPage
            title="DateRangePicker"
            slug="date-range-picker"
            group="Forms"
            description="Date range workbench with complete API coverage and callback-state feedback."
        >
            <Playground title="Hello World (Default Range)" code_signal=hello_code>
                <DateRangePicker
                    id_base="docs-date-range-picker-hello".to_string()
                    start_year=2026
                    start_month=8
                    end_year=2026
                    end_month=8
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="date-range-picker-workbench-controls">
                        <Switch checked=workbench_strong_tone set_checked=set_workbench_strong_tone>
                            "Strong tone"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_monday set_checked=set_workbench_monday>
                            "First weekday Monday"
                        </Switch>
                        <Switch
                            checked=workbench_show_outside_days
                            set_checked=set_workbench_show_outside_days
                        >
                            "Show outside days"
                        </Switch>
                        <Switch checked=workbench_custom_text set_checked=set_workbench_custom_text>
                            "Custom labels"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>

                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_start_day
                                        .update(|value| *value = value.map(|day| day.saturating_sub(1).max(1)));
                                })
                            >
                                "Start -1"
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_start_day
                                        .update(|value| *value = value.map(|day| (day + 1).min(31)));
                                })
                            >
                                "Start +1"
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_end_day
                                        .update(|value| *value = value.map(|day| day.saturating_sub(1).max(1)));
                                })
                            >
                                "End -1"
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_end_day
                                        .update(|value| *value = value.map(|day| (day + 1).min(31)));
                                })
                            >
                                "End +1"
                            </ui::Button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="date-range-picker-workbench-preview">
                    <DateRangePicker
                        id_base="docs-date-range-picker-workbench".to_string()
                        start_year=2026
                        start_month=8
                        end_year=2026
                        end_month=8
                        tone=if workbench_strong_tone.get() {
                            DateRangePickerTone::Strong
                        } else {
                            DateRangePickerTone::Default
                        }
                        disabled=workbench_disabled.get()
                        start_day=workbench_start_day_signal
                        default_start_day=8
                        on_start_day_change=on_start_day_change
                        end_day=workbench_end_day_signal
                        default_end_day=19
                        on_end_day_change=on_end_day_change
                        first_weekday=if workbench_monday.get() {
                            CalendarFirstWeekday::Monday
                        } else {
                            CalendarFirstWeekday::Sunday
                        }
                        show_outside_days=workbench_show_outside_days.get()
                        start_label=if workbench_custom_text.get() {
                            "Ship from".to_string()
                        } else {
                            "Start date".to_string()
                        }
                        end_label=if workbench_custom_text.get() {
                            "Ship to".to_string()
                        } else {
                            "End date".to_string()
                        }
                        start_placeholder="Start day".to_string()
                        end_placeholder="End day".to_string()
                        start_aria_label="Start date picker".to_string()
                        end_aria_label="End date picker".to_string()
                        invalid_range_message="End date must be after start date".to_string()
                        aria_label="Release window".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-date-range-picker-custom".to_string()
                        } else {
                            String::new()
                        }
                    />

                    <span class="ui-muted" data-slot="date-range-picker-workbench-feedback">
                        "start: "
                        {move || workbench_start_day.get().map_or_else(|| "none".to_string(), |day| day.to_string())}
                        " · end: "
                        {move || workbench_end_day.get().map_or_else(|| "none".to_string(), |day| day.to_string())}
                        " · on_start_day_change: " {move || on_start_day_change_runs.get()}
                        " · on_end_day_change: " {move || on_end_day_change_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Invalid / Disabled)" code_signal=matrix_code>
                <div class="docs-row" data-slot="date-range-picker-state-matrix">
                    <DateRangePicker
                        id_base="docs-date-range-picker-matrix-default".to_string()
                        start_year=2026
                        start_month=8
                        end_year=2026
                        end_month=8
                        default_start_day=8
                        default_end_day=19
                    />
                    <DateRangePicker
                        id_base="docs-date-range-picker-matrix-invalid".to_string()
                        start_year=2026
                        start_month=8
                        end_year=2026
                        end_month=8
                        tone=DateRangePickerTone::Strong
                        default_start_day=22
                        default_end_day=10
                    />
                    <DateRangePicker
                        id_base="docs-date-range-picker-matrix-disabled".to_string()
                        start_year=2026
                        start_month=8
                        end_year=2026
                        end_month=8
                        disabled=true
                        default_start_day=5
                        default_end_day=12
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
