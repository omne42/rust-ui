use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::text_input::number_field::{NumberFieldStrings, logic};
use leptos::{ev, html, prelude::*};
use ui_headless::i18n;
use ui_headless::{
    FocusRingOptions, NumberFieldOptions, TextFieldOptions, use_controllable_state, use_focus_ring,
    use_number_field, use_text_field,
};

#[component]
pub fn NumberField(
    id: String,
    label: String,
    #[prop(optional, into)] value: Option<Signal<i64>>,
    #[prop(optional)] default_value: Option<i64>,
    #[prop(optional)] on_value_change: Option<Callback<i64>>,
    #[prop(optional)] set_value: Option<WriteSignal<i64>>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] min: Option<i64>,
    #[prop(optional)] max: Option<i64>,
    #[prop(optional, default = 1)] step: i64,
    #[prop(optional)] on_change: Option<Callback<i64>>,
    #[prop(optional, into)] is_required: Option<Signal<bool>>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] is_invalid: Option<Signal<bool>>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Input>,
) -> impl IntoView {
    let on_value_change = on_value_change
        .or(on_change)
        .or_else(|| set_value.map(|setter| Callback::new(move |next: i64| setter.set(next))));
    let controlled_default_value = logic::normalize_default_value(default_value);
    let value_state =
        use_controllable_state(value, Some(controlled_default_value), on_value_change);
    let value = value_state.value;
    let request_value_change = value_state.request_change;

    let accessibility = logic::normalize_accessibility_state(logic::AccessibilityStateInput {
        is_disabled,
        disabled,
        is_required,
        required,
        is_invalid,
        invalid,
    });
    let is_disabled = accessibility.is_disabled;
    let is_required = accessibility.is_required;
    let is_invalid = accessibility.is_invalid;

    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<NumberFieldStrings>();
    let decrement_aria_label: String = strings.decrement_aria_label.as_ref().into();
    let increment_aria_label: String = strings.increment_aria_label.as_ref().into();
    let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });

    let aria = use_text_field(TextFieldOptions {
        id: id.clone(),
        has_description: description.is_some(),
        has_error: error.is_some(),
        aria_describedby,
        is_invalid,
        is_required,
    });

    let base_class = "ui-number-field".to_string();
    let base_class = if is_disabled {
        format!("{base_class} ui-number-field--disabled")
    } else {
        base_class
    };
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let number_field = use_number_field(NumberFieldOptions {
        is_disabled,
        value,
        on_value_change: request_value_change,
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
            class:ui-number-field--invalid=move || is_invalid.get()
            data-slot="number-field"
            data-focused=move || focus_ring.is_focused.get().then_some("true")
            data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
            data-invalid=move || is_invalid.get().then_some("true")
            data-disabled=is_disabled.then_some("true")
            data-required=move || is_required.get().then_some("true")
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
                    disabled=is_disabled
                    required=move || is_required.get()
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
                        is_disabled=is_disabled
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::IconSm
                        aria_label=decrement_aria_label.clone()
                        on_press=number_field.handlers.decrement
                    >
                        "−"
                    </Button>
                    <Button
                        is_disabled=is_disabled
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::IconSm
                        aria_label=increment_aria_label.clone()
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
                    <Show when=move || is_invalid.get()>
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
