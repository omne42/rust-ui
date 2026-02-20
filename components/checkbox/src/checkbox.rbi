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
    checked: leptos::prelude::ReadSignal<bool>,
    set_checked: leptos::prelude::WriteSignal<bool>,
    disabled: bool,
    on_change: Option<leptos::prelude::Callback<bool>>,
    variant: CheckboxVariant,
    size: CheckboxSize,
    motion: CheckboxMotion,
    class_name: Option<String>,
    aria_label: Option<String>,
    node_ref: leptos::prelude::NodeRef<leptos::html::Button>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
