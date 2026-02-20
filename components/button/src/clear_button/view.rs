use crate::clear_button::{
    ClearButtonStateInput,
    logic::{self, ClearButtonVariant},
};
use leptos::{ev, html, prelude::*};
use ui_headless::i18n;
use ui_headless::i18n::CommonStrings;
use ui_headless::{
    ButtonOptions, FocusRingOptions, HoverOptions, OnPress, use_button, use_focus_ring, use_hover,
};

#[component]
pub fn ClearButton(
    #[prop(optional)] variant: ClearButtonVariant,
    #[prop(optional)] inset: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] prevent_focus: bool,
    #[prop(optional)] exclude_from_tab_order: bool,
    #[prop(optional, default = "clear-button")] slot_name: &'static str,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, default = "button")] button_type: &'static str,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    #[prop(optional)] on_press: Option<OnPress>,
    #[prop(optional)] is_visible: Option<Signal<bool>>,
    #[prop(optional)] is_disabled_signal: Option<Signal<bool>>,
    #[prop(optional)] aria_hidden_when_invisible: bool,
    #[prop(optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(optional)] on_pointer_up: Option<Callback<ev::PointerEvent>>,
    #[prop(optional)] on_pointer_cancel: Option<Callback<ev::PointerEvent>>,
    #[prop(optional)] on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    #[prop(optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] on_key_down: Option<Callback<String, bool>>,
    #[prop(optional)] on_key_up: Option<Callback<String, bool>>,
    #[prop(optional)] on_blur: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let (aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, common.clear_aria_label.as_ref());

    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(ClearButtonStateInput {
        variant,
        inset,
        disabled,
        prevent_focus,
        exclude_from_tab_order,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
        has_custom_press_handler: on_press.is_some(),
    });

    let aria = use_button(ButtonOptions {
        is_disabled: state.is_disabled,
        on_press,
        ..Default::default()
    });

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: state.is_disabled,
    });

    let hover = use_hover(HoverOptions {
        is_disabled: state.is_disabled,
    });

    let class = logic::compose_class_name(class_name, state);
    let is_visible = is_visible.unwrap_or_else(|| Signal::derive(|| true));
    let is_disabled_signal = is_disabled_signal.unwrap_or_else(|| Signal::derive(|| false));
    let on_pointer_down = StoredValue::new(on_pointer_down);
    let on_pointer_up = StoredValue::new(on_pointer_up);
    let on_pointer_cancel = StoredValue::new(on_pointer_cancel);
    let on_pointer_enter = StoredValue::new(on_pointer_enter);
    let on_pointer_leave = StoredValue::new(on_pointer_leave);
    let on_click = StoredValue::new(on_click);
    let on_key_down = StoredValue::new(on_key_down);
    let on_key_up = StoredValue::new(on_key_up);
    let on_blur = StoredValue::new(on_blur);

    view! {
        <button
            type=button_type
            node_ref=node_ref
            class=class
            class:ui-clear-button--focus-visible=move || focus_ring.is_focus_visible.get()
            class:is-hovered=move || hover.is_hovered.get()
            class:is-active=move || aria.is_pressed.get()
            data-slot=slot_name
            data-state=state.data_state_attr
            data-variant=state.variant_attr
            data-inset=state.is_inset.then_some("true")
            data-disabled=state.is_disabled.then_some("true")
            data-prevent-focus=state.prevent_focus.then_some("true")
            data-exclude-tab=state.exclude_from_tab_order.then_some("true")
            data-hovered=move || hover.is_hovered.get().then_some("true")
            data-pressed=move || aria.is_pressed.get().then_some("true")
            data-has-handler=state.has_custom_press_handler.then_some("true")
            data-focus-mode=state.focus_mode_attr
            data-aria-source=state.aria_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            data-visible=move || is_visible.get().then_some("true")
            role=aria.attrs.role
            tabindex=move || {
                if !is_visible.get() || state.prevent_focus || state.exclude_from_tab_order {
                    Some(-1)
                } else {
                    aria.attrs.tabindex
                }
            }
            aria-disabled=move || {
                if state.is_disabled || is_disabled_signal.get() || !is_visible.get() {
                    Some("true")
                } else {
                    aria.attrs.aria_disabled
                }
            }
            aria-hidden=move || {
                if aria_hidden_when_invisible && !is_visible.get() {
                    Some("true")
                } else {
                    None
                }
            }
            aria-label=aria_label
            disabled=move || state.is_disabled || is_disabled_signal.get() || !is_visible.get()
            on:pointerdown=move |ev: ev::PointerEvent| {
                if let Some(handler) = on_pointer_down.get_value() {
                    handler.run(ev.clone());
                }
                aria.handlers.press.on_pointer_down.run(());
            }
            on:pointerup=move |ev: ev::PointerEvent| {
                if let Some(handler) = on_pointer_up.get_value() {
                    handler.run(ev);
                }
                aria.handlers.press.on_pointer_up.run(());
            }
            on:pointercancel=move |ev: ev::PointerEvent| {
                if let Some(handler) = on_pointer_cancel.get_value() {
                    handler.run(ev);
                }
                aria.handlers.press.on_pointer_cancel.run(());
            }
            on:pointerenter=move |ev: ev::PointerEvent| {
                if let Some(handler) = on_pointer_enter.get_value() {
                    handler.run(ev);
                }
                hover.handlers.on_pointer_enter.run(());
            }
            on:pointerleave=move |ev: ev::PointerEvent| {
                if let Some(handler) = on_pointer_leave.get_value() {
                    handler.run(ev);
                }
                hover.handlers.on_pointer_leave.run(());
            }
            on:click=move |_| {
                if let Some(handler) = on_click.get_value() {
                    handler.run(());
                }
                aria.handlers.press.on_click.run(());
            }
            on:keydown=move |ev| {
                let key = ev.key();
                let mut handled = false;
                if let Some(handler) = on_key_down.get_value() {
                    handled = handler.run(key.clone());
                }
                if aria.handlers.press.on_key_down.run(key) || handled {
                    ev.prevent_default();
                }
            }
            on:keyup=move |ev| {
                let key = ev.key();
                let mut handled = false;
                if let Some(handler) = on_key_up.get_value() {
                    handled = handler.run(key.clone());
                }
                if aria.handlers.press.on_key_up.run(key) || handled {
                    ev.prevent_default();
                }
            }
            on:focus=move |_| focus_ring.handlers.on_focus.run(())
            on:blur=move |_| {
                if let Some(handler) = on_blur.get_value() {
                    handler.run(());
                }
                aria.handlers.press.on_blur.run(());
                focus_ring.handlers.on_blur.run(());
            }
        >
            <span class="ui-clear-button__label" data-slot="clear-button-label">
                {children()}
            </span>
        </button>
    }
}
