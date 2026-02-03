use leptos::{html, prelude::*};
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
    let base_class = format!("ui-button {}", variant.class_name());
    let cursor = if disabled { "not-allowed" } else { "pointer" };
    let opacity = if disabled { "0.55" } else { "1" };

    let background = match variant {
        ButtonVariant::Default => "var(--ui-bg-muted)",
        ButtonVariant::Primary => "var(--ui-accent)",
    };
    let border_color = match variant {
        ButtonVariant::Default => "var(--ui-border)",
        ButtonVariant::Primary => "var(--ui-accent)",
    };
    let text_color = match variant {
        ButtonVariant::Default => "var(--ui-fg)",
        ButtonVariant::Primary => "#ffffff",
    };

    view! {
        <button
            type="button"
            node_ref=node_ref
            class=base_class
            class:ui-button--pressed=move || aria.is_pressed.get()
            class:ui-button--focus-visible=move || focus_ring.is_focus_visible.get()
            disabled=disabled
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            style="display: inline-flex; align-items: center; justify-content: center; gap: 8px; padding: 8px 12px; border-radius: var(--ui-radius-md); border: 1px solid; box-shadow: var(--ui-shadow-sm); font-size: 14px; line-height: 1; user-select: none; transition: transform 80ms ease-out, filter 80ms ease-out;"
            style:background-color=background
            style:border-color=border_color
            style:color=text_color
            style:cursor=cursor
            style:opacity=opacity
            style:outline=move || if focus_ring.is_focus_visible.get() { "2px solid var(--ui-focus-ring)" } else { "none" }
            style:outline-offset="2px"
            style:transform=move || if aria.is_pressed.get() && !disabled { "translateY(1px)" } else { "none" }
            style:filter=move || if aria.is_pressed.get() && !disabled { "brightness(0.96)" } else { "none" }
            on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())
            on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())
            on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())
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
