pub struct TrayMotion {
    pub sheet: crate::sheet::SheetMotion,
}

pub fn Tray(
    open: leptos::prelude::Signal<bool>,
    on_close: ui_headless::OnPress,
    id_base: String,
    title: String,
    children: leptos::children::ChildrenFn,
    description: Option<String>,
    footer: Option<leptos::children::ViewFn>,
    motion: TrayMotion,
    show_close_button: bool,
    close_label: &'static str,
    is_fixed_height: bool,
    is_dismissable: bool,
    is_keyboard_dismiss_disabled: bool,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    on_exit_complete: Option<leptos::prelude::Callback<()>>,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
