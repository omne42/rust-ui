use crate::calendar::CalendarFirstWeekday;
use crate::text_input::date_picker::{DatePicker, DatePickerTone};
use crate::text_input::date_range_picker::logic::{
    self, DateRangePickerStateInput, DateRangePickerTextInput, DateRangePickerTone,
};
use leptos::prelude::*;
use ui_headless as overlay_open;

#[component]
pub fn DateRangePicker(
    id_base: String,
    start_year: i32,
    start_month: u8,
    end_year: i32,
    end_month: u8,
    #[prop(optional)] tone: DateRangePickerTone,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] start_day: Option<Signal<Option<u8>>>,
    #[prop(optional)] default_start_day: Option<u8>,
    #[prop(optional)] on_start_day_change: Option<Callback<Option<u8>>>,
    #[prop(optional, into)] end_day: Option<Signal<Option<u8>>>,
    #[prop(optional)] default_end_day: Option<u8>,
    #[prop(optional)] on_end_day_change: Option<Callback<Option<u8>>>,
    #[prop(optional)] first_weekday: CalendarFirstWeekday,
    #[prop(optional)] is_show_outside_days: Option<bool>,
    #[prop(optional)] show_outside_days: bool,
    #[prop(optional, into)] start_label: Option<String>,
    #[prop(optional, into)] end_label: Option<String>,
    #[prop(optional, into)] start_placeholder: Option<String>,
    #[prop(optional, into)] end_placeholder: Option<String>,
    #[prop(optional, into)] start_aria_label: Option<String>,
    #[prop(optional, into)] end_aria_label: Option<String>,
    #[prop(optional, into)] invalid_range_message: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let is_disabled = is_disabled.unwrap_or(disabled);
    let is_show_outside_days = is_show_outside_days.unwrap_or(show_outside_days);

    let start_month = logic::normalize_month(start_month);
    let end_month = logic::normalize_month(end_month);

    let default_start_day = logic::normalize_day(default_start_day, start_year, start_month);
    let start_state = overlay_open::use_controllable_state(
        start_day,
        Some(default_start_day),
        on_start_day_change,
    );
    let start_day = start_state.value;
    let request_start_day_change = start_state.request_change;

    let default_end_day = logic::normalize_day(default_end_day, end_year, end_month);
    let end_state =
        overlay_open::use_controllable_state(end_day, Some(default_end_day), on_end_day_change);
    let end_day = end_state.value;
    let request_end_day_change = end_state.request_change;

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let text_state = logic::resolve_text_state(DateRangePickerTextInput {
        start_label,
        end_label,
        start_placeholder,
        end_placeholder,
        start_aria_label,
        end_aria_label,
        invalid_range_message,
    });
    let text_state = StoredValue::new(text_state);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let selected_start_day =
        Memo::new(move |_| logic::normalize_day(start_day.get(), start_year, start_month));

    let selected_end_day =
        Memo::new(move |_| logic::normalize_day(end_day.get(), end_year, end_month));

    let is_invalid_range = Memo::new(move |_| {
        logic::is_range_invalid(
            selected_start_day
                .get()
                .map(|day| (start_year, start_month, day)),
            selected_end_day.get().map(|day| (end_year, end_month, day)),
        )
    });

    let state = Memo::new(move |_| {
        logic::resolve_state(DateRangePickerStateInput {
            tone,
            disabled: is_disabled,
            has_start_value: selected_start_day.get().is_some(),
            has_end_value: selected_end_day.get().is_some(),
            is_invalid_range: is_invalid_range.get(),
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let picker_tone = match tone {
        DateRangePickerTone::Default => DatePickerTone::Default,
        DateRangePickerTone::Quiet => DatePickerTone::Quiet,
        DateRangePickerTone::Strong => DatePickerTone::Strong,
    };

    let on_start_change: Callback<Option<u8>> = Callback::new(move |day| {
        let normalized = logic::normalize_day(day, start_year, start_month);
        request_start_day_change.run(normalized);
    });

    let on_end_change: Callback<Option<u8>> = Callback::new(move |day| {
        let normalized = logic::normalize_day(day, end_year, end_month);
        request_end_day_change.run(normalized);
    });

    let start_id_base = format!("{id_base}-start");
    let end_id_base = format!("{id_base}-end");

    view! {
        <div
            id=id_base
            class=move || class.get()
            data-slot="date-range-picker"
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-start-value=move || state.get().has_start_value.then_some("true")
            data-has-end-value=move || state.get().has_end_value.then_some("true")
            data-has-full-value=move || state.get().has_full_value.then_some("true")
            data-partial=move || state.get().is_partial.then_some("true")
            data-invalid-range=move || state.get().is_invalid_range.then_some("true")
            data-start-year=start_year
            data-start-month=start_month
            data-end-year=end_year
            data-end-month=end_month
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="group"
            aria-label=aria_label
        >
            <div class="ui-date-range-picker__fields" data-slot="date-range-picker-fields">
                <div class="ui-date-range-picker__field" data-slot="date-range-picker-start">
                    <div class="ui-date-range-picker__field-label" data-slot="date-range-picker-start-label">
                        {text_state.get_value().start_label}
                    </div>
                    <DatePicker
                        id_base=start_id_base
                        year=start_year
                        month=start_month
                        tone=picker_tone
                        is_disabled=is_disabled
                        selected_day=start_day
                        on_selected_day_change=on_start_change
                        first_weekday=first_weekday
                        is_show_outside_days=is_show_outside_days
                        placeholder=text_state.get_value().start_placeholder
                        aria_label=text_state.get_value().start_aria_label
                        class_name="ui-date-range-picker__picker".to_string()
                    />
                </div>

                <div class="ui-date-range-picker__field" data-slot="date-range-picker-end">
                    <div class="ui-date-range-picker__field-label" data-slot="date-range-picker-end-label">
                        {text_state.get_value().end_label}
                    </div>
                    <DatePicker
                        id_base=end_id_base
                        year=end_year
                        month=end_month
                        tone=picker_tone
                        is_disabled=is_disabled
                        selected_day=end_day
                        on_selected_day_change=on_end_change
                        first_weekday=first_weekday
                        is_show_outside_days=is_show_outside_days
                        placeholder=text_state.get_value().end_placeholder
                        aria_label=text_state.get_value().end_aria_label
                        class_name="ui-date-range-picker__picker".to_string()
                    />
                </div>
            </div>

            <Show when=move || state.get().is_invalid_range>
                <div class="ui-date-range-picker__hint" data-slot="date-range-picker-hint">
                    {text_state.get_value().invalid_range_message}
                </div>
            </Show>
        </div>
    }
}
