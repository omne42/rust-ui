use crate::text_input::input_otp::logic;
use leptos::{ev, html, prelude::*};
use ui_headless::{
    A11yDirection, FocusRingOptions, InputOtpOptions, TextFieldOptions, i18n,
    input_otp_focus_control, input_otp_focus_slot, input_otp_sync_caret_from_dom, locale_attrs,
    use_controllable_state, use_focus_ring, use_input_otp, use_text_field,
};

#[component]
pub fn InputOtp(
    id_base: String,
    #[prop(optional, into)] value: Option<Signal<String>>,
    #[prop(optional, into)] default_value: Option<String>,
    #[prop(optional)] on_value_change: Option<Callback<String>>,
    #[prop(optional)] set_value: Option<WriteSignal<String>>,
    #[prop(optional)] length: usize,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] on_complete: Option<Callback<String>>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] is_required: Option<Signal<bool>>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] is_invalid: Option<Signal<bool>>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] node_ref: NodeRef<html::Input>,
) -> impl IntoView {
    let on_value_change = on_value_change
        .or(on_change)
        .or_else(|| set_value.map(|setter| Callback::new(move |next: String| setter.set(next))));
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
    let strings = i18n.strings::<super::i18n::InputOtpStrings>();
    let length = length.clamp(1, 12);
    let locale = locale_attrs(lang, dir);

    let label = label.filter(|value| !value.trim().is_empty());
    let aria_label = aria_label.filter(|value| !value.trim().is_empty());
    let aria_label = aria_label.or_else(|| {
        label
            .is_none()
            .then_some(strings.aria_label.as_ref().into())
    });
    let aria_label = StoredValue::new(aria_label);

    let label = StoredValue::new(label);
    let description = StoredValue::new(description);
    let error = StoredValue::new(error);

    let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });

    let input_id = format!("{id_base}-input");
    let aria = use_text_field(TextFieldOptions {
        id: input_id,
        has_description: description.get_value().is_some(),
        has_error: error.get_value().is_some(),
        aria_describedby,
        is_invalid,
        is_required,
    });

    let base_class = if is_disabled {
        "ui-input-otp ui-input-otp--disabled".to_string()
    } else {
        "ui-input-otp".to_string()
    };
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let otp = use_input_otp(InputOtpOptions {
        is_disabled,
        length,
        value,
        on_value_change: request_value_change,
        on_complete,
    });

    let input_value = otp.input_value;
    let is_focused = otp.is_focused;
    let active_slot = otp.active_slot;
    let on_focus_hook = otp.handlers.on_focus;
    let on_blur_hook = otp.handlers.on_blur;
    let on_input_hook = otp.handlers.on_input;
    let on_caret_change = otp.handlers.on_caret_change;

    let sync_caret: Callback<()> = Callback::new(move |_| {
        input_otp_sync_caret_from_dom(
            &node_ref,
            input_value.get_untracked().chars().count(),
            on_caret_change,
        );
    });

    let on_input = move |ev| {
        on_input_hook.run(event_target_value(&ev));
        sync_caret.run(());
    };

    let on_focus = move |_| {
        focus_ring.handlers.on_focus.run(());
        on_focus_hook.run(());
        input_otp_focus_control(
            &node_ref,
            input_value.get_untracked().chars().count(),
            on_caret_change,
        );
    };

    let on_blur = move |_| {
        focus_ring.handlers.on_blur.run(());
        on_blur_hook.run(());
    };

    let on_control_pointer_down = move |ev: ev::PointerEvent| {
        if is_disabled {
            return;
        }
        ev.prevent_default();
        input_otp_focus_control(
            &node_ref,
            input_value.get_untracked().chars().count(),
            on_caret_change,
        );
    };

    let slots = (0..length)
        .map(|index| {
            let on_slot_pointer_down = move |ev: ev::PointerEvent| {
                if is_disabled {
                    return;
                }
                ev.prevent_default();
                ev.stop_propagation();
                input_otp_focus_slot(
                    &node_ref,
                    index,
                    input_value.get_untracked().chars().count(),
                    on_caret_change,
                );
            };

            let slot_value = move || input_value.get().chars().nth(index).unwrap_or_default();

            let slot_is_filled = move || {
                input_value
                    .get()
                    .chars()
                    .nth(index)
                    .is_some()
                    .then_some("true")
            };

            let slot_is_active =
                move || (is_focused.get() && active_slot.get() == index).then_some("true");

            let show_caret = move || is_focused.get() && !is_disabled && active_slot.get() == index;

            view! {
                <div
                    class="ui-input-otp__slot"
                    data-slot="input-otp-slot"
                    data-active=slot_is_active
                    data-filled=slot_is_filled
                    data-disabled=is_disabled.then_some("true")
                    data-invalid=move || is_invalid.get().then_some("true")
                    aria-hidden="true"
                    on:pointerdown=on_slot_pointer_down
                >
                    <div class="ui-input-otp__slot-value" data-slot="input-otp-slot-value">
                        {slot_value}
                    </div>
                    <Show when=show_caret>
                        <div class="ui-input-otp__caret" data-slot="input-otp-caret"></div>
                    </Show>
                </div>
            }
        })
        .collect_view();

    view! {
        <div
            class=class
            lang=locale.lang.clone()
            dir=locale.dir
            class:ui-input-otp--focus-visible=move || focus_ring.is_focus_visible.get()
            class:ui-input-otp--invalid=move || is_invalid.get()
            data-slot="input-otp"
        >
            <Show when=move || label.get_value().is_some()>
                <label
                    class="ui-input-otp__label"
                    for=aria.label.for_attr.clone()
                    data-slot="input-otp-label"
                >
                    {move || label.get_value().unwrap_or_default()}
                </label>
            </Show>

            <div
                class="ui-input-otp__control"
                data-slot="input-otp-control"
                on:pointerdown=on_control_pointer_down
            >
                <input
                    class="ui-input-otp__input"
                    data-slot="input-otp-input"
                    node_ref=node_ref
                    id=aria.input.id.clone()
                    type="text"
                    inputmode="numeric"
                    autocomplete="one-time-code"
                    pattern="[0-9]*"
                    maxlength=length
                    prop:value=move || input_value.get()
                    disabled=is_disabled
                    required=move || is_required.get()
                    aria-label=move || aria_label.get_value()
                    aria-describedby=move || aria.input.aria_describedby.get()
                    aria-invalid=move || aria.input.aria_invalid.get()
                    aria-required=move || aria.input.aria_required.get()
                    on:focus=on_focus
                    on:blur=on_blur
                    on:input=on_input
                    on:keyup=move |_| sync_caret.run(())
                    on:click=move |_| sync_caret.run(())
                    on:select=move |_| sync_caret.run(())
                />

                <div class="ui-input-otp__group" data-slot="input-otp-group" aria-hidden="true">
                    {slots}
                </div>
            </div>

            {description.get_value().map(|description| {
                let description_id = aria.description.id.clone();
                view! {
                    <div
                        class="ui-input-otp__description"
                        id=description_id
                        data-slot="input-otp-description"
                    >
                        {description}
                    </div>
                }
            })}

            {error.get_value().map(|error| {
                let error_id = aria.error.id.clone();
                let error_id = StoredValue::new(error_id);
                let error = StoredValue::new(error);
                view! {
                    <Show when=move || is_invalid.get()>
                        <div
                            class="ui-input-otp__error"
                            id=move || error_id.get_value()
                            data-slot="input-otp-error"
                        >
                            {move || error.get_value()}
                        </div>
                    </Show>
                }
            })}
        </div>
    }
}
