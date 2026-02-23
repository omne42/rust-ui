use super::{
    PressableFeedbackEffect, PressableFeedbackMotion, PressableFeedbackTone,
    logic::{self, PressableFeedbackStateInput},
    motion,
};
use leptos::{ev, html, prelude::*};
use ui_headless::{
    A11yDirection, OnPress, PressableFeedbackA11yOptions, use_controllable_state,
    use_pressable_feedback_a11y,
};
use ui_ripple::MotionRipple;
use ui_visual_primitive::ripple::trigger_ripple;

#[component]
pub fn PressableFeedback(
    #[prop(optional)] effect: PressableFeedbackEffect,
    #[prop(optional)] tone: PressableFeedbackTone,
    #[prop(optional)] is_bounded: Option<bool>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] is_pressed: Option<Signal<bool>>,
    #[prop(optional)] default_pressed: Option<bool>,
    #[prop(optional)] on_pressed_change: Option<Callback<bool>>,
    #[prop(optional)] motion: PressableFeedbackMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] on_press: Option<OnPress>,
    children: Children,
) -> impl IntoView {
    let motion = motion::sanitize_motion(motion);
    let state_contract =
        logic::normalize_state_contract(logic::PressableFeedbackStateContractInput {
            effect,
            is_bounded,
            is_disabled,
            aria_label,
            class_name,
            has_custom_motion: motion != PressableFeedbackMotion::default(),
            has_custom_press_handler: on_press.is_some(),
        });

    let is_bounded = state_contract.flags.is_bounded;
    let is_disabled = state_contract.flags.is_disabled;
    let has_highlight = state_contract.has_highlight;
    let has_ripple = state_contract.has_ripple;
    let has_custom_aria_label = state_contract.has_custom_aria_label;
    let has_custom_class_name = state_contract.has_custom_class_name;
    let has_custom_motion = state_contract.has_custom_motion;
    let has_custom_press_handler = state_contract.has_custom_press_handler;
    let aria_label = state_contract.aria_label;
    let class_name = StoredValue::new(state_contract.class_name);

    let pressed_axis =
        logic::normalize_pressed_axis(is_pressed, default_pressed, on_pressed_change);
    let is_pressed_controlled = pressed_axis.pressed_mode.is_controlled();
    let has_custom_default_pressed = pressed_axis.default_pressed_source.is_provided();
    let has_custom_on_pressed_change = pressed_axis.pressed_change_source.is_provided();
    let pressed_mode_attr = pressed_axis.pressed_mode.as_attr();
    let default_pressed_source_attr = pressed_axis.default_pressed_source.as_attr();
    let pressed_change_source_attr = pressed_axis.pressed_change_source.as_attr();

    let a11y = use_pressable_feedback_a11y(PressableFeedbackA11yOptions {
        is_disabled,
        on_press,
        lang,
        dir,
    });

    let headless_pressed: Signal<bool> = a11y.state.is_pressed.into();
    let pressed_state = use_controllable_state(
        pressed_axis.value,
        Some(pressed_axis.default_value),
        pressed_axis.on_value_change,
    );
    let pressed: Signal<bool> = pressed_state.value;
    let request_pressed_change = pressed_state.request_change;

    let state = Signal::derive(move || {
        logic::resolve_state(PressableFeedbackStateInput {
            tone,
            effect,
            is_disabled,
            is_pressed: pressed.get(),
            bounded: is_bounded,
            has_highlight,
            has_ripple,
            has_custom_aria_label,
            has_custom_class_name,
            has_custom_motion,
            has_custom_press_handler,
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, pressed, motion, has_highlight);

    let ripple_ref: NodeRef<html::Span> = NodeRef::new();

    let press_handlers = a11y.handlers.button.press.clone();
    let ripple_motion = motion.ripple;
    let role_attr = a11y.attrs.role;
    let tabindex_attr = a11y.attrs.tabindex;
    let aria_disabled_attr = a11y.attrs.aria_disabled;
    let lang_attr = a11y.attrs.lang;
    let dir_attr = a11y.attrs.dir;

    let on_pointer_down = {
        let press_handlers = press_handlers.clone();
        move |_ev: ev::PointerEvent| {
            press_handlers.on_pointer_down.run(());
            request_pressed_change.run(headless_pressed.get_untracked());
            if has_ripple && !is_disabled {
                trigger_ripple(ripple_ref, ripple_motion);
            }
        }
    };

    let on_pointer_up = {
        let press_handlers = press_handlers.clone();
        move |_ev: ev::PointerEvent| {
            press_handlers.on_pointer_up.run(());
            request_pressed_change.run(headless_pressed.get_untracked());
        }
    };

    let on_pointer_cancel = {
        let press_handlers = press_handlers.clone();
        move |_ev: ev::PointerEvent| {
            press_handlers.on_pointer_cancel.run(());
            request_pressed_change.run(headless_pressed.get_untracked());
        }
    };

    let on_click = {
        let press_handlers = press_handlers.clone();
        move |_ev: ev::MouseEvent| {
            press_handlers.on_click.run(());
            request_pressed_change.run(headless_pressed.get_untracked());
        }
    };

    let on_key_down = {
        let press_handlers = press_handlers.clone();
        move |ev: ev::KeyboardEvent| {
            let key = ev.key();
            let should_prevent = press_handlers.on_key_down.run(key);
            request_pressed_change.run(headless_pressed.get_untracked());
            if should_prevent {
                ev.prevent_default();
            }
        }
    };

    let on_key_up = {
        let press_handlers = press_handlers.clone();
        move |ev: ev::KeyboardEvent| {
            let key = ev.key();
            let should_prevent = press_handlers.on_key_up.run(key);
            request_pressed_change.run(headless_pressed.get_untracked());
            if should_prevent {
                ev.prevent_default();
            }
        }
    };

    let on_blur = {
        move |_ev: ev::FocusEvent| {
            press_handlers.on_blur.run(());
            request_pressed_change.run(headless_pressed.get_untracked());
        }
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
            data-pressed-mode=pressed_mode_attr
            data-default-pressed-source=default_pressed_source_attr
            data-pressed-change-source=pressed_change_source_attr
            data-pressed-controlled=is_pressed_controlled.then_some("true")
            data-pressed-uncontrolled=(!is_pressed_controlled).then_some("true")
            data-custom-default-pressed=has_custom_default_pressed.then_some("true")
            data-custom-pressed-change=has_custom_on_pressed_change.then_some("true")
            data-has-handler=move || state.get().has_custom_press_handler.then_some("true")
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            role=role_attr
            tabindex=tabindex_attr
            aria-label=aria_label.clone()
            aria-disabled=aria_disabled_attr
            lang=lang_attr.clone()
            dir=dir_attr
            on:pointerdown=on_pointer_down
            on:pointerup=on_pointer_up
            on:pointercancel=on_pointer_cancel
            on:click=on_click
            on:keydown=on_key_down
            on:keyup=on_key_up
            on:blur=on_blur
        >
            <span class="ui-pressable-feedback__highlight" data-slot="pressable-feedback-highlight" aria-hidden="true"></span>

            <MotionRipple
                node_ref=ripple_ref
                is_bounded=is_bounded
                motion=motion.ripple
                class_name="ui-pressable-feedback__ripple".to_string()
            />

            <div class="ui-pressable-feedback__content" data-slot="pressable-feedback-content">
                {children()}
            </div>
        </div>
    }
}
