use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::number_field::NumberFieldStrings;
use leptos::{ev, html, prelude::*};
use ui_headless::i18n;
use ui_headless::{
    FocusRingOptions, NumberFieldOptions, TextFieldOptions, use_focus_ring, use_number_field,
    use_text_field,
};

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
    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<NumberFieldStrings>();
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

    let on_value_change = Callback::new(move |next: i64| {
        set_value.set(next);
        if let Some(on_change) = on_change.get_value() {
            on_change.run(next);
        }
    });

    let number_field = use_number_field(NumberFieldOptions {
        is_disabled: disabled,
        value: value.into(),
        on_value_change,
        min,
        max,
        step,
    });

    let on_focus = move |_| {
        focus_ring.handlers.on_focus.run(());
        number_field.handlers.on_focus.run(());
    };

    let on_blur = move |_| {
        focus_ring.handlers.on_blur.run(());
        number_field.handlers.on_blur.run(());
    };

    let on_input = move |ev| {
        number_field.handlers.on_input.run(event_target_value(&ev));
    };

    let on_key_down = move |ev: ev::KeyboardEvent| {
        if number_field.handlers.on_key_down.run(ev.key()) {
            ev.prevent_default();
        }
    };

    view! {
        <div
            class=class
            class:ui-number-field--focus-visible=move || focus_ring.is_focus_visible.get()
            class:ui-number-field--invalid=move || invalid.get()
            data-slot="number-field"
            data-focused=move || focus_ring.is_focused.get().then_some("true")
            data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
            data-invalid=move || invalid.get().then_some("true")
            data-disabled=disabled.then_some("true")
            data-required=move || required.get().then_some("true")
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
                    prop:value=move || number_field.input_value.get()
                    disabled=disabled
                    required=move || required.get()
                    role=number_field.input.role
                    aria-valuenow=move || number_field.input.aria_valuenow.get()
                    aria-valuemin=number_field.input.aria_valuemin.clone()
                    aria-valuemax=number_field.input.aria_valuemax.clone()
                    aria-disabled=number_field.input.aria_disabled
                    aria-describedby=move || aria.input.aria_describedby.get()
                    aria-invalid=move || aria.input.aria_invalid.get()
                    aria-required=move || aria.input.aria_required.get()
                    on:input=on_input
                    on:keydown=on_key_down
                    on:focus=on_focus
                    on:blur=on_blur
                />

                <div class="ui-number-field__stepper" data-slot="number-field-stepper">
                    <Button
                        is_disabled=disabled
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::IconSm
                        aria_label=strings.decrement_aria_label.as_ref().to_string()
                        on_press=number_field.handlers.decrement
                    >
                        "−"
                    </Button>
                    <Button
                        is_disabled=disabled
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::IconSm
                        aria_label=strings.increment_aria_label.as_ref().to_string()
                        on_press=number_field.handlers.increment
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
