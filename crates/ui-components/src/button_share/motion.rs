use crate::button_flip::FlipButtonMotion;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ShareButtonMotion {
    pub flip: FlipButtonMotion,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_matches_flip_button_defaults() {
        let motion = ShareButtonMotion::default();

        assert_eq!(motion.flip, FlipButtonMotion::default());
    }

    #[test]
    fn supports_custom_flip_motion_contract() {
        let motion = ShareButtonMotion {
            flip: FlipButtonMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: 294.0,
                    damping: 20.0,
                    mass: 1.0,
                    precision: 0.002,
                },
            },
        };

        assert_eq!(motion.flip.spring.stiffness, 294.0);
        assert_eq!(motion.flip.spring.damping, 20.0);
        assert_eq!(motion.flip.spring.mass, 1.0);
        assert_eq!(motion.flip.spring.precision, 0.002);
    }
}
