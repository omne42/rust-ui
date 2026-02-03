use crate::button::{ButtonSize, ButtonVariant};
use crate::link_button::logic;
use leptos::prelude::*;
use ui_headless::{FocusRingOptions, HoverOptions, use_focus_ring, use_hover};

#[component]
pub fn LinkButton(
    href: String,
    #[prop(optional)] target: Option<&'static str>,
    #[prop(optional, into)] rel: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional, into)] aria_label: Option<String>,
    children: Children,
) -> impl IntoView {
    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: disabled,
    });

    let hover = use_hover(HoverOptions {
        is_disabled: disabled,
    });

    let rel = logic::resolve_rel(target, rel);

    let class = format!(
        "ui-link-button ui-button {} {}",
        variant.class_name(),
        size.class_name()
    );

    view! {
        <a
            class=class
            class:ui-button--focus-visible=move || focus_ring.is_focus_visible.get()
            data-slot="link-button"
            data-hovered=move || hover.is_hovered.get().then_some("true")
            data-disabled=disabled.then_some("true")
            href=(!disabled).then_some(href)
            target=target
            rel=rel
            aria-label=aria_label
            aria-disabled=disabled.then_some("true")
            tabindex=disabled.then_some(-1)
            on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
            on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
            on:focus=move |_| focus_ring.handlers.on_focus.run(())
            on:blur=move |_| focus_ring.handlers.on_blur.run(())
        >
            <span class="ui-button__label" data-slot="link-button-label">
                {children()}
            </span>
        </a>
    }
}
