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
