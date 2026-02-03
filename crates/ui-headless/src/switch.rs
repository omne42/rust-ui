use crate::press::{OnPress, PressActivationKeys, PressHandlers, PressOptions, use_press};
use leptos::prelude::*;

#[derive(Clone)]
pub struct SwitchOptions {
    pub is_disabled: bool,
    pub is_checked: ReadSignal<bool>,
    pub on_press: Option<OnPress>,
}

#[derive(Clone)]
pub struct SwitchHandlers {
    pub press: PressHandlers,
}

#[derive(Clone)]
pub struct SwitchAttrs {
    pub role: &'static str,
    pub tabindex: i32,
    pub aria_checked: Memo<&'static str>,
    pub aria_disabled: Option<&'static str>,
}

#[derive(Clone)]
pub struct SwitchAria {
    pub is_pressed: ReadSignal<bool>,
    pub handlers: SwitchHandlers,
    pub attrs: SwitchAttrs,
}

pub fn use_switch(options: SwitchOptions) -> SwitchAria {
    let is_checked = options.is_checked;
    let aria_checked = Memo::new(move |_| if is_checked.get() { "true" } else { "false" });

    let press = use_press(PressOptions {
        is_disabled: options.is_disabled,
        on_press: options.on_press,
        activation_keys: PressActivationKeys::SPACE,
        prevent_default_for_keyboard: true,
        ..Default::default()
    });

    SwitchAria {
        is_pressed: press.is_pressed,
        handlers: SwitchHandlers {
            press: press.handlers,
        },
        attrs: SwitchAttrs {
            role: "switch",
            tabindex: if options.is_disabled { -1 } else { 0 },
            aria_checked,
            aria_disabled: options.is_disabled.then_some("true"),
        },
    }
}
