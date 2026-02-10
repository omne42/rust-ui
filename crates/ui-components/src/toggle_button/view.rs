use crate::toggle_button::{
    ToggleButtonMotion, ToggleButtonSize, ToggleButtonVariant, logic, motion,
};
use leptos::{html, prelude::*};
use ui_headless::{
    ButtonOptions, FocusRingOptions, HoverOptions, use_button, use_focus_ring, use_hover,
};

#[component]
pub fn ToggleButton(
    selected: ReadSignal<bool>,
    set_selected: WriteSignal<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] variant: ToggleButtonVariant,
    #[prop(optional)] size: ToggleButtonSize,
    #[prop(optional)] motion: ToggleButtonMotion,
    #[prop(optional)] on_change: Option<Callback<bool>>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    children: Children,
) -> impl IntoView {
    let on_press = Callback::new(move |_| {
        let next = !selected.get_untracked();
        set_selected.set(next);
        if let Some(on_change) = on_change {
            on_change.run(next);
        }
    });

    let aria = use_button(ButtonOptions {
        is_disabled: disabled,
        on_press: Some(on_press),
        ..Default::default()
    });

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: disabled,
    });
    let hover = use_hover(HoverOptions {
        is_disabled: disabled,
    });

    motion::attach_motion(
        node_ref,
        hover.is_hovered,
        aria.is_pressed,
        disabled,
        motion,
    );

    let state = Memo::new(move |_| {
        logic::resolve_state(
            selected.get(),
            disabled,
            aria.is_pressed.get(),
            hover.is_hovered.get(),
            focus_ring.is_focused.get(),
            focus_ring.is_focus_visible.get(),
        )
    });

    let base_class = format!(
        "ui-toggle-button {} {}",
        variant.class_name(),
        size.class_name()
    );
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

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
            disabled=disabled
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
