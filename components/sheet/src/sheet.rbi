pub enum SheetPlacement {
    Bottom,
    Left,
    Right,
}

pub struct SheetMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_opacity: f64,
    pub initial_y_px: f64,
    pub initial_x_px: f64,
}

pub fn Sheet(
    open: leptos::prelude::Signal<bool>,
    on_close: ui_headless::OnPress,
    children: leptos::children::ChildrenFn,
    placement: SheetPlacement,
    aria_labelledby: Option<String>,
    aria_describedby: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    is_dismissable: bool,
    is_keyboard_dismiss_disabled: bool,
    motion: SheetMotion,
    on_exit_complete: Option<leptos::prelude::Callback<()>>,
) -> impl leptos::prelude::IntoView;
