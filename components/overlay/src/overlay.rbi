pub enum OverlaySlot {
    Root,
    Backdrop,
    Panel,
}

impl OverlaySlot {
    pub fn as_attr(self) -> &'static str;
    pub fn base_class(self) -> &'static str;
}

pub struct OverlayPartStateInput {
    pub slot: OverlaySlot,
    pub open: bool,
    pub is_dismissable: bool,
    pub is_keyboard_dismiss_disabled: bool,
    pub has_custom_role: bool,
    pub has_custom_aria_labelledby: bool,
    pub has_custom_aria_describedby: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

pub struct OverlayPartState {
    pub slot: OverlaySlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub is_open: bool,
    pub is_dismissable: bool,
    pub is_keyboard_dismiss_disabled: bool,
    pub has_custom_role: bool,
    pub has_custom_aria_labelledby: bool,
    pub has_custom_aria_describedby: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
    pub dismiss_attr: &'static str,
    pub keyboard_dismiss_attr: &'static str,
    pub role_source_attr: &'static str,
    pub aria_labelledby_source_attr: &'static str,
    pub aria_describedby_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub dismiss_source_attr: &'static str,
    pub keyboard_dismiss_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}

pub struct OverlayMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_scale: f64,
    pub initial_y_px: f64,
}

pub fn Overlay(
    open: leptos::prelude::Signal<bool>,
    on_close: ui_headless::OnPress,
    children: leptos::children::ChildrenFn,
    aria_labelledby: Option<String>,
    aria_describedby: Option<String>,
    role: &'static str,
    is_dismissable: bool,
    is_keyboard_dismiss_disabled: bool,
    motion: OverlayMotion,
    class_name: Option<String>,
    on_exit_complete: Option<leptos::prelude::Callback<()>>,
) -> impl leptos::prelude::IntoView;
