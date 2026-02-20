use crate::a11y::{A11yDirection, locale_attrs};
use crate::focus_ring::{FocusRingOptions, use_focus_ring};
use crate::hover::{HoverOptions, use_hover};
use crate::press::{PressActivationKeys, PressOptions, use_press};
use leptos::prelude::*;
use ui_state_primitives::slider::{
    parse_value, resolve_percent, sanitize_bounds, sanitize_step, sanitize_value,
};

#[derive(Clone)]
pub struct SliderOptions {
    pub is_disabled: bool,
    pub value: Option<Signal<f64>>,
    pub default_value: Option<f64>,
    pub on_value_change: Option<Callback<f64>>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct SliderInputAttrs {
    pub role: &'static str,
    pub aria_disabled: Option<&'static str>,
    pub aria_valuemin: Memo<String>,
    pub aria_valuemax: Memo<String>,
    pub aria_valuenow: Memo<String>,
    pub aria_valuetext: Memo<String>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct SliderHandlers {
    pub on_input: Callback<String>,
    pub on_pointer_down: Callback<()>,
    pub on_pointer_up: Callback<()>,
    pub on_pointer_cancel: Callback<()>,
    pub on_pointer_enter: Callback<()>,
    pub on_pointer_leave: Callback<()>,
    pub on_focus: Callback<()>,
    pub on_blur: Callback<()>,
}

#[derive(Clone)]
pub struct SliderState {
    pub is_pressed: ReadSignal<bool>,
    pub is_hovered: ReadSignal<bool>,
    pub is_focused: ReadSignal<bool>,
    pub is_focus_visible: Memo<bool>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub value: Memo<f64>,
    pub value_percent: Memo<f64>,
}

#[derive(Clone)]
pub struct SliderAria {
    pub state: SliderState,
    pub handlers: SliderHandlers,
    pub input: SliderInputAttrs,
}

pub fn use_slider(options: SliderOptions) -> SliderAria {
    let SliderOptions {
        is_disabled,
        value,
        default_value,
        on_value_change,
        min,
        max,
        step,
        lang,
        dir,
    } = options;

    let locale = locale_attrs(lang, dir);
    let (min, max) = sanitize_bounds(min, max);
    let step = sanitize_step(step, min, max);
    let controllable = crate::use_controllable_state(value, default_value, on_value_change);
    let value_signal = controllable.value;
    let request_value_change = controllable.request_change;

    let value = Memo::new(move |_| sanitize_value(value_signal.get(), min, max, step));
    let value_percent = Memo::new(move |_| resolve_percent(value.get(), min, max));

    let press = use_press(PressOptions {
        is_disabled,
        activation_keys: PressActivationKeys::NONE,
        ..Default::default()
    });
    let hover = use_hover(HoverOptions { is_disabled });
    let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });

    let on_input = Callback::new(move |raw: String| {
        if is_disabled {
            return;
        }
        let Some(parsed) = parse_value(&raw) else {
            return;
        };
        let next = sanitize_value(parsed, min, max, step);
        request_value_change.run(next);
    });

    let on_blur = {
        let on_press_blur = press.handlers.on_blur;
        let on_focus_blur = focus_ring.handlers.on_blur;
        Callback::new(move |_| {
            on_press_blur.run(());
            on_focus_blur.run(());
        })
    };

    SliderAria {
        state: SliderState {
            is_pressed: press.is_pressed,
            is_hovered: hover.is_hovered,
            is_focused: focus_ring.is_focused,
            is_focus_visible: focus_ring.is_focus_visible,
            min,
            max,
            step,
            value,
            value_percent,
        },
        handlers: SliderHandlers {
            on_input,
            on_pointer_down: press.handlers.on_pointer_down,
            on_pointer_up: press.handlers.on_pointer_up,
            on_pointer_cancel: press.handlers.on_pointer_cancel,
            on_pointer_enter: hover.handlers.on_pointer_enter,
            on_pointer_leave: hover.handlers.on_pointer_leave,
            on_focus: focus_ring.handlers.on_focus,
            on_blur,
        },
        input: SliderInputAttrs {
            role: "slider",
            aria_disabled: is_disabled.then_some("true"),
            aria_valuemin: Memo::new(move |_| min.to_string()),
            aria_valuemax: Memo::new(move |_| max.to_string()),
            aria_valuenow: Memo::new(move |_| value.get().to_string()),
            aria_valuetext: Memo::new(move |_| format!("{:.0}%", value_percent.get())),
            lang: locale.lang,
            dir: locale.dir,
        },
    }
}

#[cfg(test)]
#[path = "test/slider.rs"]
mod tests;
