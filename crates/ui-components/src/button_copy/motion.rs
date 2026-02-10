use crate::button::ButtonMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ButtonCopyMotion {
    pub button: ButtonMotion,
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
