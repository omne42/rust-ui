use crate::search_field::logic::{CLEAR_BUTTON_ARIA_LABEL, use_search_field};
use leptos::{ev, html, prelude::*};
use ui_headless::{
    FocusWithinOptions, TextFieldOptions, use_focus_visible, use_focus_within, use_text_field,
};

#[component]
pub fn SearchField(
    id: String,
    label: String,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Input>,
) -> impl IntoView {
    let state = use_search_field(value, disabled);

    let focus_within = use_focus_within(FocusWithinOptions {
        is_disabled: disabled,
    });

    let global_focus_visible = use_focus_visible()
        .map(|state| state.is_focus_visible())
        .unwrap_or_else(|| signal(false).0);

    let is_focus_visible =
        Memo::new(move |_| focus_within.is_focus_within.get() && global_focus_visible.get());

    let aria = use_text_field(TextFieldOptions {
        id: id.clone(),
        has_description: description.is_some(),
        has_error: error.is_some(),
        aria_describedby: Signal::derive(|| None),
        is_invalid: invalid,
        is_required: required,
    });

    let base_class = "ui-search-field".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let on_clear_pointer_down = move |ev: ev::PointerEvent| {
        // Keep focus in the input when clicking clear.
        ev.prevent_default();
    };

    view! {
        <div
            class=class
            class:ui-search-field--focus-visible=move || is_focus_visible.get()
            class:ui-search-field--invalid=move || invalid.get()
            class:ui-search-field--disabled=disabled
            data-slot="search-field"
        >
            <label
                class="ui-search-field__label"
                for=aria.label.for_attr.clone()
                data-slot="search-field-label"
            >
                {label}
            </label>

            <div
                class="ui-search-field__control"
                data-slot="search-field-control"
                on:focusin=move |_| focus_within.handlers.on_focus_in.run(())
                on:focusout=move |_| focus_within.handlers.on_focus_out.run(())
            >
                <span
                    class="ui-search-field__icon"
                    data-slot="search-field-icon"
                    aria-hidden="true"
                >
                    <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                        <circle cx="9" cy="9" r="6" stroke="currentColor" stroke_width="2" />
                        <path
                            d="M13.5 13.5l3 3"
                            stroke="currentColor"
                            stroke_width="2"
                            stroke_linecap="round"
                        />
                    </svg>
                </span>

                <input
                    class="ui-search-field__input"
                    data-slot="search-field-input"
                    node_ref=node_ref
                    id=aria.input.id.clone()
                    type="search"
                    placeholder=placeholder
                    prop:value=move || value.get()
                    disabled=disabled
                    required=move || required.get()
                    aria-describedby=move || aria.input.aria_describedby.get()
                    aria-invalid=move || aria.input.aria_invalid.get()
                    aria-required=move || aria.input.aria_required.get()
                    on:input=move |ev| set_value.set(event_target_value(&ev))
                />

                <Show when=move || state.show_clear_button.get()>
                    <button
                        class="ui-search-field__clear"
                        data-slot="search-field-clear"
                        type="button"
                        aria-label=CLEAR_BUTTON_ARIA_LABEL
                        on:pointerdown=on_clear_pointer_down
                        on:click=move |_| set_value.set(String::new())
                    >
                        <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                            <path
                                d="M6 6l8 8M14 6l-8 8"
                                stroke="currentColor"
                                stroke_width="2"
                                stroke_linecap="round"
                            />
                        </svg>
                    </button>
                </Show>
            </div>

            {description.map(|description| {
                let description_id = aria.description.id.clone();
                view! {
                    <div
                        class="ui-search-field__description"
                        id=description_id
                        data-slot="search-field-description"
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
                            class="ui-search-field__error"
                            id=move || error_id.get_value()
                            data-slot="search-field-error"
                        >
                            {move || error.get_value()}
                        </div>
                    </Show>
                }
            })}
        </div>
    }
}
