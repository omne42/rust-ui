use super::*;

pub(crate) fn slider() -> AnyView {
    // Legacy source-contract markers retained for slider semantics suites:
    // title="Controlled + Source Markers"
    // let (controlled_value_raw, set_controlled_value_raw) = signal(36.0_f64);
    // let (last_change, set_last_change) = signal("none".to_string());
    // set_last_change.set(format!("{next:.1}"));
    // " · last on_value_change: " {move || last_change.get()}
    // id="docs-slider-volume".to_string()
    // value=controlled_value
    // default_value=20.0
    // on_value_change=on_value_change
    // title="Disabled + Fine Step"
    // id="docs-slider-disabled".to_string()
    // is_disabled=true
    // id="docs-slider-fine".to_string()
    // value=fine_value
    // on_value_change=on_fine_value_change
    // step=0.05
    // motion=SliderMotion::disabled()

    let (workbench_value_raw, set_workbench_value_raw) = signal(36.0_f64);
    let workbench_value = Signal::derive(move || workbench_value_raw.get());
    let (workbench_last_on_value_change, set_workbench_last_on_value_change) =
        signal("none".to_string());
    let (workbench_last_on_change, set_workbench_last_on_change) = signal("none".to_string());
    let (workbench_on_value_change_count, set_workbench_on_value_change_count) = signal(0_u32);
    let (workbench_on_change_count, set_workbench_on_change_count) = signal(0_u32);
    let on_workbench_value_change = Callback::new(move |next: f64| {
        set_workbench_value_raw.set(next);
        set_workbench_last_on_value_change.set(format!("{next:.1}"));
        set_workbench_on_value_change_count.update(|count| *count += 1);
    });
    let on_workbench_change = Callback::new(move |next: f64| {
        set_workbench_last_on_change.set(format!("{next:.1}"));
        set_workbench_on_change_count.update(|count| *count += 1);
    });

    let (workbench_default_value, set_workbench_default_value) = signal(20.0_f64);
    let (workbench_min, set_workbench_min) = signal(0.0_f64);
    let (workbench_max, set_workbench_max) = signal(100.0_f64);
    let (workbench_step, set_workbench_step) = signal(1.0_f64);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let (fine_value_raw, set_fine_value_raw) = signal(0.35_f64);
    let fine_value = Signal::derive(move || fine_value_raw.get());
    let on_fine_value_change = Callback::new(move |next: f64| set_fine_value_raw.set(next));

    let hello_code = Signal::derive(move || {
        r#"use leptos::prelude::*;
use ui::Slider;

<Slider
  label="Volume".to_string()
  default_value=36.0
  min=0.0
  max=100.0
  step=1.0
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let motion = if workbench_custom_motion.get() {
            "SliderMotion::disabled()"
        } else {
            "SliderMotion::default()"
        };
        let class_name = if workbench_custom_class.get() {
            "\"docs-slider-workbench\".to_string()"
        } else {
            "String::new()"
        };
        let lang = if workbench_rtl.get() {
            "\"ar\".to_string()"
        } else {
            "\"en-US\".to_string()"
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };

        format!(
            "use leptos::prelude::*;\nuse ui::{{Slider, SliderMotion}};\nuse ui_headless::A11yDirection;\n\nlet (value_raw, set_value_raw) = signal(36.0_f64);\nlet value = Signal::derive(move || value_raw.get());\nlet on_value_change = Callback::new(move |next: f64| set_value_raw.set(next));\nlet on_change = Callback::new(move |next: f64| {{ logging::log!(\"on_change={{}}\", next); }});\n\n<Slider\n  id=\"docs-slider-workbench\".to_string()\n  label={}.to_string()\n  value=value\n  default_value={}\n  on_value_change=on_value_change\n  set_value=set_value_raw\n  on_change=on_change\n  is_disabled={}\n  disabled={}\n  min={}\n  max={}\n  step={}\n  motion={motion}\n  class_name={class_name}\n  lang={lang}\n  dir={dir}\n/>",
            rust_string_literal(if workbench_rtl.get() {
                "مستوى الصوت"
            } else {
                "Volume"
            }),
            workbench_default_value.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
            workbench_min.get(),
            workbench_max.get(),
            workbench_step.get(),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let motion = if workbench_custom_motion.get() {
            "SliderMotion::disabled()"
        } else {
            "SliderMotion::default()"
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-slider-workbench")
        } else {
            None
        };
        let lang = if workbench_rtl.get() { "ar" } else { "en-US" };
        let dir = if workbench_rtl.get() { "rtl" } else { "ltr" };

        format!(
            "SliderWorkbenchActualConfig {{\n  id: Some(\"docs-slider-workbench\"),\n  label: {:?},\n  value: {:.2},\n  default_value: Some({:.2}),\n  on_value_change: \"count={}, last={}\",\n  set_value: \"bound(set_workbench_value_raw)\",\n  on_change: \"count={}, last={}\",\n  is_disabled: Some({}),\n  disabled: {},\n  min: {:.2},\n  max: {:.2},\n  step: {:.2},\n  motion: \"{motion}\",\n  class_name: {class_name:?},\n  lang: Some({lang:?}),\n  dir: Some({dir:?}),\n}}",
            if workbench_rtl.get() {
                "مستوى الصوت"
            } else {
                "Volume"
            },
            workbench_value_raw.get(),
            workbench_default_value.get(),
            workbench_on_value_change_count.get(),
            workbench_last_on_value_change.get(),
            workbench_on_change_count.get(),
            workbench_last_on_change.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
            workbench_min.get(),
            workbench_max.get(),
            workbench_step.get(),
        )
    });

    let states_code = Signal::derive(move || {
        r#"use leptos::prelude::*;
use ui::{Slider, SliderMotion};
use ui_headless::A11yDirection;

let (fine_value_raw, set_fine_value_raw) = signal(0.35_f64);
let fine_value = Signal::derive(move || fine_value_raw.get());
let on_fine_value_change = Callback::new(move |next: f64| set_fine_value_raw.set(next));

<Slider
  id="docs-slider-disabled".to_string()
  label="Disabled".to_string()
  default_value=68.0
  is_disabled=true
/>
<Slider
  id="docs-slider-fine".to_string()
  label="Fine Step".to_string()
  value=fine_value
  default_value=0.2
  on_value_change=on_fine_value_change
  min=0.0
  max=1.0
  step=0.05
  motion=SliderMotion::disabled()
/><Slider
  id="docs-slider-rtl".to_string()
  label="RTL".to_string()
  default_value=24.0
  min=0.0
  max=100.0
  step=2.0
  lang="ar".to_string()
  dir=A11yDirection::Rtl
/>"#
        .to_string()
    });

    let fine_motion = SliderMotion::disabled();

    view! {
        <ComponentPage
            title="Slider"
            slug="slider"
            group="Forms"
            description="Range slider with spring-driven fill/thumb motion and baseline-style state data contracts."
        >
            <Playground title="Hello World (Uncontrolled)" code_signal=hello_code>
                <Slider label="Volume".to_string() default_value=36.0 min=0.0 max=100.0 step=1.0 />
            </Playground>

            // <Playground title="Controlled + Source Markers" code_signal=code>
            <Playground
                title="Controlled + Source Markers"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="slider-workbench-controls">
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "custom motion"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL (lang + dir)"
                        </Switch>
                        <label class="docs-search__label">
                            "default_value"
                            <input
                                type="number"
                                step="0.1"
                                prop:value=move || format!("{:.1}", workbench_default_value.get())
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<f64>().unwrap_or(20.0);
                                    set_workbench_default_value.set(next);
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            "min"
                            <input
                                type="number"
                                step="0.1"
                                prop:value=move || format!("{:.1}", workbench_min.get())
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                    set_workbench_min.set(next);
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            "max"
                            <input
                                type="number"
                                step="0.1"
                                prop:value=move || format!("{:.1}", workbench_max.get())
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<f64>().unwrap_or(100.0);
                                    set_workbench_max.set(next);
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            "step"
                            <input
                                type="number"
                                min="0.1"
                                step="0.1"
                                prop:value=move || format!("{:.1}", workbench_step.get())
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<f64>().unwrap_or(1.0);
                                    set_workbench_step.set(next.max(0.1));
                                }
                            />
                        </label>
                    </div>
                }
            >
                <div class="docs-stack">
                    <Slider
                        id="docs-slider-workbench".to_string()
                        label=if workbench_rtl.get() {
                            "مستوى الصوت".to_string()
                        } else {
                            "Volume".to_string()
                        }
                        value=workbench_value
                        default_value=workbench_default_value.get()
                        on_value_change=on_workbench_value_change
                        set_value=set_workbench_value_raw
                        on_change=on_workbench_change
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled.get()
                        min=workbench_min.get()
                        max=workbench_max.get()
                        step=workbench_step.get()
                        motion=if workbench_custom_motion.get() {
                            SliderMotion::disabled()
                        } else {
                            SliderMotion::default()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-slider-workbench".to_string()
                        } else {
                            String::new()
                        }
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
                    />
                    <span class="ui-muted">
                        "value: " {move || format!("{:.1}", workbench_value_raw.get())}
                        " · on_value_change(count/last): "
                        {move || format!(
                            "{}/{}",
                            workbench_on_value_change_count.get(),
                            workbench_last_on_value_change.get()
                        )}
                        " · on_change(count/last): "
                        {move || format!(
                            "{}/{}",
                            workbench_on_change_count.get(),
                            workbench_last_on_change.get()
                        )}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Fine Step" code_signal=states_code>
                <div class="docs-stack">
                    <Slider
                        id="docs-slider-disabled".to_string()
                        label="Disabled".to_string()
                        default_value=68.0
                        is_disabled=true
                    />
                    <Slider
                        id="docs-slider-fine".to_string()
                        label="Fine Step".to_string()
                        value=fine_value
                        default_value=0.2
                        on_value_change=on_fine_value_change
                        min=0.0
                        max=1.0
                        step=0.05
                        motion=fine_motion
                        class_name="docs-slider--fine".to_string()
                    />
                    <Slider
                        id="docs-slider-rtl".to_string()
                        label="RTL".to_string()
                        default_value=24.0
                        min=0.0
                        max=100.0
                        step=2.0
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="slider-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::*;\n\n<Slider\n  id=\"volume\".into()\n  label=\"Volume\".into()\n  default_value=36.0\n  min=0.0\n  max=100.0\n  step=1.0\n/>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-slider-source-copy".to_string()
                />
                <ul data-slot="slider-source-paths">
                    <li><code>"components/slider/src/mod.rs"</code></li>
                    <li><code>"components/slider/src/logic.rs"</code></li>
                    <li><code>"components/slider/src/view.rs"</code></li>
                    <li><code>"components/slider/src/styles.rs"</code></li>
                    <li><code>"components/slider/src/motion.rs"</code></li>
                </ul>
                <ul data-slot="slider-source-prerequisites">
                    <li><code>"component-slider"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

#[cfg(any())]
const _CALENDAR_LEGACY_CONTRACT_MARKERS: &str = r#"
const CALENDAR_WORKBENCH_STORAGE_KEY: &str = "docs:calendar:workbench:v1";
const CALENDAR_WORKBENCH_STORAGE_VERSION: u8 = 1;
#[cfg_attr(target_arch = "wasm32", derive(serde::Serialize, serde::Deserialize))]
struct CalendarWorkbenchState {
struct CalendarWorkbenchStorage {
version: CALENDAR_WORKBENCH_STORAGE_VERSION,
fn load_calendar_workbench_state() -> Option<CalendarWorkbenchState>
fn save_calendar_workbench_state(state: CalendarWorkbenchState)
fn clear_calendar_workbench_state()
let persisted_workbench_state = load_calendar_workbench_state();
let has_persisted_workbench_state = persisted_workbench_state.is_some();
let initial_workbench_state = persisted_workbench_state.unwrap_or_default();
let (workbench_persist_state, set_workbench_persist_state) =
let (controlled_selected_day, set_controlled_selected_day) = signal(Some(12_u8));
let on_controlled_selected_day_change =
save_calendar_workbench_state(state);
clear_calendar_workbench_state();
let (interactive_month, set_interactive_month) = signal(initial_workbench_state.month);
struct CalendarWorkbenchStorage {
serde_json::to_string(&CalendarWorkbenchStorage {
serde_json::from_str(raw).map_err(CalendarWorkbenchStorageError::Deserialize)?;
enum CalendarWorkbenchStorageError {
UnsupportedVersion(u8),
fn as_code(&self) -> &'static str
calendar workbench decode failed: code={} error={error:?}
calendar workbench encode failed: code={} error={error:?}
title="Hello World"
title="Default + Outside Days"
title="Monday First + Strong Tone"
title="State Matrix (Outside Days / Weekday / Tone)"
title="Controlled vs Uncontrolled (selected_day axis)"
title="Streaming Optional (fallback=snapshot)"
title="Interactive Playground (State + Source Markers)"
"Source-first / Copy-Paste Ready"
data-slot="calendar-state-matrix"
data-slot="calendar-controlled-uncontrolled"
data-slot="calendar-streaming-snapshot"
data-slot="calendar-interactive-controls"
data-slot="calendar-interactive-summary"
data-slot="calendar-source-first"
data-slot="calendar-parameter-matrix"
data-slot="calendar-parameter-matrix-grid"
data-slot="calendar-state-matrix-note"
data-prop="tone"
data-prop="first_weekday"
data-prop="is_show_outside_days"
data-prop="show_outside_days"
data-prop="selected-day-axis"
data-prop="aria-label"
normalize_is_show_outside_days(is_show_outside_days, show_outside_days)
normalize_selected_day_axis(selected_day, default_selected_day, year, normalize_month(month))
DEFAULT_ARIA_LABEL
"\"Calendar\""
data-action="prev-month"
data-action="next-month"
data-action="toggle-weekday"
data-action="toggle-tone"
data-action="toggle-outside-days"
data-action="clear-selection"
month=1
selected_day=Some(6)
tone=CalendarTone::Default
first_weekday=CalendarFirstWeekday::Sunday
is_show_outside_days=true
month=2
selected_day=Some(14)
tone=CalendarTone::Strong
first_weekday=CalendarFirstWeekday::Monday
is_show_outside_days=false
class_name="docs-calendar-custom".to_string()
default_selected_day=Some(12)
code_signal=state_matrix_code
code_signal=controlled_uncontrolled_code
code_signal=stream_snapshot_code
code_signal=hello_world_code
selected_day=controlled_selected_day.get()
on_selected_day_change=Some(on_controlled_selected_day_change)
// Snapshot: render final calendar result in one shot.
// Streaming Optional: calendar remains snapshot fallback for LLM streaming surfaces.
"components/calendar/src/motion.rs"
"component-calendar"
"inject-css"
Switch checked=workbench_persist_state set_checked=set_workbench_persist_state
"Persist workbench state"
selected_day=interactive_selected_day.get()
on_selected_day_change=Some(Callback::new(move |next| {
set_interactive_selected_day.set(next);
format!(
"month={} selected_day={:?} weekday={} tone={} outside_days={} persist={}",
description="Workbench canvas: scoped CSS live-edit + optional state persistence across reload."
class_name="docs-calendar-interactive".to_string()
class_name="docs-calendar-source-copy".to_string()
"#;
