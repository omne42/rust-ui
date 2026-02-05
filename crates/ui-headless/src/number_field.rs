use leptos::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct NumberFieldOptions {
    pub is_disabled: bool,
    pub value: Signal<i64>,
    pub on_value_change: Callback<i64>,
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub step: i64,
}

#[derive(Clone)]
pub struct NumberFieldInputAttrs {
    pub role: &'static str,
    pub aria_valuenow: Memo<String>,
    pub aria_valuemin: Option<String>,
    pub aria_valuemax: Option<String>,
    pub aria_disabled: Option<&'static str>,
}

#[derive(Clone)]
pub struct NumberFieldHandlers {
    pub on_focus: Callback<()>,
    pub on_blur: Callback<()>,
    pub on_input: Callback<String>,
    /// Returns `true` when default should be prevented (ArrowUp/ArrowDown/PageUp/PageDown/Home/End).
    pub on_key_down: Callback<String, bool>,
    pub increment: Callback<()>,
    pub decrement: Callback<()>,
}

#[derive(Clone)]
pub struct NumberFieldAria {
    pub input_value: ReadSignal<String>,
    pub is_editing: ReadSignal<bool>,
    pub input: NumberFieldInputAttrs,
    pub handlers: NumberFieldHandlers,
}

pub fn use_number_field(options: NumberFieldOptions) -> NumberFieldAria {
    let min = options.min;
    let max = options.max;
    let step = options.step.max(1);

    let (input_value, set_input_value) = signal(options.value.get_untracked().to_string());
    let (is_editing, set_is_editing) = signal(false);

    let set_value_clamped = Callback::new({
        let on_value_change = options.on_value_change;
        let value = options.value;
        move |next: i64| {
            let next = clamp_i64(next, min, max);
            if next == value.get_untracked() {
                return;
            }
            on_value_change.run(next);
        }
    });

    let commit_input_value = Callback::new({
        let value = options.value;
        move |_| {
            let raw = input_value.get_untracked();
            let Some(parsed) = parse_i64(&raw) else {
                set_input_value.set(value.get_untracked().to_string());
                return;
            };
            let next = clamp_i64(parsed, min, max);
            set_value_clamped.run(next);
            set_input_value.set(next.to_string());
        }
    });

    Effect::new({
        let value = options.value;
        move |_| {
            let next = value.get();
            if is_editing.get() {
                return;
            }
            set_input_value.set(next.to_string());
        }
    });

    let increment = Callback::new({
        let value = options.value;
        move |_| {
            if options.is_disabled {
                return;
            }
            let next = step_i64(value.get_untracked(), 1, step, min, max);
            set_value_clamped.run(next);
            set_input_value.set(next.to_string());
        }
    });

    let decrement = Callback::new({
        let value = options.value;
        move |_| {
            if options.is_disabled {
                return;
            }
            let next = step_i64(value.get_untracked(), -1, step, min, max);
            set_value_clamped.run(next);
            set_input_value.set(next.to_string());
        }
    });

    let on_focus = Callback::new(move |_| set_is_editing.set(true));
    let on_blur = Callback::new({
        move |_| {
            set_is_editing.set(false);
            commit_input_value.run(());
        }
    });

    let on_input = Callback::new(move |raw: String| {
        if options.is_disabled {
            return;
        }
        set_input_value.set(raw.clone());
        if let Some(parsed) = parse_i64(&raw) {
            set_value_clamped.run(parsed);
        }
    });

    let on_key_down = Callback::new(move |key: String| -> bool {
        if options.is_disabled {
            return false;
        }

        match key.as_str() {
            "ArrowUp" => {
                increment.run(());
                true
            }
            "ArrowDown" => {
                decrement.run(());
                true
            }
            "PageUp" => {
                let next = step_i64(options.value.get_untracked(), 10, step, min, max);
                set_value_clamped.run(next);
                set_input_value.set(next.to_string());
                true
            }
            "PageDown" => {
                let next = step_i64(options.value.get_untracked(), -10, step, min, max);
                set_value_clamped.run(next);
                set_input_value.set(next.to_string());
                true
            }
            "Home" => {
                let Some(min) = min else {
                    return false;
                };
                set_value_clamped.run(min);
                set_input_value.set(min.to_string());
                true
            }
            "End" => {
                let Some(max) = max else {
                    return false;
                };
                set_value_clamped.run(max);
                set_input_value.set(max.to_string());
                true
            }
            _ => false,
        }
    });

    let aria_valuenow = Memo::new({
        let value = options.value;
        move |_| value.get().to_string()
    });

    NumberFieldAria {
        input_value,
        is_editing,
        input: NumberFieldInputAttrs {
            role: "spinbutton",
            aria_valuenow,
            aria_valuemin: min.map(|v| v.to_string()),
            aria_valuemax: max.map(|v| v.to_string()),
            aria_disabled: options.is_disabled.then_some("true"),
        },
        handlers: NumberFieldHandlers {
            on_focus,
            on_blur,
            on_input,
            on_key_down,
            increment,
            decrement,
        },
    }
}

fn clamp_i64(value: i64, min: Option<i64>, max: Option<i64>) -> i64 {
    let mut v = value;
    if let Some(min) = min {
        v = v.max(min);
    }
    if let Some(max) = max {
        v = v.min(max);
    }
    v
}

fn step_i64(value: i64, delta_steps: i64, step: i64, min: Option<i64>, max: Option<i64>) -> i64 {
    let step = step.max(1);
    let delta = delta_steps.saturating_mul(step);
    clamp_i64(value.saturating_add(delta), min, max)
}

fn parse_i64(text: &str) -> Option<i64> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.parse::<i64>().ok()
}
