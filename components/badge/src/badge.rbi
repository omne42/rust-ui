pub type BadgeVariant = ui_state_primitives::badge::BadgeVariant;

pub struct BadgeMotion {
    pub enter_ms: u16,
    pub exit_ms: u16,
    pub reduced_ms: u16,
}

pub fn Badge(
    variant: Option<BadgeVariant>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
