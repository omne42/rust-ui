use crate::clear_button::{
    ClearButtonStateInput,
    logic::{self, ClearButtonVariant},
};
use leptos::{html, prelude::*};
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
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, default = "button")] button_type: &'static str,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    #[prop(optional)] on_press: Option<OnPress>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

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

    view! {
        <button
            type=button_type
            node_ref=node_ref
            class=class
            class:ui-clear-button--focus-visible=move || focus_ring.is_focus_visible.get()
            class:is-hovered=move || hover.is_hovered.get()
            class:is-active=move || aria.is_pressed.get()
            disabled=state.is_disabled
            data-slot="clear-button"
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
            role=aria.attrs.role
            tabindex=move || {
                if state.prevent_focus || state.exclude_from_tab_order {
                    Some(-1)
                } else {
                    aria.attrs.tabindex
                }
            }
            aria-disabled=aria.attrs.aria_disabled
            aria-label=aria_label
            on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())
            on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())
            on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())
            on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
            on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
            on:click=move |_| aria.handlers.press.on_click.run(())
            on:keydown=move |ev| {
                let key = ev.key();
                if aria.handlers.press.on_key_down.run(key) {
                    ev.prevent_default();
                }
            }
            on:keyup=move |ev| {
                let key = ev.key();
                if aria.handlers.press.on_key_up.run(key) {
                    ev.prevent_default();
                }
            }
            on:focus=move |_| focus_ring.handlers.on_focus.run(())
            on:blur=move |_| {
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
