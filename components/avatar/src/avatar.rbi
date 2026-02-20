pub enum AvatarSize {
    Sm,
    Md,
    Lg,
}

pub fn Avatar(
    name: Option<String>,
    src: Option<String>,
    size: AvatarSize,
    alt: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
