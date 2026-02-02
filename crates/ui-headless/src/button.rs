use crate::focus_visible::use_focus_visible;
use crate::press::{use_press, OnPress, PressHandlers, PressOptions};
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonElement {
    /// A native `<button>`.
    #[default]
    Button,
    /// A custom element that should behave like a button (e.g. `<div>`).
    Custom,
}

#[derive(Clone, Default)]
pub struct ButtonOptions {
    pub is_disabled: bool,
    pub on_press: Option<OnPress>,
    pub element: ButtonElement,
}

#[derive(Clone)]
pub struct ButtonHandlers {
    pub press: PressHandlers,
}

#[derive(Clone)]
pub struct ButtonAttrs {
    pub role: Option<&'static str>,
    pub tabindex: Option<i32>,
    pub aria_disabled: Option<&'static str>,
}

#[derive(Clone)]
pub struct ButtonAria {
    pub is_pressed: ReadSignal<bool>,
    pub is_focus_visible: ReadSignal<bool>,
    pub handlers: ButtonHandlers,
    pub attrs: ButtonAttrs,
}

pub fn use_button(options: ButtonOptions) -> ButtonAria {
    let press = use_press(PressOptions {
        is_disabled: options.is_disabled,
        on_press: options.on_press,
        prevent_default_for_keyboard: matches!(options.element, ButtonElement::Custom),
        ignore_click_after_keyboard: matches!(options.element, ButtonElement::Button),
        ..Default::default()
    });

    let is_focus_visible = use_focus_visible()
        .map(|s| s.is_focus_visible())
        .unwrap_or_else(|| signal(false).0);

    let attrs = match options.element {
        ButtonElement::Button => ButtonAttrs {
            role: None,
            tabindex: None,
            aria_disabled: options.is_disabled.then_some("true"),
        },
        ButtonElement::Custom => ButtonAttrs {
            role: Some("button"),
            tabindex: Some(if options.is_disabled { -1 } else { 0 }),
            aria_disabled: options.is_disabled.then_some("true"),
        },
    };

    ButtonAria {
        is_pressed: press.is_pressed,
        is_focus_visible,
        handlers: ButtonHandlers {
            press: press.handlers,
        },
        attrs,
    }
}
