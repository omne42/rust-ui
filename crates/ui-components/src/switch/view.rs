use crate::switch::{SwitchMotion, motion};
use leptos::{html, prelude::*};
use ui_headless::{
    FocusRingOptions, HoverOptions, OnPress, SwitchOptions, use_focus_ring, use_hover, use_switch,
};

#[component]
pub fn Switch(
    checked: ReadSignal<bool>,
    set_checked: WriteSignal<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_change: Option<Callback<bool>>,
    #[prop(optional, default = 19.0)] pressed_width_px: f64,
    #[prop(optional)] motion: SwitchMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    children: Children,
) -> impl IntoView {
    let toggle: OnPress = Callback::new(move |_| {
        let next = !checked.get_untracked();
        set_checked.set(next);
        if let Some(on_change) = on_change {
            on_change.run(next);
        }
    });

    let aria = use_switch(SwitchOptions {
        is_disabled: disabled,
        is_checked: checked,
        on_press: Some(toggle),
    });

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: disabled,
    });

    let hover = use_hover(HoverOptions {
        is_disabled: disabled,
    });

    let thumb_ref: NodeRef<html::Span> = NodeRef::new();
    motion::attach_thumb_motion(
        thumb_ref,
        checked,
        aria.is_pressed,
        pressed_width_px,
        motion,
    );

    let data_state = move || {
        if checked.get() {
            "checked"
        } else {
            "unchecked"
        }
    };

    let base_class = "ui-switch".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <button
            type="button"
            node_ref=node_ref
            class=class
            class:ui-switch--focus-visible=move || focus_ring.is_focus_visible.get()
            disabled=disabled
            data-slot="switch"
            data-state=data_state
            data-pressed=move || if aria.is_pressed.get() { Some("true") } else { None }
            data-hovered=move || if hover.is_hovered.get() { Some("true") } else { None }
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-checked=move || aria.attrs.aria_checked.get()
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
            <span class="ui-switch__track" data-slot="switch-track">
                <span node_ref=thumb_ref class="ui-switch__thumb" data-slot="switch-thumb"></span>
            </span>
            <span class="ui-switch__label" data-slot="switch-label">
                {children()}
            </span>
        </button>
    }
}
