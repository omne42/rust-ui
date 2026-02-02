use leptos::prelude::*;
use ui_headless::{use_button, ButtonOptions, OnPress};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Primary,
}

impl ButtonVariant {
    fn class_name(self) -> &'static str {
        match self {
            ButtonVariant::Default => "ui-button--default",
            ButtonVariant::Primary => "ui-button--primary",
        }
    }
}

#[component]
pub fn Button(
    #[prop(optional)] disabled: bool,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] on_press: Option<OnPress>,
    children: Children,
) -> impl IntoView {
    let aria = use_button(ButtonOptions {
        is_disabled: disabled,
        on_press,
    });

    let (is_focused, set_focused) = signal(false);
    let is_focus_visible = move || is_focused.get() && aria.is_focus_visible.get();
    let base_class = format!("ui-button {}", variant.class_name());

    view! {
        <button
            type="button"
            class=base_class
            class:ui-button--pressed=move || aria.is_pressed.get()
            class:ui-button--focus-visible=is_focus_visible
            disabled=disabled
            aria-disabled=move || if disabled { Some("true") } else { None }
            on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())
            on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())
            on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())
            on:click=move |_| aria.handlers.press.on_click.run(())
            on:keydown=move |ev| aria.handlers.on_key_down.run(ev.key())
            on:focus=move |_| set_focused.set(true)
            on:blur=move |_| set_focused.set(false)
        >
            {children()}
        </button>
    }
}
