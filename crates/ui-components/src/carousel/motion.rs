fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = super::CarouselMotion::default().spring;

    ui_motion::spring::SpringConfig {
        stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0 {
            value.stiffness
        } else {
            default.stiffness
        },
        damping: if value.damping.is_finite() && value.damping > 0.0 {
            value.damping
        } else {
            default.damping
        },
        mass: if value.mass.is_finite() && value.mass > 0.0 {
            value.mass
        } else {
            default.mass
        },
        precision: if value.precision.is_finite() && value.precision > 0.0 {
            value.precision
        } else {
            default.precision
        },
    }
}

pub fn sanitize_motion(motion: super::CarouselMotion) -> super::CarouselMotion {
    super::CarouselMotion {
        spring: sanitize_spring(motion.spring),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let motion = sanitize_motion(super::super::CarouselMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
        });

        let default = super::super::CarouselMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
    }

    #[test]
    fn sanitize_motion_preserves_valid_values() {
        let motion = sanitize_motion(super::super::CarouselMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 280.0,
                damping: 24.0,
                mass: 1.0,
                precision: 0.002,
            },
        });

        assert_eq!(motion.spring.stiffness, 280.0);
        assert_eq!(motion.spring.damping, 24.0);
        assert_eq!(motion.spring.mass, 1.0);
        assert_eq!(motion.spring.precision, 0.002);
    }
}
