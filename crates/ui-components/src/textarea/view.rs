use crate::textarea::{
    TextareaStateInput,
    logic::{self},
};
use leptos::{html, prelude::*};
use ui_headless::{FocusRingOptions, TextFieldOptions, use_focus_ring, use_text_field};

#[component]
pub fn Textarea(
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

    let (label, has_custom_label) = logic::resolve_label(label);

    let description = logic::normalize_optional_text(description);
    let has_custom_description = description.is_some();

    let error = logic::normalize_optional_text(error);
    let has_custom_error = error.is_some();

    let placeholder = logic::normalize_optional_text(placeholder);
    let has_custom_placeholder = placeholder.is_some();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    let rows = rows.filter(|rows| *rows > 0);
    let has_custom_rows = rows.is_some();

    let aria = use_text_field(TextFieldOptions {
        id: id.clone(),
        has_description: description.is_some(),
        has_error: error.is_some(),
        aria_describedby,
        is_invalid: invalid,
        is_required: required,
    });

    let state = Signal::derive(move || {
        logic::resolve_state(TextareaStateInput {
            disabled,
            read_only,
            required: required.get(),
            invalid: invalid.get(),
            has_value: !value.get().is_empty(),
            has_custom_label,
            has_custom_description,
            has_custom_error,
            has_custom_placeholder,
            has_custom_rows,
            has_custom_class_name,
        })
    });

    let class = Signal::derive(move || logic::compose_class_name(class_name.clone(), state.get()));

    view! {
        <div
            class=move || class.get()
            class:ui-textarea--focus-visible=move || focus_ring.is_focus_visible.get()
            class:ui-textarea--invalid=move || invalid.get()
            class:ui-textarea--disabled=disabled
            data-slot="textarea"
            data-state=move || state.get().state_attr
            data-value=move || state.get().value_attr
            data-requirement=move || state.get().requirement_attr
            data-label-source=move || state.get().label_source_attr
            data-description-source=move || state.get().description_source_attr
            data-error-source=move || state.get().error_source_attr
            data-placeholder-source=move || state.get().placeholder_source_attr
            data-rows-source=move || state.get().rows_source_attr
            data-class-source=move || state.get().class_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-focused=move || focus_ring.is_focused.get().then_some("true")
            data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
            data-invalid=move || invalid.get().then_some("true")
            data-disabled=disabled.then_some("true")
            data-read-only=read_only.then_some("true")
            data-required=move || required.get().then_some("true")
        >
            <label
                class="ui-textarea__label"
                for=aria.label.for_attr.clone()
                data-slot="textarea-label"
            >
                {label}
            </label>

            <textarea
                class="ui-textarea__textarea"
                data-slot="textarea-input"
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
                        class="ui-textarea__description"
                        id=description_id
                        data-slot="textarea-description"
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
                            class="ui-textarea__error"
                            id=move || error_id.get_value()
                            data-slot="textarea-error"
                        >
                            {move || error.get_value()}
                        </div>
                    </Show>
                }
            })}
        </div>
    }
}
