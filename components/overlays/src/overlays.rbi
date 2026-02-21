pub struct OverlaysRootStateInput {
    pub open: bool,
    pub modal: bool,
    pub has_custom_id_base: bool,
    pub has_custom_class_name: bool,
}

pub struct OverlaysRootState {
    pub layer_kind_class: &'static str,
    pub layer_kind_attr: &'static str,
    pub data_state_attr: &'static str,
    pub id_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub is_open: bool,
    pub is_closed: bool,
    pub has_custom_id_base: bool,
    pub has_custom_class_name: bool,
}

pub struct OverlaysMotion {
    pub overlay: crate::overlay::OverlayMotion,
    pub popover: crate::popover::PopoverMotion,
    pub tray: crate::tray::TrayMotion,
}

pub fn resolve_root_state(input: OverlaysRootStateInput) -> OverlaysRootState;
pub fn sanitize_motion(motion: OverlaysMotion) -> OverlaysMotion;

pub fn OverlaysRoot(
    id_base: Option<String>,
    is_open: bool,
    is_modal: bool,
    aria_label: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    class_name: Option<String>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
