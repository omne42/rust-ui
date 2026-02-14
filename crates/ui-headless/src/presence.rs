use leptos::prelude::*;

#[derive(Clone)]
pub struct Presence {
    pub is_present: ReadSignal<bool>,
    pub finish_exit: Callback<()>,
}

pub fn use_presence(is_open: Signal<bool>) -> Presence {
    let (is_present, set_present) = signal(is_open.get_untracked());

    Effect::new(move |_| {
        if is_open.get() {
            set_present.set(true);
        }
    });

    let finish_exit = Callback::new(move |_| {
        set_present.set(false);
    });

    Presence {
        is_present,
        finish_exit,
    }
}
