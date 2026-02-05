use crate::input::{InputLabelPlacement, InputMotion, InputSize, InputVariant, logic, motion};
use leptos::{children::ViewFn, ev, html, prelude::*};
use ui_headless::{
    FocusWithinOptions, PressOptions, TextFieldOptions, use_focus_visible, use_focus_within,
    use_hover, use_press, use_text_field,
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
pub fn Input(
    id: String,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] start_content: Option<ViewFn>,
    #[prop(optional, into)] end_content: Option<ViewFn>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] read_only: bool,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] input_type: Option<&'static str>,
    #[prop(optional)] is_clearable: bool,
    #[prop(optional)] label_hidden: bool,
    #[prop(optional)] label_placement: InputLabelPlacement,
    #[prop(optional)] size: InputSize,
    #[prop(optional)] variant: InputVariant,
    #[prop(optional)] motion: InputMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Input>,
) -> impl IntoView {
    let focus_within = use_focus_within(FocusWithinOptions {
        is_disabled: disabled,
    });

    let global_focus_visible = use_focus_visible()
        .map(|state| state.is_focus_visible())
        .unwrap_or_else(|| signal(false).0);

    let is_focus_visible =
        Memo::new(move |_| focus_within.is_focus_within.get() && global_focus_visible.get());

    let is_empty = Memo::new(move |_| value.get().trim().is_empty());
    let is_focused = Memo::new(move |_| focus_within.is_focus_within.get());
    let logic_state = Memo::new(move |_| logic::InputLogicState {
        is_disabled: disabled,
        is_read_only: read_only,
        is_invalid: invalid.get(),
        is_empty: is_empty.get(),
        is_focused: is_focused.get(),
    });

    let label = StoredValue::new(label);
    let aria_label = StoredValue::new(aria_label);
    let description = StoredValue::new(description);
    let error = StoredValue::new(error);

    let start_content = start_content.map(StoredValue::new);
    let end_content = end_content.map(StoredValue::new);

    let view_state = Memo::new(move |_| {
        logic::resolve_view_state(
            label.get_value().as_deref(),
            description.get_value().as_deref(),
            error.get_value().as_deref(),
            start_content.is_some(),
            end_content.is_some(),
            is_clearable,
            logic_state.get(),
        )
    });

    let aria = use_text_field(TextFieldOptions {
        id: id.clone(),
        has_description: description.get_value().is_some(),
        has_error: error.get_value().is_some(),
        aria_describedby,
        is_invalid: invalid,
        is_required: required,
    });

    let base_class = format!(
        "ui-input {} {} {}",
        size.class_name(),
        variant.class_name(),
        label_placement.class_name()
    );
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let input_type = input_type.unwrap_or("text");
    let clear_button_ref: NodeRef<html::Button> = NodeRef::new();

    let clear_press = use_press(PressOptions {
        is_disabled: disabled || read_only,
        on_press: Some(Callback::new(move |_| set_value.set(String::new()))),
        prevent_default_for_keyboard: true,
        ..Default::default()
    });
    let clear_hover = use_hover(ui_headless::HoverOptions {
        is_disabled: disabled || read_only,
    });
    motion::attach_clear_button_motion(
        clear_button_ref,
        Signal::derive(move || view_state.get().show_clear),
        clear_hover.is_hovered,
        clear_press.is_pressed,
        disabled || read_only,
        motion,
    );

    let on_input_key_down = move |ev: ev::KeyboardEvent| {
        let key = ev.key();
        if key != "Escape" {
            return;
        }
        if disabled || read_only || !is_clearable {
            return;
        }
        if is_empty.get_untracked() {
            return;
        }

        ev.stop_propagation();
        ev.prevent_default();
        set_value.set(String::new());
    };

    view! {
        <div
            class=class
            class:ui-input--focus-visible=move || is_focus_visible.get()
            class:ui-input--invalid=move || invalid.get()
            class:ui-input--disabled=disabled
            data-slot="input"
            data-filled=move || view_state.get().is_filled.then_some("true")
            data-filled-within=move || view_state.get().is_filled_within.then_some("true")
        >
            <Show when=move || view_state.get().show_label>
                <label
                    class="ui-input__label"
                    class:ui-input__label--hidden=label_hidden
                    for=aria.label.for_attr.clone()
                    data-slot="input-label"
                >
                    {move || label.get_value().unwrap_or_default()}
                </label>
            </Show>

            <div
                class="ui-input__control"
                data-slot="input-control"
                on:focusin=move |_| focus_within.handlers.on_focus_in.run(())
                on:focusout=move |_| focus_within.handlers.on_focus_out.run(())
            >
                <Show when=move || view_state.get().show_start>
                    <span class="ui-input__start" data-slot="input-start">
                        {start_content
                            .expect("checked show_start")
                            .get_value()
                            .run()}
                    </span>
                </Show>

                <input
                    class="ui-input__input"
                    data-slot="input-input"
                    node_ref=node_ref
                    id=aria.input.id.clone()
                    type=input_type
                    placeholder=placeholder
                    prop:value=move || value.get()
                    disabled=disabled
                    readonly=read_only
                    required=move || required.get()
                    aria-label=move || aria_label.get_value()
                    aria-describedby=move || aria.input.aria_describedby.get()
                    aria-invalid=move || aria.input.aria_invalid.get()
                    aria-required=move || aria.input.aria_required.get()
                    on:input=move |ev| set_value.set(event_target_value(&ev))
                    on:keydown=on_input_key_down
                />

                <button
                    class="ui-input__clear"
                    data-slot="input-clear"
                    data-visible=move || view_state.get().show_clear.then_some("true")
                    aria-hidden=move || (!view_state.get().show_clear).then_some("true")
                    type="button"
                    tabindex="-1"
                    aria-label="Clear"
                    node_ref=clear_button_ref
                    disabled=move || disabled || read_only || !view_state.get().show_clear
                    on:pointerdown=move |ev: ev::PointerEvent| {
                        ev.prevent_default();
                        focus_input(&node_ref);
                        clear_press.handlers.on_pointer_down.run(());
                    }
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

                <Show when=move || view_state.get().show_end>
                    <span class="ui-input__end" data-slot="input-end">
                        {end_content
                            .expect("checked show_end")
                            .get_value()
                            .run()}
                    </span>
                </Show>
            </div>

            {description.get_value().map(|description| {
                let description_id = aria.description.id.clone();
                view! {
                    <div class="ui-input__description" id=description_id data-slot="input-description">
                        {description}
                    </div>
                }
            })}

            {error.get_value().map(|error| {
                let error_id = aria.error.id.clone();
                let error_id = StoredValue::new(error_id);
                let error = StoredValue::new(error);
                view! {
                    <Show when=move || invalid.get()>
                        <div class="ui-input__error" id=move || error_id.get_value() data-slot="input-error">
                            {move || error.get_value()}
                        </div>
                    </Show>
                }
            })}
        </div>
    }
}
