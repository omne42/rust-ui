pub enum StatusLightVariant {
    Default,
    Accent,
    Danger,
}

pub enum StatusLightRole {
    Status,
}

pub fn StatusLight(
    variant: Option<StatusLightVariant>,
    role: Option<StatusLightRole>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
