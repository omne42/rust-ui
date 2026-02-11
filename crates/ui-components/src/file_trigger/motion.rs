use crate::button::ButtonMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct FileTriggerMotion {
    pub trigger: ButtonMotion,
}

pub fn sanitize_motion(motion: FileTriggerMotion) -> FileTriggerMotion {
    FileTriggerMotion {
        trigger: crate::button::motion::sanitize_motion(motion.trigger),
    }
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

    #[test]
    fn sanitize_motion_delegates_to_button_contract() {
        let motion = sanitize_motion(FileTriggerMotion {
            trigger: ButtonMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: f64::NAN,
                    damping: -1.0,
                    mass: 0.0,
                    precision: f64::INFINITY,
                },
                hover_scale: 5.0,
                tap_scale: -2.0,
            },
        });

        let default = ButtonMotion::default();
        assert_eq!(motion.trigger.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.trigger.spring.damping, default.spring.damping);
        assert_eq!(motion.trigger.spring.mass, default.spring.mass);
        assert_eq!(motion.trigger.spring.precision, default.spring.precision);
        assert_eq!(motion.trigger.hover_scale, 2.0);
        assert_eq!(motion.trigger.tap_scale, 0.5);
    }
}
