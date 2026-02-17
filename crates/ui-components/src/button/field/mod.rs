use super::{
    ButtonColor, ButtonLoadingPlacement, ButtonMotion, ButtonRadius, ButtonSize, ButtonVariant,
    logic, motion,
};
use leptos::{html, prelude::*};
use ui_headless::{
    ButtonOptions, FocusRingOptions, HoverOptions, OnPress, use_button, use_focus_ring, use_hover,
};

pub mod styles;

pub const DEFAULT_ARIA_LABEL: &str = "FieldButton";

#[component]
pub fn FieldButton(
    #[prop(optional)] is_quiet: bool,
    #[prop(optional)] is_invalid: bool,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_active: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, default = "button")] button_type: &'static str,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    #[prop(optional)] on_press: Option<OnPress>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let normalized_aria_label = logic::normalize_optional_text(aria_label);
    let has_custom_aria_label = normalized_aria_label.is_some();
    let aria_label = normalized_aria_label.unwrap_or_else(|| DEFAULT_ARIA_LABEL.to_string());
    let has_custom_class_name = class_name.is_some();
    let has_custom_press_handler = on_press.is_some();

    let variant = if is_quiet {
        ButtonVariant::Ghost
    } else {
        ButtonVariant::Default
    };
    let color = if is_invalid {
        ButtonColor::Danger
    } else {
        ButtonColor::Default
    };

    let state = logic::resolve_state(logic::ButtonStateInput {
        is_disabled,
        is_loading: false,
        variant,
        color,
        radius: ButtonRadius::default(),
        size: ButtonSize::S,
        loading_placement: ButtonLoadingPlacement::Start,
        is_icon_only: false,
        is_full_width: false,
        has_start_content: false,
        has_end_content: false,
        has_custom_class_name: false,
        has_custom_motion: false,
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

    motion::attach_motion(
        node_ref,
        hover.is_hovered,
        aria.is_pressed,
        state.is_disabled,
        ButtonMotion::default(),
    );

    let mut classes = vec![
        logic::compose_class_name(None, state),
        "ui-field-button".to_string(),
    ];
    if is_quiet {
        classes.push("ui-field-button--quiet".to_string());
    }
    if is_invalid {
        classes.push("ui-field-button--invalid".to_string());
    }
    if is_active {
        classes.push("ui-field-button--active".to_string());
    }
    if state.is_disabled {
        classes.push("ui-field-button--disabled".to_string());
    }
    if has_custom_press_handler {
        classes.push("ui-field-button--custom-handler".to_string());
    }
    if has_custom_aria_label {
        classes.push("ui-field-button--custom-aria-label".to_string());
    }
    if has_custom_class_name {
        classes.push("ui-field-button--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }
    let class = classes.join(" ");

    let data_state_attr = if state.is_disabled && is_invalid {
        "invalid-disabled"
    } else if state.is_disabled {
        "disabled"
    } else if is_invalid {
        "invalid"
    } else if is_active {
        "active"
    } else if is_quiet {
        "quiet"
    } else {
        "default"
    };

    view! {
        <button
            type=button_type
            node_ref=node_ref
            class=class
            class:ui-button--focus-visible=move || focus_ring.is_focus_visible.get()
            class:ui-field-button--focus-visible=move || focus_ring.is_focus_visible.get()
            class:is-hovered=move || hover.is_hovered.get()
            class:is-active=move || is_active || aria.is_pressed.get()
            disabled=state.is_disabled
            data-slot="field-button"
            data-state=data_state_attr
            data-quiet=is_quiet.then_some("true")
            data-invalid=is_invalid.then_some("true")
            data-disabled=state.is_disabled.then_some("true")
            data-active=move || (is_active || aria.is_pressed.get()).then_some("true")
            data-hovered=move || hover.is_hovered.get().then_some("true")
            data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
            data-pressed=move || aria.is_pressed.get().then_some("true")
            data-has-handler=has_custom_press_handler.then_some("true")
            data-active-mode=if is_active { "forced" } else { "interactive" }
            data-quiet-mode=if is_quiet { "true" } else { "false" }
            data-invalid-mode=if is_invalid { "true" } else { "false" }
            data-disabled-mode=if state.is_disabled { "true" } else { "false" }
            data-aria-source=if has_custom_aria_label { "custom" } else { "default" }
            data-custom-class=has_custom_class_name.then_some("true")
            data-class-source=if has_custom_class_name { "custom" } else { "default" }
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-label=aria_label
            aria-invalid=is_invalid.then_some("true")
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
            <span class="ui-field-button__label" data-slot="field-button-label">
                {children()}
            </span>
        </button>
    }
}
