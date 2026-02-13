use crate::logic_button::{
    LogicButtonMotion, LogicButtonStateInput,
    logic::{self, LogicButtonVariant},
    motion,
};
use leptos::{html, prelude::*};
use ui_headless::{
    ButtonOptions, FocusRingOptions, HoverOptions, OnPress, use_button, use_focus_ring, use_hover,
};

#[component]
pub fn LogicButton(
    #[prop(optional)] variant: LogicButtonVariant,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, default = "button")] button_type: &'static str,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    #[prop(optional)] on_press: Option<OnPress>,
    #[prop(optional)] motion: LogicButtonMotion,
    children: Children,
) -> impl IntoView {
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != LogicButtonMotion::default();

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(LogicButtonStateInput {
        variant,
        disabled,
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

    view! {
        <button
            type=button_type
            node_ref=node_ref
            class=class
            class:ui-logic-button--focus-visible=move || focus_ring.is_focus_visible.get()
            class:is-hovered=move || hover.is_hovered.get()
            class:is-active=move || aria.is_pressed.get()
            disabled=state.is_disabled
            data-slot="logic-button"
            data-state=state.data_state_attr
            data-variant=state.variant_attr
            data-disabled=state.is_disabled.then_some("true")
            data-hovered=move || hover.is_hovered.get().then_some("true")
            data-pressed=move || aria.is_pressed.get().then_some("true")
            data-has-handler=state.has_custom_press_handler.then_some("true")
            data-aria-source=state.aria_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
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
            <span class="ui-logic-button__label" data-slot="logic-button-label">
                {children()}
            </span>
        </button>
    }
}
