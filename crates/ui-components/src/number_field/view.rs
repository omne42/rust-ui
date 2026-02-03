use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    number_field::logic::{clamp_i64, parse_i64, step_i64},
};
use leptos::{html, prelude::*};
use ui_headless::{FocusRingOptions, TextFieldOptions, use_focus_ring, use_text_field};

#[component]
pub fn NumberField(
    id: String,
    label: String,
    value: ReadSignal<i64>,
    set_value: WriteSignal<i64>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] min: Option<i64>,
    #[prop(optional)] max: Option<i64>,
    #[prop(optional, default = 1)] step: i64,
    #[prop(optional)] on_change: Option<Callback<i64>>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Input>,
) -> impl IntoView {
    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: disabled,
    });

    let aria = use_text_field(TextFieldOptions {
        id: id.clone(),
        has_description: description.is_some(),
        has_error: error.is_some(),
        aria_describedby,
        is_invalid: invalid,
        is_required: required,
    });

    let base_class = "ui-number-field".to_string();
    let base_class = if disabled {
        format!("{base_class} ui-number-field--disabled")
    } else {
        base_class
    };
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let on_change = StoredValue::new(on_change);

    let set_and_notify = move |next: i64| {
        set_value.set(next);
        if let Some(on_change) = on_change.get_value() {
            on_change.run(next);
        }
    };

    let decrement = Callback::new({
        move |_| {
            let next = step_i64(value.get_untracked(), -1, step, min, max);
            set_and_notify(next);
        }
    });

    let increment = Callback::new({
        move |_| {
            let next = step_i64(value.get_untracked(), 1, step, min, max);
            set_and_notify(next);
        }
    });

    let on_input = move |ev| {
        if disabled {
            return;
        }
        let raw = event_target_value(&ev);
        let Some(parsed) = parse_i64(&raw) else {
            return;
        };
        let next = clamp_i64(parsed, min, max);
        set_and_notify(next);
    };

    view! {
        <div
            class=class
            class:ui-number-field--focus-visible=move || focus_ring.is_focus_visible.get()
            class:ui-number-field--invalid=move || invalid.get()
            data-slot="number-field"
        >
            <label
                class="ui-number-field__label"
                for=aria.label.for_attr.clone()
                data-slot="number-field-label"
            >
                {label}
            </label>

            <div class="ui-number-field__control" data-slot="number-field-control">
                <input
                    class="ui-number-field__input"
                    data-slot="number-field-input"
                    node_ref=node_ref
                    id=aria.input.id.clone()
                    type="text"
                    inputmode="numeric"
                    pattern="-?[0-9]*"
                    placeholder=placeholder
                    prop:value=move || value.get().to_string()
                    disabled=disabled
                    required=move || required.get()
                    aria-describedby=move || aria.input.aria_describedby.get()
                    aria-invalid=move || aria.input.aria_invalid.get()
                    aria-required=move || aria.input.aria_required.get()
                    on:input=on_input
                    on:focus=move |_| focus_ring.handlers.on_focus.run(())
                    on:blur=move |_| focus_ring.handlers.on_blur.run(())
                />

                <div class="ui-number-field__stepper" data-slot="number-field-stepper">
                    <Button
                        disabled=disabled
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::IconSm
                        aria_label="Decrement"
                        on_press=decrement
                    >
                        "−"
                    </Button>
                    <Button
                        disabled=disabled
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::IconSm
                        aria_label="Increment"
                        on_press=increment
                    >
                        "+"
                    </Button>
                </div>
            </div>

            {description.map(|description| {
                let description_id = aria.description.id.clone();
                view! {
                    <div
                        class="ui-number-field__description"
                        id=description_id
                        data-slot="number-field-description"
                    >
                        {description}
                    </div>
                }
            })}

            {error.map(|error| {
                let error_id = aria.error.id.clone();
                let error_id = StoredValue::new(error_id);
                let error = StoredValue::new(error);
                view! {
                    <Show when=move || invalid.get()>
                        <div
                            class="ui-number-field__error"
                            id=move || error_id.get_value()
                            data-slot="number-field-error"
                        >
                            {move || error.get_value()}
                        </div>
                    </Show>
                }
            })}
        </div>
    }
}
