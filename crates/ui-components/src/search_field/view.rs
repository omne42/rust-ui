use crate::search_field::{SearchFieldMotion, logic, motion};
use leptos::{ev, html, prelude::*};
use ui_headless::{
    FocusWithinOptions, HoverOptions, PressOptions, TextFieldOptions, use_focus_visible,
    use_focus_within, use_hover, use_press, use_text_field,
};

#[cfg(target_arch = "wasm32")]
fn focus_input(input_ref: &NodeRef<html::Input>) {
    let Some(el) = input_ref.get_untracked() else {
        return;
    };
    let _ = el.focus();
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_input(_input_ref: &NodeRef<html::Input>) {}

#[component]
pub fn SearchField(
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
    #[prop(optional)] on_submit: Option<Callback<String>>,
    #[prop(optional)] on_clear: Option<Callback<()>>,
    #[prop(optional)] motion: SearchFieldMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Input>,
) -> impl IntoView {
    let state = logic::use_search_field(value, disabled, read_only);

    let focus_within = use_focus_within(FocusWithinOptions {
        is_disabled: disabled,
    });

    let global_focus_visible = use_focus_visible()
        .map(|state| state.is_focus_visible())
        .unwrap_or_else(|| signal(false).0);

    let is_focus_visible =
        Memo::new(move |_| focus_within.is_focus_within.get() && global_focus_visible.get());

    let on_submit = StoredValue::new(on_submit);
    let on_clear = StoredValue::new(on_clear);

    let aria = use_text_field(TextFieldOptions {
        id: id.clone(),
        has_description: description.is_some(),
        has_error: error.is_some(),
        aria_describedby,
        is_invalid: invalid,
        is_required: required,
    });

    let base_class = "ui-search-field".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let clear_hover = use_hover(HoverOptions {
        is_disabled: disabled || read_only,
    });

    let clear_press = use_press(PressOptions {
        is_disabled: disabled || read_only,
        on_press: Some(Callback::new(move |_| {
            if disabled || read_only {
                return;
            }
            set_value.set(String::new());
            if let Some(on_clear) = on_clear.get_value() {
                on_clear.run(());
            }
        })),
        ..Default::default()
    });

    let is_clear_visible = Signal::derive(move || state.show_clear_button.get());

    let clear_button_ref: NodeRef<html::Button> = NodeRef::new();
    motion::attach_clear_motion(
        clear_button_ref,
        is_clear_visible,
        clear_hover.is_hovered,
        clear_press.is_pressed,
        motion,
    );

    let on_clear_pointer_down = move |ev: ev::PointerEvent| {
        ev.prevent_default();
        focus_input(&node_ref);
        clear_press.handlers.on_pointer_down.run(());
    };

    let on_input_key_down = move |ev: ev::KeyboardEvent| {
        let key = ev.key();
        if key == "Enter" {
            if disabled || read_only {
                ev.prevent_default();
                return;
            }
            if let Some(on_submit) = on_submit.get_value() {
                ev.prevent_default();
                on_submit.run(value.get_untracked());
            }
            return;
        }

        if key != "Escape" {
            return;
        }

        if value.get_untracked().is_empty() {
            return;
        }

        ev.prevent_default();
        set_value.set(String::new());
        if let Some(on_clear) = on_clear.get_value() {
            on_clear.run(());
        }
    };

    view! {
        <div
            class=class
            class:ui-search-field--focus-visible=move || is_focus_visible.get()
            class:ui-search-field--invalid=move || invalid.get()
            class:ui-search-field--disabled=disabled
            class:ui-search-field--readonly=read_only
            data-slot="search-field"
            data-empty=move || value.get().is_empty().then_some("true")
            data-readonly=read_only.then_some("true")
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
                    readonly=read_only
                    required=move || required.get()
                    aria-describedby=move || aria.input.aria_describedby.get()
                    aria-invalid=move || aria.input.aria_invalid.get()
                    aria-required=move || aria.input.aria_required.get()
                    on:input=move |ev| set_value.set(event_target_value(&ev))
                    on:keydown=on_input_key_down
                />

                <button
                    class="ui-search-field__clear"
                    data-slot="search-field-clear"
                    data-visible=move || state.show_clear_button.get().then_some("true")
                    aria-hidden=move || (!state.show_clear_button.get()).then_some("true")
                    type="button"
                    tabindex="-1"
                    aria-label=logic::CLEAR_BUTTON_ARIA_LABEL
                    node_ref=clear_button_ref
                    disabled=move || disabled || read_only || !state.show_clear_button.get()
                    on:pointerdown=on_clear_pointer_down
                    on:pointerenter=move |_| clear_hover.handlers.on_pointer_enter.run(())
                    on:pointerleave=move |_| clear_hover.handlers.on_pointer_leave.run(())
                    on:pointerup=move |_| clear_press.handlers.on_pointer_up.run(())
                    on:pointercancel=move |_| clear_press.handlers.on_pointer_cancel.run(())
                    on:click=move |_| clear_press.handlers.on_click.run(())
                    on:keydown=move |ev| {
                        let key = ev.key();
                        if clear_press.handlers.on_key_down.run(key) {
                            ev.prevent_default();
                        }
                    }
                    on:keyup=move |ev| {
                        let key = ev.key();
                        if clear_press.handlers.on_key_up.run(key) {
                            ev.prevent_default();
                        }
                    }
                    on:blur=move |_| clear_press.handlers.on_blur.run(())
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
