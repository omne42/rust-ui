use leptos::prelude::*;

pub type OnPress = Callback<()>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ActivePress {
    Pointer,
    KeyboardEnter,
    KeyboardSpace,
}

fn is_space_key(key: &str) -> bool {
    key == " " || key == "Space" || key == "Spacebar"
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PressActivationKeys {
    pub enter: bool,
    pub space: bool,
}

impl PressActivationKeys {
    pub const ALL: Self = Self {
        enter: true,
        space: true,
    };
    pub const ENTER: Self = Self {
        enter: true,
        space: false,
    };
    pub const SPACE: Self = Self {
        enter: false,
        space: true,
    };
    pub const NONE: Self = Self {
        enter: false,
        space: false,
    };
}

impl Default for PressActivationKeys {
    fn default() -> Self {
        Self::ALL
    }
}

#[derive(Clone)]
pub struct PressHandlers {
    pub on_pointer_down: Callback<()>,
    pub on_pointer_up: Callback<()>,
    pub on_pointer_cancel: Callback<()>,
    pub on_click: Callback<()>,
    pub on_key_down: Callback<String, bool>,
    pub on_key_up: Callback<String, bool>,
    pub on_blur: Callback<()>,
}

#[derive(Clone)]
pub struct PressState {
    pub is_pressed: ReadSignal<bool>,
    pub handlers: PressHandlers,
}

#[derive(Clone)]
pub struct PressOptions {
    pub is_disabled: bool,
    pub on_press: Option<OnPress>,
    /// Which keyboard keys trigger press activation.
    ///
    /// - `Enter` triggers on key down.
    /// - `Space` triggers on key up.
    pub activation_keys: PressActivationKeys,
    /// When `true`, pointer presses should not move focus to the target element.
    ///
    /// This is typically implemented by `preventDefault` on pointer down. This option is reserved
    /// for future work; the current implementation does not yet enforce it.
    pub prevent_focus_on_press: bool,
    /// When `true`, callers should `preventDefault` for keyboard activation keys (Enter/Space),
    /// to avoid page scrolling and to align custom elements with native button behavior.
    pub prevent_default_for_keyboard: bool,
    /// When `true`, keyboard handling will trigger `on_press` directly and ignore the next `click`
    /// event to avoid double-firing (native `<button>` emits clicks for Enter/Space).
    ///
    /// Set this to `false` for custom elements that do not generate native clicks for keyboard
    /// activation.
    pub ignore_click_after_keyboard: bool,
}

impl Default for PressOptions {
    fn default() -> Self {
        Self {
            is_disabled: false,
            on_press: None,
            activation_keys: PressActivationKeys::ALL,
            prevent_focus_on_press: false,
            prevent_default_for_keyboard: false,
            ignore_click_after_keyboard: true,
        }
    }
}

pub fn use_press(options: PressOptions) -> PressState {
    let (is_pressed, set_pressed) = signal(false);
    let (did_pointer_press, set_did_pointer_press) = signal(false);
    let (ignore_click, set_ignore_click) = signal(false);
    let (active_press, set_active_press) = signal(None::<ActivePress>);

    let on_pointer_down = {
        let is_disabled = options.is_disabled;
        Callback::new(move |_| {
            if is_disabled {
                return;
            }
            set_ignore_click.set(false);
            set_did_pointer_press.set(true);
            set_active_press.set(Some(ActivePress::Pointer));
            set_pressed.set(true);
        })
    };

    let on_pointer_up = {
        let is_disabled = options.is_disabled;
        let on_press = options.on_press;
        Callback::new(move |_| {
            if is_disabled {
                return;
            }
            if active_press.get_untracked() != Some(ActivePress::Pointer) {
                return;
            }
            set_active_press.set(None);
            set_pressed.set(false);
            if let Some(on_press) = on_press {
                on_press.run(());
            }
        })
    };

    let on_pointer_cancel = {
        Callback::new(move |_| {
            if active_press.get_untracked() != Some(ActivePress::Pointer) {
                return;
            }
            set_active_press.set(None);
            set_pressed.set(false);
            set_did_pointer_press.set(false);
            set_ignore_click.set(false);
        })
    };

    let on_click = {
        let is_disabled = options.is_disabled;
        let on_press = options.on_press;
        Callback::new(move |_| {
            if is_disabled {
                return;
            }
            if did_pointer_press.get_untracked() {
                set_did_pointer_press.set(false);
                return;
            }
            if ignore_click.get_untracked() {
                set_ignore_click.set(false);
                return;
            }
            if let Some(on_press) = on_press {
                on_press.run(());
            }
        })
    };

    let on_key_down = {
        let is_disabled = options.is_disabled;
        let activation_keys = options.activation_keys;
        let prevent_default_for_keyboard = options.prevent_default_for_keyboard;
        let ignore_click_after_keyboard = options.ignore_click_after_keyboard;
        let on_press = options.on_press;
        Callback::new(move |key: String| -> bool {
            if is_disabled {
                return false;
            }

            set_ignore_click.set(false);

            if active_press.get_untracked().is_some() {
                return false;
            }

            if key == "Enter" {
                if !activation_keys.enter {
                    return false;
                }
                set_active_press.set(Some(ActivePress::KeyboardEnter));
                set_pressed.set(true);
                if let Some(on_press) = on_press {
                    on_press.run(());
                }
                if ignore_click_after_keyboard {
                    set_ignore_click.set(true);
                }
                return prevent_default_for_keyboard;
            } else if is_space_key(&key) {
                if !activation_keys.space {
                    return false;
                }
                set_active_press.set(Some(ActivePress::KeyboardSpace));
                set_pressed.set(true);
                return prevent_default_for_keyboard;
            }

            false
        })
    };

    let on_key_up = {
        let is_disabled = options.is_disabled;
        let activation_keys = options.activation_keys;
        let prevent_default_for_keyboard = options.prevent_default_for_keyboard;
        let ignore_click_after_keyboard = options.ignore_click_after_keyboard;
        let on_press = options.on_press;
        Callback::new(move |key: String| -> bool {
            if is_disabled {
                return false;
            }

            let should_prevent_default = prevent_default_for_keyboard
                && ((activation_keys.enter && key == "Enter")
                    || (activation_keys.space && is_space_key(&key)));

            match (active_press.get_untracked(), key.as_str()) {
                (Some(ActivePress::KeyboardEnter), "Enter") => {
                    set_active_press.set(None);
                    set_pressed.set(false);
                }
                (Some(ActivePress::KeyboardSpace), k) if is_space_key(k) => {
                    set_active_press.set(None);
                    set_pressed.set(false);
                    if let Some(on_press) = on_press {
                        on_press.run(());
                    }
                    if ignore_click_after_keyboard {
                        set_ignore_click.set(true);
                    }
                }
                _ => {}
            }

            should_prevent_default
        })
    };

    let on_blur = {
        Callback::new(move |_| {
            set_active_press.set(None);
            set_pressed.set(false);
            set_did_pointer_press.set(false);
            set_ignore_click.set(false);
        })
    };

    PressState {
        is_pressed,
        handlers: PressHandlers {
            on_pointer_down,
            on_pointer_up,
            on_pointer_cancel,
            on_click,
            on_key_down,
            on_key_up,
            on_blur,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    #[test]
    fn enter_does_not_trigger_when_activation_keys_disallow_enter() {
        let called = Arc::new(AtomicUsize::new(0));
        let called2 = Arc::clone(&called);

        let press = use_press(PressOptions {
            on_press: Some(Callback::new(move |_| {
                called2.fetch_add(1, Ordering::SeqCst);
            })),
            activation_keys: PressActivationKeys::SPACE,
            ..Default::default()
        });

        press.handlers.on_key_down.run("Enter".to_string());
        press.handlers.on_key_up.run("Enter".to_string());

        assert_eq!(called.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn space_does_not_trigger_when_activation_keys_disallow_space() {
        let called = Arc::new(AtomicUsize::new(0));
        let called2 = Arc::clone(&called);

        let press = use_press(PressOptions {
            on_press: Some(Callback::new(move |_| {
                called2.fetch_add(1, Ordering::SeqCst);
            })),
            activation_keys: PressActivationKeys::ENTER,
            ..Default::default()
        });

        press.handlers.on_key_down.run(" ".to_string());
        press.handlers.on_key_up.run(" ".to_string());

        assert_eq!(called.load(Ordering::SeqCst), 0);
    }
}
