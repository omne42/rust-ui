use crate::button::ButtonMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct FileTriggerMotion {
    pub trigger: ButtonMotion,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_uses_default_button_motion_contract() {
        let motion = FileTriggerMotion::default();

        assert_eq!(motion.trigger, ButtonMotion::default());
    }

    #[test]
    fn supports_custom_button_motion_contract() {
        let motion = FileTriggerMotion {
            trigger: ButtonMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: 276.0,
                    damping: 17.0,
                    mass: 1.0,
                    precision: 0.002,
                },
                hover_scale: 1.04,
                tap_scale: 0.94,
            },
        };

        assert_eq!(motion.trigger.spring.stiffness, 276.0);
        assert_eq!(motion.trigger.spring.damping, 17.0);
        assert_eq!(motion.trigger.spring.mass, 1.0);
        assert_eq!(motion.trigger.spring.precision, 0.002);
        assert_eq!(motion.trigger.hover_scale, 1.04);
        assert_eq!(motion.trigger.tap_scale, 0.94);
    }
}
