pub type KbdSize = ui_state_primitives::kbd::KbdSize;

pub mod styles {
    pub const CSS: &str;
}

pub fn Kbd(
    size: Option<KbdSize>,
    keys: Option<String>,
    class_name: Option<String>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
