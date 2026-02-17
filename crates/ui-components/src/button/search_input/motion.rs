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
mod tests {
    use super::*;

    #[test]
    fn default_motion_matches_search_input_button_spring_contract() {
        let motion = SearchInputButtonMotion::default();

        assert_eq!(
            motion.spring,
            ui_motion::spring::SpringConfig {
                stiffness: 260.0,
                damping: 16.0,
                mass: 1.0,
                ..Default::default()
            }
        );
        assert_eq!(motion.hover_scale, 1.0);
        assert_eq!(motion.tap_scale, 0.98);
    }

    #[test]
    fn supports_custom_search_input_button_motion_contract() {
        let motion = SearchInputButtonMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 284.0,
                damping: 18.0,
                mass: 1.0,
                precision: 0.002,
            },
            hover_scale: 1.03,
            tap_scale: 0.95,
        };

        assert_eq!(motion.spring.stiffness, 284.0);
        assert_eq!(motion.spring.damping, 18.0);
        assert_eq!(motion.spring.mass, 1.0);
        assert_eq!(motion.spring.precision, 0.002);
        assert_eq!(motion.hover_scale, 1.03);
        assert_eq!(motion.tap_scale, 0.95);
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let motion = sanitize_motion(SearchInputButtonMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            hover_scale: f64::NAN,
            tap_scale: f64::NAN,
        });

        let default = SearchInputButtonMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.hover_scale, default.hover_scale);
        assert_eq!(motion.tap_scale, default.tap_scale);
    }

    #[test]
    fn sanitize_motion_clamps_scale_values() {
        let motion = sanitize_motion(SearchInputButtonMotion {
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
