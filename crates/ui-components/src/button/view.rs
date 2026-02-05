use crate::button::{
    ButtonLoadingPlacement, ButtonMotion, ButtonSize, ButtonVariant, logic, motion,
};
use leptos::{html, prelude::*};
use ui_headless::{
    ButtonOptions, FocusRingOptions, HoverOptions, OnPress, use_button, use_focus_ring, use_hover,
};

#[component]
pub fn Button(
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] is_loading: bool,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] motion: ButtonMotion,
    #[prop(optional)] loading_placement: ButtonLoadingPlacement,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] button_type: Option<&'static str>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] aria_haspopup: Option<&'static str>,
    #[prop(optional)] aria_expanded: Option<Signal<bool>>,
    #[prop(optional, into)] aria_controls: Option<String>,
    #[prop(optional)] aria_controls_signal: Option<Signal<Option<String>>>,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    #[prop(optional)] on_press: Option<OnPress>,
    children: Children,
) -> impl IntoView {
    let state = logic::resolve_state(disabled, is_loading);
    let aria = use_button(ButtonOptions {
        is_disabled: state.is_disabled,
        on_press,
        ..Default::default()
    });

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: state.is_disabled,
    });

    let hover = use_hover(HoverOptions {
        is_disabled: state.is_disabled,
    });

    motion::attach_motion(
        node_ref,
        hover.is_hovered,
        aria.is_pressed,
        state.is_disabled,
        motion,
    );

    let base_class = format!("ui-button {} {}", variant.class_name(), size.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);
    let button_type = button_type.unwrap_or("button");

    view! {
        <button
            id=id
            type=button_type
            node_ref=node_ref
            class=class
            class:ui-button--focus-visible=move || focus_ring.is_focus_visible.get()
            disabled=state.is_disabled
            data-slot="button"
            data-hovered=move || if hover.is_hovered.get() { Some("true") } else { None }
            data-pressed=move || if aria.is_pressed.get() { Some("true") } else { None }
            data-loading=state.is_loading.then_some("true")
            data-loading-placement=loading_placement.as_attr()
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-label=aria_label
            aria-haspopup=aria_haspopup
            aria-controls=move || {
                aria_controls_signal
                    .map(|signal| signal.get())
                    .unwrap_or_else(|| aria_controls.clone())
            }
            aria-busy=state.is_loading.then_some("true")
            aria-expanded=move || {
                aria_expanded.map(|signal| if signal.get() { "true" } else { "false" })
            }
            on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())
            on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())
            on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())
            on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
            on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
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
            <Show when=move || state.is_loading && matches!(loading_placement, ButtonLoadingPlacement::Start)>
                <span class="ui-button__spinner" data-slot="button-spinner" aria-hidden="true"></span>
            </Show>

            <span class="ui-button__label" data-slot="button-label">
                {children()}
            </span>

            <Show when=move || state.is_loading && matches!(loading_placement, ButtonLoadingPlacement::End)>
                <span class="ui-button__spinner" data-slot="button-spinner" aria-hidden="true"></span>
            </Show>

            <Show when=move || state.is_loading && matches!(loading_placement, ButtonLoadingPlacement::Center)>
                <span class="ui-button__spinner" data-slot="button-spinner" aria-hidden="true"></span>
            </Show>
        </button>
    }
}
