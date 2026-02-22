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
    let default = super::super::CarouselMotion::default();
    let custom_spring = ui_motion::spring::SpringConfig {
        stiffness: default.spring.stiffness + 20.0,
        damping: default.spring.damping + 4.0,
        mass: default.spring.mass,
        precision: default.spring.precision * 2.0,
    };
    let motion = sanitize_motion(super::super::CarouselMotion {
        spring: custom_spring,
    });

    assert_eq!(motion.spring.stiffness, custom_spring.stiffness);
    assert_eq!(motion.spring.damping, custom_spring.damping);
    assert_eq!(motion.spring.mass, custom_spring.mass);
    assert_eq!(motion.spring.precision, custom_spring.precision);
}
