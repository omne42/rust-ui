use leptos::prelude::*;
use ui_headless::{use_focus_ring, use_switch, FocusRingOptions, OnPress, SwitchOptions};

#[component]
pub fn Switch(
    checked: ReadSignal<bool>,
    set_checked: WriteSignal<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_change: Option<Callback<bool>>,
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

    let cursor = if disabled { "not-allowed" } else { "pointer" };
    let opacity = if disabled { "0.55" } else { "1" };

    view! {
        <div
            class="ui-switch"
            class:ui-switch--checked=move || checked.get()
            class:ui-switch--disabled=disabled
            class:ui-switch--pressed=move || aria.is_pressed.get()
            class:ui-switch--focus-visible=move || focus_ring.is_focus_visible.get()
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-checked=move || aria.attrs.aria_checked.get()
            style="display: inline-flex; align-items: center; gap: 10px; user-select: none;"
            style:cursor=cursor
            style:opacity=opacity
            style:outline=move || if focus_ring.is_focus_visible.get() { "2px solid #2563eb" } else { "none" }
            style:outline-offset="2px"
            on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())
            on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())
            on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())
            on:click=move |_| aria.handlers.press.on_click.run(())
            on:keydown=move |ev| {
                if aria.handlers.press.on_key_down.run(ev.key()) {
                    ev.prevent_default();
                }
            }
            on:keyup=move |ev| {
                if aria.handlers.press.on_key_up.run(ev.key()) {
                    ev.prevent_default();
                }
            }
            on:focus=move |_| focus_ring.handlers.on_focus.run(())
            on:blur=move |_| {
                aria.handlers.press.on_blur.run(());
                focus_ring.handlers.on_blur.run(());
            }
        >
            <div
                class="ui-switch__track"
                style="position: relative; width: 36px; height: 20px; border-radius: 999px; background: #d1d5db; box-sizing: border-box; transition: background-color 120ms ease-out;"
                style:background-color=move || if checked.get() { "#2563eb" } else { "#d1d5db" }
            >
                <div
                    class="ui-switch__thumb"
                    style="position: absolute; top: 2px; left: 2px; width: 16px; height: 16px; border-radius: 999px; background: white; box-shadow: 0 1px 2px rgba(0,0,0,0.25); transition: transform 120ms ease-out;"
                    style:transform=move || if checked.get() { "translateX(16px)" } else { "translateX(0)" }
                />
            </div>
            <div class="ui-switch__label" style="font-size: 14px; line-height: 1.2;">
                {children()}
            </div>
        </div>
    }
}
