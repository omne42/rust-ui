use leptos::prelude::*;
use ui_headless::{use_button, use_focus_ring, ButtonOptions, FocusRingOptions, OnPress};

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

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: disabled,
    });
    let base_class = format!("ui-button {}", variant.class_name());

    view! {
        <button
            type="button"
            class=base_class
            class:ui-button--pressed=move || aria.is_pressed.get()
            class:ui-button--focus-visible=move || focus_ring.is_focus_visible.get()
            disabled=disabled
            aria-disabled=move || if disabled { Some("true") } else { None }
            on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())
            on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())
            on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())
            on:click=move |_| aria.handlers.press.on_click.run(())
            on:keydown=move |ev| aria.handlers.on_key_down.run(ev.key())
            on:focus=move |_| focus_ring.handlers.on_focus.run(())
            on:blur=move |_| focus_ring.handlers.on_blur.run(())
        >
            {children()}
        </button>
    }
}
