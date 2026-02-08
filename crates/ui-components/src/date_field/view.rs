use crate::date_field::{
    DateFieldStateInput,
    logic::{self, DateFieldTone},
};
use crate::overlay_open;
use leptos::prelude::*;

#[component]
pub fn DateField(
    id_base: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] tone: DateFieldTone,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] value: Option<Signal<Option<String>>>,
    #[prop(optional)] default_value: Option<String>,
    #[prop(optional)] on_value_change: Option<Callback<Option<String>>>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let default_value = logic::normalize_date_value(default_value);
    let value_state =
        overlay_open::use_controllable_state(value, Some(default_value), on_value_change);
    let value = value_state.value;
    let request_value_change = value_state.request_change;

    let (label, has_custom_label) = logic::normalize_label(label);
    let label = StoredValue::new(label);

    let (placeholder, has_custom_placeholder) = logic::normalize_placeholder(placeholder);
    let placeholder = StoredValue::new(placeholder);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let ids = logic::resolve_ids(&id_base);
    let root_id = ids.root_id;
    let label_id = ids.label_id;
    let year_id = ids.year_id;
    let month_id = ids.month_id;
    let day_id = ids.day_id;

    let parts = Memo::new(move |_| logic::resolve_date_parts(value.get()));

    let state = Memo::new(move |_| {
        let (_, _, _, has_value) = parts.get();
        logic::resolve_state(DateFieldStateInput {
            tone,
            disabled,
            has_value,
            has_custom_label,
            has_custom_placeholder,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let input_placeholders = logic::resolve_input_placeholders(&placeholder.get_value());
    let year_placeholder = StoredValue::new(input_placeholders.0);
    let month_placeholder = StoredValue::new(input_placeholders.1);
    let day_placeholder = StoredValue::new(input_placeholders.2);

    let year_value = Memo::new(move |_| {
        let (year, _, _, has_value) = parts.get();
        if has_value {
            format!("{year:04}")
        } else {
            String::new()
        }
    });

    let month_value = Memo::new(move |_| {
        let (_, month, _, has_value) = parts.get();
        if has_value {
            format!("{month:02}")
        } else {
            String::new()
        }
    });

    let day_value = Memo::new(move |_| {
        let (_, _, day, has_value) = parts.get();
        if has_value {
            format!("{day:02}")
        } else {
            String::new()
        }
    });

    let on_year_input = move |ev| {
        if disabled {
            return;
        }

        let next = logic::update_year_from_input(value.get_untracked(), &event_target_value(&ev));
        request_value_change.run(next);
    };

    let on_month_input = move |ev| {
        if disabled {
            return;
        }

        let next = logic::update_month_from_input(value.get_untracked(), &event_target_value(&ev));
        request_value_change.run(next);
    };

    let on_day_input = move |ev| {
        if disabled {
            return;
        }

        let next = logic::update_day_from_input(value.get_untracked(), &event_target_value(&ev));
        request_value_change.run(next);
    };

    let on_clear = move |_| {
        if disabled {
            return;
        }

        request_value_change.run(None);
    };

    view! {
        <div
            id=root_id
            class=move || class.get()
            data-slot="date-field"
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-value=move || state.get().has_value.then_some("true")
            data-label-source=move || state.get().label_source_attr
            data-placeholder-source=move || state.get().placeholder_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="group"
            aria-label=aria_label
            aria-labelledby=label_id.clone()
        >
            <label id=label_id.clone() class="ui-date-field__label" data-slot="date-field-label" for=year_id.clone()>
                {label.get_value()}
            </label>

            <div class="ui-date-field__control" data-slot="date-field-control">
                <input
                    id=year_id.clone()
                    class="ui-date-field__input ui-date-field__input--year"
                    data-slot="date-field-year"
                    type="number"
                    min="1"
                    max="9999"
                    step="1"
                    placeholder=year_placeholder.get_value()
                    prop:value=move || year_value.get()
                    disabled=disabled
                    aria-label="Year"
                    on:input=on_year_input
                />

                <span class="ui-date-field__separator" data-slot="date-field-separator" aria-hidden="true">
                    "-"
                </span>

                <input
                    id=month_id.clone()
                    class="ui-date-field__input ui-date-field__input--month"
                    data-slot="date-field-month"
                    type="number"
                    min="1"
                    max="12"
                    step="1"
                    placeholder=month_placeholder.get_value()
                    prop:value=move || month_value.get()
                    disabled=disabled
                    aria-label="Month"
                    on:input=on_month_input
                />

                <span class="ui-date-field__separator" data-slot="date-field-separator" aria-hidden="true">
                    "-"
                </span>

                <input
                    id=day_id
                    class="ui-date-field__input ui-date-field__input--day"
                    data-slot="date-field-day"
                    type="number"
                    min="1"
                    max="31"
                    step="1"
                    placeholder=day_placeholder.get_value()
                    prop:value=move || day_value.get()
                    disabled=disabled
                    aria-label="Day"
                    on:input=on_day_input
                />

                <Show when=move || state.get().has_value>
                    <button
                        type="button"
                        class="ui-date-field__clear"
                        data-slot="date-field-clear"
                        disabled=disabled
                        aria-label="Clear date"
                        on:click=on_clear
                    >
                        "Clear"
                    </button>
                </Show>
            </div>
        </div>
    }
}
