use crate::toggle::{ToggleMotion, ToggleSize, ToggleStateInput, ToggleVariant, logic};
use crate::toggle_button::motion;
use leptos::{html, prelude::*};
use ui_headless::{
    ButtonOptions, FocusRingOptions, HoverOptions, use_button, use_focus_ring, use_hover,
};

#[component]
pub fn Toggle(
    pressed: ReadSignal<bool>,
    set_pressed: WriteSignal<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] variant: ToggleVariant,
    #[prop(optional)] size: ToggleSize,
    #[prop(optional)] motion: ToggleMotion,
    #[prop(optional)] on_pressed_change: Option<Callback<bool>>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let aria_label = logic::normalize_optional_text(aria_label);

    let has_custom_class_name = class_name.is_some();
    let has_custom_motion = motion != ToggleMotion::default();
    let has_custom_aria_label = aria_label.is_some();
    let has_on_pressed_change = on_pressed_change.is_some();

    let on_press = Callback::new(move |_| {
        let next = !pressed.get_untracked();
        set_pressed.set(next);
        if let Some(on_pressed_change) = on_pressed_change {
            on_pressed_change.run(next);
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
        logic::resolve_state(ToggleStateInput {
            selected: pressed.get(),
            disabled,
            hovered: !disabled && hover.is_hovered.get(),
            pressed_interaction: !disabled && aria.is_pressed.get(),
            focused: !disabled && focus_ring.is_focused.get(),
            focus_visible: !disabled && focus_ring.is_focus_visible.get(),
            variant,
            size,
            has_custom_class_name,
            has_custom_motion,
            has_custom_aria_label,
            has_on_pressed_change,
        })
    });

    let class = logic::compose_class_name(class_name, state.get_untracked());

    view! {
        <button
            type="button"
            node_ref=node_ref
            class=class
            class:ui-toggle-button--focus-visible=move || !disabled && focus_ring.is_focus_visible.get()
            disabled=disabled
            data-slot="toggle"
            data-state=move || state.get().state_attr
            data-interaction=move || state.get().interaction_attr
            data-variant=move || state.get().variant_attr
            data-size=move || state.get().size_attr
            data-selected=move || state.get().is_selected.then_some("true")
            data-unselected=move || (!state.get().is_selected).then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-enabled=move || (!state.get().is_disabled).then_some("true")
            data-hovered=move || state.get().is_hovered.then_some("true")
            data-pressed=move || state.get().is_pressed.then_some("true")
            data-focused=move || state.get().is_focused.then_some("true")
            data-focus-visible=move || state.get().is_focus_visible.then_some("true")
            data-variant-source=move || state.get().variant_source_attr
            data-size-source=move || state.get().size_source_attr
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-handler-source=move || state.get().handler_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-custom-motion=move || state.get().has_custom_motion.then_some("true")
            data-custom-aria-label=move || state.get().has_custom_aria_label.then_some("true")
            data-custom-aria=move || state.get().has_custom_aria_label.then_some("true")
            data-custom-handler=move || state.get().has_on_pressed_change.then_some("true")
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
            <span class="ui-toggle-button__label" data-slot="toggle-label">
                {children()}
            </span>
        </button>
    }
}
