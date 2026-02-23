use crate::close_button::{
    CloseButtonStateInput, DEFAULT_ARIA_LABEL,
    logic::{self, CloseButtonSize, CloseButtonVariant},
};
use leptos::{html, prelude::*};
use ui_headless::i18n;
use ui_headless::i18n::CommonStrings;
use ui_headless::{
    ButtonOptions, FocusRingOptions, HoverOptions, OnPress, use_button, use_focus_ring, use_hover,
};

#[component]
pub fn CloseButton(
    #[prop(optional)] variant: CloseButtonVariant,
    #[prop(optional)] size: CloseButtonSize,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, default = "button")] button_type: &'static str,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    #[prop(optional)] on_press: Option<OnPress>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let default_aria_label = common.close_aria_label.as_ref();
    let default_aria_label = if default_aria_label.trim().is_empty() {
        DEFAULT_ARIA_LABEL
    } else {
        default_aria_label
    };
    let (aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, default_aria_label);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(CloseButtonStateInput {
        variant,
        size,
        disabled: is_disabled,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
        has_custom_press_handler: on_press.is_some(),
    });

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

    let class = logic::compose_class_name(class_name, state);

    let content: AnyView = if let Some(children) = children {
        children().into_any()
    } else {
        view! {
            <span class="ui-close-button__icon" data-slot="close-button-icon" aria-hidden="true">
                <svg viewBox="0 0 20 20" fill="none" focusable="false">
                    <path
                        d="M5.5 5.5L14.5 14.5M14.5 5.5L5.5 14.5"
                        stroke="currentColor"
                        stroke-width="1.75"
                        stroke-linecap="round"
                    ></path>
                </svg>
            </span>
        }
        .into_any()
    };

    view! {
        <button
            type=button_type
            node_ref=node_ref
            class=class
            class:ui-close-button--focus-visible=move || focus_ring.is_focus_visible.get()
            class:is-hovered=move || hover.is_hovered.get()
            class:is-active=move || aria.is_pressed.get()
            disabled=state.is_disabled
            data-slot="close-button"
            data-state=state.data_state_attr
            data-variant=state.variant_attr
            data-size=state.size_attr
            data-disabled=state.is_disabled.then_some("true")
            data-hovered=move || hover.is_hovered.get().then_some("true")
            data-pressed=move || aria.is_pressed.get().then_some("true")
            data-has-handler=state.has_custom_press_handler.then_some("true")
            data-aria-source=state.aria_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-label=aria_label
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
            {content}
        </button>
    }
}
