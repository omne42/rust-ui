use crate::clear_button::ClearButton;
use crate::text_input::input::{
    InputLabelPlacement, InputMotion, InputSize, InputVariant, logic, motion,
};
use leptos::{children::ViewFn, ev, html, prelude::*};
use ui_headless::{
    A11yDirection, ClearableTextFieldOptions, FocusWithinOptions, PressOptions, TextFieldOptions,
    locale_attrs, use_clearable_text_field, use_focus_visible, use_focus_within, use_hover,
    use_press, use_text_field,
};

#[cfg(target_arch = "wasm32")]
fn focus_input(input_ref: &NodeRef<html::Input>) {
    let Some(el) = input_ref.get_untracked() else {
        return;
    };
    drop(el.focus());
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
    #[prop(optional, into)] clear_aria_label: Option<String>,
    #[prop(optional)] input_type: Option<&'static str>,
    #[prop(optional)] is_clearable: bool,
    #[prop(optional)] label_hidden: bool,
    #[prop(optional)] label_placement: InputLabelPlacement,
    #[prop(optional)] size: InputSize,
    #[prop(optional)] variant: InputVariant,
    #[prop(optional)] motion: InputMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
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
    let clear_aria_label = StoredValue::new(logic::resolve_clear_aria_label(clear_aria_label));

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

    let motion_source = if motion == InputMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != InputMotion::default()).then_some("true");

    let input_type = input_type.unwrap_or("text");
    let clear_button_ref: NodeRef<html::Button> = NodeRef::new();
    let locale = locale_attrs(lang, dir);

    let clear_press = use_press(PressOptions {
        is_disabled: disabled || read_only,
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

    let clearable = use_clearable_text_field(ClearableTextFieldOptions {
        is_disabled: disabled,
        is_read_only: read_only,
        is_clearable,
        is_empty: Signal::derive(move || is_empty.get()),
        on_clear: Some(Callback::new(move |_| set_value.set(String::new()))),
    });
    let on_clear = Callback::new(move |_: ()| set_value.set(String::new()));
    let clear_pointer_down_handler = clear_press.handlers.on_pointer_down;
    let clear_pointer_up_handler = clear_press.handlers.on_pointer_up;
    let clear_pointer_cancel_handler = clear_press.handlers.on_pointer_cancel;
    let clear_click_handler = clear_press.handlers.on_click;
    let clear_key_down_handler = clear_press.handlers.on_key_down;
    let clear_key_up_handler = clear_press.handlers.on_key_up;
    let clear_blur_handler = clear_press.handlers.on_blur;
    let clear_hover_enter_handler = clear_hover.handlers.on_pointer_enter;
    let clear_hover_leave_handler = clear_hover.handlers.on_pointer_leave;
    let input_ref_for_clear = node_ref;
    let on_clear_pointer_down = Callback::new(move |ev: ev::PointerEvent| {
        ev.prevent_default();
        focus_input(&input_ref_for_clear);
        clear_pointer_down_handler.run(());
    });

    view! {
        <div
            class=class
            lang=locale.lang.clone()
            dir=locale.dir
            class:ui-input--focus-visible=move || is_focus_visible.get()
            class:ui-input--invalid=move || invalid.get()
            class:ui-input--disabled=disabled
            data-slot="input"
            data-focused=move || is_focused.get().then_some("true")
            data-focus-visible=move || is_focus_visible.get().then_some("true")
            data-invalid=move || invalid.get().then_some("true")
            data-disabled=disabled.then_some("true")
            data-read-only=read_only.then_some("true")
            data-required=move || required.get().then_some("true")
            data-filled=move || view_state.get().is_filled.then_some("true")
            data-filled-within=move || view_state.get().is_filled_within.then_some("true")
            data-motion-source=motion_source
            data-custom-motion=custom_motion
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
                        {move || {
                            start_content
                                .map(|content| content.get_value().run())
                                .unwrap_or_else(|| ().into_any())
                        }}
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
                    aria-keyshortcuts=move || clearable.attrs.aria_keyshortcuts.get()
                    on:input=move |ev| set_value.set(event_target_value(&ev))
                    on:keydown=move |ev: ev::KeyboardEvent| {
                        if clearable.handlers.on_key_down.run(ev.key()) {
                            ev.stop_propagation();
                            ev.prevent_default();
                        }
                    }
                />

                <ClearButton
                    slot_name="input-clear"
                    class_name="ui-input__clear".to_string()
                    aria_label=clear_aria_label.get_value()
                    node_ref=clear_button_ref
                    on_press=on_clear
                    is_visible=Signal::derive(move || view_state.get().show_clear)
                    is_disabled_signal=
                        Signal::derive(move || disabled || read_only || !view_state.get().show_clear)
                    aria_hidden_when_invisible=true
                    exclude_from_tab_order=true
                    on_pointer_down=on_clear_pointer_down
                    on_pointer_up=Callback::new(move |_| clear_pointer_up_handler.run(()))
                    on_pointer_cancel=Callback::new(move |_| clear_pointer_cancel_handler.run(()))
                    on_pointer_enter=Callback::new(move |_| clear_hover_enter_handler.run(()))
                    on_pointer_leave=Callback::new(move |_| clear_hover_leave_handler.run(()))
                    on_click=Callback::new(move |_| clear_click_handler.run(()))
                    on_key_down=Callback::new(move |key: String| clear_key_down_handler.run(key))
                    on_key_up=Callback::new(move |key: String| clear_key_up_handler.run(key))
                    on_blur=Callback::new(move |_| clear_blur_handler.run(()))
                >
                    <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                        <path
                            d="M6 6l8 8M14 6l-8 8"
                            stroke="currentColor"
                            stroke_width="2"
                            stroke_linecap="round"
                        />
                    </svg>
                </ClearButton>

                <Show when=move || view_state.get().show_end>
                    <span class="ui-input__end" data-slot="input-end">
                        {move || {
                            end_content
                                .map(|content| content.get_value().run())
                                .unwrap_or_else(|| ().into_any())
                        }}
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
