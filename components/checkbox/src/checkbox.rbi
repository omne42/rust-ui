pub enum CheckboxVariant {
    Default,
    Accent,
}

pub enum CheckboxSize {
    Default,
    Sm,
    Lg,
}

pub struct CheckboxMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub hover_scale: f64,
    pub tap_scale: f64,
    pub indicator_spring: ui_motion::spring::SpringConfig,
}

pub fn Checkbox(
    is_checked: Option<leptos::prelude::ReadSignal<bool>>,
    checked: Option<leptos::prelude::ReadSignal<bool>>,
    on_checked_change: Option<leptos::prelude::WriteSignal<bool>>,
    set_checked: Option<leptos::prelude::WriteSignal<bool>>,
    default_checked: Option<bool>,
    is_disabled: Option<bool>,
    disabled: bool,
    on_change: Option<leptos::prelude::Callback<bool>>,
    variant: CheckboxVariant,
    size: CheckboxSize,
    motion: CheckboxMotion,
    class_name: Option<String>,
    aria_label: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
