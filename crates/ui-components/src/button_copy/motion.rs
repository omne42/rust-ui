use crate::button::ButtonMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ButtonCopyMotion {
    pub button: ButtonMotion,
}

pub fn sanitize_motion(motion: ButtonCopyMotion) -> ButtonCopyMotion {
    ButtonCopyMotion {
        button: crate::button::motion::sanitize_motion(motion.button),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_matches_button_contract_defaults() {
        let motion = ButtonCopyMotion::default();

        assert_eq!(motion.button, ButtonMotion::default());
        assert_eq!(motion.button.hover_scale, 1.05);
        assert_eq!(motion.button.tap_scale, 0.95);
    }

    #[test]
    fn sanitize_motion_delegates_to_button_contract() {
        let input = ButtonMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            hover_scale: f64::NAN,
            tap_scale: f64::INFINITY,
        };
        let motion = sanitize_motion(ButtonCopyMotion { button: input });
        let expected = crate::button::motion::sanitize_motion(input);

        assert_eq!(motion.button, expected);
        assert_eq!(motion.button.hover_scale, 1.05);
        assert_eq!(motion.button.tap_scale, 0.95);
    }

    #[test]
    fn supports_custom_button_motion_contract() {
        let motion = ButtonCopyMotion {
            button: ButtonMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: 288.0,
                    damping: 19.0,
                    mass: 1.0,
                    precision: 0.002,
                },
                hover_scale: 1.08,
                tap_scale: 0.93,
            },
        };

        assert_eq!(motion.button.spring.stiffness, 288.0);
        assert_eq!(motion.button.spring.damping, 19.0);
        assert_eq!(motion.button.spring.mass, 1.0);
        assert_eq!(motion.button.spring.precision, 0.002);
        assert_eq!(motion.button.hover_scale, 1.08);
        assert_eq!(motion.button.tap_scale, 0.93);
    }
}
