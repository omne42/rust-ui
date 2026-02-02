use crate::focus_visible::use_focus_visible;
use crate::press::{use_press, OnPress, PressHandlers, PressOptions};
use leptos::prelude::*;

#[derive(Clone, Default)]
pub struct ButtonOptions {
    pub is_disabled: bool,
    pub on_press: Option<OnPress>,
}

#[derive(Clone)]
pub struct ButtonHandlers {
    pub press: PressHandlers,
    pub on_key_down: Callback<String>,
}

#[derive(Clone)]
pub struct ButtonAria {
    pub is_pressed: ReadSignal<bool>,
    pub is_focus_visible: ReadSignal<bool>,
    pub handlers: ButtonHandlers,
}

pub fn use_button(options: ButtonOptions) -> ButtonAria {
    let press = use_press(PressOptions {
        is_disabled: options.is_disabled,
        on_press: options.on_press,
    });

    let is_focus_visible = use_focus_visible()
        .map(|s| s.is_focus_visible())
        .unwrap_or_else(|| signal(false).0);

    // MVP: only handle Enter/Space to trigger press; do not attempt full key press state machine yet.
    let on_key_down = {
        let is_disabled = options.is_disabled;
        let on_press = options.on_press;
        Callback::new(move |key: String| {
            if is_disabled {
                return;
            }
            if key == "Enter" || key == " " {
                if let Some(on_press) = on_press {
                    on_press.run(());
                }
            }
        })
    };

    ButtonAria {
        is_pressed: press.is_pressed,
        is_focus_visible,
        handlers: ButtonHandlers {
            press: press.handlers,
            on_key_down,
        },
    }
}
