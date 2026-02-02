use leptos::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct FocusWithinOptions {
    pub is_disabled: bool,
}

#[derive(Clone)]
pub struct FocusWithinHandlers {
    pub on_focus_in: Callback<()>,
    pub on_focus_out: Callback<()>,
}

#[derive(Clone)]
pub struct FocusWithinState {
    pub is_focus_within: ReadSignal<bool>,
    pub handlers: FocusWithinHandlers,
}

pub fn use_focus_within(options: FocusWithinOptions) -> FocusWithinState {
    let (is_focus_within, set_focus_within) = signal(false);
    let (_blur_pending, set_blur_pending) = signal(false);

    let on_focus_in = {
        let is_disabled = options.is_disabled;
        Callback::new(move |_| {
            if is_disabled {
                return;
            }
            set_blur_pending.set(false);
            set_focus_within.set(true);
        })
    };

    let on_focus_out = Callback::new(move |_| {
        set_blur_pending.set(true);

        #[cfg(all(feature = "web", target_arch = "wasm32"))]
        schedule_focus_out_check(
            is_focus_within,
            set_focus_within,
            _blur_pending,
            set_blur_pending,
        );

        #[cfg(not(all(feature = "web", target_arch = "wasm32")))]
        {
            set_blur_pending.set(false);
            set_focus_within.set(false);
        }
    });

    FocusWithinState {
        is_focus_within,
        handlers: FocusWithinHandlers {
            on_focus_in,
            on_focus_out,
        },
    }
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn schedule_focus_out_check(
    is_focus_within: ReadSignal<bool>,
    set_focus_within: WriteSignal<bool>,
    blur_pending: ReadSignal<bool>,
    set_blur_pending: WriteSignal<bool>,
) {
    use wasm_bindgen::JsCast;

    let Some(window) = web_sys::window() else {
        set_blur_pending.set(false);
        set_focus_within.set(false);
        return;
    };

    // `focusout` fires before `focusin` when moving focus between descendants.
    // Defer turning off until end of the tick; `focusin` cancels via blur_pending=false.
    let callback: js_sys::Function = wasm_bindgen::closure::Closure::once_into_js(move || {
        if blur_pending.get_untracked() && is_focus_within.get_untracked() {
            set_focus_within.set(false);
        }
        set_blur_pending.set(false);
    })
    .unchecked_into();

    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(&callback, 0);
}
