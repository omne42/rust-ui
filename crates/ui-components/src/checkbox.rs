use leptos::prelude::*;
use ui_headless::{use_checkbox, use_focus_ring, CheckboxOptions, FocusRingOptions, OnPress};

#[component]
pub fn Checkbox(
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

    let aria = use_checkbox(CheckboxOptions {
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
            class="ui-checkbox"
            class:ui-checkbox--checked=move || checked.get()
            class:ui-checkbox--disabled=disabled
            class:ui-checkbox--pressed=move || aria.is_pressed.get()
            class:ui-checkbox--focus-visible=move || focus_ring.is_focus_visible.get()
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-checked=move || aria.attrs.aria_checked.get()
            style="display: inline-flex; align-items: center; gap: 8px; user-select: none;"
            style:cursor=cursor
            style:opacity=opacity
            style:outline=move || if focus_ring.is_focus_visible.get() { "2px solid var(--ui-focus-ring)" } else { "none" }
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
                class="ui-checkbox__box"
                style="width: 16px; height: 16px; border-radius: 4px; border: 1px solid var(--ui-border); display: flex; align-items: center; justify-content: center; font-size: 12px; line-height: 1;"
                style:background-color=move || if checked.get() { "var(--ui-accent)" } else { "var(--ui-bg)" }
                style:border-color=move || if checked.get() { "var(--ui-accent)" } else { "var(--ui-border)" }
                style:color=move || if checked.get() { "white" } else { "transparent" }
            >
                "✓"
            </div>
            <div class="ui-checkbox__label" style="font-size: 14px; line-height: 1.2;">
                {children()}
            </div>
        </div>
    }
}
