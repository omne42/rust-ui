pub enum PopoverSlot {
    Root,
    Panel,
}

impl PopoverSlot {
    pub fn as_attr(self) -> &'static str;
    pub fn base_class(self) -> &'static str;
}

pub struct PopoverPartStateInput {
    pub slot: PopoverSlot,
    pub open: bool,
    pub is_modal: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_placement: bool,
    pub has_on_exit_complete: bool,
}

pub struct PopoverPartState {
    pub slot: PopoverSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub is_open: bool,
    pub is_modal: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_placement: bool,
    pub has_on_exit_complete: bool,
    pub modal_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub placement_source_attr: &'static str,
    pub modal_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}

pub struct PopoverMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_scale: f64,
    pub initial_y_px: f64,
}

pub fn Popover(
    open: leptos::prelude::Signal<bool>,
    anchor_ref: leptos::prelude::NodeRef<leptos::html::Button>,
    on_close: ui_headless::OnPress,
    children: leptos::children::ChildrenFn,
    placement: ui_headless::PopoverPlacement,
    motion: PopoverMotion,
    is_modal: bool,
    class_name: Option<String>,
    on_exit_complete: Option<leptos::prelude::Callback<()>>,
) -> impl leptos::prelude::IntoView;
