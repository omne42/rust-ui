use leptos::{html, prelude::*};
use ui_headless::{FocusRingOptions, TextFieldOptions, use_focus_ring, use_text_field};

#[component]
pub fn TextField(
    id: String,
    label: String,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] read_only: bool,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] input_type: Option<&'static str>,
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

    let base_class = "ui-text-field".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let input_type = input_type.unwrap_or("text");

    view! {
        <div
            class=class
            class:ui-text-field--focus-visible=move || focus_ring.is_focus_visible.get()
            class:ui-text-field--invalid=move || invalid.get()
            class:ui-text-field--disabled=disabled
            data-slot="text-field"
            data-focused=move || focus_ring.is_focused.get().then_some("true")
            data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
            data-invalid=move || invalid.get().then_some("true")
            data-disabled=disabled.then_some("true")
            data-read-only=read_only.then_some("true")
            data-required=move || required.get().then_some("true")
        >
            <label
                class="ui-text-field__label"
                for=aria.label.for_attr.clone()
                data-slot="text-field-label"
            >
                {label}
            </label>

            <input
                class="ui-text-field__input"
                data-slot="text-field-input"
                node_ref=node_ref
                id=aria.input.id.clone()
                type=input_type
                placeholder=placeholder
                prop:value=move || value.get()
                disabled=disabled
                readonly=read_only
                required=move || required.get()
                aria-describedby=move || aria.input.aria_describedby.get()
                aria-invalid=move || aria.input.aria_invalid.get()
                aria-required=move || aria.input.aria_required.get()
                on:input=move |ev| set_value.set(event_target_value(&ev))
                on:focus=move |_| focus_ring.handlers.on_focus.run(())
                on:blur=move |_| focus_ring.handlers.on_blur.run(())
            />

            {description.map(|description| {
                let description_id = aria.description.id.clone();
                view! {
                    <div
                        class="ui-text-field__description"
                        id=description_id
                        data-slot="text-field-description"
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
                            class="ui-text-field__error"
                            id=move || error_id.get_value()
                            data-slot="text-field-error"
                        >
                            {move || error.get_value()}
                        </div>
                    </Show>
                }
            })}
        </div>
    }
}
