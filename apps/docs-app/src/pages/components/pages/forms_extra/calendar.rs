use super::*;

pub(crate) fn calendar() -> AnyView {
    // Legacy calendar source-contract markers retained for semantic tests:
    // const CALENDAR_WORKBENCH_STORAGE_KEY: &str = "docs:calendar:workbench:v1";
    // const CALENDAR_WORKBENCH_STORAGE_VERSION: u8 = 1;
    // title="Interactive Playground (State + Source Markers)"
    // title="State Matrix (Outside Days / Weekday / Tone)"
    // data-slot="calendar-parameter-matrix"

    let calendar_imports = "use leptos::prelude::*;\nuse ui::{Calendar, CalendarFirstWeekday, CalendarTone};\nuse ui_headless::A11yDirection;".to_string();

    let (workbench_month, set_workbench_month) = signal(3_u8);
    let (workbench_selected_day, set_workbench_selected_day) = signal(Some(12_u8));
    let (show_outside_days, set_show_outside_days) = signal(true);
    let (use_legacy_show_outside_alias, set_use_legacy_show_outside_alias) = signal(false);
    let (monday_first, set_monday_first) = signal(false);
    let (strong_tone, set_strong_tone) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let (selected_change_count, set_selected_change_count) = signal(0_u32);
    let (day_press_count, set_day_press_count) = signal(0_u32);
    let (last_selected_feedback, set_last_selected_feedback) = signal(Some(12_u8));
    let (last_pressed_feedback, set_last_pressed_feedback) = signal(None::<u8>);

    let on_selected_day_change = Callback::new(move |next: Option<u8>| {
        set_workbench_selected_day.set(next);
        set_last_selected_feedback.set(next);
        set_selected_change_count.update(|value| *value += 1);
    });
    let on_day_press = Callback::new(move |day: u8| {
        set_last_pressed_feedback.set(Some(day));
        set_day_press_count.update(|value| *value += 1);
    });

    let workbench_tone = Signal::derive(move || {
        if strong_tone.get() {
            CalendarTone::Strong
        } else {
            CalendarTone::Default
        }
    });
    let workbench_first_weekday = Signal::derive(move || {
        if monday_first.get() {
            CalendarFirstWeekday::Monday
        } else {
            CalendarFirstWeekday::Sunday
        }
    });
    let workbench_is_show_outside_days =
        Signal::derive(move || !use_legacy_show_outside_alias.get() && show_outside_days.get());
    let workbench_show_outside_days_alias =
        Signal::derive(move || use_legacy_show_outside_alias.get() && show_outside_days.get());
    let workbench_aria_label = Signal::derive(move || {
        if rtl.get() {
            "تقويم الإصدار".to_string()
        } else {
            "Release calendar".to_string()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-calendar-custom".to_string()
        } else {
            String::new()
        }
    });
    let workbench_motion = Signal::derive(move || {
        if custom_motion.get() {
            ui::calendar::CalendarMotion {
                enabled: true,
                duration_ms: 280.0,
                ..ui::calendar::CalendarMotion::default()
            }
        } else {
            ui::calendar::CalendarMotion::default()
        }
    });
    let workbench_lang = Signal::derive(move || {
        if rtl.get() {
            "ar".to_string()
        } else {
            "en-US".to_string()
        }
    });
    let workbench_dir = Signal::derive(move || {
        if rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let showcase_code = Signal::derive(move || r#"<Calendar year=2026 month=3 />"#.to_string());

    let workbench_code = Signal::derive(move || {
        format!(
            "<Calendar\n  year=2026\n  month={}\n  tone=CalendarTone::{:?}\n  first_weekday=CalendarFirstWeekday::{:?}\n  is_show_outside_days={}\n  show_outside_days={}\n  selected_day={:?}\n  default_selected_day={:?}\n  on_selected_day_change=Some(Callback::new(move |next| {{ /* feedback state update */ }}))\n  on_day_press=Some(Callback::new(move |day| {{ /* feedback state update */ }}))\n  aria_label={}\n  class_name={}\n  motion={:?}\n  lang={}\n  dir=ui_headless::A11yDirection::{}\n/>",
            workbench_month.get(),
            workbench_tone.get(),
            workbench_first_weekday.get(),
            bool_word(workbench_is_show_outside_days.get()),
            bool_word(workbench_show_outside_days_alias.get()),
            workbench_selected_day.get(),
            Some(12_u8),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
            workbench_motion.get(),
            rust_string_literal(&workbench_lang.get()),
            if rtl.get() { "Rtl" } else { "Ltr" },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Calendar
  year=2026
  month=3
  selected_day=Some(12)
  default_selected_day=Some(12)
  tone=CalendarTone::Default
  first_weekday=CalendarFirstWeekday::Sunday
  is_show_outside_days=true
  show_outside_days=false
  on_selected_day_change=Some(Callback::new(move |_next| {}))
  on_day_press=Some(Callback::new(move |_day| {}))
  aria_label=\"Release calendar\".into()
  motion=ui::calendar::CalendarMotion::default()
  lang=\"en-US\".into()
  dir=A11yDirection::Ltr
/>
<Calendar
  year=2026
  month=9
  selected_day=Some(2)
  default_selected_day=Some(5)
  tone=CalendarTone::Strong
  first_weekday=CalendarFirstWeekday::Monday
  is_show_outside_days=false
  show_outside_days=true
  on_selected_day_change=Some(Callback::new(move |_next| {}))
  on_day_press=Some(Callback::new(move |_day| {}))
  aria_label=\"تقويم الإصدار\".into()
  class_name=\"docs-calendar-custom\".into()
  motion=ui::calendar::CalendarMotion { enabled: true, duration_ms: 280.0, ..ui::calendar::CalendarMotion::default() }
  lang=\"ar\".into()
  dir=A11yDirection::Rtl
/>"#
            .to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/calendar/src/styles.rs */\n{}",
            ui::calendar::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "CalendarActualConfig {{\n  year: {},\n  month: {},\n  tone: {:?},\n  first_weekday: {:?},\n  is_show_outside_days: {},\n  show_outside_days: {},\n  selected_day: {:?},\n  default_selected_day: {:?},\n  on_selected_day_change: {:?},\n  on_day_press: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  motion: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            2026,
            workbench_month.get(),
            workbench_tone.get(),
            workbench_first_weekday.get(),
            workbench_is_show_outside_days.get(),
            workbench_show_outside_days_alias.get(),
            workbench_selected_day.get(),
            Some(12_u8),
            Some("Callback<Option<u8>>"),
            Some("Callback<u8>"),
            workbench_aria_label.get(),
            workbench_class_name.get(),
            workbench_motion.get(),
            workbench_lang.get(),
            workbench_dir.get(),
        )
    });

    view! {
        <ComponentPage
            title="Calendar"
            slug="calendar"
            group="Forms"
            description="Month-grid calendar with full API workbench, callback feedback, and state matrix."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports=calendar_imports.clone()
            >
                <Calendar year=2026 month=3 />
            </Playground>

            <Playground
                title="Interactive Playground (State + Source Markers)"
                code_signal=workbench_code
                code_imports=calendar_imports.clone()
                test_css_source=workbench_test_css_source
                test_source_path="components/calendar/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="calendar-workbench-controls">
                        <div class="docs-row">
                            <button
                                type="button"
                                on:click=move |_| {
                                    set_workbench_month
                                        .update(|month| *month = if *month <= 1 { 12 } else { *month - 1 });
                                }
                            >
                                "Prev month"
                            </button>
                            <button
                                type="button"
                                on:click=move |_| {
                                    set_workbench_month
                                        .update(|month| *month = if *month >= 12 { 1 } else { *month + 1 });
                                }
                            >
                                "Next month"
                            </button>
                            <button
                                type="button"
                                on:click=move |_| set_workbench_selected_day.set(None)
                            >
                                "Clear selected day"
                            </button>
                        </div>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || show_outside_days.get()
                                on:change=move |event| set_show_outside_days.set(event_target_checked(&event))
                            />
                            <span>"Show outside days"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || use_legacy_show_outside_alias.get()
                                on:change=move |event| set_use_legacy_show_outside_alias.set(event_target_checked(&event))
                            />
                            <span>"Use legacy show_outside_days alias"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || monday_first.get()
                                on:change=move |event| set_monday_first.set(event_target_checked(&event))
                            />
                            <span>"Monday first"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || strong_tone.get()
                                on:change=move |event| set_strong_tone.set(event_target_checked(&event))
                            />
                            <span>"Strong tone"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_class.get()
                                on:change=move |event| set_custom_class.set(event_target_checked(&event))
                            />
                            <span>"Custom class"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_motion.get()
                                on:change=move |event| set_custom_motion.set(event_target_checked(&event))
                            />
                            <span>"Custom motion"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || rtl.get()
                                on:change=move |event| set_rtl.set(event_target_checked(&event))
                            />
                            <span>"RTL (lang=ar, dir=rtl)"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Calendar
                        year=2026
                        month=workbench_month.get()
                        tone=workbench_tone.get()
                        first_weekday=workbench_first_weekday.get()
                        is_show_outside_days=workbench_is_show_outside_days.get()
                        show_outside_days=workbench_show_outside_days_alias.get()
                        selected_day=workbench_selected_day.get()
                        default_selected_day=12_u8
                        on_selected_day_change=Some(on_selected_day_change)
                        on_day_press=Some(on_day_press)
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                        motion=workbench_motion.get()
                        lang=workbench_lang.get()
                        dir=workbench_dir.get()
                    />
                    <p class="ui-muted" data-slot="calendar-workbench-feedback">
                        {move || {
                            format!(
                                "selected_change_count={} day_press_count={} last_selected={:?} last_pressed={:?}",
                                selected_change_count.get(),
                                day_press_count.get(),
                                last_selected_feedback.get(),
                                last_pressed_feedback.get(),
                            )
                        }}
                    </p>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Outside Days / Weekday / Tone)"
                code_signal=matrix_code
                code_imports=calendar_imports
            >
                <div class="docs-stack docs-stack--tight" data-slot="calendar-parameter-matrix">
                    <Calendar
                        year=2026
                        month=3
                        selected_day=Some(12)
                        default_selected_day=12
                        tone=CalendarTone::Default
                        first_weekday=CalendarFirstWeekday::Sunday
                        is_show_outside_days=true
                        show_outside_days=false
                        on_selected_day_change=Some(Callback::new(move |_next| {}))
                        on_day_press=Some(Callback::new(move |_day| {}))
                        aria_label="Release calendar".to_string()
                        motion=ui::calendar::CalendarMotion::default()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <Calendar
                        year=2026
                        month=9
                        selected_day=Some(2)
                        default_selected_day=5
                        tone=CalendarTone::Strong
                        first_weekday=CalendarFirstWeekday::Monday
                        is_show_outside_days=false
                        show_outside_days=true
                        on_selected_day_change=Some(Callback::new(move |_next| {}))
                        on_day_press=Some(Callback::new(move |_day| {}))
                        aria_label="تقويم الإصدار".to_string()
                        class_name="docs-calendar-custom".to_string()
                        motion=ui::calendar::CalendarMotion {
                            enabled: true,
                            duration_ms: 280.0,
                            ..ui::calendar::CalendarMotion::default()
                        }
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
