use crate::{
    logic::{self, CalendarFirstWeekday, CalendarStateInput, CalendarTone},
    motion::CalendarMotion,
};
use leptos::prelude::*;
use std::borrow::Cow;
use ui_headless::{
    A11yDirection, CalendarDayA11yInput, CalendarDayOptions, CalendarRootOptions, use_calendar_day,
    use_calendar_root,
};

#[cfg(feature = "wasm-debug")]
mod debug_trace {
    use crate::logic::CalendarSelectedDaySource;

    pub const MAX_TRACE_EVENTS: usize = 24;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CalendarDebugInteraction {
        DayPress,
        ReplayLast,
    }

    impl CalendarDebugInteraction {
        pub const fn as_attr(self) -> &'static str {
            match self {
                Self::DayPress => "day-press",
                Self::ReplayLast => "replay-last",
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct CalendarDebugEvent {
        pub trace_id: u64,
        pub tick: u64,
        pub interaction: CalendarDebugInteraction,
        pub prev_selected_day: Option<u8>,
        pub next_selected_day: Option<u8>,
        pub prev_source: CalendarSelectedDaySource,
        pub next_source: CalendarSelectedDaySource,
    }

    pub fn next_tick(current_tick: u64) -> u64 {
        current_tick.saturating_add(1)
    }

    pub fn trim_trace_buffer(events: &mut Vec<CalendarDebugEvent>) {
        if events.len() > MAX_TRACE_EVENTS {
            let overflow = events.len() - MAX_TRACE_EVENTS;
            events.drain(0..overflow);
        }
    }

    pub fn format_event_summary(event: CalendarDebugEvent) -> String {
        format!(
            "trace={} tick={} interaction={} prev_day={:?} next_day={:?} prev_source={} next_source={}",
            event.trace_id,
            event.tick,
            event.interaction.as_attr(),
            event.prev_selected_day,
            event.next_selected_day,
            event.prev_source.as_attr(),
            event.next_source.as_attr(),
        )
    }
}

fn compose_class_name(base_class_name: Option<String>, state: logic::CalendarState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-calendar"),
        Cow::Borrowed(state.tone_class),
        Cow::Borrowed(state.first_weekday_class),
    ];

    if state.show_outside_days {
        classes.push(Cow::Borrowed("ui-calendar--outside-days"));
    }
    if state.has_selected_day {
        classes.push(Cow::Borrowed("ui-calendar--has-selection"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-calendar--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .iter()
        .map(Cow::as_ref)
        .collect::<Vec<_>>()
        .join(" ")
}

fn render_header(title: String) -> impl IntoView {
    view! {
        <div class="ui-calendar__header" data-slot="calendar-header">
            <span class="ui-calendar__title" data-slot="calendar-title">{title}</span>
        </div>
    }
}

fn render_weekday(index: usize, label: String) -> impl IntoView {
    view! {
        <span class="ui-calendar__weekday" data-slot="calendar-weekday" data-index=index>
            {label}
        </span>
    }
}

fn render_weekdays(weekdays: Vec<String>) -> impl IntoView {
    view! {
        <div class="ui-calendar__weekdays" data-slot="calendar-weekdays">
            {weekdays
                .into_iter()
                .enumerate()
                .map(|(index, label)| render_weekday(index, label))
                .collect_view()}
        </div>
    }
}

fn compose_day_class(cell: logic::CalendarGridCell) -> String {
    let mut class = "ui-calendar__day".to_string();
    if !cell.in_current_month {
        class.push_str(" ui-calendar__day--outside");
    }
    if cell.is_selected {
        class.push_str(" ui-calendar__day--selected");
    }
    class
}

fn render_empty_day(index: usize) -> AnyView {
    view! {
        <span
            class="ui-calendar__day-empty"
            data-slot="calendar-day-empty"
            data-index=index
            aria-hidden="true"
        ></span>
    }
    .into_any()
}

#[cfg(feature = "wasm-debug")]
fn render_debug_event_entry(event: debug_trace::CalendarDebugEvent) -> impl IntoView {
    let summary = debug_trace::format_event_summary(event);
    let summary_for_aria = summary.clone();
    view! {
        <li
            class="ui-calendar__debug-event"
            data-slot="calendar-debug-event"
            data-trace-id=event.trace_id.to_string()
            data-tick=event.tick.to_string()
            data-interaction=event.interaction.as_attr()
            data-prev-selected-day=event.prev_selected_day.map(|day| day.to_string())
            data-next-selected-day=event.next_selected_day.map(|day| day.to_string())
            data-prev-source=event.prev_source.as_attr()
            data-next-source=event.next_source.as_attr()
            aria-label=summary_for_aria
        >
            {summary}
        </li>
    }
}

#[cfg(feature = "wasm-debug")]
fn render_debug_panel(
    debug_events: RwSignal<Vec<debug_trace::CalendarDebugEvent>>,
    on_replay_last_trace: Callback<()>,
) -> AnyView {
    let has_events = Signal::derive(move || !debug_events.get().is_empty());
    let event_count = Signal::derive(move || debug_events.get().len());

    view! {
        <section
            class="ui-calendar__debug"
            data-slot="calendar-debug"
            data-debug-feature="wasm-debug"
            data-debug-events=move || event_count.get().to_string()
        >
            <div class="ui-calendar__debug-header" data-slot="calendar-debug-header">
                <strong class="ui-calendar__debug-title">"WASM Debug Timeline"</strong>
                <button
                    type="button"
                    class="ui-calendar__debug-replay"
                    data-action="replay-last-debug-event"
                    disabled=move || !has_events.get()
                    on:click=move |_| on_replay_last_trace.run(())
                >
                    "Replay last"
                </button>
            </div>
            <ol class="ui-calendar__debug-list" data-slot="calendar-debug-events">
                {move || {
                    debug_events
                        .get()
                        .into_iter()
                        .map(render_debug_event_entry)
                        .collect_view()
                }}
            </ol>
        </section>
    }
    .into_any()
}

struct CalendarDayRenderInput {
    index: usize,
    cell: logic::CalendarGridCell,
    selected_day_mode: logic::CalendarSelectedDayMode,
    #[cfg(feature = "wasm-debug")]
    selected_day: Signal<Option<u8>>,
    set_uncontrolled_selected_day: WriteSignal<Option<u8>>,
    selected_day_source: RwSignal<logic::CalendarSelectedDaySource>,
    on_selected_day_change: StoredValue<Option<Callback<Option<u8>>>>,
    on_day_press: StoredValue<Option<Callback<u8>>>,
    #[cfg(feature = "wasm-debug")]
    debug_events: RwSignal<Vec<debug_trace::CalendarDebugEvent>>,
    #[cfg(feature = "wasm-debug")]
    debug_tick: RwSignal<u64>,
}

fn render_day(input: CalendarDayRenderInput) -> AnyView {
    let CalendarDayRenderInput {
        index,
        cell,
        selected_day_mode,
        #[cfg(feature = "wasm-debug")]
        selected_day,
        set_uncontrolled_selected_day,
        selected_day_source,
        on_selected_day_change,
        on_day_press,
        #[cfg(feature = "wasm-debug")]
        debug_events,
        #[cfg(feature = "wasm-debug")]
        debug_tick,
    } = input;

    let Some(day) = cell.day else {
        return render_empty_day(index);
    };

    let day_contract = use_calendar_day(
        CalendarDayA11yInput {
            year: cell.year,
            month: cell.month,
            day,
            in_current_month: cell.in_current_month,
            is_selected: cell.is_selected,
        },
        CalendarDayOptions {
            on_press: Some(Callback::new(move |_| {
                #[cfg(feature = "wasm-debug")]
                let previous_selected_day = selected_day.get_untracked();
                #[cfg(feature = "wasm-debug")]
                let previous_source = selected_day_source.get_untracked();
                let selection_update =
                    logic::resolve_selected_day_press_update(selected_day_mode, day);
                if let Some(next_uncontrolled_selected_day) =
                    selection_update.next_uncontrolled_selected_day
                {
                    set_uncontrolled_selected_day.set(next_uncontrolled_selected_day);
                }
                selected_day_source.set(selection_update.next_source);

                if let Some(on_selected_day_change) = on_selected_day_change.get_value() {
                    on_selected_day_change.run(Some(day));
                }

                if let Some(on_day_press) = on_day_press.get_value() {
                    on_day_press.run(day);
                }

                #[cfg(feature = "wasm-debug")]
                {
                    let trace_id = debug_trace::next_tick(debug_tick.get_untracked());
                    let prev_source_attr = previous_source.as_attr();
                    let next_source_attr = selection_update.next_source.as_attr();
                    let span = tracing::info_span!(
                        target: "ui.calendar",
                        "calendar_interaction",
                        trace_id,
                        interaction = debug_trace::CalendarDebugInteraction::DayPress.as_attr(),
                        year = cell.year,
                        month = cell.month,
                        day,
                    );
                    let _guard = span.enter();
                    tracing::info!(
                        target: "ui.calendar",
                        prev_selected_day = ?previous_selected_day,
                        next_selected_day = ?Some(day),
                        prev_source = prev_source_attr,
                        next_source = next_source_attr,
                        "calendar debug event recorded",
                    );
                    debug_tick.set(trace_id);
                    debug_events.update(|events| {
                        events.push(debug_trace::CalendarDebugEvent {
                            trace_id,
                            tick: trace_id,
                            interaction: debug_trace::CalendarDebugInteraction::DayPress,
                            prev_selected_day: previous_selected_day,
                            next_selected_day: Some(day),
                            prev_source: previous_source,
                            next_source: selection_update.next_source,
                        });
                        debug_trace::trim_trace_buffer(events);
                    });
                }
            })),
        },
    );

    let class = compose_day_class(cell);
    let press_handlers = day_contract.handlers.press.press.clone();
    let on_pointer_down = press_handlers.on_pointer_down;
    let on_pointer_up = press_handlers.on_pointer_up;
    let on_pointer_cancel = press_handlers.on_pointer_cancel;
    let on_click = press_handlers.on_click;
    let on_key_down = press_handlers.on_key_down;
    let on_key_up = press_handlers.on_key_up;
    let on_blur = press_handlers.on_blur;
    let is_pressable = day_contract.state.is_pressable;
    let month_source = day_contract.state.month_source;

    view! {
        <button
            type="button"
            class=class
            role=day_contract.attrs.role
            tabindex=day_contract.attrs.tabindex
            data-slot="calendar-day"
            data-index=index
            data-year=cell.year.to_string()
            data-month=cell.month.to_string()
            data-day=day
            data-month-source=month_source
            data-selected=cell.is_selected.then_some("true")
            data-pressable=is_pressable.then_some("true")
            aria-selected=day_contract.attrs.aria_selected
            aria-disabled=day_contract.attrs.aria_disabled
            aria-label=day_contract.attrs.aria_label
            disabled=day_contract.attrs.disabled
            on:pointerdown=move |_| on_pointer_down.run(())
            on:pointerup=move |_| on_pointer_up.run(())
            on:pointercancel=move |_| on_pointer_cancel.run(())
            on:click=move |_| on_click.run(())
            on:keydown=move |event| {
                if on_key_down.run(event.key()) {
                    event.prevent_default();
                }
            }
            on:keyup=move |event| {
                if on_key_up.run(event.key()) {
                    event.prevent_default();
                }
            }
            on:blur=move |_| on_blur.run(())
        >
            {day}
        </button>
    }
    .into_any()
}

#[component]
pub fn Calendar(
    year: i32,
    month: u8,
    #[prop(optional)] tone: CalendarTone,
    #[prop(optional)] first_weekday: CalendarFirstWeekday,
    #[prop(optional)] is_show_outside_days: Option<bool>,
    #[prop(optional)] show_outside_days: Option<bool>,
    #[prop(default = None)] selected_day: Option<u8>,
    #[prop(optional)] default_selected_day: Option<u8>,
    #[prop(default = None)] on_selected_day_change: Option<Callback<Option<u8>>>,
    #[prop(default = None)] on_day_press: Option<Callback<u8>>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] motion: CalendarMotion,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let normalized_month = logic::normalize_month(month);
    let show_outside_days =
        logic::normalize_is_show_outside_days(is_show_outside_days, show_outside_days);
    let selected_day_axis = logic::normalize_selected_day_axis(
        selected_day,
        default_selected_day,
        year,
        normalized_month,
    );

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);

    let (uncontrolled_selected_day, set_uncontrolled_selected_day) =
        signal(selected_day_axis.selected_day);
    let selected_day_source = RwSignal::new(selected_day_axis.source);
    let selected_day_mode = selected_day_axis.mode;
    let selected_day = Signal::derive(move || {
        logic::resolve_effective_selected_day(selected_day_axis, uncontrolled_selected_day.get())
    });
    #[cfg(feature = "wasm-debug")]
    let debug_events = RwSignal::new(Vec::<debug_trace::CalendarDebugEvent>::new());
    #[cfg(feature = "wasm-debug")]
    let debug_tick = RwSignal::new(0_u64);

    let on_day_press = StoredValue::new(on_day_press);
    let on_selected_day_change = StoredValue::new(on_selected_day_change);

    let state = Signal::derive(move || {
        logic::resolve_state(CalendarStateInput {
            year,
            month: normalized_month,
            tone,
            first_weekday,
            show_outside_days,
            selected_day: selected_day.get(),
            has_custom_aria_label,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let root = use_calendar_root(CalendarRootOptions {
        aria_label,
        lang: logic::normalize_optional_text(lang),
        dir,
    });
    let class = Signal::derive(move || compose_class_name(class_name.get_value(), state.get()));
    let agent_contract = Signal::derive(move || logic::resolve_agent_contract(state.get()));
    let motion = crate::motion::sanitize_motion(motion);
    let motion_source = crate::motion::source_attr(motion);
    let panel_vars = crate::motion::attach_motion(None, motion);
    let title = logic::month_title(year, normalized_month);
    let weekdays = logic::weekday_labels(first_weekday);
    let grid = Signal::derive(move || {
        logic::build_month_grid(
            state.get().year,
            state.get().month,
            state.get().first_weekday,
            state.get().show_outside_days,
            state.get().selected_day,
        )
    });

    let weekdays = StoredValue::new(weekdays);
    let grid = StoredValue::new(grid);
    #[cfg(feature = "wasm-debug")]
    let replay_last_trace = Callback::new(move |_| {
        let Some(last_event) = debug_events.get_untracked().last().copied() else {
            return;
        };
        let Some(day) = last_event.next_selected_day else {
            return;
        };

        let previous_selected_day = selected_day.get_untracked();
        let previous_source = selected_day_source.get_untracked();
        let selection_update = logic::resolve_selected_day_press_update(selected_day_mode, day);
        if let Some(next_uncontrolled_selected_day) =
            selection_update.next_uncontrolled_selected_day
        {
            set_uncontrolled_selected_day.set(next_uncontrolled_selected_day);
        }
        selected_day_source.set(selection_update.next_source);

        if let Some(on_selected_day_change) = on_selected_day_change.get_value() {
            on_selected_day_change.run(Some(day));
        }
        if let Some(on_day_press) = on_day_press.get_value() {
            on_day_press.run(day);
        }

        let trace_id = debug_trace::next_tick(debug_tick.get_untracked());
        let prev_source_attr = previous_source.as_attr();
        let next_source_attr = selection_update.next_source.as_attr();
        let span = tracing::info_span!(
            target: "ui.calendar",
            "calendar_interaction",
            trace_id,
            interaction = debug_trace::CalendarDebugInteraction::ReplayLast.as_attr(),
            year,
            month = normalized_month,
            day,
        );
        let _guard = span.enter();
        tracing::info!(
            target: "ui.calendar",
            prev_selected_day = ?previous_selected_day,
            next_selected_day = ?Some(day),
            prev_source = prev_source_attr,
            next_source = next_source_attr,
            "calendar replay event recorded",
        );
        debug_tick.set(trace_id);
        debug_events.update(|events| {
            events.push(debug_trace::CalendarDebugEvent {
                trace_id,
                tick: trace_id,
                interaction: debug_trace::CalendarDebugInteraction::ReplayLast,
                prev_selected_day: previous_selected_day,
                next_selected_day: Some(day),
                prev_source: previous_source,
                next_source: selection_update.next_source,
            });
            debug_trace::trim_trace_buffer(events);
        });
    });
    #[cfg(feature = "wasm-debug")]
    let debug_panel = render_debug_panel(debug_events, replay_last_trace);
    #[cfg(not(feature = "wasm-debug"))]
    let debug_panel = ().into_any();

    view! {
        <div
            class=move || class.get()
            style=panel_vars
            data-slot="calendar"
            data-tone=move || state.get().tone_attr
            data-first-weekday=move || state.get().first_weekday_attr
            data-state=move || state.get().data_state_attr
            data-show-outside-days=move || state.get().show_outside_days.then_some("true")
            data-selected-day=move || state.get().selected_day.map(|day| day.to_string())
            data-selected-day-mode=selected_day_mode.as_attr()
            data-selected-day-source=move || selected_day_source.get().as_attr()
            data-year=move || state.get().year.to_string()
            data-month=move || state.get().month.to_string()
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=motion_source
            data-custom-motion=(motion_source == "custom").then_some("true")
            data-ui-schema=move || agent_contract.get().schema_attr
            data-ui-intent=move || agent_contract.get().intent_attr
            data-ui-action=move || agent_contract.get().action.as_attr()
            data-ui-state=move || agent_contract.get().state.as_attr()
            data-ui-source=move || agent_contract.get().source.as_attr()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_attr()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_attr()
            data-ui-stream-mode="snapshot"
            data-ui-output-status=move || agent_contract.get().output_status.as_attr()
            role=root.attrs.role
            aria-label=root.attrs.aria_label.clone()
            lang=root.attrs.lang.clone()
            dir=root.attrs.dir
        >
            {render_header(title)}
            {render_weekdays(
                weekdays
                    .get_value()
                    .iter()
                    .map(|label| (*label).to_string())
                    .collect(),
            )}
            <div class="ui-calendar__grid" data-slot="calendar-grid">
                {move || {
                    grid.get_value()
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(index, cell)| {
                            if cell.day.is_none() {
                                return render_empty_day(index);
                            }

                            render_day(CalendarDayRenderInput {
                                index,
                                cell,
                                selected_day_mode,
                                #[cfg(feature = "wasm-debug")]
                                selected_day,
                                set_uncontrolled_selected_day,
                                selected_day_source,
                                on_selected_day_change,
                                on_day_press,
                                #[cfg(feature = "wasm-debug")]
                                debug_events,
                                #[cfg(feature = "wasm-debug")]
                                debug_tick,
                            })
                        })
                        .collect_view()
                }}
            </div>
            {debug_panel}
        </div>
    }
}
