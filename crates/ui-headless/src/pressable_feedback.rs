use crate::a11y::{A11yDirection, locale_attrs};
use crate::button::{ButtonElement, ButtonHandlers, ButtonOptions, use_button};
use crate::press::OnPress;
use leptos::prelude::*;

#[derive(Clone)]
pub struct PressableFeedbackA11yOptions {
    pub is_disabled: bool,
    pub on_press: Option<OnPress>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct PressableFeedbackA11yHandlers {
    pub button: ButtonHandlers,
}

#[derive(Clone)]
pub struct PressableFeedbackA11yAttrs {
    pub role: Option<&'static str>,
    pub tabindex: Option<i32>,
    pub aria_disabled: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct PressableFeedbackA11yState {
    pub is_pressed: ReadSignal<bool>,
    pub is_focus_visible: ReadSignal<bool>,
}

#[derive(Clone)]
pub struct PressableFeedbackA11yContract {
    pub attrs: PressableFeedbackA11yAttrs,
    pub handlers: PressableFeedbackA11yHandlers,
    pub state: PressableFeedbackA11yState,
}

pub fn use_pressable_feedback_a11y(
    options: PressableFeedbackA11yOptions,
) -> PressableFeedbackA11yContract {
    let locale = locale_attrs(options.lang, options.dir);
    let button = use_button(ButtonOptions {
        is_disabled: options.is_disabled,
        on_press: options.on_press,
        element: ButtonElement::Custom,
    });

    PressableFeedbackA11yContract {
        attrs: PressableFeedbackA11yAttrs {
            role: button.attrs.role,
            tabindex: button.attrs.tabindex,
            aria_disabled: button.attrs.aria_disabled,
            lang: locale.lang,
            dir: locale.dir,
        },
        handlers: PressableFeedbackA11yHandlers {
            button: button.handlers,
        },
        state: PressableFeedbackA11yState {
            is_pressed: button.is_pressed,
            is_focus_visible: button.is_focus_visible,
        },
    }
}

#[cfg(test)]
#[path = "test/pressable_feedback.rs"]
mod tests;
