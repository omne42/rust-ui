use leptos::{html, prelude::*};

fn digits_only(value: &str) -> impl Iterator<Item = char> + '_ {
    value.chars().filter(|c| c.is_ascii_digit())
}

fn normalize_otp_value(value: &str, length: usize) -> String {
    let length = length.max(1);
    digits_only(value).take(length).collect()
}

#[cfg(target_arch = "wasm32")]
fn focus_input(input_ref: &NodeRef<html::Input>) {
    let Some(el) = input_ref.get_untracked() else {
        return;
    };
    ui_observability::observe_js_result!(el.focus());
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_input(_input_ref: &NodeRef<html::Input>) {}

#[cfg(target_arch = "wasm32")]
fn set_selection_range(input_ref: &NodeRef<html::Input>, start: usize, end: usize) {
    let Some(el) = input_ref.get_untracked() else {
        return;
    };
    let start = start.min(u32::MAX as usize) as u32;
    let end = end.min(u32::MAX as usize) as u32;
    drop(el.set_selection_range(start, end));
}

#[cfg(not(target_arch = "wasm32"))]
fn set_selection_range(_input_ref: &NodeRef<html::Input>, _start: usize, _end: usize) {}

#[cfg(target_arch = "wasm32")]
fn selection_start(input_ref: &NodeRef<html::Input>) -> Option<usize> {
    let el = input_ref.get_untracked()?;
    el.selection_start()
        .ok()
        .flatten()
        .and_then(|value| usize::try_from(value).ok())
}

#[cfg(not(target_arch = "wasm32"))]
fn selection_start(_input_ref: &NodeRef<html::Input>) -> Option<usize> {
    None
}

pub fn input_otp_sync_caret_from_dom(
    input_ref: &NodeRef<html::Input>,
    fallback: usize,
    on_caret_change: Callback<usize>,
) {
    let caret = selection_start(input_ref).unwrap_or(fallback);
    on_caret_change.run(caret);
}

pub fn input_otp_focus_control(
    input_ref: &NodeRef<html::Input>,
    value_len: usize,
    on_caret_change: Callback<usize>,
) {
    focus_input(input_ref);
    set_selection_range(input_ref, value_len, value_len);
    on_caret_change.run(value_len);
}

pub fn input_otp_focus_slot(
    input_ref: &NodeRef<html::Input>,
    slot_index: usize,
    value_len: usize,
    on_caret_change: Callback<usize>,
) {
    let (caret, end) = input_otp_slot_selection_range(slot_index, value_len);
    focus_input(input_ref);
    set_selection_range(input_ref, caret, end);
    on_caret_change.run(caret);
}

pub fn input_otp_slot_selection_range(slot_index: usize, value_len: usize) -> (usize, usize) {
    let caret = slot_index.min(value_len);
    let end = if caret < value_len {
        (caret + 1).min(value_len)
    } else {
        caret
    };
    (caret, end)
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
