use crate::a11y::{A11yDirection, locale_attrs};
use leptos::prelude::*;
use ui_state_primitives::color_wheel::{
    MAX_VALUE, MIN_VALUE, format_value_text, move_value_by_delta, page_step, parse_value,
    sanitize_step, sanitize_value,
};

const BOOL_TRUE: &str = "true";

#[derive(Clone, Debug)]
pub struct ColorWheelOptions {
    pub is_disabled: bool,
    pub value: Signal<f64>,
    pub step: f64,
    pub aria_label: String,
    pub label_id: String,
    pub value_id: Option<String>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorWheelRootAttrs {
    pub role: &'static str,
    pub aria_labelledby: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct ColorWheelInputAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub aria_labelledby: String,
    pub aria_describedby: Option<String>,
    pub aria_disabled: Option<&'static str>,
    pub aria_valuemin: String,
    pub aria_valuemax: String,
    pub aria_valuenow: Memo<String>,
    pub aria_valuetext: Memo<String>,
}

#[derive(Clone)]
pub struct ColorWheelTrackAttrs {
    pub data_dragging: Signal<Option<&'static str>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorWheelKeyboardResult {
    pub next_value: f64,
    pub prevent_default: bool,
}

#[derive(Clone)]
pub struct ColorWheelHandlers {
    pub on_input: Callback<String, Option<f64>>,
    pub on_key_down: Callback<String, Option<ColorWheelKeyboardResult>>,
    pub on_track_pointer_down: Callback<f64, Option<f64>>,
    pub on_track_pointer_move: Callback<f64, Option<f64>>,
    pub on_track_pointer_up: Callback<()>,
    pub on_track_pointer_cancel: Callback<()>,
    pub on_track_pointer_leave: Callback<()>,
}

#[derive(Clone)]
pub struct ColorWheelSemanticState {
    pub is_dragging: ReadSignal<bool>,
    pub is_disabled: bool,
    pub step: f64,
    pub value: Signal<f64>,
}

#[derive(Clone)]
pub struct ColorWheelContract {
    pub root_attrs: ColorWheelRootAttrs,
    pub input_attrs: ColorWheelInputAttrs,
    pub track_attrs: ColorWheelTrackAttrs,
    pub handlers: ColorWheelHandlers,
    pub state: ColorWheelSemanticState,
}

pub fn use_color_wheel(options: ColorWheelOptions) -> ColorWheelContract {
    let ColorWheelOptions {
        is_disabled,
        value,
        step,
        aria_label,
        label_id,
        value_id,
        lang,
        dir,
    } = options;

    let locale = locale_attrs(lang, dir);
    let step = sanitize_step(step);
    let (is_dragging, set_dragging) = signal(false);

    let on_input = Callback::new(move |raw: String| {
        if is_disabled {
            return None;
        }

        let parsed = parse_value(raw.as_str())?;
        Some(sanitize_value(parsed, step))
    });

    let on_key_down = Callback::new(move |key: String| {
        if is_disabled {
            return None;
        }

        let current = value.get();
        let next = match key.as_str() {
            "ArrowRight" | "ArrowUp" => Some(move_value_by_delta(current, step, step)),
            "ArrowLeft" | "ArrowDown" => Some(move_value_by_delta(current, -step, step)),
            "PageUp" => Some(move_value_by_delta(current, page_step(step), step)),
            "PageDown" => Some(move_value_by_delta(current, -page_step(step), step)),
            "Home" => Some(MIN_VALUE),
            "End" => Some(MAX_VALUE),
            _ => None,
        }?;

        Some(ColorWheelKeyboardResult {
            next_value: next,
            prevent_default: true,
        })
    });

    let on_track_pointer_down = Callback::new(move |raw_hue: f64| {
        if is_disabled {
            return None;
        }

        set_dragging.set(true);
        Some(sanitize_value(raw_hue, step))
    });

    let on_track_pointer_move = Callback::new(move |raw_hue: f64| {
        if is_disabled || !is_dragging.get() {
            return None;
        }

        Some(sanitize_value(raw_hue, step))
    });

    let on_track_pointer_up = Callback::new(move |_| set_dragging.set(false));
    let on_track_pointer_cancel = Callback::new(move |_| set_dragging.set(false));
    let on_track_pointer_leave = Callback::new(move |_| set_dragging.set(false));

    ColorWheelContract {
        root_attrs: ColorWheelRootAttrs {
            role: "group",
            aria_labelledby: label_id.clone(),
            lang: locale.lang,
            dir: locale.dir,
        },
        input_attrs: ColorWheelInputAttrs {
            role: "slider",
            aria_label,
            aria_labelledby: label_id,
            aria_describedby: value_id,
            aria_disabled: is_disabled.then_some(BOOL_TRUE),
            aria_valuemin: MIN_VALUE.to_string(),
            aria_valuemax: MAX_VALUE.to_string(),
            aria_valuenow: Memo::new(move |_| {
                sanitize_value(value.get(), step).round().to_string()
            }),
            aria_valuetext: Memo::new(move |_| format_value_text(value.get())),
        },
        track_attrs: ColorWheelTrackAttrs {
            data_dragging: Signal::derive(move || is_dragging.get().then_some(BOOL_TRUE)),
        },
        handlers: ColorWheelHandlers {
            on_input,
            on_key_down,
            on_track_pointer_down,
            on_track_pointer_move,
            on_track_pointer_up,
            on_track_pointer_cancel,
            on_track_pointer_leave,
        },
        state: ColorWheelSemanticState {
            is_dragging,
            is_disabled,
            step,
            value,
        },
    }
}

#[cfg(test)]
#[path = "test/color_wheel.rs"]
mod tests;
