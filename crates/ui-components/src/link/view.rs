use crate::link::logic;
use leptos::prelude::*;
use ui_headless::{FocusRingOptions, HoverOptions, use_focus_ring, use_hover};

#[component]
pub fn Link(
    href: String,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] target: Option<&'static str>,
    #[prop(optional, into)] rel: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: disabled,
    });

    let hover = use_hover(HoverOptions {
        is_disabled: disabled,
    });

    let rel = logic::resolve_rel(target, rel);

    let base_class = "ui-link".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <a
            class=class
            data-slot="link"
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
            {children()}
        </a>
    }
}
