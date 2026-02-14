use crate::time_field::{
    TimeFieldStateInput,
    logic::{self, TimeFieldTone},
};
use leptos::prelude::*;
use ui_headless as overlay_open;

#[component]
pub fn TimeField(
    id_base: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] tone: TimeFieldTone,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] value: Option<Signal<Option<String>>>,
    #[prop(optional)] default_value: Option<String>,
    #[prop(optional)] on_value_change: Option<Callback<Option<String>>>,
    #[prop(optional, default = 1)] minute_step: u8,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let minute_step = logic::normalize_minute_step(minute_step);

    let default_value = logic::normalize_time_value(default_value, minute_step);
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
    let hour_id = ids.hour_id;
    let minute_id = ids.minute_id;

    let parts = Memo::new(move |_| logic::resolve_time_parts(value.get(), minute_step));

    let state = Memo::new(move |_| {
        let (_, _, has_value) = parts.get();
        logic::resolve_state(TimeFieldStateInput {
            tone,
            disabled,
            has_value,
            minute_step,
            has_custom_label,
            has_custom_placeholder,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let input_placeholders = logic::resolve_input_placeholders(&placeholder.get_value());
    let hour_placeholder = StoredValue::new(input_placeholders.0);
    let minute_placeholder = StoredValue::new(input_placeholders.1);

    let hour_value = Memo::new(move |_| {
        let (hour, _, has_value) = parts.get();
        if has_value {
            format!("{hour:02}")
        } else {
            String::new()
        }
    });

    let minute_value = Memo::new(move |_| {
        let (_, minute, has_value) = parts.get();
        if has_value {
            format!("{minute:02}")
        } else {
            String::new()
        }
    });

    let on_hour_input = move |ev| {
        if disabled {
            return;
        }

        let next = logic::update_hour_from_input(
            value.get_untracked(),
            &event_target_value(&ev),
            minute_step,
        );
        request_value_change.run(next);
    };

    let on_minute_input = move |ev| {
        if disabled {
            return;
        }

        let next = logic::update_minute_from_input(
            value.get_untracked(),
            &event_target_value(&ev),
            minute_step,
        );
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
            data-slot="time-field"
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-value=move || state.get().has_value.then_some("true")
            data-minute-step=move || state.get().minute_step.to_string()
            data-label-source=move || state.get().label_source_attr
            data-placeholder-source=move || state.get().placeholder_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="group"
            aria-label=aria_label
            aria-labelledby=label_id.clone()
        >
            <label id=label_id.clone() class="ui-time-field__label" data-slot="time-field-label" for=hour_id.clone()>
                {label.get_value()}
            </label>

            <div class="ui-time-field__control" data-slot="time-field-control">
                <input
                    id=hour_id.clone()
                    class="ui-time-field__input"
                    data-slot="time-field-hour"
                    type="number"
                    min="0"
                    max="23"
                    step="1"
                    placeholder=hour_placeholder.get_value()
                    prop:value=move || hour_value.get()
                    disabled=disabled
                    aria-label="Hour"
                    on:input=on_hour_input
                />

                <span class="ui-time-field__separator" data-slot="time-field-separator" aria-hidden="true">
                    ":"
                </span>

                <input
                    id=minute_id
                    class="ui-time-field__input"
                    data-slot="time-field-minute"
                    type="number"
                    min="0"
                    max="59"
                    step=minute_step.to_string()
                    placeholder=minute_placeholder.get_value()
                    prop:value=move || minute_value.get()
                    disabled=disabled
                    aria-label="Minute"
                    on:input=on_minute_input
                />

                <Show when=move || state.get().has_value>
                    <button
                        type="button"
                        class="ui-time-field__clear"
                        data-slot="time-field-clear"
                        disabled=disabled
                        aria-label="Clear time"
                        on:click=on_clear
                    >
                        "Clear"
                    </button>
                </Show>
            </div>
        </div>
    }
}
