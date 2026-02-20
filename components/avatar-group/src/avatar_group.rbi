pub struct AvatarGroupItem {
    pub name: Option<String>,
    pub src: Option<String>,
    pub alt: Option<String>,
}

pub fn AvatarGroup(
    items: Vec<AvatarGroupItem>,
    max: Option<usize>,
    size: ui_avatar::AvatarSize,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
