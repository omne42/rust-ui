use crate::switch::{SwitchMotion, motion};
use leptos::{html, prelude::*};
use ui_headless::{OnPress, SwitchOptions, use_switch};

#[component]
pub fn Switch(
    checked: ReadSignal<bool>,
    set_checked: WriteSignal<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(optional, default = motion::default_pressed_width_px())] pressed_width_px: f64,
    #[prop(optional)] motion: SwitchMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    children: Children,
) -> impl IntoView {
    let toggle: OnPress = Callback::new(move |_| {
        let next = !checked.get_untracked();
        set_checked.set(next);
        if let Some(on_checked_change) = on_checked_change {
            on_checked_change.run(next);
        }
    });

    let aria = use_switch(SwitchOptions {
        is_disabled: disabled,
        is_checked: checked,
        on_press: Some(toggle),
        lang: None,
        dir: None,
    });

    let thumb_ref: NodeRef<html::Span> = NodeRef::new();
    motion::attach_thumb_motion(
        thumb_ref,
        checked,
        aria.state.is_pressed,
        pressed_width_px,
        motion,
    );

    let base_class = "ui-switch".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let motion_source = if motion == SwitchMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != SwitchMotion::default()).then_some("true");

    view! {
        <button
            type="button"
            node_ref=node_ref
            class=class
            class:ui-switch--focus-visible=move || aria.state.resolved.get().is_focus_visible
            disabled=disabled
            data-slot="switch"
            data-state=move || aria.state.resolved.get().data_state()
            data-checked=move || aria.state.resolved.get().is_checked.then_some("true")
            data-unchecked=move || aria.state.resolved.get().is_unchecked.then_some("true")
            data-disabled=move || aria.state.resolved.get().is_disabled.then_some("true")
            data-enabled=move || aria.state.resolved.get().is_enabled.then_some("true")
            data-pressed=move || aria.state.resolved.get().is_pressed.then_some("true")
            data-hovered=move || aria.state.resolved.get().is_hovered.then_some("true")
            data-focused=move || aria.state.resolved.get().is_focused.then_some("true")
            data-focus-visible=move || aria.state.resolved.get().is_focus_visible.then_some("true")
            data-motion-source=motion_source
            data-custom-motion=custom_motion
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-checked=move || aria.attrs.aria_checked.get()
            aria-label=aria_label
            lang=move || aria.attrs.lang.clone()
            dir=move || aria.attrs.dir
            on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())
            on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())
            on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())
            on:pointerenter=move |_| aria.handlers.hover.on_pointer_enter.run(())
            on:pointerleave=move |_| aria.handlers.hover.on_pointer_leave.run(())
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
            on:focus=move |_| aria.handlers.focus_ring.on_focus.run(())
            on:blur=move |_| {
                aria.handlers.press.on_blur.run(());
                aria.handlers.focus_ring.on_blur.run(());
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
