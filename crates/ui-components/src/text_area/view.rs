use leptos::{html, prelude::*};
use ui_headless::{FocusRingOptions, TextFieldOptions, use_focus_ring, use_text_field};

#[component]
pub fn TextArea(
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
    #[prop(optional)] rows: Option<u32>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Textarea>,
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

    let base_class = "ui-text-area".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <div
            class=class
            class:ui-text-area--focus-visible=move || focus_ring.is_focus_visible.get()
            class:ui-text-area--invalid=move || invalid.get()
            class:ui-text-area--disabled=disabled
            data-slot="text-area"
        >
            <label
                class="ui-text-area__label"
                for=aria.label.for_attr.clone()
                data-slot="text-area-label"
            >
                {label}
            </label>

            <textarea
                class="ui-text-area__textarea"
                data-slot="text-area-input"
                node_ref=node_ref
                id=aria.input.id.clone()
                rows=rows
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
            ></textarea>

            {description.map(|description| {
                let description_id = aria.description.id.clone();
                view! {
                    <div
                        class="ui-text-area__description"
                        id=description_id
                        data-slot="text-area-description"
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
                            class="ui-text-area__error"
                            id=move || error_id.get_value()
                            data-slot="text-area-error"
                        >
                            {move || error.get_value()}
                        </div>
                    </Show>
                }
            })}
        </div>
    }
}
