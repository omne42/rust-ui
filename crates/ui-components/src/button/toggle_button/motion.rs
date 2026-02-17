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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_has_reasonable_params() {
        let motion = ToggleButtonMotion::default();
        let button_motion = ButtonMotion::default();
        assert_eq!(motion.spring, button_motion.spring);
        assert_eq!(motion.hover_scale, button_motion.hover_scale);
        assert_eq!(motion.tap_scale, button_motion.tap_scale);
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let motion = sanitize_motion(ToggleButtonMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            hover_scale: f64::NAN,
            tap_scale: f64::NAN,
        });

        let default = ToggleButtonMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.hover_scale, default.hover_scale);
        assert_eq!(motion.tap_scale, default.tap_scale);
    }

    #[test]
    fn sanitize_motion_clamps_scale_values() {
        let motion = sanitize_motion(ToggleButtonMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 320.0,
                damping: 20.0,
                mass: 1.1,
                precision: 0.002,
            },
            hover_scale: 5.0,
            tap_scale: -2.0,
        });

        assert_eq!(motion.spring.stiffness, 320.0);
        assert_eq!(motion.spring.damping, 20.0);
        assert_eq!(motion.spring.mass, 1.1);
        assert_eq!(motion.spring.precision, 0.002);
        assert_eq!(motion.hover_scale, 2.0);
        assert_eq!(motion.tap_scale, 0.5);
    }
}
