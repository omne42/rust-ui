use leptos::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct HoverOptions {
    pub is_disabled: bool,
}

#[derive(Clone)]
pub struct HoverHandlers {
    pub on_pointer_enter: Callback<()>,
    pub on_pointer_leave: Callback<()>,
}

#[derive(Clone)]
pub struct HoverState {
    pub is_hovered: ReadSignal<bool>,
    pub handlers: HoverHandlers,
}

pub fn use_hover(options: HoverOptions) -> HoverState {
    let (is_hovered, set_hovered) = signal(false);

    let on_pointer_enter = {
        let is_disabled = options.is_disabled;
        Callback::new(move |_| {
            if is_disabled {
                return;
            }
            set_hovered.set(true);
        })
    };

    let on_pointer_leave = Callback::new(move |_| set_hovered.set(false));

    HoverState {
        is_hovered,
        handlers: HoverHandlers {
            on_pointer_enter,
            on_pointer_leave,
        },
    }
}
