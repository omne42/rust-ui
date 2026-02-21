use super::{ToggleButtonMotion, ToggleButtonSize, ToggleButtonVariant, logic, motion};
use leptos::{html, prelude::*};
use ui_headless as overlay_open;
#[cfg(feature = "component-toggle_button_group")]
use ui_headless::{A11yDirection, labeled_group_attrs};
use ui_headless::{
    ButtonOptions, FocusRingOptions, HoverOptions, use_button, use_focus_ring, use_hover,
};

#[component]
pub fn ToggleButton(
    #[prop(optional)] is_pressed: Option<Signal<bool>>,
    #[prop(optional)] default_pressed: Option<bool>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] variant: ToggleButtonVariant,
    #[prop(optional)] size: ToggleButtonSize,
    #[prop(optional)] motion: ToggleButtonMotion,
    #[prop(optional)] on_pressed_change: Option<Callback<bool>>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);

    let pressed_state =
        overlay_open::use_controllable_state(is_pressed, default_pressed, on_pressed_change);
    let pressed = pressed_state.value;
    let request_pressed_change = pressed_state.request_change;

    let on_press = Callback::new(move |_| {
        let next = !pressed.get_untracked();
        request_pressed_change.run(next);
    });

    let aria = use_button(ButtonOptions {
        is_disabled,
        on_press: Some(on_press),
        ..Default::default()
    });

    let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });
    let hover = use_hover(HoverOptions { is_disabled });

    motion::attach_motion(
        node_ref,
        hover.is_hovered,
        aria.is_pressed,
        is_disabled,
        motion,
    );

    let state = Memo::new(move |_| {
        logic::resolve_state(
            pressed.get(),
            is_disabled,
            aria.is_pressed.get(),
            hover.is_hovered.get(),
            focus_ring.is_focused.get(),
            focus_ring.is_focus_visible.get(),
        )
    });

    let class = logic::compose_class_name(class_name, variant, size);

    let motion_source = if motion == ToggleButtonMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != ToggleButtonMotion::default()).then_some("true");

    view! {
        <button
            type="button"
            node_ref=node_ref
            class=class
            class:ui-toggle-button--focus-visible=move || state.get().is_focus_visible
            disabled=is_disabled
            data-slot="toggle-button"
            data-state=move || state.get().data_state()
            data-selected=move || state.get().is_selected.then_some("true")
            data-unselected=move || state.get().is_unselected.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-enabled=move || state.get().is_enabled.then_some("true")
            data-hovered=move || state.get().is_hovered.then_some("true")
            data-pressed=move || state.get().is_pressed.then_some("true")
            data-focused=move || state.get().is_focused.then_some("true")
            data-focus-visible=move || state.get().is_focus_visible.then_some("true")
            data-motion-source=motion_source
            data-custom-motion=custom_motion
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-pressed=move || if state.get().is_selected { "true" } else { "false" }
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
            <span class="ui-toggle-button__label" data-slot="toggle-button-label">
                {children()}
            </span>
        </button>
    }
}

#[cfg(feature = "component-toggle_button_group")]
#[component]
pub fn ToggleButtonGroup(
    #[prop(optional)] orientation: logic::ToggleButtonGroupOrientation,
    #[prop(optional)] is_attached: bool,
    #[prop(optional)] motion: motion::ToggleButtonGroupMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_explicit_label) =
        logic::normalize_toggle_button_group_aria_label(aria_label);
    let group_a11y = labeled_group_attrs(aria_label, lang, dir);
    let class_name = logic::normalize_optional_text(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_toggle_button_group_state(orientation, is_attached, has_explicit_label)
    });
    let motion = motion::sanitize_toggle_button_group_motion(motion);
    let has_custom_motion = motion != motion::ToggleButtonGroupMotion::default();
    let style_vars = motion::attach_toggle_button_group_motion(motion);

    let class = logic::compose_toggle_button_group_class_name(class_name, orientation, is_attached);

    view! {
        <div
            class=class
            style=style_vars
            data-slot="toggle-button-group"
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-orientation=orientation.data_orientation()
            data-horizontal=move || state.get().is_horizontal.then_some("true")
            data-vertical=move || state.get().is_vertical.then_some("true")
            data-attached=move || state.get().is_attached.then_some("true")
            data-detached=move || state.get().is_detached.then_some("true")
            data-has-explicit-label=move || state.get().has_explicit_label.then_some("true")
            data-has-fallback-label=move || state.get().has_fallback_label.then_some("true")
            role=group_a11y.role
            aria-label=group_a11y.aria_label.clone()
            lang=group_a11y.lang.clone()
            dir=group_a11y.dir
        >
            {children()}
        </div>
    }
}
