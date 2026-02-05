use crate::action_button::{
    ActionButtonLoadingPlacement, ActionButtonMotion, ActionButtonSize, logic, motion,
};
use crate::action_button_group;
use leptos::children::ViewFn;
use leptos::{html, prelude::*};
use ui_headless::{
    ButtonOptions, FocusRingOptions, HoverOptions, OnPress, use_button, use_focus_ring, use_hover,
};

#[component]
pub fn ActionButton(
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional)] is_loading: bool,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] size: Option<ActionButtonSize>,
    #[prop(optional)] is_quiet: Option<bool>,
    #[prop(optional)] is_icon_only: bool,
    #[prop(optional, into)] start_content: Option<ViewFn>,
    #[prop(optional, into)] end_content: Option<ViewFn>,
    #[prop(optional)] motion: ActionButtonMotion,
    #[prop(optional)] loading_placement: ActionButtonLoadingPlacement,
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
    let group = action_button_group::logic::use_action_button_group_context();

    let disabled = disabled
        .or_else(|| group.map(|ctx| ctx.is_disabled))
        .unwrap_or(false);
    let size = size
        .or_else(|| group.map(|ctx| ctx.size))
        .unwrap_or_default();
    let is_quiet = is_quiet
        .or_else(|| group.map(|ctx| ctx.is_quiet))
        .unwrap_or(false);

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

    let base_class = format!(
        "ui-action-button {} {} {}",
        if is_quiet {
            "ui-action-button--quiet"
        } else {
            "ui-action-button--filled"
        },
        size.class_name(),
        if is_icon_only {
            "ui-action-button--icon-only"
        } else {
            ""
        }
    );

    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let button_type = button_type.unwrap_or("button");

    let start_content = start_content.map(StoredValue::new);
    let end_content = end_content.map(StoredValue::new);

    view! {
        <button
            id=id
            type=button_type
            node_ref=node_ref
            class=class
            class:ui-action-button--focus-visible=move || focus_ring.is_focus_visible.get()
            disabled=state.is_disabled
            data-slot="action-button"
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
            <Show when=move || state.is_loading && matches!(loading_placement, ActionButtonLoadingPlacement::Start)>
                <span class="ui-action-button__spinner" data-slot="action-button-spinner" aria-hidden="true"></span>
            </Show>

            <Show when=move || start_content.is_some()>
                <span class="ui-action-button__start" data-slot="action-button-start">
                    {start_content
                        .expect("checked start_content")
                        .get_value()
                        .run()}
                </span>
            </Show>

            <span class="ui-action-button__label" data-slot="action-button-label">
                {children()}
            </span>

            <Show when=move || end_content.is_some()>
                <span class="ui-action-button__end" data-slot="action-button-end">
                    {end_content
                        .expect("checked end_content")
                        .get_value()
                        .run()}
                </span>
            </Show>

            <Show when=move || state.is_loading && matches!(loading_placement, ActionButtonLoadingPlacement::End)>
                <span class="ui-action-button__spinner" data-slot="action-button-spinner" aria-hidden="true"></span>
            </Show>

            <Show when=move || state.is_loading && matches!(loading_placement, ActionButtonLoadingPlacement::Center)>
                <span class="ui-action-button__spinner" data-slot="action-button-spinner" aria-hidden="true"></span>
            </Show>
        </button>
    }
}
