pub enum AlertLayout {
    Banner,
    Inline,
}

pub enum AlertVariant {
    Default,
    Accent,
    Danger,
}

pub type AlertTone = ui_state_primitives::alert_banner::AlertBannerTone;
pub type AlertFill = ui_state_primitives::alert_banner::AlertBannerFill;

pub struct AlertMotion {
    pub spring: ui_motion::spring::SpringConfig,
}

pub fn Alert(
    tone: Option<AlertTone>,
    variant: Option<AlertVariant>,
    layout: Option<AlertLayout>,
    fill: Option<AlertFill>,
    title: Option<String>,
    description: Option<String>,
    is_hide_icon: Option<bool>,
    hide_icon: Option<bool>,
    icon_label: Option<String>,
    start_content: Option<leptos::children::ViewFn>,
    end_content: Option<leptos::children::ViewFn>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    motion: AlertMotion,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
