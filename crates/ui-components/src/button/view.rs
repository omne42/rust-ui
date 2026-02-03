use crate::button::{motion, ButtonMotion, ButtonSize, ButtonVariant};
use leptos::{html, prelude::*};
use ui_headless::{
    use_button, use_focus_ring, use_hover, ButtonOptions, FocusRingOptions, HoverOptions, OnPress,
};

#[component]
pub fn Button(
    #[prop(optional)] disabled: bool,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] motion: ButtonMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] button_type: Option<&'static str>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    #[prop(optional)] on_press: Option<OnPress>,
    children: Children,
) -> impl IntoView {
    let aria = use_button(ButtonOptions {
        is_disabled: disabled,
        on_press,
        ..Default::default()
    });

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: disabled,
    });

    let hover = use_hover(HoverOptions {
        is_disabled: disabled,
    });

    motion::attach_motion(
        node_ref,
        hover.is_hovered,
        aria.is_pressed,
        disabled,
        motion,
    );

    let base_class = format!("ui-button {} {}", variant.class_name(), size.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);
    let button_type = button_type.unwrap_or("button");

    view! {
        <button
            type=button_type
            node_ref=node_ref
            class=class
            class:ui-button--focus-visible=move || focus_ring.is_focus_visible.get()
            disabled=disabled
            data-slot="button"
            data-hovered=move || if hover.is_hovered.get() { Some("true") } else { None }
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
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
            {children()}
        </button>
    }
}
