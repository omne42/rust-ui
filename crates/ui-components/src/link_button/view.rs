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
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    children: Children,
) -> impl IntoView {
    let href = logic::normalize_href(href);
    let class_name = logic::normalize_optional_text(class_name);
    let aria_label = logic::normalize_optional_text(aria_label);
    let has_explicit_rel = rel.as_ref().is_some_and(|value| !value.trim().is_empty());

    let state = logic::resolve_state(
        disabled,
        href.as_deref(),
        target,
        has_explicit_rel,
        aria_label.is_some(),
        class_name.is_some(),
    );

    let rel = logic::resolve_rel(target, rel);
    let class = logic::compose_class_name(variant, size, class_name, state);

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: state.is_disabled,
    });

    let hover = use_hover(HoverOptions {
        is_disabled: state.is_disabled,
    });

    view! {
        <a
            class=class
            class:ui-button--focus-visible=move || focus_ring.is_focus_visible.get()
            data-slot="link-button"
            data-state=if state.is_disabled { "disabled" } else { "enabled" }
            data-enabled=state.is_enabled.then_some("true")
            data-disabled=state.is_disabled.then_some("true")
            data-hovered=move || hover.is_hovered.get().then_some("true")
            data-target=state.target_kind
            data-rel=if state.has_explicit_rel {
                Some("custom")
            } else if state.opens_new_context {
                Some("auto-security")
            } else {
                None
            }
            href=if state.is_enabled { href } else { None }
            target=target
            rel=rel
            aria-label=aria_label
            aria-disabled=state.is_disabled.then_some("true")
            tabindex=state.is_disabled.then_some(-1)
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
