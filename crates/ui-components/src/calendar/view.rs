use crate::calendar::{
    CalendarStateInput,
    logic::{self, CalendarFirstWeekday, CalendarTone},
};
use leptos::prelude::*;

#[component]
pub fn Calendar(
    year: i32,
    month: u8,
    #[prop(optional)] tone: CalendarTone,
    #[prop(optional)] first_weekday: CalendarFirstWeekday,
    #[prop(optional)] show_outside_days: bool,
    #[prop(default = None)] selected_day: Option<u8>,
    #[prop(default = None)] on_day_press: Option<Callback<u8>>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let normalized_month = logic::normalize_month(month);
    let normalized_selected_day =
        logic::normalize_selected_day(selected_day, year, normalized_month);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(CalendarStateInput {
        year,
        month: normalized_month,
        tone,
        first_weekday,
        show_outside_days,
        selected_day: normalized_selected_day,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);
    let title = logic::month_title(state.year, state.month);
    let weekdays = logic::weekday_labels(state.first_weekday);
    let grid = logic::build_month_grid(
        state.year,
        state.month,
        state.first_weekday,
        state.show_outside_days,
        state.selected_day,
    );

    let weekdays = StoredValue::new(weekdays);
    let grid = StoredValue::new(grid);
    let on_day_press = StoredValue::new(on_day_press);

    view! {
        <div
            class=class
            data-slot="calendar"
            data-tone=state.tone_attr
            data-first-weekday=state.first_weekday_attr
            data-state=state.data_state_attr
            data-show-outside-days=state.show_outside_days.then_some("true")
            data-selected-day=state.selected_day.map(|day| day.to_string())
            data-year=state.year.to_string()
            data-month=state.month.to_string()
            data-aria-source=state.aria_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            role="group"
            aria-label=aria_label
        >
            <div class="ui-calendar__header" data-slot="calendar-header">
                <span class="ui-calendar__title" data-slot="calendar-title">{title}</span>
            </div>
            <div class="ui-calendar__weekdays" data-slot="calendar-weekdays">
                {weekdays
                    .get_value()
                    .into_iter()
                    .enumerate()
                    .map(|(index, label)| {
                        view! {
                            <span class="ui-calendar__weekday" data-slot="calendar-weekday" data-index=index>
                                {label}
                            </span>
                        }
                    })
                    .collect_view()}
            </div>
            <div class="ui-calendar__grid" data-slot="calendar-grid">
                {grid
                    .get_value()
                    .into_iter()
                    .enumerate()
                    .map(|(index, cell)| {
                        let month_source = if cell.in_current_month { "current" } else { "outside" };
                        if let Some(day) = cell.day {
                            let mut class = String::from("ui-calendar__day");
                            if !cell.in_current_month {
                                class.push_str(" ui-calendar__day--outside");
                            }
                            if cell.is_selected {
                                class.push_str(" ui-calendar__day--selected");
                            }

                            let is_pressable = cell.in_current_month;
                            let on_click = move |_| {
                                if !is_pressable {
                                    return;
                                }
                                if let Some(on_day_press) = on_day_press.get_value() {
                                    on_day_press.run(day);
                                }
                            };

                            view! {
                                <button
                                    type="button"
                                    class=class
                                    data-slot="calendar-day"
                                    data-index=index
                                    data-year=cell.year.to_string()
                                    data-month=cell.month.to_string()
                                    data-day=day.to_string()
                                    data-month-source=month_source
                                    data-selected=cell.is_selected.then_some("true")
                                    data-pressable=is_pressable.then_some("true")
                                    aria-selected=cell.is_selected.then_some("true")
                                    aria-label=format!("{}-{:02}-{:02}", cell.year, cell.month, day)
                                    disabled=!is_pressable
                                    on:click=on_click
                                >
                                    {day}
                                </button>
                            }
                            .into_any()
                        } else {
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
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
