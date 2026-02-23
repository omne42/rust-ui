use super::*;

pub(crate) fn date_picker() -> AnyView {
    let (workbench_month, set_workbench_month) = signal(5_u8);
    let (workbench_selected_day, set_workbench_selected_day) = signal(Some(18_u8));
    let (workbench_open, set_workbench_open) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_strong_tone, set_workbench_strong_tone) = signal(false);
    let (workbench_monday_first, set_workbench_monday_first) = signal(false);
    let (workbench_show_outside_days, set_workbench_show_outside_days) = signal(true);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_text, set_workbench_custom_text) = signal(false);
    let (workbench_top_end_placement, set_workbench_top_end_placement) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_open_signal = Signal::derive(move || workbench_open.get());
    let workbench_selected_day_signal = Signal::derive(move || workbench_selected_day.get());
    let on_workbench_open_change = Callback::new(move |next: bool| set_workbench_open.set(next));
    let on_workbench_selected_day_change =
        Callback::new(move |next: Option<u8>| set_workbench_selected_day.set(next));

    let code = Signal::derive(move || {
        r#"<DatePicker
  id_base="release-date".to_string()
  year=2026
  month=3
  default_selected_day=12
  tone=DatePickerTone::Default
  first_weekday=CalendarFirstWeekday::Sunday
  show_outside_days=true
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<DatePicker
  id_base="ship-date".to_string()
  year=2026
  month=4
  default_selected_day=21
  tone=DatePickerTone::Strong
  first_weekday=CalendarFirstWeekday::Monday
  show_outside_days=false
  placeholder="Pick ship date".to_string()
  class_name="docs-date-picker-custom".to_string()
  motion=DatePickerMotion {
    popover: PopoverMotion { initial_scale: 0.95, offset_y_px: 10.0, ..PopoverMotion::default() },
  }
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let month = workbench_month.get();
        let selected_day = workbench_selected_day.get();
        let open = workbench_open.get();
        let disabled = workbench_disabled.get();
        let tone = if workbench_strong_tone.get() {
            "DatePickerTone::Strong"
        } else {
            "DatePickerTone::Default"
        };
        let first_weekday = if workbench_monday_first.get() {
            "CalendarFirstWeekday::Monday"
        } else {
            "CalendarFirstWeekday::Sunday"
        };
        let show_outside_days = workbench_show_outside_days.get();
        let motion = if workbench_custom_motion.get() {
            "DatePickerMotion { popover: PopoverMotion { initial_scale: 0.92, offset_y_px: 12.0, ..PopoverMotion::default() } }"
        } else {
            "DatePickerMotion::default()"
        };
        let placeholder = if workbench_custom_text.get() {
            "\"Pick ship date\".into()"
        } else {
            "\"\".into()"
        };
        let aria_label = if workbench_custom_text.get() {
            "\"Ship date picker\".into()"
        } else {
            "\"\".into()"
        };
        let class_name = if workbench_custom_text.get() {
            "\"docs-date-picker-custom\".into()"
        } else {
            "\"\".into()"
        };
        let popover_placement = if workbench_top_end_placement.get() {
            "PopoverPlacement::TopEnd"
        } else {
            "PopoverPlacement::BottomStart"
        };
        let lang = if workbench_rtl.get() {
            "\"ar\".into()"
        } else {
            "\"en-US\".into()"
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        format!(
            "let (open, set_open) = signal({open});\nlet (selected_day, set_selected_day) = signal({selected_day:?});\n\n<DatePicker\n  id_base=\"docs-date-picker-workbench\".into()\n  year=2026\n  month={month}\n  tone={tone}\n  disabled={disabled}\n  open=Signal::derive(move || open.get())\n  default_open=false\n  on_open_change=Callback::new(move |next| set_open.set(next))\n  selected_day=Signal::derive(move || selected_day.get())\n  default_selected_day=Some(12)\n  on_selected_day_change=Callback::new(move |next| set_selected_day.set(next))\n  first_weekday={first_weekday}\n  show_outside_days={show_outside_days}\n  popover_placement={popover_placement}\n  motion={motion}\n  placeholder={placeholder}\n  aria_label={aria_label}\n  lang={lang}\n  dir={dir}\n  class_name={class_name}\n/>"
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let popover_placement = if workbench_top_end_placement.get() {
            PopoverPlacement::TopEnd
        } else {
            PopoverPlacement::BottomStart
        };
        let dir = if workbench_rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        format!(
            "DatePickerActualConfig {{\n  id_base: {:?},\n  year: {},\n  month: {},\n  tone: {:?},\n  disabled: {},\n  open: {:?},\n  default_open: {:?},\n  on_open_change: {:?},\n  selected_day: {:?},\n  default_selected_day: {:?},\n  on_selected_day_change: {:?},\n  first_weekday: {:?},\n  show_outside_days: {},\n  popover_placement: {:?},\n  motion: {:?},\n  placeholder: {:?},\n  aria_label: {:?},\n  lang: {:?},\n  dir: {:?},\n  class_name: {:?},\n}}",
            "docs-date-picker-workbench",
            2026,
            workbench_month.get(),
            if workbench_strong_tone.get() {
                DatePickerTone::Strong
            } else {
                DatePickerTone::Default
            },
            workbench_disabled.get(),
            workbench_selected_day.get(),
            Some(false),
            Some("Callback<bool>"),
            workbench_selected_day.get(),
            Some(12_u8),
            Some("Callback<Option<u8>>"),
            if workbench_monday_first.get() {
                CalendarFirstWeekday::Monday
            } else {
                CalendarFirstWeekday::Sunday
            },
            workbench_show_outside_days.get(),
            popover_placement,
            if workbench_custom_motion.get() {
                DatePickerMotion {
                    popover: PopoverMotion {
                        initial_scale: 0.92,
                        offset_y_px: 12.0,
                        ..PopoverMotion::default()
                    },
                }
            } else {
                DatePickerMotion::default()
            },
            if workbench_custom_text.get() {
                Some("Pick ship date")
            } else {
                None
            },
            if workbench_custom_text.get() {
                Some("Ship date picker")
            } else {
                None
            },
            if workbench_rtl.get() {
                Some("ar")
            } else {
                Some("en-US")
            },
            dir,
            if workbench_custom_text.get() {
                Some("docs-date-picker-custom")
            } else {
                None
            },
        )
    });

    let workbench_test_css_source: Signal<String> =
        Signal::derive(move || ui::text_input::date_picker::styles::CSS.to_string());

    view! {
        <ComponentPage
            title="DatePicker"
            slug="date-picker"
            group="Forms"
            description="Date picker trigger + popover calendar with centralized open/value/source state contracts and baseline-level popover motion handoff."
        >
            <Playground title="Default + Outside Days" code_signal=code>
                <DatePicker
                    id_base="docs-date-picker-release".to_string()
                    year=2026
                    month=3
                    default_selected_day=12
                    tone=DatePickerTone::Default
                    first_weekday=CalendarFirstWeekday::Sunday
                    show_outside_days=true
                />
            </Playground>

            <Playground
                title="展示 / Config / Code / CSS Test"
                description="Workbench canvas: preview (展示) + settings panel (config) + copy-ready source (code) + scoped css verification (css test)."
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="components/text-input/src/date_picker/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="date-picker-workbench-controls">
                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_month
                                        .update(|value| *value = if *value <= 1 { 12 } else { *value - 1 });
                                })
                            >
                                "Prev month"
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_month
                                        .update(|value| *value = if *value >= 12 { 1 } else { *value + 1 });
                                })
                            >
                                "Next month"
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_selected_day.set(None);
                                })
                            >
                                "Clear day"
                            </ui::Button>
                        </div>

                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_open.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_open.get() { "Set closed" } else { "Set open" }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_disabled.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_disabled.get() { "Set enabled" } else { "Set disabled" }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_strong_tone.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_strong_tone.get() { "Tone strong" } else { "Tone default" }}
                            </ui::Button>
                        </div>

                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_monday_first.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_monday_first.get() {
                                    "Weekday Monday"
                                } else {
                                    "Weekday Sunday"
                                }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_show_outside_days.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_show_outside_days.get() {
                                    "Outside days on"
                                } else {
                                    "Outside days off"
                                }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_motion.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_custom_motion.get() {
                                    "Motion custom"
                                } else {
                                    "Motion default"
                                }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_text.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_custom_text.get() {
                                    "Text custom"
                                } else {
                                    "Text default"
                                }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_top_end_placement.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_top_end_placement.get() {
                                    "Placement top-end"
                                } else {
                                    "Placement bottom-start"
                                }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_rtl.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_rtl.get() {
                                    "RTL locale"
                                } else {
                                    "LTR locale"
                                }}
                            </ui::Button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="date-picker-workbench">
                    {move || {
                        let tone = if workbench_strong_tone.get() {
                            DatePickerTone::Strong
                        } else {
                            DatePickerTone::Default
                        };
                        let first_weekday = if workbench_monday_first.get() {
                            CalendarFirstWeekday::Monday
                        } else {
                            CalendarFirstWeekday::Sunday
                        };
                        let motion = if workbench_custom_motion.get() {
                            DatePickerMotion {
                                popover: PopoverMotion {
                                    initial_scale: 0.92,
                                    offset_y_px: 12.0,
                                    ..PopoverMotion::default()
                                },
                            }
                        } else {
                            DatePickerMotion::default()
                        };
                        let placeholder = if workbench_custom_text.get() {
                            "Pick ship date".to_string()
                        } else {
                            String::new()
                        };
                        let aria_label = if workbench_custom_text.get() {
                            "Ship date picker".to_string()
                        } else {
                            String::new()
                        };
                        let class_name = if workbench_custom_text.get() {
                            "docs-date-picker-custom".to_string()
                        } else {
                            String::new()
                        };

                        view! {
                            <DatePicker
                                id_base="docs-date-picker-workbench".to_string()
                                year=2026
                                month=workbench_month.get()
                                tone=tone
                                disabled=workbench_disabled.get()
                                open=workbench_open_signal
                                default_open=false
                                on_open_change=on_workbench_open_change
                                selected_day=workbench_selected_day_signal
                                default_selected_day=12
                                on_selected_day_change=on_workbench_selected_day_change
                                first_weekday=first_weekday
                                show_outside_days=workbench_show_outside_days.get()
                                popover_placement=if workbench_top_end_placement.get() {
                                    PopoverPlacement::TopEnd
                                } else {
                                    PopoverPlacement::BottomStart
                                }
                                motion=motion
                                placeholder=placeholder
                                aria_label=aria_label
                                lang=if workbench_rtl.get() {
                                    "ar".to_string()
                                } else {
                                    "en-US".to_string()
                                }
                                dir=if workbench_rtl.get() {
                                    A11yDirection::Rtl
                                } else {
                                    A11yDirection::Ltr
                                }
                                class_name=class_name
                            />
                        }
                    }}

                    <span class="ui-muted" data-slot="date-picker-workbench-summary">
                        {move || format!(
                            "month={} · selected_day={:?} · open={} · disabled={} · tone={} · weekday={} · outside_days={} · custom_motion={} · custom_text={} · placement={} · dir={}",
                            workbench_month.get(),
                            workbench_selected_day.get(),
                            workbench_open.get(),
                            workbench_disabled.get(),
                            if workbench_strong_tone.get() { "strong" } else { "default" },
                            if workbench_monday_first.get() { "monday" } else { "sunday" },
                            workbench_show_outside_days.get(),
                            workbench_custom_motion.get(),
                            workbench_custom_text.get(),
                            if workbench_top_end_placement.get() { "top-end" } else { "bottom-start" },
                            if workbench_rtl.get() { "rtl" } else { "ltr" },
                        )}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Comparison Matrix (Default / Quiet / Strong / Disabled)"
                description="Multiple side-by-side states for quick visual and semantic contract comparison."
                code_signal=Signal::derive(move || r#"<DatePicker id_base="date-default".to_string() year=2026 month=3 default_selected_day=12 />
<DatePicker id_base="date-quiet".to_string() year=2026 month=3 tone=DatePickerTone::Quiet />
<DatePicker id_base="date-strong".to_string() year=2026 month=4 tone=DatePickerTone::Strong default_selected_day=21 first_weekday=CalendarFirstWeekday::Monday />
<DatePicker id_base="date-disabled".to_string() year=2026 month=4 disabled=true placeholder="Unavailable".to_string() />"#.to_string())
            >
                <div class="docs-grid docs-grid--2" data-slot="date-picker-comparison-matrix">
                    <DatePicker
                        id_base="docs-date-picker-compare-default".to_string()
                        year=2026
                        month=3
                        default_selected_day=12
                    />
                    <DatePicker
                        id_base="docs-date-picker-compare-quiet".to_string()
                        year=2026
                        month=3
                        tone=DatePickerTone::Quiet
                    />
                    <DatePicker
                        id_base="docs-date-picker-compare-strong".to_string()
                        year=2026
                        month=4
                        tone=DatePickerTone::Strong
                        default_selected_day=21
                        first_weekday=CalendarFirstWeekday::Monday
                    />
                    <DatePicker
                        id_base="docs-date-picker-compare-disabled".to_string()
                        year=2026
                        month=4
                        disabled=true
                        placeholder="Unavailable".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Monday First + Strong Tone" code_signal=states_code>
                <DatePicker
                    id_base="docs-date-picker-ship".to_string()
                    year=2026
                    month=4
                    default_selected_day=21
                    tone=DatePickerTone::Strong
                    first_weekday=CalendarFirstWeekday::Monday
                    show_outside_days=false
                    placeholder="Pick ship date".to_string()
                    class_name="docs-date-picker-custom".to_string()
                    motion=DatePickerMotion {
                        popover: PopoverMotion {
                            initial_scale: 0.95,
                            offset_y_px: 10.0,
                            ..PopoverMotion::default()
                        },
                    }
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
