use crate::pressable_feedback::{
    PressableFeedbackEffect, PressableFeedbackMotion, PressableFeedbackStateInput,
    PressableFeedbackTone,
    logic::{self},
    motion,
};
use crate::{MotionRipple, OnPress};
use leptos::{ev, html, prelude::*};
use ui_headless::{PressOptions, use_press};
use ui_visual_primitive::ripple::trigger_ripple;

#[component]
pub fn PressableFeedback(
    #[prop(optional)] effect: PressableFeedbackEffect,
    #[prop(optional)] tone: PressableFeedbackTone,
    #[prop(optional, default = true)] bounded: bool,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] motion: PressableFeedbackMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] on_press: Option<OnPress>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let has_highlight = effect.has_highlight();
    let has_ripple = effect.has_ripple();

    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != PressableFeedbackMotion::default();

    let press = use_press(PressOptions {
        is_disabled,
        on_press,
        prevent_default_for_keyboard: true,
        ..Default::default()
    });

    let pressed: Signal<bool> = press.is_pressed.into();

    let state = Signal::derive(move || {
        logic::resolve_state(PressableFeedbackStateInput {
            tone,
            effect,
            is_disabled,
            is_pressed: pressed.get(),
            bounded,
            has_highlight,
            has_ripple,
            has_custom_aria_label,
            has_custom_class_name,
            has_custom_motion,
            has_custom_press_handler: on_press.is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, pressed, motion, has_highlight);

    let ripple_ref: NodeRef<html::Span> = NodeRef::new();

    let press_handlers = press.handlers.clone();
    let ripple_motion = motion.ripple;

    let on_pointer_down = {
        let press_handlers = press_handlers.clone();
        move |_ev: ev::PointerEvent| {
            press_handlers.on_pointer_down.run(());
            if has_ripple && !is_disabled {
                trigger_ripple(ripple_ref, ripple_motion);
            }
        }
    };

    let on_pointer_up = {
        let press_handlers = press_handlers.clone();
        move |_ev: ev::PointerEvent| {
            press_handlers.on_pointer_up.run(());
        }
    };

    let on_pointer_cancel = {
        let press_handlers = press_handlers.clone();
        move |_ev: ev::PointerEvent| {
            press_handlers.on_pointer_cancel.run(());
        }
    };

    let on_click = {
        let press_handlers = press_handlers.clone();
        move |_ev: ev::MouseEvent| {
            press_handlers.on_click.run(());
        }
    };

    let on_key_down = {
        let press_handlers = press_handlers.clone();
        move |ev: ev::KeyboardEvent| {
            let key = ev.key();
            if press_handlers.on_key_down.run(key) {
                ev.prevent_default();
            }
        }
    };

    let on_key_up = {
        let press_handlers = press_handlers.clone();
        move |ev: ev::KeyboardEvent| {
            let key = ev.key();
            if press_handlers.on_key_up.run(key) {
                ev.prevent_default();
            }
        }
    };

    let on_blur = move |_ev: ev::FocusEvent| {
        press_handlers.on_blur.run(());
    };

    view! {
        <div
            node_ref=root_ref
            class=move || class.get()
            data-slot="pressable-feedback"
            data-tone=move || state.get().tone_attr
            data-effect=move || state.get().effect_attr
            data-state=move || state.get().state_attr
            data-boundary=move || state.get().boundary_attr
            data-bounded=move || state.get().is_bounded.then_some("true")
            data-unbounded=move || state.get().is_unbounded.then_some("true")
            data-pressed=move || state.get().is_pressed.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-highlight=move || state.get().highlight_attr
            data-ripple=move || state.get().ripple_attr
            data-aria-source=move || state.get().aria_source_attr
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-has-handler=move || state.get().has_custom_press_handler.then_some("true")
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            role="button"
            tabindex=move || if state.get().is_disabled { -1 } else { 0 }
            aria-label=aria_label.clone()
            aria-disabled=move || state.get().is_disabled.then_some("true")
            on:pointerdown=on_pointer_down
            on:pointerup=on_pointer_up
            on:pointercancel=on_pointer_cancel
            on:click=on_click
            on:keydown=on_key_down
            on:keyup=on_key_up
            on:blur=on_blur
        >
            <Show when=move || state.get().has_highlight>
                <span class="ui-pressable-feedback__highlight" data-slot="pressable-feedback-highlight" aria-hidden="true"></span>
            </Show>

            <Show when=move || state.get().has_ripple>
                <MotionRipple
                    node_ref=ripple_ref
                    is_bounded=bounded
                    motion=motion.ripple
                    class_name="ui-pressable-feedback__ripple".to_string()
                />
            </Show>

            <div class="ui-pressable-feedback__content" data-slot="pressable-feedback-content">
                {children()}
            </div>
        </div>
    }
}
