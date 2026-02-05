use leptos::prelude::*;

fn digits_only(value: &str) -> impl Iterator<Item = char> + '_ {
    value.chars().filter(|c| c.is_ascii_digit())
}

fn normalize_otp_value(value: &str, length: usize) -> String {
    let length = length.max(1);
    digits_only(value).take(length).collect()
}

#[derive(Clone, Debug)]
pub struct InputOtpOptions {
    pub is_disabled: bool,
    pub length: usize,
    pub value: Signal<String>,
    pub on_value_change: Callback<String>,
    pub on_complete: Option<Callback<String>>,
}

#[derive(Clone)]
pub struct InputOtpHandlers {
    pub on_focus: Callback<()>,
    pub on_blur: Callback<()>,
    pub on_input: Callback<String>,
    pub on_caret_change: Callback<usize>,
}

#[derive(Clone)]
pub struct InputOtpAria {
    pub input_value: Memo<String>,
    pub is_focused: ReadSignal<bool>,
    pub caret_index: ReadSignal<usize>,
    pub active_slot: Memo<usize>,
    pub handlers: InputOtpHandlers,
}

pub fn use_input_otp(options: InputOtpOptions) -> InputOtpAria {
    let length = options.length.max(1);

    let input_value = Memo::new({
        let value = options.value;
        move |_| normalize_otp_value(&value.get(), length)
    });

    let (is_focused, set_is_focused) = signal(false);
    let (caret_index, set_caret_index) = signal(0_usize);
    let (was_complete, set_was_complete) = signal(false);

    let on_focus = Callback::new(move |_| {
        if options.is_disabled {
            return;
        }
        set_is_focused.set(true);
        set_caret_index.set(input_value.get_untracked().chars().count().min(length));
    });

    let on_blur = Callback::new(move |_| set_is_focused.set(false));

    let on_input = Callback::new(move |raw: String| {
        if options.is_disabled {
            return;
        }

        let next = normalize_otp_value(&raw, length);
        let next_len = next.chars().count();

        options.on_value_change.run(next.clone());
        set_caret_index.set(next_len.min(length));

        let is_complete = next_len >= length;
        if is_complete
            && !was_complete.get_untracked()
            && let Some(on_complete) = options.on_complete
        {
            on_complete.run(next);
        }
        set_was_complete.set(is_complete);
    });

    let on_caret_change = Callback::new(move |index: usize| {
        if options.is_disabled {
            return;
        }
        set_caret_index.set(index.min(length));
    });

    let active_slot = Memo::new(move |_| {
        if !is_focused.get() {
            return 0;
        }
        caret_index.get().min(length.saturating_sub(1))
    });

    InputOtpAria {
        input_value,
        is_focused,
        caret_index,
        active_slot,
        handlers: InputOtpHandlers {
            on_focus,
            on_blur,
            on_input,
            on_caret_change,
        },
    }
}
