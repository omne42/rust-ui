use crate::button_search_input::{SearchInputButtonMotion, logic, motion};
use leptos::{html, prelude::*};
use ui_headless::{
    ButtonOptions, FocusRingOptions, HoverOptions, OnPress, use_button, use_focus_ring, use_hover,
};

#[component]
pub fn SearchInputButton(
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] compact_placeholder: Option<String>,
    #[prop(optional, into)] meta_key_label: Option<String>,
    #[prop(optional, into)] key_label: Option<String>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: SearchInputButtonMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] button_type: Option<&'static str>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    #[prop(optional)] on_press: Option<OnPress>,
) -> impl IntoView {
    let state = logic::resolve_state(is_disabled, disabled);
    let view_state = logic::resolve_view_state(
        placeholder.as_deref(),
        compact_placeholder.as_deref(),
        meta_key_label.as_deref(),
        key_label.as_deref(),
    );

    let aria_label = aria_label
        .or_else(|| Some(view_state.placeholder.clone()))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "Search".to_string());
    let aria_label = StoredValue::new(aria_label);

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

    let base_class = "ui-search-input-button".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let button_type = button_type.unwrap_or("button");

    let show_shortcut = view_state.show_shortcut;
    let placeholder = StoredValue::new(view_state.placeholder);
    let compact_placeholder = StoredValue::new(view_state.compact_placeholder);
    let meta_key_label = StoredValue::new(view_state.meta_key_label.unwrap_or_default());
    let key_label = StoredValue::new(view_state.key_label.unwrap_or_default());

    view! {
        <button
            type=button_type
            node_ref=node_ref
            class=class
            class:ui-search-input-button--focus-visible=move || focus_ring.is_focus_visible.get()
            disabled=state.is_disabled
            data-slot="search-input-button"
            data-disabled=state.is_disabled.then_some("true")
            data-hovered=move || if hover.is_hovered.get() { Some("true") } else { None }
            data-pressed=move || if aria.is_pressed.get() { Some("true") } else { None }
            aria-label=move || aria_label.get_value()
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
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
            <svg class="ui-search-input-button__icon" viewBox="0 0 20 20" fill="none" aria-hidden="true">
                <circle cx="9" cy="9" r="6" stroke="currentColor" stroke_width="1.5" />
                <path
                    d="M13.5 13.5l3 3"
                    stroke="currentColor"
                    stroke_width="1.5"
                    stroke_linecap="round"
                />
            </svg>

            <span class="ui-search-input-button__placeholder ui-search-input-button__placeholder--full">
                {move || placeholder.get_value()}
            </span>
            <span class="ui-search-input-button__placeholder ui-search-input-button__placeholder--compact">
                {move || compact_placeholder.get_value()}
            </span>

            <Show when=move || show_shortcut>
                <span class="ui-search-input-button__shortcut" aria-hidden="true">
                    <span class="ui-search-input-button__key">{move || meta_key_label.get_value()}</span>
                    <span class="ui-search-input-button__key">{move || key_label.get_value()}</span>
                </span>
            </Show>
        </button>
    }
}
