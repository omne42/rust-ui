use crate::button::motion::ButtonMotion;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToggleButtonMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub hover_scale: f64,
    pub tap_scale: f64,
}

impl Default for ToggleButtonMotion {
    fn default() -> Self {
        let button_motion = ButtonMotion::default();
        Self {
            spring: button_motion.spring,
            hover_scale: button_motion.hover_scale,
            tap_scale: button_motion.tap_scale,
        }
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    crate::button::motion::sanitize_spring_with_fallback(value, ButtonMotion::default().spring)
}

pub fn sanitize_motion(motion: ToggleButtonMotion) -> ToggleButtonMotion {
    let defaults = ButtonMotion::default();
    let sanitized_scales = crate::button::motion::sanitize_motion(ButtonMotion {
        spring: defaults.spring,
        hover_scale: motion.hover_scale,
        tap_scale: motion.tap_scale,
    });

    ToggleButtonMotion {
        spring: sanitize_spring(motion.spring),
        hover_scale: sanitized_scales.hover_scale,
        tap_scale: sanitized_scales.tap_scale,
    }
}

fn as_button_motion(motion: ToggleButtonMotion) -> ButtonMotion {
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
    motion: ToggleButtonMotion,
) {
    let motion = as_button_motion(sanitize_motion(motion));
    crate::button::motion::attach_motion(node_ref, is_hovered, is_pressed, is_disabled, motion);
}

#[cfg(feature = "component-toggle_button_group")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToggleButtonGroupMotion {
    pub duration_ms: f64,
}

#[cfg(feature = "component-toggle_button_group")]
impl Default for ToggleButtonGroupMotion {
    fn default() -> Self {
        Self { duration_ms: 160.0 }
    }
}

#[cfg(feature = "component-toggle_button_group")]
pub fn sanitize_toggle_button_group_motion(
    motion: ToggleButtonGroupMotion,
) -> ToggleButtonGroupMotion {
    let default = ToggleButtonGroupMotion::default();
    let duration_ms = if motion.duration_ms.is_finite() {
        motion.duration_ms
    } else {
        default.duration_ms
    };

    ToggleButtonGroupMotion {
        duration_ms: duration_ms.clamp(1.0, 1000.0),
    }
}

#[cfg(feature = "component-toggle_button_group")]
pub fn attach_toggle_button_group_motion(motion: ToggleButtonGroupMotion) -> String {
    let motion = sanitize_toggle_button_group_motion(motion);
    format!(
        "--ui-toggle-button-group-motion-duration: {}ms;",
        motion.duration_ms
    )
}

#[cfg(test)]
#[path = "../../test/toggle_button/motion.rs"]
mod tests;
