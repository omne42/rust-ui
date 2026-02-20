use super::{CheckboxMotion, CheckboxSize, CheckboxVariant, logic, motion};
use leptos::{html, prelude::*};
use ui_headless::{
    CheckboxOptions, FocusRingOptions, HoverOptions, OnPress, use_checkbox, use_focus_ring,
    use_hover,
};

#[component]
pub fn Checkbox(
    checked: ReadSignal<bool>,
    set_checked: WriteSignal<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_change: Option<Callback<bool>>,
    #[prop(optional)] variant: CheckboxVariant,
    #[prop(optional)] size: CheckboxSize,
    #[prop(optional)] motion: CheckboxMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    children: Children,
) -> impl IntoView {
    let toggle: OnPress = Callback::new(move |_| {
        let next = !checked.get_untracked();
        set_checked.set(next);
        if let Some(on_change) = on_change {
            on_change.run(next);
        }
    });

    let aria = use_checkbox(CheckboxOptions {
        is_disabled: disabled,
        is_checked: checked,
        on_press: Some(toggle),
    });

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: disabled,
    });

    let hover = use_hover(HoverOptions {
        is_disabled: disabled,
    });

    motion::attach_root_motion(
        node_ref,
        hover.is_hovered,
        aria.is_pressed,
        disabled,
        motion,
    );

    let indicator_ref: NodeRef<html::Span> = NodeRef::new();
    motion::attach_indicator_motion(indicator_ref, checked, motion);

    let state = Memo::new(move |_| {
        logic::resolve_state(
            checked.get(),
            disabled,
            aria.is_pressed.get(),
            hover.is_hovered.get(),
            focus_ring.is_focused.get(),
            focus_ring.is_focus_visible.get(),
        )
    });

    let base_class = format!("ui-checkbox {} {}", variant.class_name(), size.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <button
            type="button"
            node_ref=node_ref
            class=class
            class:ui-checkbox--focus-visible=move || state.get().is_focus_visible
            disabled=disabled
            data-slot="checkbox"
            data-state=move || state.get().data_state()
            data-checked=move || state.get().is_checked.then_some("true")
            data-unchecked=move || state.get().is_unchecked.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-enabled=move || state.get().is_enabled.then_some("true")
            data-hovered=move || state.get().is_hovered.then_some("true")
            data-pressed=move || state.get().is_pressed.then_some("true")
            data-focused=move || state.get().is_focused.then_some("true")
            data-focus-visible=move || state.get().is_focus_visible.then_some("true")
            data-motion-source=if motion == CheckboxMotion::default() {
                "default"
            } else {
                "custom"
            }
            data-custom-motion=(motion != CheckboxMotion::default()).then_some("true")
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-checked=move || aria.attrs.aria_checked.get()
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
            <span class="ui-checkbox__box" data-slot="checkbox-box">
                <span node_ref=indicator_ref class="ui-checkbox__indicator" data-slot="checkbox-indicator">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke_width="3.5"
                        stroke="currentColor"
                    >
                        <path
                            stroke_linecap="round"
                            stroke_linejoin="round"
                            d="M4.5 12.75l6 6 9-13.5"
                        />
                    </svg>
                </span>
            </span>
            <span class="ui-checkbox__label" data-slot="checkbox-label">
                {children()}
            </span>
        </button>
    }
}
