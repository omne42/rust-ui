pub struct BottomSheetMotion {
    pub sheet: crate::sheet::SheetMotion,
}

pub fn BottomSheet(
    open: leptos::prelude::Signal<bool>,
    on_close: crate::OnPress,
    id_base: String,
    title: String,
    children: leptos::children::ChildrenFn,
    description: Option<String>,
    footer: Option<leptos::children::ViewFn>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    motion: BottomSheetMotion,
    is_handle_visible: Option<bool>,
    is_close_button_visible: Option<bool>,
    close_label: Option<&'static str>,
    is_detached: Option<bool>,
    bottom_inset_px: Option<f64>,
    is_dismissable: Option<bool>,
    is_keyboard_dismiss_disabled: Option<bool>,
    show_handle: Option<bool>,
    show_close_button: Option<bool>,
    detached: Option<bool>,
    on_exit_complete: Option<leptos::prelude::Callback<()>>,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
