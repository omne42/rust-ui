use crate::switch::{SwitchMotion, logic, motion};
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, OnPress, SwitchOptions, use_controllable_state, use_switch};

#[component]
pub fn Switch(
    #[prop(optional, into)] checked: Option<Signal<bool>>,
    #[prop(optional)] set_checked: Option<WriteSignal<bool>>,
    #[prop(optional)] default_checked: Option<bool>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(optional, default = motion::default_pressed_width_px())] pressed_width_px: f64,
    #[prop(optional)] motion: SwitchMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    children: Children,
) -> impl IntoView {
    let checked_axis = logic::normalize_checked_axis(logic::CheckedAxisInput {
        checked,
        set_checked,
        default_checked,
        on_checked_change,
    });
    let is_checked_controlled = checked_axis.is_controlled;
    let checked_control_mode_attr = checked_axis.control_mode.data_attr();
    let checked_source_attr = checked_axis.checked_source_attr;
    let default_checked_source_attr = checked_axis.default_checked_source_attr;
    let checked_change_source_attr = checked_axis.checked_change_source_attr;

    let checked_state = use_controllable_state(
        checked_axis.controlled_checked,
        Some(checked_axis.default_checked),
        checked_axis.on_checked_change,
    );
    let (checked, set_checked_signal) = signal(checked_state.value.get_untracked());
    Effect::new(move |_| {
        set_checked_signal.set(checked_state.value.get());
    });
    let request_checked_change = checked_state.request_change;

    let toggle: OnPress = Callback::new(move |_| {
        let next = logic::next_checked(checked.get_untracked());
        request_checked_change.run(next);
    });

    let aria = use_switch(SwitchOptions {
        is_disabled,
        is_checked: checked,
        on_press: Some(toggle),
        lang,
        dir,
    });

    let thumb_ref: NodeRef<html::Span> = NodeRef::new();
    motion::attach_thumb_motion(
        thumb_ref,
        checked,
        aria.state.is_pressed,
        pressed_width_px,
        motion,
    );

    let class = logic::compose_class_name(class_name);
    let (motion_source, custom_motion) =
        logic::resolve_motion_markers(motion != SwitchMotion::default());

    view! {
        <button
            type="button"
            node_ref=node_ref
            class=class
            class:ui-switch--focus-visible=move || aria.state.resolved.get().is_focus_visible
            disabled=is_disabled
            data-slot="switch"
            data-state=move || aria.state.resolved.get().data_state()
            data-checked=move || aria.state.resolved.get().is_checked.then_some("true")
            data-unchecked=move || aria.state.resolved.get().is_unchecked.then_some("true")
            data-checked-control-mode=checked_control_mode_attr
            data-checked-controlled=is_checked_controlled.then_some("true")
            data-checked-uncontrolled=(!is_checked_controlled).then_some("true")
            data-checked-source=checked_source_attr
            data-default-checked-source=default_checked_source_attr
            data-checked-change-source=checked_change_source_attr
            data-disabled=move || aria.state.resolved.get().is_disabled.then_some("true")
            data-enabled=move || aria.state.resolved.get().is_enabled.then_some("true")
            data-pressed=move || aria.state.resolved.get().is_pressed.then_some("true")
            data-hovered=move || aria.state.resolved.get().is_hovered.then_some("true")
            data-focused=move || aria.state.resolved.get().is_focused.then_some("true")
            data-focus-visible=move || aria.state.resolved.get().is_focus_visible.then_some("true")
            data-motion-source=motion_source
            data-custom-motion=custom_motion
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-checked=move || aria.attrs.aria_checked.get()
            aria-label=aria_label
            lang=move || aria.attrs.lang.clone()
            dir=move || aria.attrs.dir
            on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())
            on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())
            on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())
            on:pointerenter=move |_| aria.handlers.hover.on_pointer_enter.run(())
            on:pointerleave=move |_| aria.handlers.hover.on_pointer_leave.run(())
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
            on:focus=move |_| aria.handlers.focus_ring.on_focus.run(())
            on:blur=move |_| {
                aria.handlers.press.on_blur.run(());
                aria.handlers.focus_ring.on_blur.run(());
            }
        >
            <span class="ui-switch__track" data-slot="switch-track">
                <span node_ref=thumb_ref class="ui-switch__thumb" data-slot="switch-thumb"></span>
            </span>
            <span class="ui-switch__label" data-slot="switch-label">
                {children()}
            </span>
        </button>
    }
}
