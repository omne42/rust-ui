pub struct TrayMotion {
    pub sheet: crate::sheet::SheetMotion,
}

pub fn Tray(
    is_open: Option<leptos::prelude::Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<leptos::prelude::Callback<bool>>,
    on_close: Option<ui_headless::OnPress>,
    id_base: String,
    title: String,
    children: leptos::children::ChildrenFn,
    description: Option<String>,
    footer: Option<leptos::children::ViewFn>,
    motion: Option<TrayMotion>,
    is_show_close_button: Option<bool>,
    close_label: Option<&'static str>,
    is_fixed_height: Option<bool>,
    is_dismissable: Option<bool>,
    is_keyboard_dismiss_disabled: Option<bool>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    on_exit_complete: Option<leptos::prelude::Callback<()>>,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
