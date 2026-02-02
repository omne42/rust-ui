use crate::focus_visible::use_focus_visible;
use leptos::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct FocusRingOptions {
    pub is_disabled: bool,
}

#[derive(Clone)]
pub struct FocusRingHandlers {
    pub on_focus: Callback<()>,
    pub on_blur: Callback<()>,
}

#[derive(Clone)]
pub struct FocusRingState {
    pub is_focused: ReadSignal<bool>,
    pub is_focus_visible: Memo<bool>,
    pub handlers: FocusRingHandlers,
}

pub fn use_focus_ring(options: FocusRingOptions) -> FocusRingState {
    let (is_focused, set_focused) = signal(false);

    let global_focus_visible = use_focus_visible()
        .map(|s| s.is_focus_visible())
        .unwrap_or_else(|| signal(false).0);

    let is_focus_visible = Memo::new(move |_| is_focused.get() && global_focus_visible.get());

    let on_focus = {
        let is_disabled = options.is_disabled;
        Callback::new(move |_| {
            if is_disabled {
                return;
            }
            set_focused.set(true);
        })
    };

    let on_blur = Callback::new(move |_| set_focused.set(false));

    FocusRingState {
        is_focused,
        is_focus_visible,
        handlers: FocusRingHandlers { on_focus, on_blur },
    }
}
