use crate::button::motion::ButtonMotion;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SearchInputButtonMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub hover_scale: f64,
    pub tap_scale: f64,
}

impl Default for SearchInputButtonMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 260.0,
                damping: 16.0,
                mass: 1.0,
                ..Default::default()
            },
            hover_scale: 1.0,
            tap_scale: 0.98,
        }
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = SearchInputButtonMotion::default().spring;
    crate::button::motion::sanitize_spring_with_fallback(value, default)
}

pub fn sanitize_motion(motion: SearchInputButtonMotion) -> SearchInputButtonMotion {
    let default = SearchInputButtonMotion::default();

    SearchInputButtonMotion {
        spring: sanitize_spring(motion.spring),
        hover_scale: crate::button::motion::sanitize_hover_scale_with_fallback(
            motion.hover_scale,
            default.hover_scale,
        ),
        tap_scale: crate::button::motion::sanitize_tap_scale_with_fallback(
            motion.tap_scale,
            default.tap_scale,
        ),
    }
}

fn as_button_motion(motion: SearchInputButtonMotion) -> ButtonMotion {
    ButtonMotion {
        spring: motion.spring,
        hover_scale: motion.hover_scale,
        tap_scale: motion.tap_scale,
    }
}

pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Button>,
    is_hovered: leptos::prelude::ReadSignal<bool>,
    is_pressed: leptos::prelude::ReadSignal<bool>,
    is_disabled: bool,
    motion: SearchInputButtonMotion,
) {
    let motion = as_button_motion(sanitize_motion(motion));
    crate::button::motion::attach_motion(node_ref, is_hovered, is_pressed, is_disabled, motion);
}

#[cfg(test)]
#[path = "../../test/search_input/motion.rs"]
mod tests;
