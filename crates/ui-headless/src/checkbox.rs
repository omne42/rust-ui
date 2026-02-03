use crate::press::{OnPress, PressActivationKeys, PressHandlers, PressOptions, use_press};
use leptos::prelude::*;

#[derive(Clone)]
pub struct CheckboxOptions {
    pub is_disabled: bool,
    pub is_checked: ReadSignal<bool>,
    pub on_press: Option<OnPress>,
}

#[derive(Clone)]
pub struct CheckboxHandlers {
    pub press: PressHandlers,
}

#[derive(Clone)]
pub struct CheckboxAttrs {
    pub role: &'static str,
    pub tabindex: i32,
    pub aria_checked: Memo<&'static str>,
    pub aria_disabled: Option<&'static str>,
}

#[derive(Clone)]
pub struct CheckboxAria {
    pub is_pressed: ReadSignal<bool>,
    pub handlers: CheckboxHandlers,
    pub attrs: CheckboxAttrs,
}

pub fn use_checkbox(options: CheckboxOptions) -> CheckboxAria {
    let is_checked = options.is_checked;
    let aria_checked = Memo::new(move |_| if is_checked.get() { "true" } else { "false" });

    let press = use_press(PressOptions {
        is_disabled: options.is_disabled,
        on_press: options.on_press,
        activation_keys: PressActivationKeys::SPACE,
        prevent_default_for_keyboard: true,
        ..Default::default()
    });

    CheckboxAria {
        is_pressed: press.is_pressed,
        handlers: CheckboxHandlers {
            press: press.handlers,
        },
        attrs: CheckboxAttrs {
            role: "checkbox",
            tabindex: if options.is_disabled { -1 } else { 0 },
            aria_checked,
            aria_disabled: options.is_disabled.then_some("true"),
        },
    }
}
