use crate::link::logic::{self, LinkStateInput};
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
    let href = logic::normalize_href(href);
    let rel = logic::normalize_optional_text(rel);
    let aria_label = logic::normalize_optional_text(aria_label);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(LinkStateInput {
        disabled,
        has_href: href.is_some(),
        target,
        has_explicit_rel: rel.is_some(),
        has_aria_label: aria_label.is_some(),
        has_custom_class_name: class_name.is_some(),
    });

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: state.is_disabled,
    });

    let hover = use_hover(HoverOptions {
        is_disabled: state.is_disabled,
    });

    let rel = logic::resolve_rel(target, rel);
    let class = logic::compose_class_name(class_name, state);

    view! {
        <a
            class=class
            class:ui-link--focus-visible=move || focus_ring.is_focus_visible.get()
            data-slot="link"
            data-state=state.state_attr
            data-enabled=state.is_enabled.then_some("true")
            data-disabled=state.is_disabled.then_some("true")
            data-missing-href=(!state.has_href).then_some("true")
            data-hovered=move || hover.is_hovered.get().then_some("true")
            data-focused=move || focus_ring.is_focused.get().then_some("true")
            data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
            data-target=state.target_kind
            data-external=state.opens_new_context.then_some("true")
            data-rel=state.rel_source_attr
            data-aria-label=if state.has_aria_label { "custom" } else { "none" }
            data-custom-class=state.has_custom_class_name.then_some("true")
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
            {children()}
        </a>
    }
}
