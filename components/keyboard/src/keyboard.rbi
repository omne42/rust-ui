pub type KeyboardTone = crate::KeyboardTone;

pub const DEFAULT_ARIA_LABEL: &str;

pub mod styles {
    pub const CSS: &str;
}

pub fn Keyboard(
    tone: Option<KeyboardTone>,
    is_compact: Option<bool>,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
