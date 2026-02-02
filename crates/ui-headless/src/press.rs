use leptos::prelude::*;
use std::rc::Rc;

pub type OnPress = Rc<dyn Fn()>;

#[derive(Clone)]
pub struct PressHandlers {
    pub on_pointer_down: Rc<dyn Fn()>,
    pub on_pointer_up: Rc<dyn Fn()>,
    pub on_pointer_cancel: Rc<dyn Fn()>,
    pub on_click: Rc<dyn Fn()>,
}

#[derive(Clone)]
pub struct PressState {
    pub is_pressed: ReadSignal<bool>,
    pub handlers: PressHandlers,
}

#[derive(Clone, Default)]
pub struct PressOptions {
    pub is_disabled: bool,
    pub on_press: Option<OnPress>,
}

pub fn use_press(options: PressOptions) -> PressState {
    let (is_pressed, set_pressed) = signal(false);
    let (did_pointer_press, set_did_pointer_press) = signal(false);

    let on_pointer_down = {
        let is_disabled = options.is_disabled;
        Rc::new(move || {
            if is_disabled {
                return;
            }
            set_did_pointer_press.set(true);
            set_pressed.set(true);
        })
    };

    let on_pointer_up = {
        let is_disabled = options.is_disabled;
        let on_press = options.on_press.clone();
        Rc::new(move || {
            if is_disabled {
                return;
            }
            set_pressed.set(false);
            if let Some(on_press) = &on_press {
                on_press();
            }
        })
    };

    let on_pointer_cancel = {
        Rc::new(move || {
            set_pressed.set(false);
            set_did_pointer_press.set(false);
        })
    };

    let on_click = {
        let is_disabled = options.is_disabled;
        let on_press = options.on_press;
        Rc::new(move || {
            if is_disabled {
                return;
            }
            if did_pointer_press.get_untracked() {
                set_did_pointer_press.set(false);
                return;
            }
            if let Some(on_press) = &on_press {
                on_press();
            }
        })
    };

    PressState {
        is_pressed,
        handlers: PressHandlers {
            on_pointer_down,
            on_pointer_up,
            on_pointer_cancel,
            on_click,
        },
    }
}
