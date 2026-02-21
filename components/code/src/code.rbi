pub type CodeVariant = ui_state_primitives::code::CodeVariant;

pub fn Code(
    variant: Option<CodeVariant>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::a11y::A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
