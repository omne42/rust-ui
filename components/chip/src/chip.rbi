pub type ChipVariant = ui_state_primitives::chip::ChipVariant;
pub type ChipSize = ui_state_primitives::chip::ChipSize;

pub struct ChipMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub enter_offset_y_px: f64,
    pub enter_scale: f64,
}

pub fn Chip(
    variant: ChipVariant,
    size: ChipSize,
    is_disabled: bool,
    on_dismiss: Option<ui_headless::OnPress>,
    motion: ChipMotion,
    dismiss_aria_label: Option<String>,
    class_name: Option<String>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
