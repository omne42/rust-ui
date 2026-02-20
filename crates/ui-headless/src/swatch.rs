use crate::a11y::{A11yDirection, locale_attrs};
use crate::button::{ButtonElement, ButtonHandlers, ButtonOptions, use_button};
use crate::press::OnPress;
use leptos::prelude::*;

#[derive(Clone)]
pub struct SwatchOptions {
    pub is_disabled: bool,
    pub is_decorative: bool,
    pub is_mixed_value: bool,
    pub is_selected: Signal<bool>,
    pub aria_label: Option<String>,
    pub on_press: Option<OnPress>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct SwatchHandlers {
    pub button: ButtonHandlers,
}

#[derive(Clone)]
pub struct SwatchAttrs {
    pub role: Option<&'static str>,
    pub tabindex: Option<i32>,
    pub aria_disabled: Option<&'static str>,
    pub aria_pressed: Signal<Option<&'static str>>,
    pub aria_checked: Option<&'static str>,
    pub aria_hidden: Option<&'static str>,
    pub aria_label: Option<String>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct SwatchState {
    pub is_pressed: ReadSignal<bool>,
    pub is_interactive: bool,
}

#[derive(Clone)]
pub struct SwatchAria {
    pub state: SwatchState,
    pub handlers: SwatchHandlers,
    pub attrs: SwatchAttrs,
}

pub fn use_swatch(options: SwatchOptions) -> SwatchAria {
    let SwatchOptions {
        is_disabled,
        is_decorative,
        is_mixed_value,
        is_selected,
        aria_label,
        on_press,
        lang,
        dir,
    } = options;

    let is_interactive = !is_disabled && !is_decorative && !is_mixed_value;
    let locale = locale_attrs(lang, dir);

    let button = use_button(ButtonOptions {
        is_disabled: !is_interactive,
        on_press,
        element: ButtonElement::Custom,
    });

    let aria_pressed = Signal::derive(move || {
        (!is_decorative && !is_mixed_value).then_some(if is_selected.get() {
            "true"
        } else {
            "false"
        })
    });

    SwatchAria {
        state: SwatchState {
            is_pressed: button.is_pressed,
            is_interactive,
        },
        handlers: SwatchHandlers {
            button: button.handlers,
        },
        attrs: SwatchAttrs {
            role: (!is_decorative).then_some("button"),
            tabindex: is_interactive.then_some(0),
            aria_disabled: is_disabled.then_some("true"),
            aria_pressed,
            aria_checked: is_mixed_value.then_some("mixed"),
            aria_hidden: is_decorative.then_some("true"),
            aria_label: (!is_decorative).then_some(aria_label).flatten(),
            lang: locale.lang,
            dir: locale.dir,
        },
    }
}

#[cfg(test)]
#[path = "test/swatch.rs"]
mod tests;
